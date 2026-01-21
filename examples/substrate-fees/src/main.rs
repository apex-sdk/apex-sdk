//! This example demonstrates the advanced fee estimation capabilities including:
//! - Weight-based dynamic fee calculation
//! - Network congestion monitoring
//! - Multiple fee strategies (Fast, Normal, Slow)
//! - Fee estimation accuracy tracking
//!
//! Run with: cargo run --example substrate_fees

use apex_sdk_substrate::{
    ChainConfig, EnhancedFeeEstimator, FeeStrategy, SubstrateAdapter, Wallet,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    println!("Fee Estimation Example for Substrate");
    println!("============================================\n");

    let config = ChainConfig::westend();
    let adapter = SubstrateAdapter::connect_with_config(config).await?;

    println!("Connected to {}\n", adapter.chain_name());

    let estimator = adapter.enhanced_fee_estimator();

    println!("1. Network Congestion Analysis");
    println!("------------------------------");

    println!("Analyzing recent blocks for congestion...");
    estimator.update_congestion().await?;

    let congestion = estimator.get_congestion().await;
    println!("Network congestion level: {:?}", congestion.level);
    println!("Average block fullness: {:.2}%", congestion.avg_block_fullness * 100.0);
    println!("Average fee: {} Planck", congestion.avg_fee);
    println!("Blocks analyzed: {}", congestion.blocks_analyzed);
    println!("Congestion multiplier: {:.2}x\n", congestion.multiplier());

    println!("2. Fee Estimation with Different Strategies");
    println!("-------------------------------------------");

    let test_address = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
    let amount = 1_000_000_000_000u128;

    let executor = adapter.transaction_executor();

    let seed = std::env::var("TEST_SEED")
        .unwrap_or_else(|_| "//Alice".to_string());
    let wallet = Wallet::from_seed(
        seed.as_bytes(),
        apex_sdk_substrate::KeyPairType::Sr25519,
    )?;

    for strategy in [FeeStrategy::Slow, FeeStrategy::Normal, FeeStrategy::Fast] {
        println!("\nStrategy: {:?}", strategy);
        println!("Description: {}", strategy.description());

        match executor.estimate_transfer_fee_with_strategy(
            test_address,
            amount,
            &wallet,
            strategy,
        ).await {
            Ok(estimate) => {
                println!("  Total fee: {} Planck", estimate.total_fee);
                println!("  Base fee: {} Planck", estimate.base_fee);
                println!("  Length fee: {} Planck", estimate.length_fee);
                println!("  Weight fee: {} Planck", estimate.weight_fee);
                println!("  Tip: {} Planck", estimate.tip);
                println!("  Strategy multiplier: {:.2}x", strategy.multiplier());

                if let Some(weight) = estimate.weight {
                    println!("  Weight: ref_time={}, proof_size={}",
                        weight.ref_time, weight.proof_size);
                }
            }
            Err(e) => {
                println!("  Error estimating fee: {}", e);
            }
        }
    }

    println!("\n3. Comparing All Strategies");
    println!("----------------------------");

    let mut strategy_fees = Vec::new();

    for strategy in [FeeStrategy::Slow, FeeStrategy::Normal, FeeStrategy::Fast] {
        match executor.estimate_transfer_fee_with_strategy(
            test_address,
            amount,
            &wallet,
            strategy,
        ).await {
            Ok(estimate) => {
                strategy_fees.push((strategy, estimate.total_fee));
            }
            Err(e) => {
                println!("Error with {:?} strategy: {}", strategy, e);
            }
        }
    }

    if !strategy_fees.is_empty() {
        let slow_fee = strategy_fees.iter()
            .find(|(s, _)| *s == FeeStrategy::Slow)
            .map(|(_, f)| *f)
            .unwrap_or(0);

        println!("\nFee Comparison:");
        for (strategy, fee) in &strategy_fees {
            let diff_pct = if slow_fee > 0 {
                ((*fee as f64 - slow_fee as f64) / slow_fee as f64) * 100.0
            } else {
                0.0
            };

            println!("  {:?}: {} Planck ({:+.1}% vs Slow)", strategy, fee, diff_pct);
        }
    }

    println!("\n4. Fee Estimation Accuracy Tracking");
    println!("------------------------------------");

    println!("Recording sample accuracy metrics...");
    estimator.record_actual_fee(1_200_000, 1_000_000).await;
    estimator.record_actual_fee(1_150_000, 1_000_000).await;
    estimator.record_actual_fee(1_100_000, 1_050_000).await;
    estimator.record_actual_fee(1_050_000, 1_000_000).await;
    estimator.record_actual_fee(1_000_000, 1_000_000).await;

    if let Some(stats) = estimator.get_accuracy_stats().await {
        println!("Accuracy statistics:");
        println!("  Sample count: {}", stats.sample_count);
        println!("  Average absolute error: {:.2} Planck", stats.avg_absolute_error);
        println!("  Average percentage error: {:.2}%", stats.avg_percentage_error);
        println!("  Max percentage error: {:.2}%", stats.max_percentage_error);
        println!("  Min percentage error: {:.2}%", stats.min_percentage_error);
    } else {
        println!("No accuracy data available yet");
    }

    println!("\n5. Recommendation Engine");
    println!("-------------------------");

    let congestion = estimator.get_congestion().await;
    let recommended_strategy = match congestion.level {
        apex_sdk_substrate::CongestionLevel::Low => {
            println!("Network congestion is LOW");
            println!("Recommendation: Use SLOW strategy to save on fees");
            FeeStrategy::Slow
        }
        apex_sdk_substrate::CongestionLevel::Medium => {
            println!("Network congestion is MEDIUM");
            println!("Recommendation: Use NORMAL strategy for balanced confirmation");
            FeeStrategy::Normal
        }
        apex_sdk_substrate::CongestionLevel::High => {
            println!("Network congestion is HIGH");
            println!("Recommendation: Use FAST strategy for priority confirmation");
            FeeStrategy::Fast
        }
    };

    match executor.estimate_transfer_fee_with_strategy(
        test_address,
        amount,
        &wallet,
        recommended_strategy,
    ).await {
        Ok(estimate) => {
            println!("\nRecommended fee: {} Planck", estimate.total_fee);
        }
        Err(e) => {
            println!("\nError getting recommended fee: {}", e);
        }
    }

    println!("\nExample completed successfully!");

    Ok(())
}
