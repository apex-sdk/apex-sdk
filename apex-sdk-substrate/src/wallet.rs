//! Substrate wallet and account management
//!
//! This module provides comprehensive wallet functionality including:
//! - Key pair generation (SR25519, ED25519)
//! - Mnemonic phrase support (BIP-39)
//! - SS58 address encoding
//! - Message and transaction signing
//! - Multi-wallet management
//!
//! # Security
//!
//! This module handles sensitive cryptographic material (private keys).
//! The underlying `sp_core::Pair` types implement secure memory handling.
//! For additional safety:
//! - Avoid unnecessary cloning of wallet instances
//! - Use `Arc<Wallet>` for shared access
//! - Ensure wallets are dropped when no longer needed

use crate::{Error, Result};
use apex_sdk_core::{SdkError, Signer as CoreSigner};
use apex_sdk_types::Address;
use async_trait::async_trait;
use parking_lot::RwLock;
use sp_core::crypto::{Ss58AddressFormat, Ss58Codec};
use sp_core::{ed25519, sr25519, Pair as PairTrait};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, info};
use zeroize::Zeroize;

/// Supported key pair types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KeyPairType {
    /// SR25519 (Schnorrkel) - Default for Substrate
    #[default]
    Sr25519,
    /// ED25519 - Alternative signing algorithm
    Ed25519,
}

/// A unified wallet that can hold either SR25519 or ED25519 keys
///
/// # Security
///
/// This struct handles sensitive cryptographic material and implements
/// secure memory practices:
///
/// - **Cloning**: This struct implements `Clone` to support wallet management.
///   Cloning duplicates private key material in memory. For shared access
///   without duplication, wrap in `Arc<Wallet>`.
///
/// - **Memory Zeroing**: The underlying `sp_core::Pair` types (SR25519 and ED25519)
///   implement secure memory clearing on drop. However, cloned instances each
///   maintain their own copy until dropped.
///
/// - **Best Practices**:
///   - Minimize the lifetime of wallet instances
///   - Use `Arc<Wallet>` for shared access
///   - Avoid logging or printing wallet contents
///   - Drop wallets explicitly when no longer needed
///
/// # Example
///
/// ```rust
/// use apex_sdk_substrate::wallet::{Wallet, KeyPairType};
/// use std::sync::Arc;
///
/// // For single ownership
/// let wallet = Wallet::new_random();
/// // Use wallet...
/// drop(wallet); // Explicitly drop when done
///
/// // For shared access (recommended)
/// let wallet = Arc::new(Wallet::new_random());
/// let wallet_clone = Arc::clone(&wallet); // Increments reference count, no key duplication
/// ```
#[derive(Clone)]
pub struct Wallet {
    /// The key pair type
    key_type: KeyPairType,
    /// SR25519 pair (if applicable)
    sr25519_pair: Option<sr25519::Pair>,
    /// ED25519 pair (if applicable)
    ed25519_pair: Option<ed25519::Pair>,
    /// SS58 address format (network prefix)
    ss58_format: Ss58AddressFormat,
}

impl Wallet {
    /// Create a new random wallet with SR25519 keys
    pub fn new_random() -> Self {
        Self::new_random_with_type(KeyPairType::Sr25519)
    }

    /// Create a new random wallet with specified key type
    pub fn new_random_with_type(key_type: KeyPairType) -> Self {
        info!("Creating new random {:?} wallet", key_type);

        match key_type {
            KeyPairType::Sr25519 => {
                let (pair, _seed) = sr25519::Pair::generate();
                Self {
                    key_type,
                    sr25519_pair: Some(pair),
                    ed25519_pair: None,
                    ss58_format: Ss58AddressFormat::custom(42), // Default to generic
                }
            }
            KeyPairType::Ed25519 => {
                let (pair, _seed) = ed25519::Pair::generate();
                Self {
                    key_type,
                    sr25519_pair: None,
                    ed25519_pair: Some(pair),
                    ss58_format: Ss58AddressFormat::custom(42),
                }
            }
        }
    }

    /// Create wallet from mnemonic phrase
    pub fn from_mnemonic(mnemonic: &str, key_type: KeyPairType) -> Result<Self> {
        Self::from_mnemonic_with_path(mnemonic, None, key_type)
    }

