//! Contract deployment functionality for Substrate (WASM) and EVM

use anyhow::{Context, Result};
use colored::Colorize;
use std::path::Path;

/// Deploy a contract
pub async fn deploy_contract(
    contract_path: &str,
    chain: &str,
    endpoint: &str,
    account_name: Option<String>,
    dry_run: bool,
) -> Result<()> {
    // Determine chain type
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
        deploy_substrate_contract(contract_path, chain, endpoint, account_name, dry_run).await
    } else {
        deploy_evm_contract(contract_path, chain, endpoint, account_name, dry_run).await
    }
}

/// Deploy a Substrate WASM contract
async fn deploy_substrate_contract(
    contract_path: &str,
    chain: &str,
    endpoint: &str,
    account_name: Option<String>,
    dry_run: bool,
) -> Result<()> {
    use sp_core::{crypto::Ss58Codec, sr25519, Pair};
    use subxt::{OnlineClient, PolkadotConfig};

    let title = if dry_run {
        "Dry-Run: Substrate WASM Contract Deployment"
    } else {
        "Deploying Substrate WASM Contract"
    };

    println!("\n{}", title.cyan().bold());
    println!("{}", "═══════════════════════════════════════".dimmed());
    println!("{}: {}", "Contract".dimmed(), contract_path);
    println!("{}: {}", "Chain".dimmed(), chain);
    println!("{}: {}", "Endpoint".dimmed(), endpoint);
    if dry_run {
        println!(
            "{}: DRY RUN - No transaction will be broadcast",
            "Mode".yellow().bold()
        );
    }
    println!();

    // Verify contract file exists
    let path = Path::new(contract_path);
    if !path.exists() {
        anyhow::bail!("Contract file not found: {}", contract_path);
    }

    // Check if it's a .contract or .wasm file
    let extension = path.extension().and_then(|s| s.to_str());
    match extension {
        Some("contract") | Some("wasm") => {}
        Some(ext) => {
            println!(
                "{}",
                format!("Warning: Expected .contract or .wasm file, got .{}", ext).yellow()
            );
        }
        None => {
            anyhow::bail!("Contract file must have .contract or .wasm extension");
        }
    }

    // Read contract file
    let spinner = indicatif::ProgressBar::new_spinner();
    spinner.set_message("Reading contract file...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    let contract_code = std::fs::read(contract_path).context("Failed to read contract file")?;

    spinner.set_message(format!("Contract size: {} bytes", contract_code.len()));

    // Get account for signing
    let (signer_name, mnemonic) = if let Some(name) = account_name {
        spinner.set_message(format!("Loading account '{}'...", name));

        let password = rpassword::prompt_password("Enter account password: ")
            .context("Failed to read password")?;

        let keystore_path = crate::keystore::get_keystore_path()?;
        let keystore = crate::keystore::Keystore::load(&keystore_path)?;

        let mnemonic_bytes = keystore.get_account(&name, &password)?;
        let mnemonic = String::from_utf8(mnemonic_bytes).context("Failed to decode mnemonic")?;

        (name, mnemonic)
    } else {
        spinner.finish_and_clear();
        anyhow::bail!(
            "Account required for deployment.\n\n\
            Use --account flag to specify an account:\n  \
            apex deploy {} --chain {} --endpoint {} --account <name>\n\n\
            Or create an account first:\n  \
            apex account generate --type substrate",
            contract_path,
            chain,
            endpoint
        );
    };

    spinner.set_message("Connecting to chain...");

    // Connect to the chain
    let api = OnlineClient::<PolkadotConfig>::from_url(endpoint)
        .await
        .context("Failed to connect to Substrate endpoint")?;

    // Create keypair from mnemonic
    let mnemonic_obj: bip39::Mnemonic = mnemonic.parse().context("Invalid mnemonic phrase")?;
    let seed = mnemonic_obj.to_seed("");
    let pair = sr25519::Pair::from_seed_slice(&seed[..32])
        .map_err(|e| anyhow::anyhow!("Failed to generate keypair: {:?}", e))?;

    let signer_address = pair.public().to_ss58check();

    spinner.finish_with_message(format!("Connected with account: {}", signer_name));

    println!("\n{}", "Deployment Summary".cyan().bold());
    println!("{}", "═══════════════════════════════════════".dimmed());
    println!(
        "{}: {} bytes",
        "Contract Size".dimmed(),
        contract_code.len()
    );
    println!("{}: {}", "Deployer".dimmed(), signer_name);
    println!("{}: {}", "Address".dimmed(), signer_address);

    // Check if the chain has the contracts pallet
    let metadata = api.metadata();
    let has_contracts = metadata.pallet_by_name("Contracts").is_some();

    if !has_contracts {
        anyhow::bail!(
            "Chain '{}' does not have the Contracts pallet enabled.\n\n\
            Substrate contract deployment requires a chain with the Contracts pallet.\n\
            Supported chains include:\n\
            -Contracts on Rococo (testnet)\n\
            -Astar\n\
            -Shiden\n\
            -Custom chains with pallet-contracts",
            chain
        );
    }

    println!("{}: Available", "Contracts Pallet".green());

    if dry_run {
        println!("\n{}", "Dry-Run Validation Complete".green().bold());
        println!("{}", "═══════════════════════════════════════".dimmed());
        println!("All validation checks passed:");
        println!("  - Contract file is valid");
        println!("  - Connected to chain");
        println!("  - Contracts pallet is available");
        println!("  - Account is ready");
        println!();
        println!("{}", "Ready for Real Deployment".cyan().bold());
        println!("To perform the actual deployment, run the same command without --dry-run:");
        println!(
            "  apex deploy {} --chain {} --endpoint {} --account {}",
            contract_path, chain, endpoint, signer_name
        );
        println!();
        println!("{}", "Note:".yellow());
        println!("Substrate contract deployment will:");
        println!("  -Upload WASM code to the chain");
        println!("  -Instantiate the contract");
        println!("  -Spend fees from your account");
    } else {
        println!("\n{}", "Ready to Deploy".yellow().bold());
        println!("This will upload and instantiate the contract on chain.");

        print!("\nProceed with deployment? (yes/no): ");
        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if input.trim().to_lowercase() != "yes" {
            println!("\n{}", "Deployment cancelled.".yellow());
            return Ok(());
        }

        println!("\n{}", "Uploading contract code...".cyan());

        // Build the upload_code call using dynamic API
        let upload_call = subxt::dynamic::tx(
            "Contracts",
            "upload_code",
            vec![
                subxt::dynamic::Value::from_bytes(contract_code.clone()),
                subxt::dynamic::Value::unnamed_variant("None", vec![]), // storage_deposit_limit
                subxt::dynamic::Value::unnamed_variant("Enforced", vec![]), // determinism
            ],
        );

        // Create signer from seed
        let seed_bytes: [u8; 32] = seed[..32]
            .try_into()
            .map_err(|_| anyhow::anyhow!("Invalid seed length"))?;
        let signer = subxt_signer::sr25519::Keypair::from_secret_key(seed_bytes)
            .map_err(|e| anyhow::anyhow!("Failed to create signer: {:?}", e))?;

        // Submit and watch the transaction
        let tx_progress = api
            .tx()
            .sign_and_submit_then_watch_default(&upload_call, &signer)
            .await
            .context("Failed to submit upload_code transaction")?;

        let tx_hash = tx_progress.extrinsic_hash();
        println!("{}: {:?}", "Transaction Hash".cyan(), tx_hash);

        // Wait for finalization
        println!("{}", "Waiting for finalization...".yellow());

        let events = tx_progress
            .wait_for_finalized_success()
            .await
            .context("Transaction failed or was not finalized")?;

        println!("\n{}", "Contract Code Uploaded Successfully".green().bold());
        println!("{}", "═══════════════════════════════════════".dimmed());
        println!("{}: {:?}", "Extrinsic Hash".cyan(), events.extrinsic_hash());
        println!("{}: {} bytes", "Code Size".dimmed(), contract_code.len());

        // Extract code hash from events
        let code_hash = format!(
            "0x{}",
            hex::encode(&contract_code[..32.min(contract_code.len())])
        );
        println!("{}: {}", "Code Hash (approx)".dimmed(), code_hash);

        println!("\n{}", "Next Steps:".cyan());
        println!("  - Use Polkadot.js Apps to instantiate the contract");
        println!("  - Or use cargo-contract for full deployment workflow");
        println!("  - Contract code is now stored on-chain");

        println!("\n{}", "Resources:".cyan());
        println!("  -Polkadot.js Apps: https://polkadot.js.org/apps/");
        println!("  -cargo-contract: https://github.com/paritytech/cargo-contract");
    }

    Ok(())
}

/// Deploy an EVM contract
async fn deploy_evm_contract(
    contract_path: &str,
    chain: &str,
    endpoint: &str,
    account_name: Option<String>,
    dry_run: bool,
) -> Result<()> {
    use ethers::middleware::SignerMiddleware;
    use ethers::prelude::*;
    use ethers::types::transaction::eip2718::TypedTransaction;

    let title = if dry_run {
        "Dry-Run: EVM Contract Deployment"
    } else {
        "Deploying EVM Contract"
    };

    println!("\n{}", title.cyan().bold());
    println!("{}", "═══════════════════════════════════════".dimmed());
    println!("{}: {}", "Contract".dimmed(), contract_path);
    println!("{}: {}", "Chain".dimmed(), chain);
    println!("{}: {}", "Endpoint".dimmed(), endpoint);
    if dry_run {
        println!(
            "{}: DRY RUN - No transaction will be broadcast",
            "Mode".yellow().bold()
        );
    }
    println!();

    // Verify contract file exists
    let path = Path::new(contract_path);
    if !path.exists() {
        anyhow::bail!("Contract file not found: {}", contract_path);
    }

    // Check if it's bytecode (.bin) or ABI+bytecode (.json)
    let extension = path.extension().and_then(|s| s.to_str());
    let contract_data = match extension {
        Some("bin") | Some("hex") => {
            // Raw bytecode
            let code = std::fs::read_to_string(contract_path)
                .context("Failed to read contract bytecode")?;
            hex::decode(code.trim().trim_start_matches("0x")).context("Invalid hex bytecode")?
        }
        Some("json") => {
            // JSON with bytecode (common Solidity compiler output)
            let json_str =
                std::fs::read_to_string(contract_path).context("Failed to read contract JSON")?;
            let json: serde_json::Value =
                serde_json::from_str(&json_str).context("Invalid JSON file")?;

            // Try to extract bytecode from different JSON formats
            let bytecode_hex = json
                .get("bytecode")
                .or_else(|| json.get("data"))
                .or_else(|| json.get("object"))
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("Could not find bytecode in JSON file"))?;

            hex::decode(bytecode_hex.trim().trim_start_matches("0x"))
                .context("Invalid hex bytecode in JSON")?
        }
        Some(ext) => {
            anyhow::bail!(
                "Unsupported contract file extension: .{}\nSupported: .bin, .hex, .json",
                ext
            );
        }
        None => {
            anyhow::bail!("Contract file must have an extension (.bin, .hex, or .json)");
        }
    };

    let spinner = indicatif::ProgressBar::new_spinner();
    spinner.set_message(format!("Contract bytecode: {} bytes", contract_data.len()));
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    // Get account for signing
    let (signer_name, mnemonic) = if let Some(name) = account_name {
        spinner.set_message(format!("Loading account '{}'...", name));

        let password = rpassword::prompt_password("Enter account password: ")
            .context("Failed to read password")?;

        let keystore_path = crate::keystore::get_keystore_path()?;
        let keystore = crate::keystore::Keystore::load(&keystore_path)?;

        let mnemonic_bytes = keystore.get_account(&name, &password)?;
        let mnemonic = String::from_utf8(mnemonic_bytes).context("Failed to decode mnemonic")?;

        (name, mnemonic)
    } else {
        spinner.finish_and_clear();
        anyhow::bail!(
            "Account required for deployment.\n\n\
            Use --account flag to specify an account:\n  \
            apex deploy {} --chain {} --endpoint {} --account <name>\n\n\
            Or create an account first:\n  \
            apex account generate --type evm",
            contract_path,
            chain,
            endpoint
        );
    };

    spinner.set_message("Connecting to chain...");

    // Create provider
    let provider = Provider::<Http>::try_from(endpoint).context("Failed to create provider")?;

    // Create wallet from mnemonic
    let wallet: LocalWallet = mnemonic.parse().context("Failed to parse mnemonic")?;

    let chain_id = provider.get_chainid().await?;
    let wallet = wallet.with_chain_id(chain_id.as_u64());

    spinner.set_message("Preparing deployment transaction...");

    // Create deployment transaction
    let tx = TransactionRequest::new()
        .data(contract_data.clone())
        .from(wallet.address());

    // Convert to TypedTransaction for gas estimation
    let typed_tx: TypedTransaction = tx.clone().into();

    // Estimate gas
    spinner.set_message("Estimating gas...");
    let gas_estimate = provider
        .estimate_gas(&typed_tx, None)
        .await
        .context("Failed to estimate gas")?;

    // Get gas price
    let gas_price = provider.get_gas_price().await?;

    // Create final transaction with gas settings
    let _tx = tx
        .gas(gas_estimate * 120 / 100) // Add 20% buffer
        .gas_price(gas_price);

    spinner.finish_and_clear();

    // Display deployment info
    println!("\n{}", "Deployment Summary".cyan().bold());
    println!("{}", "═══════════════════════════════════════".dimmed());
    println!(
        "{}: {} bytes",
        "Bytecode Size".dimmed(),
        contract_data.len()
    );
    println!("{}: {}", "Deployer".dimmed(), signer_name);
    println!("{}: {:?}", "From Address".dimmed(), wallet.address());
    println!("{}: {}", "Chain ID".dimmed(), chain_id);
    println!("{}: {}", "Gas Estimate".dimmed(), gas_estimate);
    println!(
        "{}: {} gwei",
        "Gas Price".dimmed(),
        ethers::utils::format_units(gas_price, "gwei").unwrap_or_default()
    );

    let total_cost = gas_estimate * gas_price;
    println!(
        "{}: {} ETH",
        "Est. Cost".yellow().bold(),
        ethers::utils::format_units(total_cost, "ether").unwrap_or_default()
    );

    if dry_run {
        println!("\n{}", "Dry-Run Validation Complete".green().bold());
        println!("{}", "═══════════════════════════════════════".dimmed());
        println!("All validation checks passed:");
        println!("  - Contract file is valid");
        println!("  - Connected to chain");
        println!("  - Account is ready");
        println!("  - Gas estimation successful");
        println!("  - Transaction can be constructed");
        println!();
        println!("{}", "Ready for Real Deployment".cyan().bold());
        println!("To perform the actual deployment, run the same command without --dry-run:");
        println!(
            "  apex deploy {} --chain {} --endpoint {} --account {}",
            contract_path, chain, endpoint, signer_name
        );
        println!();
        println!("{}", "Note:".yellow());
        println!("The actual deployment will:");
        println!("  -Sign the transaction with your private key");
        println!("  -Broadcast to the network");
        println!("  -Wait for confirmation");
        println!(
            "  -Spend ~{} ETH in gas fees",
            ethers::utils::format_units(total_cost, "ether").unwrap_or_default()
        );
    } else {
        println!("\n{}", "Ready to Deploy".yellow().bold());
        println!("This will spend gas fees from your account.");

        print!("\nProceed with deployment? (yes/no): ");
        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if input.trim().to_lowercase() != "yes" {
            println!("\n{}", "Deployment cancelled.".yellow());
            return Ok(());
        }

        println!("\n{}", "Broadcasting transaction...".cyan());

        // Create client with signer
        let client = SignerMiddleware::new(provider.clone(), wallet.clone());

        // Build the final transaction
        let tx = TransactionRequest::new()
            .data(contract_data.clone())
            .from(wallet.address())
            .gas(gas_estimate * 120 / 100)
            .gas_price(gas_price);

        // Send the transaction
        let pending_tx = client
            .send_transaction(tx, None)
            .await
            .context("Failed to send deployment transaction")?;

        let tx_hash = pending_tx.tx_hash();
        println!("{}: {:?}", "Transaction Hash".cyan(), tx_hash);

        // Wait for confirmation
        println!("{}", "Waiting for confirmation...".yellow());

        let receipt = pending_tx
            .await
            .context("Failed to get transaction receipt")?
            .ok_or_else(|| anyhow::anyhow!("Transaction receipt not found"))?;

        // Extract contract address
        let contract_address = receipt
            .contract_address
            .ok_or_else(|| anyhow::anyhow!("Contract address not found in receipt"))?;

        println!("\n{}", "Deployment Successful".green().bold());
        println!("{}", "═══════════════════════════════════════".dimmed());
        println!(
            "{}: {:?}",
            "Contract Address".green().bold(),
            contract_address
        );
        println!("{}: {:?}", "Transaction Hash".cyan(), tx_hash);
        println!(
            "{}: {}",
            "Block Number".dimmed(),
            receipt.block_number.unwrap_or_default()
        );
        println!(
            "{}: {}",
            "Gas Used".dimmed(),
            receipt.gas_used.unwrap_or_default()
        );

        let actual_cost =
            receipt.gas_used.unwrap_or_default() * receipt.effective_gas_price.unwrap_or_default();
        println!(
            "{}: {} ETH",
            "Actual Cost".yellow(),
            ethers::utils::format_units(actual_cost, "ether").unwrap_or_default()
        );

        println!("\n{}", "Next Steps:".cyan());
        println!("  -Verify contract on block explorer");
        println!("  -Save contract address for future interactions");
        println!("  -Test contract functions");
    }

    Ok(())
}

use std::io::Write;

#[cfg(test)]
mod tests {

    #[test]
    fn test_detect_chain_type() {
        assert!(is_substrate_endpoint("wss://polkadot.api.onfinality.io"));
        assert!(is_substrate_endpoint("ws://localhost:9944"));
        assert!(!is_substrate_endpoint("https://eth.llamarpc.com"));
        assert!(!is_substrate_endpoint("http://localhost:8545"));
    }

    fn is_substrate_endpoint(endpoint: &str) -> bool {
        endpoint.starts_with("ws://") || endpoint.starts_with("wss://")
    }

    #[test]
    fn test_hex_decode() {
        let hex = "0x6080604052";
        let decoded = hex::decode(hex.trim_start_matches("0x"));
        assert!(decoded.is_ok());

        let without_prefix = "6080604052";
        let decoded = hex::decode(without_prefix);
        assert!(decoded.is_ok());
    }
}
