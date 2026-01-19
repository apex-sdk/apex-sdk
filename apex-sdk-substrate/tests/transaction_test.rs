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

#[cfg(test)]
mod broadcast_tests {
    use super::*;
    use apex_sdk_core::Broadcaster;
    use apex_sdk_substrate::{ChainConfig, SubstrateAdapter, Wallet};
    use parity_scale_codec::Encode;

    #[test]
    fn test_validate_extrinsic_empty_bytes() {
        let result = validate_extrinsic_format_standalone(&[]);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("empty"));
    }

    #[test]
    fn test_validate_extrinsic_too_short() {
        let result = validate_extrinsic_format_standalone(&[0x84]);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_extrinsic_unsigned() {
        let unsigned_extrinsic = vec![0x04, 0x00, 0x00, 0x00];
        let result = validate_extrinsic_format_standalone(&unsigned_extrinsic);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("must be signed"));
    }

    #[test]
    fn test_validate_extrinsic_wrong_version() {
        let wrong_version = vec![0x83, 0x00, 0x00, 0x00];
        let result = validate_extrinsic_format_standalone(&wrong_version);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("version"));
    }

    #[test]
    fn test_validate_extrinsic_valid_format() {
        let valid_extrinsic = create_mock_signed_extrinsic();
        let result = validate_extrinsic_format_standalone(&valid_extrinsic);
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_broadcast_empty_transaction() {
        let adapter = SubstrateAdapter::connect_with_config(ChainConfig::westend())
            .await
            .expect("Failed to connect");
        let result = adapter.broadcast(&[]).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("empty"));
    }

    #[tokio::test]
    #[ignore]
    async fn test_broadcast_too_short() {
        let adapter = SubstrateAdapter::connect_with_config(ChainConfig::westend())
            .await
            .expect("Failed to connect");
        let result = adapter.broadcast(&[0x84, 0x00]).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("too short"));
    }

    #[tokio::test]
    #[ignore]
    async fn test_broadcast_unsigned_transaction() {
        let adapter = SubstrateAdapter::connect_with_config(ChainConfig::westend())
            .await
            .expect("Failed to connect");
        let unsigned = vec![0x04, 0x00, 0x00, 0x00, 0x00];
        let result = adapter.broadcast(&unsigned).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("signed"));
    }

    #[tokio::test]
    #[ignore]
    async fn test_broadcast_integration_westend() {
        let adapter = SubstrateAdapter::connect_with_config(ChainConfig::westend())
            .await
            .expect("Failed to connect to Westend");

        let wallet = Wallet::new_random_with_type(apex_sdk_substrate::KeyPairType::Sr25519);

        let tx_executor = adapter.transaction_executor();

        let dummy_recipient = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
        let amount = 1_000_000_000u128;

        let balance = adapter
            .get_balance(&wallet.address())
            .await
            .expect("Failed to get balance");

        if balance < amount * 2 {
            println!("Insufficient balance for integration test, skipping broadcast");
            return;
        }

        let result = tx_executor.transfer(&wallet, dummy_recipient, amount).await;

        match result {
            Ok(tx_hash) => {
                assert!(tx_hash.starts_with("0x"));
                assert_eq!(tx_hash.len(), 66);
                println!("Transaction broadcast successful: {}", tx_hash);
            }
            Err(e) => {
                println!("Transaction failed (expected for unfunded wallet): {}", e);
            }
        }
    }

    #[tokio::test]
    #[ignore]
    async fn test_broadcast_and_watch_integration() {
        let adapter = SubstrateAdapter::connect_with_config(ChainConfig::westend())
            .await
            .expect("Failed to connect");

        let wallet = Wallet::new_random_with_type(apex_sdk_substrate::KeyPairType::Sr25519);

        let tx_executor = adapter.transaction_executor();

        let balance = adapter
            .get_balance(&wallet.address())
            .await
            .expect("Failed to get balance");

        if balance < 10_000_000_000u128 {
            println!("Insufficient balance, skipping test");
            return;
        }

        let result = tx_executor
            .transfer(
                &wallet,
                "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
                1_000_000_000u128,
            )
            .await;

        if let Ok(tx_hash) = result {
            assert!(tx_hash.starts_with("0x"));

            let status = adapter.get_transaction_status(&tx_hash).await;
            assert!(status.is_ok());
        }
    }

    #[tokio::test]
    #[ignore]
    async fn test_broadcast_batch_transactions() {
        let adapter = SubstrateAdapter::connect_with_config(ChainConfig::westend())
            .await
            .expect("Failed to connect");

        let wallet = Wallet::new_random_with_type(apex_sdk_substrate::KeyPairType::Sr25519);

        let balance = adapter
            .get_balance(&wallet.address())
            .await
            .expect("Failed to get balance");

        if balance < 100_000_000_000u128 {
            println!("Insufficient balance for batch test");
            return;
        }

        let tx_executor = adapter.transaction_executor();

        let transfers = vec![
            (
                "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
                1_000_000_000u128,
            ),
            (
                "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty".to_string(),
                1_000_000_000u128,
            ),
        ];

        let result = tx_executor
            .execute_batch_transfers(transfers, &wallet, BatchMode::AllOrNothing)
            .await;

        match result {
            Ok(tx_hash) => {
                assert!(tx_hash.starts_with("0x"));
                println!("Batch transaction successful: {}", tx_hash);
            }
            Err(e) => {
                println!(
                    "Batch transaction failed (expected for unfunded wallet): {}",
                    e
                );
            }
        }
    }

    #[tokio::test]
    #[ignore]
    async fn test_fee_estimation_integration() {
        let adapter = SubstrateAdapter::connect_with_config(ChainConfig::westend())
            .await
            .expect("Failed to connect");

        let wallet = Wallet::new_random_with_type(apex_sdk_substrate::KeyPairType::Sr25519);

        let tx_executor = adapter.transaction_executor();

        let fee = tx_executor
            .estimate_transfer_fee(
                "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
                1_000_000_000u128,
                &wallet,
            )
            .await;

        assert!(fee.is_ok());
        let estimated_fee = fee.unwrap();
        assert!(estimated_fee > 0);
        assert!(estimated_fee < 10_000_000_000u128);
        println!("Estimated fee: {} Planck", estimated_fee);
    }

    #[tokio::test]
    #[ignore]
    async fn test_broadcast_with_retry_integration() {
        let adapter = SubstrateAdapter::connect_with_config(ChainConfig::westend())
            .await
            .expect("Failed to connect");

        let wallet = Wallet::new_random_with_type(apex_sdk_substrate::KeyPairType::Sr25519);

        let tx_executor = adapter
            .transaction_executor()
            .with_retry_config(RetryConfig::new().with_max_retries(2));

        let result = tx_executor
            .transfer(
                &wallet,
                "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
                1_000_000_000u128,
            )
            .await;

        match result {
            Ok(_) => println!("Transaction succeeded"),
            Err(e) => println!("Transaction failed after retries: {}", e),
        }
    }

    #[test]
    fn test_extrinsic_validation_version_4() {
        let valid = create_mock_signed_extrinsic();
        let result = validate_extrinsic_format_standalone(&valid);
        assert!(result.is_ok());
    }

    #[test]
    fn test_extrinsic_hash_format() {
        use sp_core::blake2_256;

        let extrinsic = create_mock_signed_extrinsic();
        let hash = blake2_256(&extrinsic);
        let hash_string = format!("0x{}", hex::encode(hash));

        assert_eq!(hash_string.len(), 66);
        assert!(hash_string.starts_with("0x"));
    }

    fn validate_extrinsic_format_standalone(
        extrinsic_bytes: &[u8],
    ) -> std::result::Result<(), apex_sdk_core::SdkError> {
        use apex_sdk_core::SdkError;
        use parity_scale_codec::Decode;

        if extrinsic_bytes.is_empty() {
            return Err(SdkError::TransactionError(
                "Cannot validate empty extrinsic".to_string(),
            ));
        }

        if extrinsic_bytes.len() < 4 {
            return Err(SdkError::TransactionError(
                "Extrinsic too short to be valid".to_string(),
            ));
        }

        let first_byte = extrinsic_bytes[0];

        let has_signature = (first_byte & 0b1000_0000) != 0;
        if !has_signature {
            return Err(SdkError::TransactionError(
                "Extrinsic must be signed for broadcasting".to_string(),
            ));
        }

        let version = first_byte & 0b0111_1111;
        if version != 4 {
            return Err(SdkError::TransactionError(format!(
                "Unsupported extrinsic version: {}. Expected version 4",
                version
            )));
        }

        let length_result = parity_scale_codec::Compact::<u32>::decode(&mut &extrinsic_bytes[1..]);
        if length_result.is_err() {
            return Err(SdkError::TransactionError(
                "Invalid extrinsic length encoding".to_string(),
            ));
        }

        Ok(())
    }

    fn create_mock_signed_extrinsic() -> Vec<u8> {
        let mut extrinsic = Vec::new();
        extrinsic.push(0x84);

        use parity_scale_codec::Compact;
        let length = Compact(100u32);
        extrinsic.extend_from_slice(&length.encode());

        extrinsic.extend_from_slice(&[0u8; 100]);

        extrinsic
    }
}
