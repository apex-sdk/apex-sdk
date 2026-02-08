//! Secure keystore implementation for managing accounts
//!
//! # Security Features
//!
//! - AES-256-GCM authenticated encryption
//! - Argon2id password-based key derivation (OWASP recommended parameters)
//! - Memory zeroing for sensitive data
//! - Rate limiting and failed attempt tracking
//! - Password strength validation
//! - Restricted file permissions (Unix: 0o600)
//!
//! # Threat Model
//!
//! Protects against:
//! - Offline brute-force attacks (via Argon2)
//! - Memory dumps (via zeroizing)
//! - Unauthorized file access (via permissions)
//! - Weak passwords (via validation)
//! - Online brute-force (via rate limiting)
//!
//! Does NOT protect against:
//! - Malicious code with same user privileges
//! - Keyloggers or memory scanners while keys are in use
//! - Physical access to unlocked system

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use anyhow::{Context, Result};
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2, ParamsBuilder, Version,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use zeroize::Zeroize;

const NONCE_SIZE: usize = 12;
const KEYSTORE_VERSION: u32 = 1;

// OWASP recommended Argon2 parameters (2023)
const ARGON2_MEM_COST: u32 = 19 * 1024; // 19 MiB
const ARGON2_TIME_COST: u32 = 2; // 2 iterations
const ARGON2_PARALLELISM: u32 = 1; // Single thread

// Rate limiting
const MAX_FAILED_ATTEMPTS: u32 = 5;
const LOCKOUT_DURATION_SECS: u64 = 300; // 5 minutes

