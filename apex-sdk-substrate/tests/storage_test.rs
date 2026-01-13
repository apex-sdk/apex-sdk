//! Comprehensive tests for storage module
//!
//! These tests verify storage query functionality including:
//! - Account information queries
//! - Balance queries
//! - Storage queries
//! - Metadata queries
//! - Helper functions

use apex_sdk_substrate::storage::*;

#[test]
fn test_account_info_default() {
    let info = AccountInfo::default();

    assert_eq!(info.nonce, 0);
    assert_eq!(info.consumers, 0);
    assert_eq!(info.providers, 0);
    assert_eq!(info.sufficients, 0);
    assert_eq!(info.free, 0);
    assert_eq!(info.reserved, 0);
    assert_eq!(info.frozen, 0);
}

#[test]
fn test_account_info_total() {
    let info = AccountInfo {
        nonce: 5,
        consumers: 1,
        providers: 1,
        sufficients: 0,
        free: 1_000_000_000_000,
        reserved: 500_000_000_000,
        frozen: 100_000_000_000,
    };

    assert_eq!(info.total(), 1_500_000_000_000);
}

#[test]
fn test_account_info_total_overflow_prevention() {
    let info = AccountInfo {
        nonce: 0,
        consumers: 0,
        providers: 0,
        sufficients: 0,
        free: u128::MAX / 2,
        reserved: u128::MAX / 2,
        frozen: 0,
    };

    // Should use saturating_add to prevent overflow
    let total = info.total();
    assert!(total > 0);
}

#[test]
fn test_account_info_transferable() {
    let info = AccountInfo {
        nonce: 5,
        consumers: 1,
        providers: 1,
        sufficients: 0,
        free: 1_000_000_000_000,
        reserved: 500_000_000_000,
        frozen: 100_000_000_000,
    };

    assert_eq!(info.transferable(), 900_000_000_000);
}

#[test]
fn test_account_info_transferable_frozen_exceeds_free() {
    let info = AccountInfo {
        nonce: 0,
        consumers: 0,
        providers: 0,
        sufficients: 0,
        free: 100_000_000_000,
        reserved: 0,
        frozen: 200_000_000_000,
    };

    // Should use saturating_sub to prevent underflow
    assert_eq!(info.transferable(), 0);
}

#[test]
fn test_account_info_transferable_all_frozen() {
    let info = AccountInfo {
        nonce: 0,
        consumers: 0,
        providers: 0,
        sufficients: 0,
        free: 1_000_000_000_000,
        reserved: 0,
        frozen: 1_000_000_000_000,
    };

    assert_eq!(info.transferable(), 0);
}

#[test]
fn test_account_info_transferable_none_frozen() {
    let info = AccountInfo {
        nonce: 0,
        consumers: 0,
        providers: 0,
        sufficients: 0,
        free: 1_000_000_000_000,
        reserved: 0,
        frozen: 0,
    };

    assert_eq!(info.transferable(), 1_000_000_000_000);
}

#[test]
fn test_account_info_with_nonce() {
    let info = AccountInfo {
        nonce: 42,
        consumers: 0,
        providers: 0,
        sufficients: 0,
        free: 0,
        reserved: 0,
        frozen: 0,
    };

    assert_eq!(info.nonce, 42);
}

#[test]
fn test_account_info_with_consumers() {
    let info = AccountInfo {
        nonce: 0,
        consumers: 5,
        providers: 0,
        sufficients: 0,
        free: 0,
        reserved: 0,
        frozen: 0,
    };

    assert_eq!(info.consumers, 5);
}

#[test]
fn test_account_info_with_providers() {
    let info = AccountInfo {
        nonce: 0,
        consumers: 0,
        providers: 3,
        sufficients: 0,
        free: 0,
        reserved: 0,
        frozen: 0,
    };

    assert_eq!(info.providers, 3);
}

#[test]
fn test_account_info_with_sufficients() {
    let info = AccountInfo {
        nonce: 0,
        consumers: 0,
        providers: 0,
        sufficients: 2,
        free: 0,
        reserved: 0,
        frozen: 0,
    };

    assert_eq!(info.sufficients, 2);
}

