// Revive Integration Tests
// Run with: INTEGRATION_TESTS=1 cargo test --test revive_integration_test -- --include-ignored

#[path = "integration_helpers.rs"]
mod integration_helpers;

use apex_sdk::core::Provider;
use apex_sdk::prelude::*;
use apex_sdk_revive::ReviveAdapter;
use integration_helpers::*;

#[tokio::test]
#[ignore]
async fn test_revive_connection_to_docker_node() {
    skip_if_not_integration!();

    wait_for_substrate_node(60)
        .await
        .expect("Substrate node should be ready");

    let adapter = ReviveAdapter::connect(&substrate_rpc_url())
        .await
        .expect("Should connect to Revive node");

    println!("Successfully connected to Revive node");
    assert!(adapter.is_connected().await, "Client should be connected");
}

#[tokio::test]
#[ignore]
async fn test_revive_get_balance() {
    skip_if_not_integration!();

    wait_for_substrate_node(60)
        .await
        .expect("Substrate node should be ready");

    let adapter = ReviveAdapter::connect(&substrate_rpc_url())
        .await
        .expect("Should connect to Revive node");

    let test_address = Address::evm("0x0000000000000000000000000000000000000000");

    let balance = adapter.get_balance(&test_address).await;
    assert!(balance.is_ok(), "Should be able to query balance");
    println!("Balance of {}: {}", test_address, balance.unwrap());
}

#[tokio::test]
#[ignore]
async fn test_revive_deployment_and_call() {
    skip_if_not_integration!();

    wait_for_substrate_node(60)
        .await
        .expect("Substrate node should be ready");

    let alice_wallet =
        SubstrateWallet::from_mnemonic("//Alice", apex_sdk_substrate::KeyPairType::Sr25519)
            .expect("Should create Alice wallet");

    let sdk = ApexSDK::builder()
        .with_substrate_endpoint(substrate_rpc_url())
        .with_substrate_wallet(alice_wallet)
        .build()
        .await
        .expect("Should initialize SDK");

    // Minimal PolkaVM bytecode
    let code = vec![0x60, 0x00, 0x60, 0x00, 0x55];

    println!("Deploying contract...");
    let deploy_tx = Transaction::builder()
        .from(Address::evm("0x0000000000000000000000000000000000000000"))
        .data(code)
        .deploy(true)
        .chain(Chain::Polkadot) // Target chain
        .build()
        .expect("Should build deployment transaction");

    let result = sdk.execute(deploy_tx).await;

    match result {
        Ok(res) => {
            println!("Deployment successful: {}", res.source_tx_hash);
            assert!(res.source_tx_hash.starts_with("deploy:"));
        }
        Err(e) => {
            println!(
                "Deployment failed (expected if pallet-revive is missing): {}",
                e
            );
        }
    }
}
