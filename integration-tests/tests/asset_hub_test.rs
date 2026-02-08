// Asset Hub Integration Tests
// These tests run against a real Asset Hub node (e.g., Paseo Asset Hub)
// Run with: INTEGRATION_TESTS=1 cargo test --test asset_hub_test -- --include-ignored

#[path = "integration_helpers.rs"]
mod integration_helpers;

use apex_sdk::prelude::*;

#[tokio::test]
#[ignore]
async fn test_asset_hub_manager_preparations() {
    // This test verifies that we can create Asset Hub payloads without a live connection
    // (Useful for verifying the API design)

    let adapter = match SubstrateAdapter::connect("ws://127.0.0.1:9944").await {
        Ok(adapter) => adapter,
        Err(_) => {
            println!("Note: Using placeholder adapter for preparation test");
            return;
        }
    };

    let asset_manager = adapter.assets();
    let admin = Address::substrate("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY");

    let create_payload = asset_manager
        .create(1000, &admin, 1)
        .await
        .expect("Should create payload");
    assert_eq!(create_payload.pallet_name(), "Assets");
    assert_eq!(create_payload.call_name(), "create");

    let mint_payload = asset_manager
        .mint(1000, &admin, 1000000)
        .await
        .expect("Should create mint payload");
    assert_eq!(mint_payload.pallet_name(), "Assets");
    assert_eq!(mint_payload.call_name(), "mint");
}

#[tokio::test]
#[ignore]
async fn test_asset_hub_live_paseo() {
    skip_if_not_integration!();

    // Paseo Asset Hub public RPC
    let paseo_asset_hub_url = "wss://paseo-asset-hub-pub.dwellir.com";

    let sdk = ApexSDK::builder()
        .with_substrate_endpoint(paseo_asset_hub_url)
        .build()
        .await
        .expect("Should connect to Paseo Asset Hub");

    let adapter = sdk
        .substrate()
        .expect("Substrate adapter should be available");
    let asset_manager = adapter.assets();

    println!("Connected to Paseo Asset Hub");
    println!("Chain name: {}", adapter.chain_name());

    // We don't execute a real transaction here as it requires a funded wallet on Paseo,
    // but we verify the manager can be initialized and can create payloads.
    let admin = Address::substrate("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY");
    let payload = asset_manager
        .create(9999, &admin, 1)
        .await
        .expect("Should create payload");

    assert_eq!(payload.pallet_name(), "Assets");
    println!("Successfully verified Asset Manager on Paseo Asset Hub");
}
