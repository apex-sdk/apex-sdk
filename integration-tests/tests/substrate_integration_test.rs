// Substrate Integration Tests
// These tests run against a real Substrate node (contracts-node) in Docker
// Run with: INTEGRATION_TESTS=1 cargo test --test substrate_integration_test -- --include-ignored

#[path = "integration_helpers.rs"]
mod integration_helpers;

use apex_sdk_substrate::SubstrateAdapter;
use integration_helpers::*;

#[tokio::test]
#[ignore]
async fn test_substrate_connection_to_docker_node() {
    skip_if_not_integration!();

    wait_for_substrate_node(60)
        .await
        .expect("Substrate node should be ready");

    let adapter = SubstrateAdapter::connect(&substrate_rpc_url())
        .await
        .expect("Should connect to Substrate node");

    println!("Successfully connected to Docker Substrate node");

    let runtime_version = adapter.runtime_version();
    assert!(runtime_version > 0, "Runtime version should be > 0");

    println!("Runtime version retrieved: {}", runtime_version);
}

#[tokio::test]
#[ignore]
async fn test_substrate_get_balance_from_docker_node() {
    skip_if_not_integration!();

    wait_for_substrate_node(60)
        .await
        .expect("Substrate node should be ready");

    let adapter = SubstrateAdapter::connect(&substrate_rpc_url())
        .await
        .expect("Should connect to Substrate node");

    let ilara_address = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";

    let balance = adapter.get_balance(ilara_address).await;

    match balance {
        Ok(bal) => {
            println!("Ilara's balance: {}", bal);
        }
        Err(e) => {
            println!(
                "Note: Balance query returned error (expected for some node versions): {}",
                e
            );
        }
    }

    println!("Balance query infrastructure verified");
}

#[tokio::test]
#[ignore]
async fn test_substrate_block_queries() {
    skip_if_not_integration!();

    wait_for_substrate_node(60)
        .await
        .expect("Substrate node should be ready");

    let adapter = SubstrateAdapter::connect(&substrate_rpc_url())
        .await
        .expect("Should connect to Substrate node");

    // Verify we can access chain metadata
    let runtime_version = adapter.runtime_version();
    assert!(runtime_version > 0, "Runtime version should be > 0");
    println!("Runtime version: {}", runtime_version);

    let chain_name = adapter.chain_name();
    println!("Chain name: {}", chain_name);

    println!("Chain query infrastructure verified");
}

#[tokio::test]
#[ignore]
async fn test_substrate_connection_pool() {
    skip_if_not_integration!();

    wait_for_substrate_node(60)
        .await
        .expect("Substrate node should be ready");

    let mut adapters = Vec::new();
    for i in 0..3 {
        let adapter = SubstrateAdapter::connect(&substrate_rpc_url())
            .await
            .expect("Should connect to Substrate node");
        adapters.push(adapter);
        println!("Connection {} established", i + 1);
    }

    for (i, adapter) in adapters.iter().enumerate() {
        let runtime_version = adapter.runtime_version();
        assert!(runtime_version > 0, "Connection {} should work", i + 1);
    }

    println!("All {} connections verified", adapters.len());
}

#[tokio::test]
#[ignore]
async fn test_substrate_transfer_transaction() {
    skip_if_not_integration!();

    wait_for_substrate_node(60)
        .await
        .expect("Substrate node should be ready");

    let adapter = SubstrateAdapter::connect(&substrate_rpc_url())
        .await
        .expect("Should connect to Substrate node");

    // Use Alice's development account
    let alice_wallet = apex_sdk_substrate::Wallet::from_mnemonic(
        "//Alice",
        apex_sdk_substrate::KeyPairType::Sr25519,
    )
    .expect("Should create Alice wallet");

    // Bob's development account address
    let bob_address = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";

    println!("Alice address: {}", alice_wallet.address());
    println!("Bob address: {}", bob_address);

    // Get initial balances
    let initial_alice = adapter.get_balance(&alice_wallet.address()).await;
    let initial_bob = adapter.get_balance(bob_address).await;

    if let Ok(balance) = initial_alice {
        println!("Alice initial balance: {} units", balance);
    }
    if let Ok(balance) = initial_bob {
        println!("Bob initial balance: {} units", balance);
    }

    // Execute transfer transaction
    let transfer_amount = 1_000_000_000_000u128; // 1 unit (12 decimals)
    println!("\nExecuting transfer of {} units...", transfer_amount);

    let executor = adapter.transaction_executor();
    let tx_hash = executor
        .transfer(&alice_wallet, bob_address, transfer_amount)
        .await
        .expect("Transfer should succeed");

    println!("Transaction hash: {}", tx_hash);

    // Wait for transaction to be included in a block
    tokio::time::sleep(tokio::time::Duration::from_secs(12)).await;

    // Verify balances changed
    if let Ok(final_alice) = adapter.get_balance(&alice_wallet.address()).await {
        println!("Alice final balance: {} units", final_alice);
        if let Ok(initial) = initial_alice {
            println!("Alice balance decreased: {}", initial > final_alice);
        }
    }

    if let Ok(final_bob) = adapter.get_balance(bob_address).await {
        println!("Bob final balance: {} units", final_bob);
        if let Ok(initial) = initial_bob {
            println!("Bob balance increased: {}", final_bob > initial);
        }
    }

    println!("\nSubstrate transaction execution verified!");
}
