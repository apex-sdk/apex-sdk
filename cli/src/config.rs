//! Configuration management for Apex SDK CLI

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub default_chain: String,
    pub default_endpoint: String,
    pub default_account: Option<String>,
    #[serde(default)]
    pub endpoints: std::collections::HashMap<String, String>,
    #[serde(default)]
    pub preferences: Preferences,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preferences {
    #[serde(default = "default_true")]
    pub color_output: bool,
    #[serde(default = "default_true")]
    pub progress_bars: bool,
    #[serde(default)]
    pub log_level: String,
}

fn default_true() -> bool {
    true
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            color_output: true,
            progress_bars: true,
            log_level: "info".to_string(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        let mut endpoints = std::collections::HashMap::new();

        // Default chain configuration
        const DEFAULT_CHAIN: &str = "paseo";

        // Substrate mainnets
        endpoints.insert(
            "polkadot".to_string(),
            "wss://polkadot.api.onfinality.io/public-ws".to_string(),
        );
        endpoints.insert(
            "kusama".to_string(),
            "wss://kusama.api.onfinality.io/public-ws".to_string(),
        );

        // Substrate testnets
        endpoints.insert(
            "paseo".to_string(),
            "wss://paseo.rpc.amforc.com".to_string(),
        );
        endpoints.insert(
            "westend".to_string(),
            "wss://westend-rpc.polkadot.io".to_string(),
        );

        // Parachains
        endpoints.insert(
            "moonbeam".to_string(),
            "wss://wss.api.moonbeam.network".to_string(),
        );
        endpoints.insert("astar".to_string(), "wss://rpc.astar.network".to_string());

        // EVM chains
        endpoints.insert(
            "ethereum".to_string(),
            "https://eth.llamarpc.com".to_string(),
        );
        endpoints.insert("polygon".to_string(), "https://polygon-rpc.com".to_string());
        endpoints.insert(
            "sepolia".to_string(),
            "https://ethereum-sepolia-rpc.publicnode.com".to_string(),
        );

        let default_endpoint = endpoints.get(DEFAULT_CHAIN).cloned().unwrap_or_else(|| {
            panic!(
                "Default chain '{}' must have an endpoint configured in default config",
                DEFAULT_CHAIN
            )
        });

        Self {
            default_chain: DEFAULT_CHAIN.to_string(),
            default_endpoint,
            default_account: None,
            endpoints,
            preferences: Preferences::default(),
        }
    }
}

impl Config {
    /// Load configuration from disk
    pub fn load(path: &Path) -> Result<Self> {
        if path.exists() {
            let data = std::fs::read_to_string(path).context("Failed to read config file")?;
            let config: Config =
                serde_json::from_str(&data).context("Failed to parse config file")?;
            Ok(config)
        } else {
            Ok(Self::default())
        }
    }

    /// Save configuration to disk
    pub fn save(&self, path: &Path) -> Result<()> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let data = serde_json::to_string_pretty(self).context("Failed to serialize config")?;
        std::fs::write(path, data).context("Failed to write config file")?;

        Ok(())
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<Vec<String>> {
        let mut warnings = Vec::new();

        // Check if default chain has an endpoint
        if !self.endpoints.contains_key(&self.default_chain) {
            warnings.push(format!(
                "Default chain '{}' has no endpoint defined",
                self.default_chain
            ));
        }

        // Validate endpoint URLs
        for (chain, endpoint) in &self.endpoints {
            if !endpoint.starts_with("wss://")
                && !endpoint.starts_with("ws://")
                && !endpoint.starts_with("https://")
                && !endpoint.starts_with("http://")
            {
                warnings.push(format!(
                    "Chain '{}' has invalid endpoint URL: {}",
                    chain, endpoint
                ));
            }
        }

        // Validate log level
        let valid_levels = ["trace", "debug", "info", "warn", "error"];
        if !valid_levels.contains(&self.preferences.log_level.as_str()) {
            warnings.push(format!(
                "Invalid log level '{}'. Valid levels: trace, debug, info, warn, error",
                self.preferences.log_level
            ));
        }

        Ok(warnings)
    }

    /// Set a configuration value
    pub fn set(&mut self, key: &str, value: &str) -> Result<()> {
        match key {
            "default_chain" => {
                self.default_chain = value.to_string();
            }
            "default_endpoint" => {
                self.default_endpoint = value.to_string();
            }
            "default_account" => {
                self.default_account = Some(value.to_string());
            }
            "preferences.color_output" => {
                self.preferences.color_output = value
                    .parse()
                    .context("Invalid boolean value for color_output")?;
            }
            "preferences.progress_bars" => {
                self.preferences.progress_bars = value
                    .parse()
                    .context("Invalid boolean value for progress_bars")?;
            }
            "preferences.log_level" => {
                let valid_levels = ["trace", "debug", "info", "warn", "error"];
                if !valid_levels.contains(&value) {
                    anyhow::bail!(
                        "Invalid log level. Valid levels: trace, debug, info, warn, error"
                    );
                }
                self.preferences.log_level = value.to_string();
            }
            key if key.starts_with("endpoints.") => {
                let chain = key
                    .strip_prefix("endpoints.")
                    .expect("Key must start with 'endpoints.' prefix");
                self.endpoints.insert(chain.to_string(), value.to_string());
            }
            _ => {
                anyhow::bail!("Unknown configuration key: {}", key);
            }
        }
        Ok(())
    }