    /// Create wallet from mnemonic phrase with derivation path
    pub fn from_mnemonic_with_path(
        mnemonic: &str,
        path: Option<&str>,
        key_type: KeyPairType,
    ) -> Result<Self> {
        info!("Creating wallet from mnemonic with {:?} keys", key_type);

        // Validate mnemonic
        let _ = bip39::Mnemonic::parse(mnemonic)
            .map_err(|e| Error::Wallet(format!("Invalid mnemonic: {}", e)))?;

        // Create derivation path string
        let full_path = if let Some(p) = path {
            format!("{}//{}", mnemonic, p)
        } else {
            mnemonic.to_string()
        };

        match key_type {
            KeyPairType::Sr25519 => {
                let pair = sr25519::Pair::from_string(&full_path, None)
                    .map_err(|e| Error::Wallet(format!("Failed to derive key: {:?}", e)))?;

                Ok(Self {
                    key_type,
                    sr25519_pair: Some(pair),
                    ed25519_pair: None,
                    ss58_format: Ss58AddressFormat::custom(42),
                })
            }
            KeyPairType::Ed25519 => {
                let pair = ed25519::Pair::from_string(&full_path, None)
                    .map_err(|e| Error::Wallet(format!("Failed to derive key: {:?}", e)))?;

                Ok(Self {
                    key_type,
                    sr25519_pair: None,
                    ed25519_pair: Some(pair),
                    ss58_format: Ss58AddressFormat::custom(42),
                })
            }
        }
    }

    /// Create wallet from private key (seed)
    pub fn from_seed(seed: &[u8], key_type: KeyPairType) -> Result<Self> {
        info!("Creating wallet from seed with {:?} keys", key_type);

        if seed.len() != 32 {
            return Err(Error::Wallet("Seed must be 32 bytes".to_string()));
        }

        let mut seed_array = [0u8; 32];
        seed_array.copy_from_slice(seed);

        let result = match key_type {
            KeyPairType::Sr25519 => {
                let pair = sr25519::Pair::from_seed(&seed_array);
                Ok(Self {
                    key_type,
                    sr25519_pair: Some(pair),
                    ed25519_pair: None,
                    ss58_format: Ss58AddressFormat::custom(42),
                })
            }
            KeyPairType::Ed25519 => {
                let pair = ed25519::Pair::from_seed(&seed_array);
                Ok(Self {
                    key_type,
                    sr25519_pair: None,
                    ed25519_pair: Some(pair),
                    ss58_format: Ss58AddressFormat::custom(42),
                })
            }
        };

        seed_array.zeroize();
        result
    }

    /// Generate a new mnemonic phrase
    pub fn generate_mnemonic() -> Result<String> {
        use bip39::{Language, Mnemonic};
        use rand::RngCore;

        let mut entropy = [0u8; 32];
        rand::rng().fill_bytes(&mut entropy);

        let result = Mnemonic::from_entropy_in(Language::English, &entropy)
            .map(|m| m.to_string())
            .map_err(|e| Error::Wallet(format!("Failed to generate mnemonic: {}", e)));

        entropy.zeroize();
        result
    }

    /// Set the SS58 address format (network prefix)
    pub fn with_ss58_format(mut self, format: u16) -> Self {
        self.ss58_format = Ss58AddressFormat::custom(format);
        self
    }

    /// Get the public key as bytes
    pub fn public_key(&self) -> Vec<u8> {
        match self.key_type {
            KeyPairType::Sr25519 => self
                .sr25519_pair
                .as_ref()
                .expect("SR25519 pair must exist for SR25519 key type")
                .public()
                .0
                .to_vec(),
            KeyPairType::Ed25519 => self
                .ed25519_pair
                .as_ref()
                .expect("ED25519 pair must exist for ED25519 key type")
                .public()
                .0
                .to_vec(),
        }
    }

    /// Get the SS58-encoded address
    pub fn address(&self) -> String {
        match self.key_type {
            KeyPairType::Sr25519 => {
                let public = self
                    .sr25519_pair
                    .as_ref()
                    .expect("SR25519 pair must exist for SR25519 key type")
                    .public();
                public.to_ss58check_with_version(self.ss58_format)
            }
            KeyPairType::Ed25519 => {
                let public = self
                    .ed25519_pair
                    .as_ref()
                    .expect("ED25519 pair must exist for ED25519 key type")
                    .public();
                public.to_ss58check_with_version(self.ss58_format)
            }
        }
    }

    /// Get the key pair type
    pub fn key_type(&self) -> KeyPairType {
        self.key_type
    }

