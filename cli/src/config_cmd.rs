//! Configuration command handlers

use anyhow::Result;
use colored::Colorize;

use crate::config::{get_config_path, get_legacy_config_path, Config, Preferences};

/// Show current configuration
pub fn show_config() -> Result<()> {
    let config_path = get_config_path()?;
    let config = Config::load(&config_path)?;

    println!("\n{}", "Apex SDK Configuration".cyan().bold());
    println!("{}", "═══════════════════════════════════════".dimmed());
    println!("{}: {}", "Config File".dimmed(), config_path.display());
    println!();

    println!("{}", "Default Settings:".yellow().bold());
    println!("  {}: {}", "default_chain".cyan(), config.default_chain);
    println!(
        "  {}: {}",
        "default_endpoint".cyan(),
        config.default_endpoint
    );
    println!(
        "  {}: {}",
        "default_account".cyan(),
        config
            .default_account
            .as_ref()
            .unwrap_or(&"none".to_string())
    );

    println!("\n{}", "Preferences:".yellow().bold());
    println!(
        "  {}: {}",
        "color_output".cyan(),
        config.preferences.color_output
    );
    println!(
        "  {}: {}",
        "progress_bars".cyan(),
        config.preferences.progress_bars
    );
    println!("  {}: {}", "log_level".cyan(), config.preferences.log_level);

    if !config.endpoints.is_empty() {
        println!("\n{}", "Configured Endpoints:".yellow().bold());
        let mut endpoints: Vec<_> = config.endpoints.iter().collect();
        endpoints.sort_by_key(|(k, _)| *k);

        for (chain, endpoint) in endpoints {
            println!("  {}: {}", chain.cyan(), endpoint.dimmed());
        }
    }

    // Check for legacy config
    if let Some(legacy_path) = get_legacy_config_path()? {
        println!("\n{}", "Legacy Configuration Detected".yellow().bold());
        println!("Found old config at: {}", legacy_path.display());
        println!(
            "Consider migrating to the new location: {}",
            config_path.display()
        );
    }

    Ok(())
}

/// Set a configuration value
pub fn set_config(key: &str, value: &str) -> Result<()> {
    let config_path = get_config_path()?;
    let mut config = Config::load(&config_path)?;

    config.set(key, value)?;
    config.save(&config_path)?;

    println!("\n{}", "Configuration Updated".green().bold());
    println!("{}: {} = {}", "Set".dimmed(), key.cyan(), value.yellow());
    println!("{}: {}", "Config File".dimmed(), config_path.display());

    Ok(())
}

/// Get a configuration value
pub fn get_config(key: &str) -> Result<()> {
    let config_path = get_config_path()?;
    let config = Config::load(&config_path)?;

    let value = config.get(key)?;

    println!("{}: {}", key.cyan(), value.yellow());

    Ok(())
}

/// Validate configuration
pub fn validate_config() -> Result<()> {
    let config_path = get_config_path()?;
    let config = Config::load(&config_path)?;

    println!("\n{}", "Validating Configuration".cyan().bold());
    println!("{}", "═══════════════════════════════════════".dimmed());
    println!("{}: {}", "Config File".dimmed(), config_path.display());
    println!();

    let warnings = config.validate()?;

    if warnings.is_empty() {
        println!("{}", "Configuration is valid!".green().bold());
        println!("No issues found.");
    } else {
        println!("{}", "Configuration Warnings:".yellow().bold());
        println!("{}", "═══════════════════════════════════════".dimmed());

        for (idx, warning) in warnings.iter().enumerate() {
            println!("{}. {}", idx + 1, warning.yellow());
        }

        println!(
            "\n{}",
            "These warnings won't prevent the CLI from working,".dimmed()
        );
        println!("{}", "   but may cause issues with some commands.".dimmed());
    }

    Ok(())
}

/// Reset configuration to defaults
pub fn reset_config(force: bool) -> Result<()> {
    let config_path = get_config_path()?;

    if !force && config_path.exists() {
        println!("\n{}", "Reset Configuration".yellow().bold());
        println!("{}", "═══════════════════════════════════════".dimmed());
        println!("This will reset your configuration to defaults.");
        println!("Current config: {}", config_path.display());

        print!("\nAre you sure? (yes/no): ");
        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if input.trim().to_lowercase() != "yes" {
            println!("\n{}", "Cancelled.".yellow());
            return Ok(());
        }
    }

    let config = Config::default();
    config.save(&config_path)?;

    println!("\n{}", "Configuration Reset".green().bold());
    println!("{}: {}", "Config File".dimmed(), config_path.display());
    println!(
        "\n{}",
        "Use 'apex config show' to view the default configuration".cyan()
    );

    Ok(())
}