// Password requirements
const MIN_PASSWORD_LENGTH: usize = 12;
const REQUIRE_UPPERCASE: bool = true;
const REQUIRE_LOWERCASE: bool = true;
const REQUIRE_DIGIT: bool = true;
const REQUIRE_SPECIAL: bool = false; // Optional for better UX

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedAccount {
    pub name: String,
    pub account_type: AccountType,
    pub address: String,
    pub encrypted_data: Vec<u8>,
    pub nonce: Vec<u8>,
    pub salt: Vec<u8>,
    pub created_at: u64,
    #[serde(default)]
    pub encryption_version: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AccountType {
    Substrate,
    Revive,
}

impl std::fmt::Display for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountType::Substrate => write!(f, "substrate"),
            AccountType::Revive => write!(f, "revive"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Keystore {
    pub accounts: Vec<EncryptedAccount>,
    #[serde(default = "default_version")]
    pub version: u32,
    #[serde(skip)]
    failed_attempts: HashMap<String, Vec<Instant>>,
}

fn default_version() -> u32 {
    KEYSTORE_VERSION
}

impl Default for Keystore {
    fn default() -> Self {
        Self {
            accounts: Vec::new(),
            version: KEYSTORE_VERSION,
            failed_attempts: HashMap::new(),
        }
    }
}

impl Keystore {
    /// Load keystore from disk
    pub fn load(path: &Path) -> Result<Self> {
        if path.exists() {
            let data = std::fs::read_to_string(path).context("Failed to read keystore file")?;
            let keystore: Keystore =
                serde_json::from_str(&data).context("Failed to parse keystore file")?;
            Ok(keystore)
        } else {
            Ok(Self::default())
        }
    }

    /// Save keystore to disk
    pub fn save(&self, path: &Path) -> Result<()> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let data = serde_json::to_string_pretty(self).context("Failed to serialize keystore")?;
        std::fs::write(path, data).context("Failed to write keystore file")?;

        // Set restrictive permissions on Unix systems
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(path)?.permissions();
            perms.set_mode(0o600); // Read/write for owner only
            std::fs::set_permissions(path, perms)?;
        }

        Ok(())
    }

    /// Validate password strength
    fn validate_password(password: &str) -> Result<()> {
        if password.len() < MIN_PASSWORD_LENGTH {
            anyhow::bail!(
                "Password must be at least {} characters long (current: {})",
                MIN_PASSWORD_LENGTH,
                password.len()
            );
        }

        if REQUIRE_UPPERCASE && !password.chars().any(|c| c.is_uppercase()) {
            anyhow::bail!("Password must contain at least one uppercase letter");
        }

        if REQUIRE_LOWERCASE && !password.chars().any(|c| c.is_lowercase()) {
            anyhow::bail!("Password must contain at least one lowercase letter");
        }

        if REQUIRE_DIGIT && !password.chars().any(|c| c.is_numeric()) {
            anyhow::bail!("Password must contain at least one digit");
        }

        if REQUIRE_SPECIAL && !password.chars().any(|c| !c.is_alphanumeric()) {
            anyhow::bail!("Password must contain at least one special character");
        }

        // Check against common weak passwords
        const WEAK_PASSWORDS: &[&str] = &[
            "password123",
            "123456789",
            "qwerty123",
            "admin123",
            "letmein123",
            "welcome123",
        ];

        if WEAK_PASSWORDS.contains(&password.to_lowercase().as_str()) {
            anyhow::bail!("Password is too common. Please choose a stronger password.");
        }

        Ok(())
    }

    /// Check if account is locked out due to failed attempts
    fn is_locked_out(&mut self, account_name: &str) -> bool {
        let now = Instant::now();

        // Clean up old failed attempts
        if let Some(attempts) = self.failed_attempts.get_mut(account_name) {
            attempts
                .retain(|&t| now.duration_since(t) < Duration::from_secs(LOCKOUT_DURATION_SECS));

            attempts.len() >= MAX_FAILED_ATTEMPTS as usize
        } else {
            false
        }
    }

    /// Record a failed decryption attempt
    fn record_failed_attempt(&mut self, account_name: &str) {
        let attempts = self
            .failed_attempts
            .entry(account_name.to_string())
            .or_default();
        attempts.push(Instant::now());
    }

    /// Clear failed attempts for an account (on successful access)
    fn clear_failed_attempts(&mut self, account_name: &str) {
        self.failed_attempts.remove(account_name);
    }

    /// Encrypt and add an account
    pub fn add_account(
        &mut self,
        name: String,
        account_type: AccountType,
        address: String,
        secret_data: &[u8],
        password: &str,
    ) -> Result<()> {
        // Validate password strength
        Self::validate_password(password)?;

        // Check if account name already exists
        if self.accounts.iter().any(|a| a.name == name) {
            anyhow::bail!("Account with name '{}' already exists", name);
        }

        let (encrypted_data, nonce, salt) = encrypt_data(secret_data, password)?;

        let account = EncryptedAccount {
            name,
            account_type,
            address,
            encrypted_data,
            nonce,
            salt,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .context("Failed to get system time")?
                .as_secs(),
            encryption_version: KEYSTORE_VERSION,
        };

        self.accounts.push(account);
        Ok(())
    }

    /// Decrypt and retrieve account data
    pub fn get_account(&mut self, name: &str, password: &str) -> Result<Vec<u8>> {
        // Check for lockout
        if self.is_locked_out(name) {
            anyhow::bail!(
                "Account '{}' is temporarily locked due to too many failed attempts. \
                 Please wait {} minutes before trying again.",
                name,
                LOCKOUT_DURATION_SECS / 60
            );
        }

        let account = self
            .accounts
            .iter()
            .find(|a| a.name == name)
            .ok_or_else(|| anyhow::anyhow!("Account '{}' not found", name))?;

        // Attempt decryption
        match decrypt_data(
            &account.encrypted_data,
            &account.nonce,
            &account.salt,
            password,
        ) {
            Ok(data) => {
                // Success - clear failed attempts
                self.clear_failed_attempts(name);
                Ok(data)
            }
            Err(_e) => {
                // Failed - record attempt
                self.record_failed_attempt(name);

                // Check if now locked out
                let remaining_attempts = MAX_FAILED_ATTEMPTS.saturating_sub(
                    self.failed_attempts.get(name).map(|v| v.len()).unwrap_or(0) as u32,
                );

                if remaining_attempts > 0 {
                    Err(anyhow::anyhow!(
                        "Incorrect password. {} attempt(s) remaining before lockout.",
                        remaining_attempts
                    ))
                } else {
                    Err(anyhow::anyhow!(
                        "Incorrect password. Account is now locked for {} minutes.",
                        LOCKOUT_DURATION_SECS / 60
                    ))
                }
            }
        }
    }

    /// List all account names
    pub fn list_accounts(&self) -> Vec<&EncryptedAccount> {
        self.accounts.iter().collect()
    }

    /// Remove an account
    pub fn remove_account(&mut self, name: &str) -> Result<()> {
        let initial_len = self.accounts.len();
        self.accounts.retain(|a| a.name != name);

        if self.accounts.len() == initial_len {
            anyhow::bail!("Account '{}' not found", name);
        }

        Ok(())
    }

    /// Check if account exists
    pub fn has_account(&self, name: &str) -> bool {
        self.accounts.iter().any(|a| a.name == name)
    }
}