    /// Sign a message
    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        match self.key_type {
            KeyPairType::Sr25519 => {
                let pair = self
                    .sr25519_pair
                    .as_ref()
                    .expect("SR25519 pair must exist for SR25519 key type");
                pair.sign(message).0.to_vec()
            }
            KeyPairType::Ed25519 => {
                let pair = self
                    .ed25519_pair
                    .as_ref()
                    .expect("ED25519 pair must exist for ED25519 key type");
                pair.sign(message).0.to_vec()
            }
        }
    }

    /// Verify a signature
    pub fn verify(&self, message: &[u8], signature: &[u8]) -> bool {
        match self.key_type {
            KeyPairType::Sr25519 => {
                if signature.len() != 64 {
                    return false;
                }
                let mut sig_array = [0u8; 64];
                sig_array.copy_from_slice(signature);
                let sig = sr25519::Signature::from_raw(sig_array);
                let public = self
                    .sr25519_pair
                    .as_ref()
                    .expect("SR25519 pair must exist for SR25519 key type")
                    .public();
                sr25519::Pair::verify(&sig, message, &public)
            }
            KeyPairType::Ed25519 => {
                if signature.len() != 64 {
                    return false;
                }
                let mut sig_array = [0u8; 64];
                sig_array.copy_from_slice(signature);
                let sig = ed25519::Signature::from_raw(sig_array);
                let public = self
                    .ed25519_pair
                    .as_ref()
                    .expect("ED25519 pair must exist for ED25519 key type")
                    .public();
                ed25519::Pair::verify(&sig, message, &public)
            }
        }
    }

    /// Get the seed/private key (if available)
    /// Note: This should be kept secure and not exposed in production
    pub fn seed(&self) -> Option<[u8; 32]> {
        match self.key_type {
            KeyPairType::Sr25519 => {
                // SR25519 doesn't expose seed directly in a simple way
                None
            }
            KeyPairType::Ed25519 => {
                // ED25519 also doesn't expose seed directly
                None
            }
        }
    }

    /// Get the SR25519 pair for signing (if this is an SR25519 wallet)
    pub fn sr25519_pair(&self) -> Option<&sr25519::Pair> {
        self.sr25519_pair.as_ref()
    }

    /// Get the ED25519 pair for signing (if this is an ED25519 wallet)
    pub fn ed25519_pair(&self) -> Option<&ed25519::Pair> {
        self.ed25519_pair.as_ref()
    }

    /// Convert the wallet to a subxt-compatible signer
    #[allow(clippy::clone_on_copy)]
    pub fn to_subxt_signer(&self) -> crate::signer::ApexSigner {
        match self.key_type {
            KeyPairType::Sr25519 => {
                let pair = self.sr25519_pair.as_ref().expect("SR25519 pair missing");
                crate::signer::ApexSigner::Sr25519(Box::new(crate::signer::Sr25519Signer::new(
                    pair.clone(),
                )))
            }
            KeyPairType::Ed25519 => {
                let pair = self.ed25519_pair.as_ref().expect("ED25519 pair missing");
                crate::signer::ApexSigner::Ed25519(Box::new(crate::signer::Ed25519Signer::new(
                    pair.clone(),
                )))
            }
        }
    }
}

impl std::fmt::Debug for Wallet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Wallet")
            .field("key_type", &self.key_type)
            .field("address", &self.address())
            .field("ss58_format", &self.ss58_format)
            .finish()
    }
}

impl Drop for Wallet {
    fn drop(&mut self) {
        // Log wallet cleanup for security auditing
        // Note: sp_core::Pair types handle actual memory zeroing internally
        debug!(
            "Dropping wallet of type {:?} at address {}",
            self.key_type,
            self.address()
        );
    }
}

#[async_trait]
impl CoreSigner for Wallet {
    async fn sign_transaction(&self, tx: &[u8]) -> std::result::Result<Vec<u8>, SdkError> {
        Ok(self.sign(tx))
    }

    fn address(&self) -> Address {
        Address::Substrate(self.address())
    }
}

/// Manager for multiple wallets
pub struct WalletManager {
    wallets: Arc<RwLock<HashMap<String, Wallet>>>,
    default_key_type: KeyPairType,
}

impl WalletManager {
    /// Create a new wallet manager
    pub fn new() -> Self {
        Self {
            wallets: Arc::new(RwLock::new(HashMap::new())),
            default_key_type: KeyPairType::Sr25519,
        }
    }

