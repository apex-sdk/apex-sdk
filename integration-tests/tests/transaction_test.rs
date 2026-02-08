//! Transaction Integration Tests
//!
//! These tests execute ACTUAL transactions on testnets.
//! They are disabled by default to prevent accidental testnet spending.
//!
//! To run:
//! ```bash
//! # EVM tests (requires PRIVATE_KEY and Sepolia ETH)
//! PRIVATE_KEY=0x... REAL_TX_TESTS=1 cargo test --test real_transaction_test test_evm_real_transfer -- --ignored --nocapture
//!
//! # Substrate tests (requires SUBSTRATE_SEED and Westend tokens)
//! SUBSTRATE_SEED="your twelve word seed phrase" REAL_TX_TESTS=1 cargo test --test real_transaction_test test_substrate_real_transfer -- --ignored --nocapture
//! ```

// use alloy_primitives::U256; // Unused
use apex_sdk::prelude::*;
use apex_sdk_substrate::Wallet as SubstrateWallet;

/// Skip test if REAL_TX_TESTS environment variable is not set
macro_rules! skip_if_not_real_tx_test {
    () => {
        if std::env::var("REAL_TX_TESTS").is_err() {
            eprintln!("Skipping real transaction test. Set REAL_TX_TESTS=1 to run.");
            return;
        }
    };
}

#[tokio::test]
#[ignore] // Must be explicitly enabled with --ignored
async fn test_substrate_real_transfer_on_westend() {
    skip_if_not_real_tx_test!();

    println!("\n=== Real Substrate Transaction Test on Westend ===\n");

    // Get seed phrase from environment
    let seed = std::env::var("SUBSTRATE_SEED").expect(
        "SUBSTRATE_SEED environment variable required. Set it to your testnet seed phrase.",
    );

    // Create wallet from mnemonic phrase
    let wallet = SubstrateWallet::from_mnemonic(&seed, apex_sdk_substrate::KeyPairType::Sr25519)
        .expect("Failed to create wallet");

    let from_address = wallet.address();
    println!("From Address: {}", from_address);

    // Recipient address (another test account)
    let to_address = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";

    // Create SDK with wallet
    let sdk = ApexSDK::builder()
        .with_substrate_endpoint("wss://westend-rpc.polkadot.io")
        .with_substrate_wallet(wallet)
        .build()
        .await
        .expect("Failed to build SDK");

    println!("✓ SDK initialized with Westend endpoint");

    // Get initial balance
    let adapter = sdk
        .substrate()
        .expect("Substrate adapter should be available");
    let initial_balance = adapter
        .get_balance(&from_address)
        .await
        .expect("Failed to get balance");

    println!("Initial balance: {} planck", initial_balance);

    if initial_balance < 1_000_000_000_000u128 {
        panic!(
            "Insufficient balance for test. Need at least 1 WND, have {} planck",
            initial_balance
        );
    }

    // Build transaction (0.01 WND = 10000000000 planck)
    let amount = 10_000_000_000u128;
    let tx = sdk
        .transaction()
        .from_substrate_account(&from_address)
        .to_substrate_account(to_address)
        .amount(amount)
        .build()
        .expect("Failed to build transaction");

    println!("\nExecuting transaction...");
    println!("  To: {}", to_address);
    println!("  Amount: {} planck (0.01 WND)", amount);

    // Execute transaction
    let result = sdk.execute(tx).await.expect("Transaction execution failed");

    println!("\n✓ Transaction executed successfully!");
    println!("  TX Hash: {}", result.source_tx_hash);
    println!("  Status: {:?}", result.status);

    // Verify transaction was actually sent
    assert!(!result.source_tx_hash.is_empty());
    assert!(result.source_tx_hash.starts_with("0x"));

    println!("\nTransaction Details:");
    println!(
        "  View on Subscan: https://westend.subscan.io/extrinsic/{}",
        result.source_tx_hash
    );
    println!("\n=== Real Transaction Test PASSED ===\n");
}