#[test]
fn test_account_info_clone() {
    let info = AccountInfo {
        nonce: 10,
        consumers: 1,
        providers: 1,
        sufficients: 0,
        free: 1_000_000,
        reserved: 500_000,
        frozen: 100_000,
    };

    let cloned = info.clone();

    assert_eq!(cloned.nonce, info.nonce);
    assert_eq!(cloned.free, info.free);
    assert_eq!(cloned.reserved, info.reserved);
    assert_eq!(cloned.frozen, info.frozen);
}

#[test]
fn test_account_info_debug() {
    let info = AccountInfo {
        nonce: 5,
        consumers: 1,
        providers: 1,
        sufficients: 0,
        free: 1_000_000_000_000,
        reserved: 500_000_000_000,
        frozen: 100_000_000_000,
    };

    let debug_output = format!("{:?}", info);
    assert!(debug_output.contains("AccountInfo"));
    assert!(debug_output.contains("nonce"));
    assert!(debug_output.contains("5"));
}

#[test]
fn test_pallet_metadata_structure() {
    let metadata = PalletMetadata {
        name: "Balances".to_string(),
        index: 5,
        storage_count: 10,
        call_count: 8,
        event_count: 6,
        constant_count: 4,
        error_count: 12,
    };

    assert_eq!(metadata.name, "Balances");
    assert_eq!(metadata.index, 5);
    assert_eq!(metadata.storage_count, 10);
    assert_eq!(metadata.call_count, 8);
    assert_eq!(metadata.event_count, 6);
    assert_eq!(metadata.constant_count, 4);
    assert_eq!(metadata.error_count, 12);
}

#[test]
fn test_pallet_metadata_clone() {
    let metadata = PalletMetadata {
        name: "System".to_string(),
        index: 0,
        storage_count: 15,
        call_count: 10,
        event_count: 5,
        constant_count: 3,
        error_count: 8,
    };

    let cloned = metadata.clone();

    assert_eq!(cloned.name, metadata.name);
    assert_eq!(cloned.index, metadata.index);
    assert_eq!(cloned.storage_count, metadata.storage_count);
}

#[test]
fn test_pallet_metadata_debug() {
    let metadata = PalletMetadata {
        name: "Timestamp".to_string(),
        index: 3,
        storage_count: 2,
        call_count: 1,
        event_count: 0,
        constant_count: 1,
        error_count: 0,
    };

    let debug_output = format!("{:?}", metadata);
    assert!(debug_output.contains("PalletMetadata"));
    assert!(debug_output.contains("Timestamp"));
}

#[test]
fn test_storage_query_new() {
    let _query = StorageQuery::new("System", "Account");
    // StorageQuery fields are private, so we can only test that it constructs successfully
}

#[test]
fn test_storage_query_with_key() {
    use subxt::dynamic::Value;

    let _query = StorageQuery::new("System", "Account").key(Value::from_bytes([0u8; 32]));
    // Successfully created query with key
}

#[test]
fn test_storage_query_with_multiple_keys() {
    use subxt::dynamic::Value;

    let _query = StorageQuery::new("Staking", "Bonded")
        .key(Value::from_bytes([1u8; 32]))
        .key(Value::from_bytes([2u8; 32]));
    // Successfully created query with multiple keys
}

#[test]
fn test_storage_query_with_keys_vector() {
    use subxt::dynamic::Value;

    let keys = vec![
        Value::from_bytes([1u8; 32]),
        Value::from_bytes([2u8; 32]),
        Value::from_bytes([3u8; 32]),
    ];

    let _query = StorageQuery::new("System", "Account").keys(keys);
    // Successfully created query with keys vector
}

#[test]
fn test_storage_query_builder_pattern() {
    use subxt::dynamic::Value;

    let _query = StorageQuery::new("Balances", "TotalIssuance")
        .key(Value::u128(0))
        .key(Value::u128(1));
    // Successfully created query with builder pattern
}

#[test]
fn test_account_info_various_balances() {
    // Test case 1: Normal account
    let info1 = AccountInfo {
        nonce: 10,
        consumers: 1,
        providers: 1,
        sufficients: 0,
        free: 10_000_000_000_000,
        reserved: 1_000_000_000_000,
        frozen: 500_000_000_000,
    };
    assert_eq!(info1.total(), 11_000_000_000_000);
    assert_eq!(info1.transferable(), 9_500_000_000_000);

    // Test case 2: Account with no reserved balance
    let info2 = AccountInfo {
        nonce: 5,
        consumers: 0,
        providers: 1,
        sufficients: 0,
        free: 5_000_000_000_000,
        reserved: 0,
        frozen: 0,
    };
    assert_eq!(info2.total(), 5_000_000_000_000);
    assert_eq!(info2.transferable(), 5_000_000_000_000);

    // Test case 3: Account with everything frozen
    let info3 = AccountInfo {
        nonce: 0,
        consumers: 1,
        providers: 1,
        sufficients: 0,
        free: 3_000_000_000_000,
        reserved: 2_000_000_000_000,
        frozen: 3_000_000_000_000,
    };
    assert_eq!(info3.total(), 5_000_000_000_000);
    assert_eq!(info3.transferable(), 0);
}