    /// Create a new wallet manager with default key type
    pub fn with_key_type(key_type: KeyPairType) -> Self {
        Self {
            wallets: Arc::new(RwLock::new(HashMap::new())),
            default_key_type: key_type,
        }
    }

    /// Create and add a new random wallet
    pub fn create_wallet(&self, name: impl Into<String>) -> Wallet {
        let wallet = Wallet::new_random_with_type(self.default_key_type);
        let name = name.into();

        debug!("Creating wallet '{}' at address {}", name, wallet.address());

        self.wallets.write().insert(name.clone(), wallet.clone());
        wallet
    }

    /// Add an existing wallet
    pub fn add_wallet(&self, name: impl Into<String>, wallet: Wallet) {
        let name = name.into();
        debug!("Adding wallet '{}' at address {}", name, wallet.address());
        self.wallets.write().insert(name, wallet);
    }

    /// Get a wallet by name
    pub fn get_wallet(&self, name: &str) -> Option<Wallet> {
        self.wallets.read().get(name).cloned()
    }

    /// Remove a wallet
    pub fn remove_wallet(&self, name: &str) -> Option<Wallet> {
        debug!("Removing wallet '{}'", name);
        self.wallets.write().remove(name)
    }

    /// List all wallet names
    pub fn list_wallets(&self) -> Vec<String> {
        self.wallets.read().keys().cloned().collect()
    }

    /// Get number of wallets
    pub fn wallet_count(&self) -> usize {
        self.wallets.read().len()
    }

    /// Clear all wallets
    pub fn clear(&self) {
        debug!("Clearing all wallets");
        self.wallets.write().clear();
    }
}

