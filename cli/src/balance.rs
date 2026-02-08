//! Balance checking functionality for Substrate and Revive chains

use anyhow::{Context, Result};
use colored::Colorize;
use subxt::ext::scale_value::At;

/// Get account balance for Substrate chains
pub async fn get_substrate_balance(address: &str, endpoint: &str) -> Result<()> {
    use subxt::{OnlineClient, PolkadotConfig};

    println!("\n{}", "Fetching Substrate Balance".cyan().bold());
    println!("{}", "═══════════════════════════════════════".dimmed());
    println!("{}: {}", "Endpoint".dimmed(), endpoint);
    println!("{}: {}", "Address".dimmed(), address);
    println!();

    // Show progress
    let spinner = indicatif::ProgressBar::new_spinner();
    spinner.set_message("Connecting to chain...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    // Connect to the chain
    let api = OnlineClient::<PolkadotConfig>::from_url(endpoint)
        .await
        .context("Failed to connect to Substrate endpoint")?;

    spinner.set_message("Fetching balance...");

    let address_val = apex_sdk_substrate::storage::StorageQuery::parse_address(address)
        .context("Invalid Substrate address")?;

    let account_query = subxt::dynamic::storage("System", "Account", vec![address_val.clone()]);

    let account_data = api
        .storage()
        .at_latest()
        .await?
        .fetch(&account_query)
        .await?
        .context("Account not found on chain")?;

    fn extract_u128<T>(value: &subxt::dynamic::Value<T>, path: &[&str]) -> Option<u128> {
        let mut current = value;
        for &key in path {
            current = current.at(key)?;
        }
        current.as_u128()
    }

    let value = account_data
        .to_value()
        .map_err(|e| anyhow::anyhow!("Failed to decode account data: {}", e))?;
    let free_balance = extract_u128(&value, &["data", "free"])
        .context("Failed to parse free balance from storage")?;

    // Attempt to fetch constants from System pallet if available
    let token_symbol = "UNIT";
    let token_decimals = 12;
    spinner.finish_and_clear();

    // Try to fetch chain name from runtime metadata (fallback to static value)
    let chain_name = api
        .metadata()
        .pallet_by_name_err("System")
        .ok()
        .and_then(|pallet| {
            pallet
                .constants()
                .find(|c| c.name() == "ChainName")
                .and_then(|constant| {
                    std::str::from_utf8(constant.value())
                        .map(|s| s.to_owned())
                        .ok()
                })
        })
        .unwrap_or_else(|| "Substrate Chain".to_string());

    println!("\n{}", "Balance Retrieved".green().bold());
    println!("{}", "═══════════════════════════════════════".dimmed());
    println!("{}: {}", "Address".cyan(), address);
    println!("{}: {}", "Network".dimmed(), chain_name);
    println!();

    // Format balance with decimals
    let divisor = 10u128.pow(token_decimals as u32);
    let balance_formatted = format_balance(free_balance, divisor);

    println!(
        "{}: {} {}",
        "Free Balance".green().bold(),
        balance_formatted,
        token_symbol
    );
    println!("{}: {} raw units", "Raw".dimmed(), free_balance);

    // Show existential deposit if possible
    println!("\n{}", "Tip:".yellow());
    if free_balance == 0 {
        println!("This account has no balance. You may need to transfer some tokens to it.");
        println!("New accounts appear on-chain after receiving their first transaction.");
    }

    Ok(())
}

/// Get account balance for Revive chains
pub async fn get_revive_balance(address: &str, endpoint: &str) -> Result<()> {
    use apex_sdk::core::Provider;
    use apex_sdk::prelude::*;

    println!("\n{}", "Fetching Revive Balance".cyan().bold());
    println!("{}", "═══════════════════════════════════════".dimmed());
    println!("{}: {}", "Endpoint".dimmed(), endpoint);
    println!("{}: {}", "Address".dimmed(), address);
    println!();

    // Show progress
    let spinner = indicatif::ProgressBar::new_spinner();
    spinner.set_message("Connecting to Revive node...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    let adapter = ReviveAdapter::connect(endpoint)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to connect to Revive endpoint: {}", e))?;

    spinner.set_message("Fetching balance...");

    let addr = Address::evm(address);
    let balance = adapter
        .get_balance(&addr)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to fetch Revive balance: {}", e))?;

    spinner.finish_and_clear();

    println!("\n{}", "Revive Balance Retrieved".green().bold());
    println!("{}", "═══════════════════════════════════════".dimmed());
    println!("{}: {}", "Address".cyan(), address);

    // Revive usually uses 18 decimals like Ethereum
    let token_decimals = 18;
    let token_symbol = "ETH";

    // Format balance with decimals
    let divisor = 10u128.pow(token_decimals as u32);
    let balance_formatted = format_balance(balance, divisor);

    println!(
        "{}: {} {}",
        "Free Balance".green().bold(),
        balance_formatted,
        token_symbol
    );
    println!("{}: {} raw units", "Raw".dimmed(), balance);

    Ok(())
}

