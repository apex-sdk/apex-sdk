//! Integration tests for enhanced fee estimation

use apex_sdk_substrate::{ChainConfig, FeeStrategy, SubstrateAdapter, Wallet};

#[tokio::test]
#[ignore]
async fn test_enhanced_fee_estimator_creation() {
    let adapter = SubstrateAdapter::connect_with_config(ChainConfig::westend())
        .await
        .expect("Failed to connect");

    let estimator = adapter.fee_estimator();
    assert!(std::mem::size_of_val(&estimator) > 0);
}

#[tokio::test]
#[ignore]
async fn test_network_congestion_update() {
    let adapter = SubstrateAdapter::connect_with_config(ChainConfig::westend())
        .await
        .expect("Failed to connect");

    let estimator = adapter.fee_estimator();

    let result = estimator.update_congestion().await;
    assert!(result.is_ok(), "Congestion update should succeed");

    let congestion = estimator.get_congestion().await;
    assert!(congestion.blocks_analyzed > 0, "Should analyze blocks");
    assert!(
        congestion.avg_block_fullness >= 0.0 && congestion.avg_block_fullness <= 1.0,
        "Block fullness should be between 0 and 1"
    );
}

#[tokio::test]
#[ignore]
async fn test_fee_estimation_with_strategies() {
    let adapter = SubstrateAdapter::connect_with_config(ChainConfig::westend())
        .await
        .expect("Failed to connect");

    let executor = adapter.transaction_executor();
    let wallet = Wallet::from_seed(b"//Alice", apex_sdk_substrate::KeyPairType::Sr25519)
        .expect("Failed to create wallet");

    let test_address = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
    let amount = 1_000_000_000_000u128;

    let slow_estimate = executor
        .estimate_transfer_fee_with_strategy(test_address, amount, &wallet, FeeStrategy::Slow)
        .await
        .expect("Slow fee estimation should succeed");

    let normal_estimate = executor
        .estimate_transfer_fee_with_strategy(test_address, amount, &wallet, FeeStrategy::Normal)
        .await
        .expect("Normal fee estimation should succeed");

    let fast_estimate = executor
        .estimate_transfer_fee_with_strategy(test_address, amount, &wallet, FeeStrategy::Fast)
        .await
        .expect("Fast fee estimation should succeed");

    assert!(
        slow_estimate.total_fee <= normal_estimate.total_fee,
        "Slow should be cheaper than or equal to normal"
    );
    assert!(
        normal_estimate.total_fee <= fast_estimate.total_fee,
        "Normal should be cheaper than or equal to fast"
    );

    assert_eq!(slow_estimate.strategy, FeeStrategy::Slow);
    assert_eq!(normal_estimate.strategy, FeeStrategy::Normal);
    assert_eq!(fast_estimate.strategy, FeeStrategy::Fast);
}

#[tokio::test]
#[ignore]
async fn test_fee_estimate_breakdown() {
    let adapter = SubstrateAdapter::connect_with_config(ChainConfig::westend())
        .await
        .expect("Failed to connect");

    let executor = adapter.transaction_executor();
    let wallet = Wallet::from_seed(b"//Alice", apex_sdk_substrate::KeyPairType::Sr25519)
        .expect("Failed to create wallet");

    let test_address = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
    let amount = 1_000_000_000_000u128;

    let estimate = executor
        .estimate_transfer_fee_with_strategy(test_address, amount, &wallet, FeeStrategy::Normal)
        .await
        .expect("Fee estimation should succeed");

    assert!(estimate.base_fee > 0, "Base fee should be positive");
    assert!(estimate.total_fee > 0, "Total fee should be positive");
    assert!(
        estimate.total_fee >= estimate.base_fee,
        "Total should be at least base fee"
    );
}

#[tokio::test]
#[ignore]
async fn test_fee_accuracy_tracking() {
    let adapter = SubstrateAdapter::connect_with_config(ChainConfig::westend())
        .await
        .expect("Failed to connect");

    let estimator = adapter.fee_estimator();

    let stats_before = estimator.get_accuracy_stats().await;
    assert!(stats_before.is_none(), "Should have no stats initially");

    estimator.record_actual_fee(1_000_000, 900_000).await;
    estimator.record_actual_fee(1_100_000, 1_000_000).await;
    estimator.record_actual_fee(1_050_000, 1_000_000).await;

    let stats_after = estimator.get_accuracy_stats().await;
    assert!(stats_after.is_some(), "Should have stats after recording");

    let stats = stats_after.unwrap();
    assert_eq!(stats.sample_count, 3, "Should have 3 samples");
    assert!(
        stats.avg_percentage_error >= 0.0,
        "Average error should be non-negative"
    );
}

#[tokio::test]
#[ignore]
async fn test_congestion_levels() {
    let adapter = SubstrateAdapter::connect_with_config(ChainConfig::westend())
        .await
        .expect("Failed to connect");

    let estimator = adapter.fee_estimator();
    estimator.update_congestion().await.expect("Should update");

    let congestion = estimator.get_congestion().await;

    let multiplier = congestion.multiplier();
    match congestion.level {
        apex_sdk_substrate::CongestionLevel::Low => {
            assert_eq!(multiplier, 1.0, "Low congestion multiplier should be 1.0");
        }
        apex_sdk_substrate::CongestionLevel::Medium => {
            assert_eq!(
                multiplier, 1.1,
                "Medium congestion multiplier should be 1.1"
            );
        }
        apex_sdk_substrate::CongestionLevel::High => {
            assert_eq!(multiplier, 1.3, "High congestion multiplier should be 1.3");
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_fee_strategies_have_correct_properties() {
    assert_eq!(FeeStrategy::Fast.multiplier(), 1.5);
    assert_eq!(FeeStrategy::Normal.multiplier(), 1.2);
    assert_eq!(FeeStrategy::Slow.multiplier(), 1.0);

    assert_eq!(FeeStrategy::Fast.tip(), 1_000_000);
    assert_eq!(FeeStrategy::Normal.tip(), 100_000);
    assert_eq!(FeeStrategy::Slow.tip(), 0);

    assert!(!FeeStrategy::Fast.description().is_empty());
    assert!(!FeeStrategy::Normal.description().is_empty());
    assert!(!FeeStrategy::Slow.description().is_empty());
}
