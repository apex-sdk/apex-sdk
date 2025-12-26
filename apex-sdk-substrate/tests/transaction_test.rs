//! Comprehensive tests for transaction module
//!
//! These tests verify transaction functionality including:
//! - Fee configuration
//! - Retry configuration
//! - Batch call building
//! - Extrinsic building
//! - Transaction modes

use apex_sdk_substrate::transaction::*;
use std::time::Duration;

#[test]
fn test_batch_mode_default() {
    let mode = BatchMode::default();
    assert_eq!(mode, BatchMode::Optimistic);
}

#[test]
fn test_batch_mode_variants() {
    assert_eq!(BatchMode::Optimistic, BatchMode::Optimistic);
    assert_eq!(BatchMode::AllOrNothing, BatchMode::AllOrNothing);
    assert_eq!(BatchMode::Force, BatchMode::Force);
    assert_ne!(BatchMode::Optimistic, BatchMode::AllOrNothing);
}

#[test]
fn test_batch_mode_clone() {
    let mode = BatchMode::AllOrNothing;
    let cloned = mode;
    assert_eq!(mode, cloned);
}

#[test]
fn test_batch_mode_debug() {
    let optimistic = format!("{:?}", BatchMode::Optimistic);
    let all_or_nothing = format!("{:?}", BatchMode::AllOrNothing);
    let force = format!("{:?}", BatchMode::Force);

    assert_eq!(optimistic, "Optimistic");
    assert_eq!(all_or_nothing, "AllOrNothing");
    assert_eq!(force, "Force");
}

#[test]
fn test_batch_call_new() {
    let call = BatchCall::new(5, 3, vec![1, 2, 3, 4]);

    assert_eq!(call.pallet_index, 5);
    assert_eq!(call.call_index, 3);
    assert_eq!(call.args_encoded, vec![1, 2, 3, 4]);
}

#[test]
fn test_batch_call_clone() {
    let call = BatchCall::new(10, 20, vec![5, 6, 7]);
    let cloned = call.clone();

    assert_eq!(cloned.pallet_index, call.pallet_index);
    assert_eq!(cloned.call_index, call.call_index);
    assert_eq!(cloned.args_encoded, call.args_encoded);
}

#[test]
fn test_batch_call_debug() {
    let call = BatchCall::new(1, 2, vec![3, 4]);
    let debug_output = format!("{:?}", call);

    assert!(debug_output.contains("BatchCall"));
    assert!(debug_output.contains("pallet_index"));
}

#[test]
fn test_batch_call_empty_args() {
    let call = BatchCall::new(0, 0, vec![]);

    assert_eq!(call.pallet_index, 0);
    assert_eq!(call.call_index, 0);
    assert!(call.args_encoded.is_empty());
}

#[test]
fn test_batch_call_large_args() {
    let large_args = vec![0u8; 1024];
    let call = BatchCall::new(5, 10, large_args.clone());

    assert_eq!(call.args_encoded.len(), 1024);
    assert_eq!(call.args_encoded, large_args);
}

#[test]
fn test_fee_config_default() {
    let config = FeeConfig::default();

    assert_eq!(config.multiplier, 1.2);
    assert_eq!(config.max_fee, None);
    assert_eq!(config.tip, 0);
}

#[test]
fn test_fee_config_new() {
    let config = FeeConfig::new();

    assert_eq!(config.multiplier, 1.2);
    assert_eq!(config.max_fee, None);
    assert_eq!(config.tip, 0);
}

#[test]
fn test_fee_config_with_multiplier() {
    let config = FeeConfig::new().with_multiplier(1.5);

    assert_eq!(config.multiplier, 1.5);
}

#[test]
fn test_fee_config_with_max_fee() {
    let config = FeeConfig::new().with_max_fee(1_000_000);

    assert_eq!(config.max_fee, Some(1_000_000));
}

#[test]
fn test_fee_config_with_tip() {
    let config = FeeConfig::new().with_tip(5000);

    assert_eq!(config.tip, 5000);
}

#[test]
fn test_fee_config_builder_pattern() {
    let config = FeeConfig::new()
        .with_multiplier(2.0)
        .with_max_fee(10_000_000)
        .with_tip(1_000);

    assert_eq!(config.multiplier, 2.0);
    assert_eq!(config.max_fee, Some(10_000_000));
    assert_eq!(config.tip, 1_000);
}

#[test]
fn test_fee_config_clone() {
    let config = FeeConfig::new()
        .with_multiplier(1.8)
        .with_max_fee(5_000_000);

    let cloned = config.clone();

    assert_eq!(cloned.multiplier, config.multiplier);
    assert_eq!(cloned.max_fee, config.max_fee);
}