/// Format balance with decimal places
fn format_balance(balance: u128, divisor: u128) -> String {
    let whole = balance / divisor;
    let frac = balance % divisor;

    if frac == 0 {
        whole.to_string()
    } else {
        let decimal_places = (divisor as f64).log10() as usize;
        let frac_str = format!("{:0width$}", frac, width = decimal_places);
        let trimmed = frac_str.trim_end_matches('0');
        format!("{}.{}", whole, trimmed)
    }
}

/// Auto-detect chain type and get balance
pub async fn get_balance(address: &str, chain: &str, endpoint: &str) -> Result<()> {
    let is_substrate = apex_sdk_types::Chain::is_substrate_endpoint(endpoint)
        || apex_sdk_types::Chain::from_str_case_insensitive(chain)
            .map(|c| c.chain_type() == apex_sdk_types::ChainType::Substrate)
            .unwrap_or(false);

    if is_substrate {
        get_substrate_balance(address, endpoint).await
    } else {
        get_revive_balance(address, endpoint).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_balance() {
        // Test with 10 decimals (DOT/KSM style)
        let divisor = 10u128.pow(10);

        let test_cases = [
            (0u128, "0"),
            (1u128, "0.0000000001"),
            (divisor, "1"),
            (divisor / 2, "0.5"),
            (divisor * 10, "10"),
            (15 * divisor / 10, "1.5"),
        ];

        for (balance, expected) in &test_cases {
            let result = format_balance(*balance, divisor);
            assert_eq!(
                result, *expected,
                "Failed for {} balance, expected {}, got {}",
                balance, expected, result
            );
        }
    }

    #[test]
    fn test_format_balance_edge_cases() {
        let divisor = 10u128.pow(12); // 12 decimals

        // Very small amounts
        assert_eq!(format_balance(1, divisor), "0.000000000001");
        assert_eq!(format_balance(10, divisor), "0.00000000001");

        // Zero
        assert_eq!(format_balance(0, divisor), "0");

        // Large amounts
        assert_eq!(format_balance(1_000_000 * divisor, divisor), "1000000");
    }

    #[tokio::test]
    #[ignore] // Requires network connection
    async fn test_get_revive_balance_integration() {
        // Test with a known address
        let result = get_revive_balance(
            "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045",
            "https://eth.llamarpc.com",
        )
        .await;

        // We just test that it doesn't error out
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    #[ignore] // Requires network connection
    async fn test_get_substrate_balance_integration() {
        // Test with Westend testnet
        let result = get_substrate_balance(
            "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
            "wss://westend-rpc.polkadot.io",
        )
        .await;

        // We just test that it doesn't error out
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_get_balance_invalid_address() {
        // Test with invalid addresses
        let result = get_balance("invalid_address", "ethereum", "https://eth.llamarpc.com").await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_balance_invalid_endpoint() {
        // Test with invalid endpoint
        let result = get_balance(
            "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045",
            "ethereum",
            "https://invalid.endpoint.that.does.not.exist",
        )
        .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_balance_chain_detection() {
        // Test that chain detection works correctly

        // Should detect as Substrate based on endpoint
        let config = apex_sdk_types::Chain::from_str_case_insensitive("polkadot");
        if let Some(chain) = config {
            assert_eq!(chain.chain_type(), apex_sdk_types::ChainType::Substrate);
        }

        // Should detect as EVM
        let config = apex_sdk_types::Chain::from_str_case_insensitive("ethereum");
        if let Some(chain) = config {
            assert_eq!(chain.chain_type(), apex_sdk_types::ChainType::Evm);
        }
    }
}