impl Default for WalletManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_random_wallet() {
        let wallet = Wallet::new_random();
        assert_eq!(wallet.key_type(), KeyPairType::Sr25519);
        assert!(!wallet.address().is_empty());
        assert!(!wallet.public_key().is_empty());
    }

    #[test]
    fn test_create_wallet_types() {
        let sr25519_wallet = Wallet::new_random_with_type(KeyPairType::Sr25519);
        assert_eq!(sr25519_wallet.key_type(), KeyPairType::Sr25519);

        let ed25519_wallet = Wallet::new_random_with_type(KeyPairType::Ed25519);
        assert_eq!(ed25519_wallet.key_type(), KeyPairType::Ed25519);
    }

    #[test]
    fn test_sign_and_verify() {
        let wallet = Wallet::new_random();
        let message = b"Hello, Substrate!";

        let signature = wallet.sign(message);
        assert_eq!(signature.len(), 64);

        assert!(wallet.verify(message, &signature));
        assert!(!wallet.verify(b"Different message", &signature));
    }

    #[test]
    fn test_generate_mnemonic() {
        let mnemonic = Wallet::generate_mnemonic().unwrap();
        assert!(!mnemonic.is_empty());

        let wallet = Wallet::from_mnemonic(&mnemonic, KeyPairType::Sr25519);
        assert!(wallet.is_ok());
    }

    #[test]
    fn test_wallet_from_mnemonic() {
        let mnemonic = "bottom drive obey lake curtain smoke basket hold race lonely fit walk";

        let wallet1 = Wallet::from_mnemonic(mnemonic, KeyPairType::Sr25519).unwrap();
        let wallet2 = Wallet::from_mnemonic(mnemonic, KeyPairType::Sr25519).unwrap();

        // Same mnemonic should produce same address
        assert_eq!(wallet1.address(), wallet2.address());
    }

    #[test]
    fn test_wallet_with_ss58_format() {
        let wallet = Wallet::new_random().with_ss58_format(0); // Polkadot
        let address = wallet.address();

        // Polkadot addresses start with '1'
        assert!(address.starts_with('1'));
    }

    #[test]
    fn test_wallet_from_seed() {
        let seed = [42u8; 32];
        let wallet1 = Wallet::from_seed(&seed, KeyPairType::Sr25519).unwrap();
        let wallet2 = Wallet::from_seed(&seed, KeyPairType::Sr25519).unwrap();

        // Same seed should produce same address
        assert_eq!(wallet1.address(), wallet2.address());
    }

    #[test]
    fn test_wallet_manager() {
        let manager = WalletManager::new();

        let wallet1 = manager.create_wallet("wallet1");
        assert_eq!(manager.wallet_count(), 1);

        let wallet2 = Wallet::new_random();
        manager.add_wallet("wallet2", wallet2.clone());
        assert_eq!(manager.wallet_count(), 2);

        let retrieved = manager.get_wallet("wallet1").unwrap();
        assert_eq!(retrieved.address(), wallet1.address());

        let names = manager.list_wallets();
        assert_eq!(names.len(), 2);
        assert!(names.contains(&"wallet1".to_string()));
        assert!(names.contains(&"wallet2".to_string()));

        manager.remove_wallet("wallet1");
        assert_eq!(manager.wallet_count(), 1);

        manager.clear();
        assert_eq!(manager.wallet_count(), 0);
    }

    #[test]
    fn test_different_key_types_produce_different_addresses() {
        let seed = [42u8; 32];
        let sr25519_wallet = Wallet::from_seed(&seed, KeyPairType::Sr25519).unwrap();
        let ed25519_wallet = Wallet::from_seed(&seed, KeyPairType::Ed25519).unwrap();

        assert_ne!(sr25519_wallet.address(), ed25519_wallet.address());
    }

    #[test]
    fn test_invalid_mnemonic() {
        let invalid_mnemonic = "invalid mnemonic phrase that should fail";
        let result = Wallet::from_mnemonic(invalid_mnemonic, KeyPairType::Sr25519);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid mnemonic"));
    }

    #[test]
    fn test_invalid_seed_length() {
        let short_seed = [42u8; 16];
        let result = Wallet::from_seed(&short_seed, KeyPairType::Sr25519);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Seed must be 32 bytes"));

        let long_seed = [42u8; 64];
        let result = Wallet::from_seed(&long_seed, KeyPairType::Ed25519);
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_with_invalid_signature_length() {
        let wallet = Wallet::new_random();
        let message = b"Test message";

        let short_signature = [0u8; 32];
        assert!(!wallet.verify(message, &short_signature));

        let long_signature = [0u8; 128];
        assert!(!wallet.verify(message, &long_signature));
    }

    #[test]
    fn test_verify_with_tampered_signature() {
        let wallet = Wallet::new_random();
        let message = b"Original message";

        let mut signature = wallet.sign(message);
        signature[0] ^= 0xFF;

        assert!(!wallet.verify(message, &signature));
    }

    #[test]
    fn test_mnemonic_generation_produces_valid_mnemonic() {
        for _ in 0..10 {
            let mnemonic = Wallet::generate_mnemonic().unwrap();
            let words: Vec<&str> = mnemonic.split_whitespace().collect();
            assert!(
                words.len() == 12 || words.len() == 15 || words.len() == 18 || words.len() == 24,
                "Expected valid mnemonic word count, got {}",
                words.len()
            );

            let wallet = Wallet::from_mnemonic(&mnemonic, KeyPairType::Sr25519);
            assert!(
                wallet.is_ok(),
                "Generated mnemonic should be valid: {}",
                mnemonic
            );
        }
    }

    #[test]
    fn test_wallet_from_mnemonic_with_path() {
        let mnemonic = "bottom drive obey lake curtain smoke basket hold race lonely fit walk";

        let wallet1 =
            Wallet::from_mnemonic_with_path(mnemonic, Some("//Alice"), KeyPairType::Sr25519)
                .unwrap();
        let wallet2 =
            Wallet::from_mnemonic_with_path(mnemonic, Some("//Bob"), KeyPairType::Sr25519).unwrap();

        assert_ne!(wallet1.address(), wallet2.address());

        let wallet_no_path = Wallet::from_mnemonic(mnemonic, KeyPairType::Sr25519).unwrap();
        let wallet_empty_path =
            Wallet::from_mnemonic_with_path(mnemonic, None, KeyPairType::Sr25519).unwrap();
        assert_eq!(wallet_no_path.address(), wallet_empty_path.address());
    }

    #[test]
    fn test_both_key_types_sign_and_verify() {
        let sr25519_wallet = Wallet::new_random_with_type(KeyPairType::Sr25519);
        let ed25519_wallet = Wallet::new_random_with_type(KeyPairType::Ed25519);
        let message = b"Test message for both key types";

        let sr25519_sig = sr25519_wallet.sign(message);
        assert!(sr25519_wallet.verify(message, &sr25519_sig));
        assert_eq!(sr25519_sig.len(), 64);

        let ed25519_sig = ed25519_wallet.sign(message);
        assert!(ed25519_wallet.verify(message, &ed25519_sig));
        assert_eq!(ed25519_sig.len(), 64);
    }
}