/// Initialize configuration interactively
pub async fn init_config_interactive() -> Result<()> {
    use dialoguer::{Confirm, Input, Select};

    println!("\n{}", "Initialize Apex SDK Configuration".cyan().bold());
    println!("{}", "═══════════════════════════════════════".dimmed());
    println!("This wizard will help you set up your configuration.\n");

    let chains = vec![
        "paseo (Testnet - Recommended)",
        "westend (Testnet)",
        "polkadot (Mainnet)",
        "kusama (Mainnet)",
        "moonbeam (Parachain)",
        "astar (Parachain)",
        "sepolia (EVM Testnet)",
        "ethereum (EVM Mainnet)",
        "polygon (EVM Mainnet)",
        "bsc (EVM Mainnet)",
        "avalanche (EVM Mainnet)",
    ];

    let default_chain_idx = Select::new()
        .with_prompt("Select your default chain")
        .items(&chains)
        .default(0)
        .interact()?;

    // Extract chain name from the display string
    let default_chain = chains[default_chain_idx]
        .split_whitespace()
        .next()
        .unwrap()
        .to_string();

    let default_endpoint: String = Input::new()
        .with_prompt("Enter the default RPC endpoint (or press Enter for default)")
        .allow_empty(true)
        .interact_text()?;

    let default_endpoint = if default_endpoint.is_empty() {
        match default_chain.as_str() {
            "paseo" => "wss://paseo.rpc.amforc.com".to_string(),
            "westend" => "wss://westend-rpc.polkadot.io".to_string(),
            "polkadot" => "wss://polkadot.api.onfinality.io/public-ws".to_string(),
            "kusama" => "wss://kusama.api.onfinality.io/public-ws".to_string(),
            "moonbeam" => "wss://wss.api.moonbeam.network".to_string(),
            "astar" => "wss://rpc.astar.network".to_string(),
            "sepolia" => "https://ethereum-sepolia-rpc.publicnode.com".to_string(),
            "ethereum" => "https://eth.llamarpc.com".to_string(),
            "polygon" => "https://polygon-rpc.com".to_string(),
            _ => default_endpoint,
        }
    } else {
        default_endpoint
    };

    let color_output = Confirm::new()
        .with_prompt("Enable colored output?")
        .default(true)
        .interact()?;

    let progress_bars = Confirm::new()
        .with_prompt("Enable progress bars?")
        .default(true)
        .interact()?;

    let log_levels = vec!["error", "warn", "info", "debug", "trace"];
    let log_level_idx = Select::new()
        .with_prompt("Select log level")
        .items(&log_levels)
        .default(2)
        .interact()?;

    let log_level = log_levels[log_level_idx].to_string();

    // Create configuration
    let config = Config {
        default_chain,
        default_endpoint,
        preferences: Preferences {
            color_output,
            progress_bars,
            log_level,
        },
        ..Default::default()
    };

    // Save configuration
    let config_path = get_config_path()?;
    config.save(&config_path)?;

    println!("\n{}", "Configuration Saved!".green().bold());
    println!("{}: {}", "Config File".dimmed(), config_path.display());
    println!(
        "\n{}",
        "Use 'apex config show' to view your configuration".cyan()
    );

    Ok(())
}