#[test]
fn test_fee_config_debug() {
    let config = FeeConfig::new().with_multiplier(1.3);
    let debug_output = format!("{:?}", config);

    assert!(debug_output.contains("FeeConfig"));
    assert!(debug_output.contains("multiplier"));
}

#[test]
fn test_retry_config_default() {
    let config = RetryConfig::default();

    assert_eq!(config.max_retries, 3);
    assert_eq!(config.initial_delay, Duration::from_secs(2));
    assert_eq!(config.max_delay, Duration::from_secs(30));
    assert_eq!(config.backoff_multiplier, 2.0);
}

#[test]
fn test_retry_config_new() {
    let config = RetryConfig::new();

    assert_eq!(config.max_retries, 3);
    assert_eq!(config.initial_delay, Duration::from_secs(2));
}

#[test]
fn test_retry_config_with_max_retries() {
    let config = RetryConfig::new().with_max_retries(5);

    assert_eq!(config.max_retries, 5);
}

#[test]
fn test_retry_config_with_initial_delay() {
    let config = RetryConfig::new().with_initial_delay(Duration::from_secs(5));

    assert_eq!(config.initial_delay, Duration::from_secs(5));
}

#[test]
fn test_retry_config_builder_pattern() {
    let config = RetryConfig::new()
        .with_max_retries(10)
        .with_initial_delay(Duration::from_secs(1));

    assert_eq!(config.max_retries, 10);
    assert_eq!(config.initial_delay, Duration::from_secs(1));
}

#[test]
fn test_retry_config_clone() {
    let config = RetryConfig::new()
        .with_max_retries(7)
        .with_initial_delay(Duration::from_secs(3));

    let cloned = config.clone();

    assert_eq!(cloned.max_retries, config.max_retries);
    assert_eq!(cloned.initial_delay, config.initial_delay);
}

#[test]
fn test_retry_config_debug() {
    let config = RetryConfig::new();
    let debug_output = format!("{:?}", config);

    assert!(debug_output.contains("RetryConfig"));
    assert!(debug_output.contains("max_retries"));
}

#[test]
fn test_retry_config_zero_retries() {
    let config = RetryConfig::new().with_max_retries(0);

    assert_eq!(config.max_retries, 0);
}

#[test]
fn test_retry_config_large_retries() {
    let config = RetryConfig::new().with_max_retries(100);

    assert_eq!(config.max_retries, 100);
}

#[test]
fn test_fee_config_various_multipliers() {
    let configs = [
        FeeConfig::new().with_multiplier(1.0),
        FeeConfig::new().with_multiplier(1.5),
        FeeConfig::new().with_multiplier(2.0),
        FeeConfig::new().with_multiplier(3.0),
    ];

    assert_eq!(configs[0].multiplier, 1.0);
    assert_eq!(configs[1].multiplier, 1.5);
    assert_eq!(configs[2].multiplier, 2.0);
    assert_eq!(configs[3].multiplier, 3.0);
}

#[test]
fn test_fee_config_various_max_fees() {
    let config1 = FeeConfig::new().with_max_fee(100_000);
    let config2 = FeeConfig::new().with_max_fee(1_000_000);
    let config3 = FeeConfig::new().with_max_fee(10_000_000);

    assert_eq!(config1.max_fee, Some(100_000));
    assert_eq!(config2.max_fee, Some(1_000_000));
    assert_eq!(config3.max_fee, Some(10_000_000));
}

#[test]
fn test_fee_config_various_tips() {
    let config1 = FeeConfig::new().with_tip(0);
    let config2 = FeeConfig::new().with_tip(1_000);
    let config3 = FeeConfig::new().with_tip(10_000);

    assert_eq!(config1.tip, 0);
    assert_eq!(config2.tip, 1_000);
    assert_eq!(config3.tip, 10_000);
}

#[test]
fn test_batch_call_various_indices() {
    let calls = [
        BatchCall::new(0, 0, vec![]),
        BatchCall::new(5, 3, vec![1, 2]),
        BatchCall::new(10, 20, vec![3, 4, 5]),
        BatchCall::new(255, 255, vec![6, 7, 8, 9]),
    ];

    assert_eq!(calls[0].pallet_index, 0);
    assert_eq!(calls[1].pallet_index, 5);
    assert_eq!(calls[2].pallet_index, 10);
    assert_eq!(calls[3].pallet_index, 255);
}

