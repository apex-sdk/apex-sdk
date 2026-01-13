//! EVM Transfer Example - Real Testnet Transaction
//!
//! This example demonstrates how to execute a real ETH transfer transaction
//! on Ethereum Sepolia testnet using Apex SDK.
//!
//! **Prerequisites:**
//! - Test ETH on Sepolia (get from https://sepoliafaucet.com)
//! - Your private key
//!
//! **How to run:**
//! ```bash
//! export PRIVATE_KEY=0x...
//! cargo run --example evm-transfer
//! ```
//!
//! **Security Warning:**
//! - Never use your mainnet private key for testing
//! - Never commit private keys to version control
//! - Use environment variables or a secure key management system

use alloy_primitives::U256;
use apex_sdk::prelude::*;
use apex_sdk_evm::wallet::Wallet;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    println!("=== Apex SDK: EVM Transfer Example ===\n");
    println!("This example executes a real ETH transfer on Sepolia testnet.\n");

    // Get private key from environment variable
    let private_key = std::env::var("PRIVATE_KEY")
        .unwrap_or_else(|_| {
            eprintln!("Error: PRIVATE_KEY environment variable not set");
            eprintln!("\nUsage:");
            eprintln!("  export PRIVATE_KEY=0x...");
            eprintln!("  cargo run --example evm-transfer");
            eprintln!("\nGet Sepolia test ETH from: https://sepoliafaucet.com");
            std::process::exit(1);
        });

    // Create wallet from private key
    println!("Creating wallet...");
    let wallet = Wallet::from_private_key(&private_key)
        .map_err(|e| apex_sdk::Error::Transaction(e.to_string()))?
        .with_chain_id(11155111); // Sepolia chain ID

    let wallet_address = wallet.eth_address();
    println!("  Wallet address: {:?}", wallet_address);
    println!();

    // Initialize SDK with Sepolia endpoint
    println!("Connecting to Sepolia testnet...");
    let sdk = ApexSDK::builder()
        .with_evm_endpoint("https://eth-sepolia.g.alchemy.com/v2/demo")
        .with_evm_wallet(wallet)
        .build()
        .await?;

    println!("  ✓ Connected to Sepolia");
    println!();

    // Get adapter to check balance
    let adapter = sdk.evm()?;
    let from_address = format!("{:?}", wallet_address);

    // Check balance
    println!("Checking balance...");
    let balance = adapter
        .get_balance(&from_address)
        .await
        .map_err(|e| apex_sdk::Error::Transaction(e.to_string()))?;
    println!("  Current balance: {} wei ({} ETH)",
        balance,
        balance.to_string().parse::<f64>().unwrap_or(0.0) / 1e18
    );

    // Check if sufficient balance for transfer + gas
    if balance < U256::from(100_000_000_000_000u128) {
        eprintln!("\nError: Insufficient balance");
        eprintln!("   Need at least 0.0001 ETH for transfer + gas fees");
        eprintln!("   Get test ETH from: https://sepoliafaucet.com");
        std::process::exit(1);
    }
    println!();

    // Prepare transaction
    let recipient = "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045"; // Well-known address
    let amount = 10_000_000_000_000u128; // 0.00001 ETH

    println!("Transaction Details:");
    println!("  From:   {}", from_address);
    println!("  To:     {}", recipient);
    println!("  Amount: {} wei (0.00001 ETH)", amount);
    println!("  Chain:  Sepolia (chain ID: 11155111)");
    println!();

    // Build transaction
    println!("Building transaction...");
    let tx = sdk
        .transaction()
        .from_evm_address(&from_address)
        .to_evm_address(recipient)
        .amount(amount)
        .build()?;

    println!("  ✓ Transaction built");
    println!();

    // Execute transaction
    println!("Signing and sending transaction...");
    println!("(This will use real test ETH on Sepolia)");
    println!();

    let result = sdk.execute(tx).await?;

    // Display results
    println!("Transaction Successful!");
    println!();
    println!("Transaction Hash:");
    println!("  {}", result.source_tx_hash);
    println!();
    println!("Status: {:?}", result.status);
    println!();
    println!("View on Etherscan:");
    println!("  https://sepolia.etherscan.io/tx/{}", result.source_tx_hash);
    println!();

    println!("=== Example Complete ===");
    println!();
    println!("What happened:");
    println!("  1. Created wallet from private key");
    println!("  2. Connected to Sepolia testnet via RPC");
    println!("  3. Checked account balance");
    println!("  4. Built transaction with recipient and amount");
    println!("  5. Signed transaction with wallet's private key");
    println!("  6. Sent signed transaction to network");
    println!("  7. Received transaction hash");
    println!();
    println!("Key Features Demonstrated:");
    println!("  ✓ Real wallet integration with private key signing");
    println!("  ✓ Type-safe transaction building");
    println!("  ✓ Actual EVM transaction execution");
    println!("  ✓ Transaction confirmation and hash retrieval");

    Ok(())
}