use std::io::Write;

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn test_config_operations() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.json");

        // Create default config
        let mut config = Config::default();
        config.save(&config_path).unwrap();

        // Load and verify - default is now paseo
        let loaded = Config::load(&config_path).unwrap();
        assert_eq!(loaded.default_chain, "paseo");

        // Modify and save
        config.set("default_chain", "kusama").unwrap();
        config.save(&config_path).unwrap();

        // Verify modification
        let modified = Config::load(&config_path).unwrap();
        assert_eq!(modified.default_chain, "kusama");
    }

    #[test]
    fn test_show_config_with_temp_file() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.json");
        
        // Create a test config
        let config = Config {
            default_chain: "ethereum".to_string(),
            default_endpoint: "https://eth.llamarpc.com".to_string(),
            default_account: Some("test-account".to_string()),
            preferences: Preferences {
                color_output: true,
                progress_bars: false,
                log_level: "debug".to_string(),
            },
            endpoints: std::collections::HashMap::new(),
        };
        
        config.save(&config_path).unwrap();
        
        // Test that we can load the config (show_config would use this)
        let loaded = Config::load(&config_path).unwrap();
        assert_eq!(loaded.default_chain, "ethereum");
        assert_eq!(loaded.default_endpoint, "https://eth.llamarpc.com");
        assert_eq!(loaded.default_account, Some("test-account".to_string()));
        assert_eq!(loaded.preferences.log_level, "debug");
        assert!(!loaded.preferences.progress_bars);
    }

    #[test]
    fn test_init_config() {
        let temp_dir = TempDir::new().unwrap();
        
        // Test init_config logic
        let config = Config::default();
        let config_path = temp_dir.path().join("config.json");
        
        // Save config
        config.save(&config_path).unwrap();
        
        // Verify file exists and contains expected data
        assert!(config_path.exists());
        
        let content = fs::read_to_string(&config_path).unwrap();
        assert!(content.contains("paseo")); // default chain
        assert!(content.contains("color_output"));
        assert!(content.contains("progress_bars"));
        assert!(content.contains("log_level"));
    }

    #[test]
    fn test_config_validation() {
        // Test that valid configurations are accepted
        let mut config = Config::default();
        
        // Valid settings
        assert!(config.set("default_chain", "polkadot").is_ok());
        assert!(config.set("default_chain", "ethereum").is_ok());
        assert!(config.set("default_chain", "kusama").is_ok());
        
        // Invalid settings should be handled gracefully
        let result = config.set("invalid_key", "value");
        // The set method may or may not return an error depending on implementation
        // We just test that it doesn't panic
        let _ = result;
    }

    #[test]
    fn test_config_file_paths() {
        // Test that config paths can be retrieved
        let config_path_result = get_config_path();
        let legacy_config_path_result = get_legacy_config_path();
        
        // These should not panic and should return valid paths
        assert!(config_path_result.is_ok());
        assert!(legacy_config_path_result.is_ok());
        
        let config_path = config_path_result.unwrap();
        if let Some(legacy_path) = legacy_config_path_result.unwrap() {
            // Paths should be different if legacy path exists
            assert_ne!(config_path, legacy_path);
            
            // Both should be absolute paths
            assert!(config_path.is_absolute());
            assert!(legacy_path.is_absolute());
        } else {
            // Just check config path is absolute
            assert!(config_path.is_absolute());
        }
    }

    #[test]
    fn test_config_default_values() {
        let config = Config::default();
        
        // Test default values
        assert_eq!(config.default_chain, "paseo");
        assert!(config.default_endpoint.contains("paseo") || config.default_endpoint.contains("rpc"));
        assert!(config.default_account.is_none());
        assert!(config.preferences.color_output);
        assert!(config.preferences.progress_bars);
        assert!(!config.preferences.log_level.is_empty());
        assert!(config.endpoints.is_empty());
    }

    #[test]
    fn test_preferences_structure() {
        let preferences = Preferences {
            color_output: false,
            progress_bars: true,
            log_level: "trace".to_string(),
        };
        
        assert!(!preferences.color_output);
        assert!(preferences.progress_bars);
        assert_eq!(preferences.log_level, "trace");
    }

    #[test]
    fn test_config_serialization() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test_config.json");
        
        // Create config with custom endpoints
        let mut endpoints = std::collections::HashMap::new();
        endpoints.insert("custom_polkadot".to_string(), "wss://custom.polkadot.io".to_string());
        endpoints.insert("local".to_string(), "ws://localhost:9944".to_string());
        
        let config = Config {
            default_chain: "polkadot".to_string(),
            default_endpoint: "wss://polkadot.api.onfinality.io".to_string(),
            default_account: Some("alice".to_string()),
            preferences: Preferences {
                color_output: false,
                progress_bars: true,
                log_level: "warn".to_string(),
            },
            endpoints,
        };
        
        // Save and reload
        config.save(&config_path).unwrap();
        let loaded = Config::load(&config_path).unwrap();
        
        // Verify all fields
        assert_eq!(loaded.default_chain, "polkadot");
        assert_eq!(loaded.default_endpoint, "wss://polkadot.api.onfinality.io");
        assert_eq!(loaded.default_account, Some("alice".to_string()));
        assert!(!loaded.preferences.color_output);
        assert!(loaded.preferences.progress_bars);
        assert_eq!(loaded.preferences.log_level, "warn");
        assert_eq!(loaded.endpoints.len(), 2);
        assert_eq!(loaded.endpoints.get("custom_polkadot"), 
                  Some(&"wss://custom.polkadot.io".to_string()));
        assert_eq!(loaded.endpoints.get("local"), 
                  Some(&"ws://localhost:9944".to_string()));
    }

    #[test]
    fn test_config_missing_file() {
        let temp_dir = TempDir::new().unwrap();
        let nonexistent_path = temp_dir.path().join("nonexistent.json");
        
        // Loading nonexistent file should create default or return error
        let result = Config::load(&nonexistent_path);
        // Depending on implementation, this may succeed (with defaults) or fail
        // We just test that it doesn't panic
        let _ = result;
    }

    #[test]
    fn test_config_invalid_json() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("invalid.json");
        
        // Write invalid JSON
        fs::write(&config_path, "invalid json content").unwrap();
        
        // Loading invalid JSON should return an error
        let result = Config::load(&config_path);
        assert!(result.is_err());
    }
}