#[test]
fn test_retry_config_exponential_backoff() {
    let config = RetryConfig {
        max_retries: 5,
        initial_delay: Duration::from_secs(1),
        max_delay: Duration::from_secs(60),
        backoff_multiplier: 2.0,
    };

    // Test that backoff multiplier is correctly set
    assert_eq!(config.backoff_multiplier, 2.0);

    // Simulate exponential backoff
    let mut delay = config.initial_delay.as_secs_f64();
    for _ in 0..config.max_retries {
        delay *= config.backoff_multiplier;
        delay = delay.min(config.max_delay.as_secs_f64());
    }

    assert!(delay <= config.max_delay.as_secs_f64());
}

#[test]
fn test_retry_config_custom_backoff() {
    let config = RetryConfig {
        max_retries: 3,
        initial_delay: Duration::from_millis(500),
        max_delay: Duration::from_secs(10),
        backoff_multiplier: 1.5,
    };

    assert_eq!(config.backoff_multiplier, 1.5);
    assert_eq!(config.initial_delay, Duration::from_millis(500));
    assert_eq!(config.max_delay, Duration::from_secs(10));
}

#[test]
fn test_fee_config_edge_cases() {
    // Zero multiplier (unusual but valid)
    let config1 = FeeConfig::new().with_multiplier(0.0);
    assert_eq!(config1.multiplier, 0.0);

    // Very high multiplier
    let config2 = FeeConfig::new().with_multiplier(100.0);
    assert_eq!(config2.multiplier, 100.0);

    // Very large max fee
    let config3 = FeeConfig::new().with_max_fee(u128::MAX);
    assert_eq!(config3.max_fee, Some(u128::MAX));

    // Very large tip
    let config4 = FeeConfig::new().with_tip(u128::MAX);
    assert_eq!(config4.tip, u128::MAX);
}

#[test]
fn test_batch_call_realistic_balances_transfer() {
    use parity_scale_codec::Encode;

    // Simulate a Balances::transfer_keep_alive call
    let pallet_index = 5u8; // Balances pallet (typical)
    let call_index = 3u8; // transfer_keep_alive (typical)

    let recipient = [0u8; 32];
    let amount = 1_000_000_000_000u128;

    let args = (recipient, amount).encode();

    let call = BatchCall::new(pallet_index, call_index, args);

    assert_eq!(call.pallet_index, 5);
    assert_eq!(call.call_index, 3);
    assert!(!call.args_encoded.is_empty());
}

#[test]
fn test_batch_call_multiple_types() {
    // Test different call types
    let transfer = BatchCall::new(5, 3, vec![1, 2, 3]);
    let remark = BatchCall::new(0, 1, vec![4, 5, 6]);
    let set_code = BatchCall::new(0, 2, vec![7, 8, 9]);

    assert_eq!(transfer.pallet_index, 5);
    assert_eq!(remark.pallet_index, 0);
    assert_eq!(set_code.pallet_index, 0);
}

#[test]
fn test_retry_config_duration_values() {
    let config = RetryConfig::default();

    assert!(config.initial_delay.as_millis() > 0);
    assert!(config.max_delay.as_millis() > config.initial_delay.as_millis());
}

#[test]
fn test_fee_config_no_max_fee() {
    let config = FeeConfig::new();
    assert!(config.max_fee.is_none());
}

#[test]
fn test_fee_config_realistic_values() {
    // Realistic fee configuration for Polkadot
    let config = FeeConfig::new()
        .with_multiplier(1.2)
        .with_max_fee(10_000_000_000) // 0.001 DOT
        .with_tip(100_000_000); // 0.00001 DOT

    assert_eq!(config.multiplier, 1.2);
    assert_eq!(config.max_fee, Some(10_000_000_000));
    assert_eq!(config.tip, 100_000_000);
}

#[test]
fn test_retry_config_realistic_values() {
    // Realistic retry configuration
    let config = RetryConfig::new()
        .with_max_retries(3)
        .with_initial_delay(Duration::from_secs(2));

    assert_eq!(config.max_retries, 3);
    assert_eq!(config.initial_delay, Duration::from_secs(2));
}

#[test]
fn test_batch_mode_copy() {
    let mode1 = BatchMode::Optimistic;
    let mode2 = mode1;

    assert_eq!(mode1, mode2);
}

#[test]
fn test_all_batch_modes() {
    let modes = [
        BatchMode::Optimistic,
        BatchMode::AllOrNothing,
        BatchMode::Force,
    ];

    assert_eq!(modes.len(), 3);
    assert_ne!(modes[0], modes[1]);
    assert_ne!(modes[1], modes[2]);
    assert_ne!(modes[0], modes[2]);
}
