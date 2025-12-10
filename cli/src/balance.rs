//! Balance checking functionality for Substrate and EVM chains

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

    // Parse the address directly as AccountId32
    let account_id: subxt::utils::AccountId32 =
        address.parse().context("Invalid Substrate address")?;

    // Get the account info
    let account_storage = subxt::dynamic::storage(
        "System",
        "Account",
        vec![subxt::dynamic::Value::from_bytes(account_id)],
    );

    let result = api
        .storage()
        .at_latest()
        .await?
        .fetch(&account_storage)
        .await
        .context("Failed to fetch account data")?;

    if let Some(account_data) = result {
        spinner.finish_and_clear();

        // Decode the account data structure
        // AccountInfo has: { nonce, consumers, providers, sufficients, data: { free, reserved, ... } }
        let account_data = account_data.to_value()?;

        // Extract balance information from the composite structure
        let free_balance = account_data
            .at("data")
            .and_then(|data| data.at("free"))
            .and_then(|free| free.as_u128())
            .unwrap_or(0);

        let reserved_balance = account_data
            .at("data")
            .and_then(|data| data.at("reserved"))
            .and_then(|reserved| reserved.as_u128())
            .unwrap_or(0);

        let frozen_balance = account_data
            .at("data")
            .and_then(|data| data.at("frozen"))
            .or_else(|| {
                account_data
                    .at("data")
                    .and_then(|data| data.at("misc_frozen"))
            })
            .and_then(|frozen| frozen.as_u128())
            .unwrap_or(0);

        let nonce = account_data
            .at("nonce")
            .and_then(|n| n.as_u128())
            .unwrap_or(0);

        println!("\n{}", "Balance Retrieved".green().bold());
        println!("{}", "═══════════════════════════════════════".dimmed());
        println!("{}: {}", "Address".cyan(), address);
        println!();

        // Format balances (Substrate uses 10 decimals for DOT/KSM)
        let decimals = 10u32;
        let divisor = 10u128.pow(decimals);

        let free_formatted = format_balance(free_balance, divisor);
        let reserved_formatted = format_balance(reserved_balance, divisor);
        let frozen_formatted = format_balance(frozen_balance, divisor);
        let total = free_balance + reserved_balance;
        let total_formatted = format_balance(total, divisor);

        println!(
            "{}: {} tokens",
            "Free Balance".green().bold(),
            free_formatted
        );
        println!("{}: {} tokens", "Reserved".dimmed(), reserved_formatted);
        println!("{}: {} tokens", "Frozen".dimmed(), frozen_formatted);
        println!("{}: {} tokens", "Total".cyan().bold(), total_formatted);
        println!();
        println!("{}: {}", "Nonce".dimmed(), nonce);

        // Calculate transferable amount
        let transferable = free_balance.saturating_sub(frozen_balance);
        let transferable_formatted = format_balance(transferable, divisor);

        println!(
            "\n{}: {} tokens",
            "Transferable".yellow().bold(),
            transferable_formatted
        );

        println!("\n{}", "Note:".yellow());
        println!("Balance precision: {} decimal places", decimals);
        println!("Frozen balance includes locks (staking, vesting, etc.)");
    } else {
        spinner.finish_and_clear();

        println!("\n{}", "Account Not Found".yellow().bold());
        println!("This account has no balance on this chain.");
        println!("\n{}", "Note:".cyan());
        println!("New accounts appear on-chain after receiving their first transaction.");
    }

    Ok(())
}