#[test]
fn test_account_info_edge_cases() {
    // Edge case: Zero balances
    let info = AccountInfo {
        nonce: 0,
        consumers: 0,
        providers: 0,
        sufficients: 0,
        free: 0,
        reserved: 0,
        frozen: 0,
    };
    assert_eq!(info.total(), 0);
    assert_eq!(info.transferable(), 0);

    // Edge case: Maximum values
    let info = AccountInfo {
        nonce: u64::MAX,
        consumers: u32::MAX,
        providers: u32::MAX,
        sufficients: u32::MAX,
        free: u128::MAX / 4,
        reserved: u128::MAX / 4,
        frozen: 0,
    };
    assert!(info.total() > 0);
    assert!(info.transferable() > 0);
}

#[test]
fn test_pallet_metadata_various_pallets() {
    // System pallet
    let system = PalletMetadata {
        name: "System".to_string(),
        index: 0,
        storage_count: 16,
        call_count: 9,
        event_count: 6,
        constant_count: 7,
        error_count: 8,
    };
    assert_eq!(system.index, 0);

    // Balances pallet
    let balances = PalletMetadata {
        name: "Balances".to_string(),
        index: 5,
        storage_count: 5,
        call_count: 6,
        event_count: 10,
        constant_count: 2,
        error_count: 10,
    };
    assert_eq!(balances.name, "Balances");

    // Custom pallet
    let custom = PalletMetadata {
        name: "MyCustomPallet".to_string(),
        index: 100,
        storage_count: 3,
        call_count: 2,
        event_count: 1,
        constant_count: 0,
        error_count: 5,
    };
    assert_eq!(custom.index, 100);
}

#[test]
fn test_storage_query_different_pallets() {
    let _queries = [
        StorageQuery::new("System", "Account"),
        StorageQuery::new("Balances", "TotalIssuance"),
        StorageQuery::new("Timestamp", "Now"),
        StorageQuery::new("Staking", "Bonded"),
    ];
    // Successfully created queries for different pallets
}

#[test]
fn test_account_info_realistic_values() {
    // Realistic Polkadot account with 100 DOT
    let dot_account = AccountInfo {
        nonce: 25,
        consumers: 1,
        providers: 1,
        sufficients: 0,
        free: 1_000_000_000_000,   // 100 DOT (10 decimals)
        reserved: 100_000_000_000, // 10 DOT staked
        frozen: 100_000_000_000,   // 10 DOT frozen
    };
    assert_eq!(dot_account.total(), 1_100_000_000_000);
    assert_eq!(dot_account.transferable(), 900_000_000_000);

    // Realistic Kusama account with 50 KSM
    let ksm_account = AccountInfo {
        nonce: 50,
        consumers: 2,
        providers: 1,
        sufficients: 0,
        free: 50_000_000_000_000,    // 50 KSM (12 decimals)
        reserved: 5_000_000_000_000, // 5 KSM reserved
        frozen: 2_000_000_000_000,   // 2 KSM frozen
    };
    assert_eq!(ksm_account.total(), 55_000_000_000_000);
    assert_eq!(ksm_account.transferable(), 48_000_000_000_000);
}

#[test]
fn test_storage_query_empty_keys() {
    let _query = StorageQuery::new("Balances", "TotalIssuance");
    // Successfully created query with no keys
}

#[test]
fn test_pallet_metadata_zero_counts() {
    let metadata = PalletMetadata {
        name: "EmptyPallet".to_string(),
        index: 99,
        storage_count: 0,
        call_count: 0,
        event_count: 0,
        constant_count: 0,
        error_count: 0,
    };

    assert_eq!(metadata.storage_count, 0);
    assert_eq!(metadata.call_count, 0);
    assert_eq!(metadata.event_count, 0);
}
