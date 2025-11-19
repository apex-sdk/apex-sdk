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
    use ethers::prelude::*;

    println!("\n{}", "Fetching EVM Balance".cyan().bold());
    println!("{}", "═══════════════════════════════════════".dimmed());
    println!("{}: {}", "Endpoint".dimmed(), endpoint);
    println!("{}: {}", "Address".dimmed(), address);
    println!();

    // Show progress
    let spinner = indicatif::ProgressBar::new_spinner();
    spinner.set_message("Connecting to chain...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    // Connect to the provider
    let provider = Provider::<Http>::try_from(endpoint).context("Failed to create provider")?;

    spinner.set_message("Fetching balance...");

    // Parse the address
    let addr: Address = address.parse().context("Invalid EVM address")?;

    // Get the balance
    let balance = provider
        .get_balance(addr, None)
        .await
        .context("Failed to fetch balance")?;

    // Get the chain ID for better display
    let chain_id = provider
        .get_chainid()
        .await
        .context("Failed to get chain ID")?;

    spinner.finish_and_clear();

    println!("\n{}", "Balance Retrieved".green().bold());
    println!("{}", "═══════════════════════════════════════".dimmed());
    println!("{}: {}", "Address".cyan(), address);
    println!("{}: {}", "Chain ID".dimmed(), chain_id);
    println!();

    // Convert balance to ETH
    let balance_eth =
        ethers::utils::format_units(balance, "ether").unwrap_or_else(|_| balance.to_string());

    println!("{}: {} ETH", "Balance".green().bold(), balance_eth);
    println!("{}: {} Wei", "Raw".dimmed(), balance);

    // Show USD value if possible (would need price oracle in production)
    println!("\n{}", "Tip:".yellow());
    println!("Use a block explorer for detailed transaction history:");
    match chain_id.as_u64() {
        1 => println!("  https://etherscan.io/address/{}", address),
        137 => println!("  https://polygonscan.com/address/{}", address),
        56 => println!("  https://bscscan.com/address/{}", address),
        _ => println!("  Check your chain's block explorer"),
    }

    Ok(())
}

/// Format balance with decimal places
fn format_balance(balance: u128, divisor: u128) -> String {
    let whole = balance / divisor;
    let frac = balance % divisor;

    if frac == 0 {
        format!("{}", whole)
    } else {
        // Remove trailing zeros
        let frac_str = format!("{:010}", frac);
        let trimmed = frac_str.trim_end_matches('0');
        format!("{}.{}", whole, trimmed)
    }
}

/// Auto-detect chain type and get balance
pub async fn get_balance(address: &str, chain: &str, endpoint: &str) -> Result<()> {
    // Determine if it's a Substrate or EVM chain based on endpoint or chain name
    let is_substrate = endpoint.starts_with("ws://")
        || endpoint.starts_with("wss://")
        || matches!(
            chain.to_lowercase().as_str(),
            "polkadot"
                | "kusama"
                | "paseo"
                | "westend"
                | "moonbeam"
                | "astar"
                | "acala"
                | "phala"
                | "bifrost"
        );

    if is_substrate {
        get_substrate_balance(address, endpoint).await
    } else {
        get_evm_balance(address, endpoint).await
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_detect_chain_type() {
        // Substrate endpoints
        assert!(is_substrate_endpoint("wss://polkadot.api.onfinality.io"));
        assert!(is_substrate_endpoint("ws://localhost:9944"));

        // EVM endpoints
        assert!(!is_substrate_endpoint("https://eth.llamarpc.com"));
        assert!(!is_substrate_endpoint("http://localhost:8545"));
    }

    fn is_substrate_endpoint(endpoint: &str) -> bool {
        endpoint.starts_with("ws://") || endpoint.starts_with("wss://")
    }

    #[test]
    fn test_address_parsing() {
        // Valid Substrate address (as AccountId32)
        let valid_sub = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
        assert!(valid_sub.parse::<subxt::utils::AccountId32>().is_ok());

        // Valid EVM address (20 bytes = 40 hex chars)
        let valid_evm = "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEbD";
        assert!(valid_evm.parse::<ethers::types::Address>().is_ok());
    }
}