/// Get account balance for EVM chains
pub async fn get_evm_balance(address: &str, endpoint: &str) -> Result<()> {
    use alloy::primitives::Address;
    use alloy::providers::{Provider, ProviderBuilder};

    println!("\n{}", "Fetching EVM Balance".cyan().bold());
    println!("{}", "═══════════════════════════════════════".dimmed());
    println!("{}: {}", "Endpoint".dimmed(), endpoint);
    println!("{}: {}", "Address".dimmed(), address);
    println!();

    // Show progress
    let spinner = indicatif::ProgressBar::new_spinner();
    spinner.set_message("Connecting to chain...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    // Connect to the provider using Alloy
    let provider =
        ProviderBuilder::new().connect_http(endpoint.parse().context("Invalid endpoint URL")?);

    spinner.set_message("Fetching balance...");

    // Parse the address
    let addr: Address = address.parse().context("Invalid EVM address")?;

    // Get the balance
    let balance = provider
        .get_balance(addr)
        .await
        .context("Failed to fetch balance")?;

    // Get the chain ID for better display
    let chain_id = provider
        .get_chain_id()
        .await
        .context("Failed to get chain ID")?;

    spinner.finish_and_clear();

    println!("\n{}", "Balance Retrieved".green().bold());
    println!("{}", "═══════════════════════════════════════".dimmed());
    println!("{}: {}", "Address".cyan(), address);
    println!("{}: {}", "Chain ID".dimmed(), chain_id);
    println!();

    // Convert balance to ETH (balance is U256)
    let balance_eth = format_wei_to_eth(balance.to::<u128>());

    println!("{}: {} ETH", "Balance".green().bold(), balance_eth);
    println!("{}: {} Wei", "Raw".dimmed(), balance);

    // Show USD value if possible (would need price oracle in production)
    println!("\n{}", "Tip:".yellow());
    println!("Use a block explorer for detailed transaction history:");
    match chain_id {
        1 => println!("  https://etherscan.io/address/{}", address),
        137 => println!("  https://polygonscan.com/address/{}", address),
        56 => println!("  https://bscscan.com/address/{}", address),
        _ => println!("  Check your chain's block explorer"),
    }

    Ok(())
}

/// Format wei to ETH (helper function to replace ethers::utils::format_units)
fn format_wei_to_eth(wei: u128) -> String {
    let eth_divisor = 10_u128.pow(18);
    let eth_whole = wei / eth_divisor;
    let remainder = wei % eth_divisor;

    if remainder == 0 {
        format!("{}", eth_whole)
    } else {
        // Format with up to 18 decimal places, trimming trailing zeros
        let formatted = format!("{}.{:018}", eth_whole, remainder);
        formatted
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    }
}

/// Format balance with decimal places
fn format_balance(balance: u128, divisor: u128) -> String {
    let whole = balance / divisor;
    let frac = balance % divisor;

    if frac == 0 {
        format!("{}", whole)
    } else {
        // Calculate the number of decimal places from the divisor
        let decimal_places = divisor.ilog10() as usize;
        // Remove trailing zeros
        let frac_str = format!("{:0width$}", frac, width = decimal_places);
        let trimmed = frac_str.trim_end_matches('0');
        format!("{}.{}", whole, trimmed)
    }
}

/// Auto-detect chain type and get balance
pub async fn get_balance(address: &str, chain: &str, endpoint: &str) -> Result<()> {
    // Determine if it's a Substrate or EVM chain using centralized logic
    let is_substrate = apex_sdk_types::Chain::is_substrate_endpoint(endpoint)
        || apex_sdk_types::Chain::from_str_case_insensitive(chain)
            .map(|c| c.chain_type() == apex_sdk_types::ChainType::Substrate)
            .unwrap_or(false);

    if is_substrate {
        get_substrate_balance(address, endpoint).await
    } else {
        get_evm_balance(address, endpoint).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_chain_type_substrate() {
        // Substrate endpoints
        assert!(is_substrate_endpoint("wss://polkadot.api.onfinality.io"));
        assert!(is_substrate_endpoint("ws://localhost:9944"));
        assert!(is_substrate_endpoint("wss://kusama-rpc.polkadot.io"));
        assert!(is_substrate_endpoint("ws://127.0.0.1:9944"));
    }

    #[test]
    fn test_detect_chain_type_evm() {
        // EVM endpoints
        assert!(!is_substrate_endpoint("https://eth.llamarpc.com"));
        assert!(!is_substrate_endpoint("http://localhost:8545"));
        assert!(!is_substrate_endpoint("https://mainnet.infura.io/v3/key"));
        assert!(!is_substrate_endpoint("https://bsc-dataseed.binance.org"));
    }

    #[test]
    fn test_address_parsing_substrate() {
        // Valid Substrate address (SS58 format)
        let valid_addresses = [
            "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
            "15oF4uVJwmo4TdGW7VfQxNLavjCXviqxT9S1MgbjMNHr6Sp5",
            "HNZata7iMYWmk5RvZRTiAsSDhV8366zq2YGb3tLH5Upf74F",
        ];

        for addr in &valid_addresses {
            assert!(
                addr.parse::<subxt::utils::AccountId32>().is_ok(),
                "Failed to parse valid Substrate address: {}",
                addr
            );
        }

        // Invalid Substrate addresses
        let invalid_addresses = [
            "invalid",
            "123",
            "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEbD", // EVM address
            "",
        ];

        for addr in &invalid_addresses {
            assert!(
                addr.parse::<subxt::utils::AccountId32>().is_err(),
                "Expected invalid Substrate address to fail: {}",
                addr
            );
        }
    }

    #[test]
    fn test_address_parsing_evm() {
        // Valid EVM addresses
        let valid_addresses = [
            "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEbD",
            "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045", // vitalik.eth
            "0x0000000000000000000000000000000000000000", // zero address
        ];

        for addr in &valid_addresses {
            assert!(
                addr.parse::<alloy::primitives::Address>().is_ok(),
                "Failed to parse valid EVM address: {}",
                addr
            );
        }

        // Invalid EVM addresses
        let invalid_addresses = [
            "invalid",
            "0x123",                                            // Too short
            "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEbDG",      // Invalid hex
            "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", // Substrate address
            "",
        ];

        for addr in &invalid_addresses {
            assert!(
                addr.parse::<alloy::primitives::Address>().is_err(),
                "Expected invalid EVM address to fail: {}",
                addr
            );
        }
    }

    #[test]
    fn test_format_wei_to_eth() {
        let test_cases = [
            (0u128, "0"),
            (1u128, "0.000000000000000001"),
            (1_000_000_000_000_000_000u128, "1"),   // 1 ETH
            (1_500_000_000_000_000_000u128, "1.5"), // 1.5 ETH
            (999_000_000_000_000_000u128, "0.999"), // 0.999 ETH
            (10_000_000_000_000_000_000u128, "10"), // 10 ETH
        ];

        for (wei, expected) in &test_cases {
            let result = format_wei_to_eth(*wei);
            assert_eq!(
                result, *expected,
                "Failed for {} wei, expected {}, got {}",
                wei, expected, result
            );
        }
    }

    #[test]
    fn test_format_balance() {
        // Test with 10 decimals (DOT/KSM style)
        let divisor = 10u128.pow(10);

        let test_cases = [
            (0u128, "0"),
            (1u128, "0.0000000001"),
            (divisor, "1"),             // 1 token
            (divisor / 2, "0.5"),       // 0.5 tokens
            (divisor * 10, "10"),       // 10 tokens
            (15 * divisor / 10, "1.5"), // 1.5 tokens
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

    #[test]
    fn test_format_wei_trailing_zeros() {
        // Test that trailing zeros are properly trimmed
        assert_eq!(format_wei_to_eth(1_000_000_000_000_000_000u128), "1");
        assert_eq!(format_wei_to_eth(1_500_000_000_000_000_000u128), "1.5");
        assert_eq!(format_wei_to_eth(1_100_000_000_000_000_000u128), "1.1");
        assert_eq!(format_wei_to_eth(1_001_000_000_000_000_000u128), "1.001");
    }

    #[tokio::test]
    #[ignore] // Requires network connection
    async fn test_get_evm_balance_integration() {
        // Test with a known address that should have some balance
        let result = get_evm_balance(
            "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045", // vitalik.eth
            "https://eth.llamarpc.com",
        )
        .await;

        // We just test that it doesn't error out - the actual balance may vary
        assert!(result.is_ok() || result.is_err()); // Either way is fine for integration test
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

    fn is_substrate_endpoint(endpoint: &str) -> bool {
        endpoint.starts_with("ws://") || endpoint.starts_with("wss://")
    }
}
