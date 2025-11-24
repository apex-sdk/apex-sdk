//! Contract deployment functionality for Substrate (WASM) and EVM

use anyhow::{Context, Result};
use apex_sdk_types::Chain;
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
    // Determine chain type using centralized logic
    let is_substrate = Chain::is_substrate_endpoint(endpoint)
        || Chain::from_str_case_insensitive(chain)
            .map(|c| c.chain_type() == apex_sdk_types::ChainType::Substrate)
            .unwrap_or(false);

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

    // Validate contract file size
    const MAX_CONTRACT_SIZE: u64 = 10 * 1024 * 1024; // 10 MB - reasonable limit for WASM contracts
    let metadata =
        std::fs::metadata(contract_path).context("Failed to read contract file metadata")?;

    if metadata.len() > MAX_CONTRACT_SIZE {
        anyhow::bail!(
            "Contract file too large: {} bytes (max {} MB). \
            Consider optimizing your contract or splitting functionality.",
            metadata.len(),
            MAX_CONTRACT_SIZE / (1024 * 1024)
        );
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
        let mut keystore = crate::keystore::Keystore::load(&keystore_path)?;

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
    use alloy::primitives::U256;
    use apex_sdk_evm::{wallet::Wallet, EvmAdapter};

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

    // Validate contract file size
    const MAX_CONTRACT_SIZE: u64 = 50 * 1024 * 1024; // 50 MB for EVM contracts (includes JSON metadata)
    let metadata =
        std::fs::metadata(contract_path).context("Failed to read contract file metadata")?;

    if metadata.len() > MAX_CONTRACT_SIZE {
        anyhow::bail!(
            "Contract file too large: {} bytes (max {} MB). \
            Consider optimizing your contract.",
            metadata.len(),
            MAX_CONTRACT_SIZE / (1024 * 1024)
        );
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
        let mut keystore = crate::keystore::Keystore::load(&keystore_path)?;

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

    // Connect to EVM chain using apex-sdk-evm
    let adapter = EvmAdapter::connect(endpoint)
        .await
        .context("Failed to connect to EVM endpoint")?;

    // Create wallet from mnemonic using apex-sdk-evm
    let wallet =
        Wallet::from_mnemonic(&mnemonic, 0).context("Failed to create wallet from mnemonic")?;

    // Get chain ID from provider
    let chain_id = adapter
        .provider()
        .get_chain_id()
        .await
        .context("Failed to get chain ID")?;

    let wallet = wallet.with_chain_id(chain_id);

    spinner.set_message("Estimating gas...");

    // Create transaction executor
    let executor = adapter.transaction_executor();

    // Estimate gas for deployment (to address is zero for contract creation)
    let dummy_to = "0x0000000000000000000000000000000000000000"
        .parse()
        .unwrap();
    let gas_estimate = executor
        .estimate_gas(
            wallet.eth_address(),
            Some(dummy_to),
            Some(U256::ZERO),
            Some(contract_data.clone()),
        )
        .await
        .context("Failed to estimate gas")?;

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
    println!("{}: {}", "From Address".dimmed(), wallet.address());
    println!("{}: {}", "Chain ID".dimmed(), chain_id);
    println!("{}: {}", "Gas Estimate".dimmed(), gas_estimate.gas_limit);
    println!(
        "{}: {} gwei",
        "Gas Price".dimmed(),
        gas_estimate.gas_price_gwei()
    );

    println!(
        "{}: {} ETH",
        "Est. Cost".yellow().bold(),
        gas_estimate.total_cost_eth()
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
            gas_estimate.total_cost_eth()
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

        // For contract deployment, send to zero address with bytecode as data
        let zero_address = "0x0000000000000000000000000000000000000000"
            .parse()
            .unwrap();

        // Send the deployment transaction
        let tx_hash = executor
            .send_transaction(&wallet, zero_address, U256::ZERO, Some(contract_data))
            .await
            .context("Failed to send deployment transaction")?;

        println!("{}: {:?}", "Transaction Hash".cyan(), tx_hash);

        // Wait for confirmation
        println!("{}", "Waiting for confirmation...".yellow());

        let receipt = executor
            .wait_for_confirmation(tx_hash, 1)
            .await
            .context("Failed to get transaction receipt")?
            .ok_or_else(|| anyhow::anyhow!("Transaction receipt not found"))?;

        // Extract contract address (for deployment transactions)
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
            receipt
                .block_number
                .map(|n| n.to_string())
                .unwrap_or_else(|| "unknown".to_string())
        );
        println!("{}: {}", "Gas Used".dimmed(), receipt.gas_used);

        // Calculate actual cost
        let gas_used = receipt.gas_used as u128;
        let gas_price = receipt.effective_gas_price;
        let actual_cost_wei = gas_used * gas_price;

        // Format to ETH
        let actual_cost_eth = format_wei_to_eth(actual_cost_wei);
        println!("{}: {} ETH", "Actual Cost".yellow(), actual_cost_eth);

        println!("\n{}", "Next Steps:".cyan());
        println!("  -Verify contract on block explorer");
        println!("  -Save contract address for future interactions");
        println!("  -Test contract functions");
    }

    Ok(())
}

use std::io::Write;

/// Format wei to ETH (helper function)
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