    /// Get a configuration value
    pub fn get(&self, key: &str) -> Result<String> {
        match key {
            "default_chain" => Ok(self.default_chain.clone()),
            "default_endpoint" => Ok(self.default_endpoint.clone()),
            "default_account" => Ok(self
                .default_account
                .clone()
                .unwrap_or_else(|| "none".to_string())),
            "preferences.color_output" => Ok(self.preferences.color_output.to_string()),
            "preferences.progress_bars" => Ok(self.preferences.progress_bars.to_string()),
            "preferences.log_level" => Ok(self.preferences.log_level.clone()),
            key if key.starts_with("endpoints.") => {
                let chain = key
                    .strip_prefix("endpoints.")
                    .expect("Key must start with 'endpoints.' prefix");
                self.endpoints
                    .get(chain)
                    .cloned()
                    .ok_or_else(|| anyhow::anyhow!("No endpoint defined for chain '{}'", chain))
            }
            _ => anyhow::bail!("Unknown configuration key: {}", key),
        }
    }
}

/// Get the default config path
pub fn get_config_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;
    Ok(config_dir.join("apex-sdk").join("config.json"))
}

/// Get the legacy config path (for migration)
pub fn get_legacy_config_path() -> Result<Option<PathBuf>> {
    let current_dir = std::env::current_dir()?;
    let legacy_path = current_dir.join(".apex").join("config.json");

    if legacy_path.exists() {
        Ok(Some(legacy_path))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.default_chain, "paseo");
        assert!(config.endpoints.contains_key("paseo"));
        assert!(config.endpoints.contains_key("polkadot"));
        assert!(config.endpoints.contains_key("ethereum"));
        assert!(config.endpoints.contains_key("westend"));
        assert!(config.endpoints.contains_key("sepolia"));
    }

    #[test]
    fn test_config_set_get() {
        let mut config = Config::default();

        config.set("default_chain", "kusama").unwrap();
        assert_eq!(config.get("default_chain").unwrap(), "kusama");

        config.set("preferences.log_level", "debug").unwrap();
        assert_eq!(config.get("preferences.log_level").unwrap(), "debug");
    }

    #[test]
    fn test_config_set_endpoint() {
        let mut config = Config::default();

        config
            .set("endpoints.testnet", "wss://testnet.example.com")
            .unwrap();
        assert_eq!(
            config.get("endpoints.testnet").unwrap(),
            "wss://testnet.example.com"
        );
    }

    #[test]
    fn test_config_validation() {
        let mut config = Config::default();

        // Should have no warnings with default config
        let warnings = config.validate().unwrap();
        assert_eq!(warnings.len(), 0);

        // Invalid log level should produce warning
        config.preferences.log_level = "invalid".to_string();
        let warnings = config.validate().unwrap();
        assert!(!warnings.is_empty());
    }

    #[test]
    fn test_config_invalid_key() {
        let mut config = Config::default();
        let result = config.set("invalid_key", "value");
        assert!(result.is_err());
    }

    #[test]
    fn test_config_invalid_log_level() {
        let mut config = Config::default();
        let result = config.set("preferences.log_level", "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_config_get_nonexistent_endpoint() {
        let config = Config::default();
        let result = config.get("endpoints.nonexistent");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("No endpoint defined"));
    }

    #[test]
    fn test_config_set_and_get_endpoint() {
        let mut config = Config::default();
        config
            .set("endpoints.custom", "wss://custom.example.com")
            .unwrap();

        let endpoint = config.get("endpoints.custom").unwrap();
        assert_eq!(endpoint, "wss://custom.example.com");
    }

    #[test]
    fn test_config_invalid_boolean_values() {
        let mut config = Config::default();

        let result = config.set("preferences.color_output", "not_a_bool");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid boolean value"));

        let result = config.set("preferences.progress_bars", "yes");
        assert!(result.is_err());
    }

    #[test]
    fn test_config_validation_with_invalid_endpoint_url() {
        let mut config = Config::default();
        config
            .endpoints
            .insert("invalid".to_string(), "not-a-url".to_string());

        let warnings = config.validate().unwrap();
        assert!(!warnings.is_empty());
        assert!(warnings.iter().any(|w| w.contains("invalid endpoint URL")));
    }

    #[test]
    fn test_config_validation_with_missing_default_chain_endpoint() {
        let config = Config {
            default_chain: "nonexistent_chain".to_string(),
            ..Default::default()
        };

        let warnings = config.validate().unwrap();
        assert!(!warnings.is_empty());
        assert!(warnings
            .iter()
            .any(|w| w.contains("has no endpoint defined")));
    }

    #[test]
    fn test_config_get_unknown_key() {
        let config = Config::default();
        let result = config.get("unknown.key");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Unknown configuration key"));
    }

    #[test]
    fn test_config_preferences_defaults() {
        let prefs = Preferences::default();
        assert!(prefs.color_output);
        assert!(prefs.progress_bars);
        assert_eq!(prefs.log_level, "info");
    }
}