/// Derive encryption key from password using Argon2 with a given salt
///
/// Uses OWASP recommended Argon2id parameters (2023):
/// - Memory cost: 19 MiB
/// - Time cost: 2 iterations
/// - Parallelism: 1 thread
fn derive_key(password: &str, salt: &SaltString) -> Result<[u8; 32]> {
    // Configure Argon2 with OWASP recommended parameters
    let params = ParamsBuilder::new()
        .m_cost(ARGON2_MEM_COST)
        .t_cost(ARGON2_TIME_COST)
        .p_cost(ARGON2_PARALLELISM)
        .build()
        .map_err(|e| anyhow::anyhow!("Failed to build Argon2 parameters: {}", e))?;

    let argon2 = Argon2::new(
        argon2::Algorithm::Argon2id, // Most secure variant
        Version::V0x13,              // Latest version
        params,
    );

    let password_hash = argon2
        .hash_password(password.as_bytes(), salt)
        .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e))?;

    let hash = password_hash
        .hash
        .ok_or_else(|| anyhow::anyhow!("Failed to extract hash"))?;

    let mut key = [0u8; 32];
    let hash_bytes = hash.as_bytes();
    let len = std::cmp::min(32, hash_bytes.len());
    key[..len].copy_from_slice(&hash_bytes[..len]);

    Ok(key)
}

/// Encrypt data using AES-256-GCM
fn encrypt_data(data: &[u8], password: &str) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
    // Generate salt
    let salt = SaltString::generate(&mut OsRng);
    let salt_bytes = salt.as_str().as_bytes().to_vec();

    let mut key = derive_key(password, &salt)?;
    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|e| anyhow::anyhow!("Failed to create cipher: {}", e))?;

    // Generate random nonce
    let mut nonce_bytes = [0u8; NONCE_SIZE];
    use ::rand::RngCore;
    ::rand::rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from(nonce_bytes);

    let encrypted = cipher
        .encrypt(&nonce, data)
        .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

    // Zeroize key after use
    key.zeroize();

    Ok((encrypted, nonce_bytes.to_vec(), salt_bytes))
}

/// Decrypt data using AES-256-GCM
fn decrypt_data(
    encrypted: &[u8],
    nonce_bytes: &[u8],
    salt_bytes: &[u8],
    password: &str,
) -> Result<Vec<u8>> {
    // Reconstruct salt
    let salt_str = std::str::from_utf8(salt_bytes).context("Invalid salt encoding")?;
    let salt = SaltString::from_b64(salt_str)
        .map_err(|e| anyhow::anyhow!("Failed to parse salt: {}", e))?;

    let mut key = derive_key(password, &salt)?;
    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|e| anyhow::anyhow!("Failed to create cipher: {}", e))?;

    let nonce_array: [u8; NONCE_SIZE] = nonce_bytes
        .try_into()
        .map_err(|_| anyhow::anyhow!("Invalid nonce length"))?;
    let nonce = Nonce::from(nonce_array);

    let decrypted = cipher
        .decrypt(&nonce, encrypted)
        .map_err(|_| anyhow::anyhow!("Decryption failed - incorrect password or corrupted data"))?;

    // Zeroize key after use
    key.zeroize();

    Ok(decrypted)
}

/// Get the default keystore path
pub fn get_keystore_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;
    Ok(config_dir.join("apex-sdk").join("keystore.json"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let data = b"secret private key data";
        let password = "test_password_123";

        let (encrypted, nonce, salt) = encrypt_data(data, password).unwrap();
        assert_ne!(encrypted.as_slice(), data);

        let decrypted = decrypt_data(&encrypted, &nonce, &salt, password).unwrap();
        assert_eq!(decrypted, data);
    }

    #[test]
    fn test_decrypt_wrong_password() {
        let data = b"secret data";
        let password = "correct_password";
        let wrong_password = "wrong_password";

        let (encrypted, nonce, salt) = encrypt_data(data, password).unwrap();
        let result = decrypt_data(&encrypted, &nonce, &salt, wrong_password);

        assert!(result.is_err());
    }

    #[test]
    fn test_keystore_add_get() {
        let mut keystore = Keystore::default();
        let password = "TestPassword123";

        keystore
            .add_account(
                "test_account".to_string(),
                AccountType::Substrate,
                "5GrwvaEF...".to_string(),
                b"private_key_data",
                password,
            )
            .unwrap();

        let retrieved = keystore.get_account("test_account", password).unwrap();
        assert_eq!(retrieved, b"private_key_data");
    }

    #[test]
    fn test_keystore_duplicate_name() {
        let mut keystore = Keystore::default();
        let password = "TestPassword123";

        keystore
            .add_account(
                "test".to_string(),
                AccountType::Substrate,
                "addr1".to_string(),
                b"data1",
                password,
            )
            .unwrap();

        let result = keystore.add_account(
            "test".to_string(),
            AccountType::Revive,
            "addr2".to_string(),
            b"data2",
            password,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_keystore_remove() {
        let mut keystore = Keystore::default();
        let password = "TestPassword123";

        keystore
            .add_account(
                "test".to_string(),
                AccountType::Substrate,
                "addr".to_string(),
                b"data",
                password,
            )
            .unwrap();

        assert!(keystore.has_account("test"));
        keystore.remove_account("test").unwrap();
        assert!(!keystore.has_account("test"));
    }
}
