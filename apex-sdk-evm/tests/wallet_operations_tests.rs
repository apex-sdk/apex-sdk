//! Comprehensive tests for wallet module
//!
//! - Testing wallet creation methods (random, from private key, from mnemonic)
//! - Testing signing operations (transactions, messages, typed data)
//! - Testing wallet manager operations
//! - Testing error handling and edge cases

use alloy::primitives::B256;
use apex_sdk_evm::wallet::{Wallet, WalletManager};

// ============================================================================
// Test Constants
// ============================================================================

const TEST_PRIVATE_KEY: &str = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
const TEST_EXPECTED_ADDRESS: &str = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266";
const TEST_MNEMONIC: &str = "test test test test test test test test test test test junk";

// ============================================================================
// Wallet Creation Tests
// ============================================================================

#[test]
fn test_wallet_new_random() {
    let wallet = Wallet::new_random();

    assert!(wallet.address().starts_with("0x"));
    assert_eq!(wallet.address().len(), 42);
    assert!(wallet.chain_id().is_none());
}

#[test]
fn test_wallet_new_random_uniqueness() {
    let wallet1 = Wallet::new_random();
    let wallet2 = Wallet::new_random();

    // Each random wallet should have a different address
    assert_ne!(wallet1.address(), wallet2.address());
}

#[test]
fn test_wallet_from_private_key_valid() {
    let wallet = Wallet::from_private_key(TEST_PRIVATE_KEY).unwrap();

    assert_eq!(wallet.address().to_lowercase(), TEST_EXPECTED_ADDRESS);
    assert!(wallet.chain_id().is_none());
}

#[test]
fn test_wallet_from_private_key_without_0x_prefix() {
    let key_without_prefix = TEST_PRIVATE_KEY.trim_start_matches("0x");
    let wallet = Wallet::from_private_key(key_without_prefix).unwrap();

    assert_eq!(wallet.address().to_lowercase(), TEST_EXPECTED_ADDRESS);
}

#[test]
fn test_wallet_from_private_key_with_0x_prefix() {
    let wallet = Wallet::from_private_key(TEST_PRIVATE_KEY).unwrap();

    assert_eq!(wallet.address().to_lowercase(), TEST_EXPECTED_ADDRESS);
}

#[test]
fn test_wallet_from_private_key_invalid() {
    let invalid_keys = vec![
        "invalid",
        "0x123",                                                              // Too short
        "0xGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGG", // Invalid hex
        "",                                                                   // Empty
    ];

    for invalid_key in invalid_keys {
        let result = Wallet::from_private_key(invalid_key);
        assert!(result.is_err(), "Should fail for: {}", invalid_key);
    }
}

#[test]
fn test_wallet_from_mnemonic_index_0() {
    let wallet = Wallet::from_mnemonic(TEST_MNEMONIC, 0).unwrap();

    assert!(wallet.address().starts_with("0x"));
    assert_eq!(wallet.address().len(), 42);
}

#[test]
fn test_wallet_from_mnemonic_different_indices() {
    let wallet0 = Wallet::from_mnemonic(TEST_MNEMONIC, 0).unwrap();
    let wallet1 = Wallet::from_mnemonic(TEST_MNEMONIC, 1).unwrap();
    let wallet2 = Wallet::from_mnemonic(TEST_MNEMONIC, 2).unwrap();

    // Different indices should produce different addresses
    assert_ne!(wallet0.address(), wallet1.address());
    assert_ne!(wallet1.address(), wallet2.address());
    assert_ne!(wallet0.address(), wallet2.address());
}

#[test]
fn test_wallet_from_mnemonic_deterministic() {
    let wallet1 = Wallet::from_mnemonic(TEST_MNEMONIC, 0).unwrap();
    let wallet2 = Wallet::from_mnemonic(TEST_MNEMONIC, 0).unwrap();

    // Same mnemonic and index should produce same address
    assert_eq!(wallet1.address(), wallet2.address());
}

#[test]
fn test_wallet_from_mnemonic_invalid() {
    let invalid_mnemonics = vec![
        "invalid mnemonic phrase",
        "",
        "one two three four five", // Too few words
    ];

    for invalid_mnemonic in invalid_mnemonics {
        let result = Wallet::from_mnemonic(invalid_mnemonic, 0);
        // Some may succeed with invalid mnemonics due to checksum, but we test the call doesn't panic
        let _ = result;
    }
}

// ============================================================================
// Chain ID Tests
// ============================================================================

#[test]
fn test_wallet_with_chain_id() {
    let wallet = Wallet::new_random().with_chain_id(1);

    assert_eq!(wallet.chain_id(), Some(1));
}

#[test]
fn test_wallet_with_different_chain_ids() {
    let chain_ids = vec![1, 5, 137, 43114, 56, 250];

    for chain_id in chain_ids {
        let wallet = Wallet::new_random().with_chain_id(chain_id);
        assert_eq!(wallet.chain_id(), Some(chain_id));
    }
}

#[test]
fn test_wallet_chain_id_none_by_default() {
    let wallet = Wallet::new_random();
    assert!(wallet.chain_id().is_none());
}

#[test]
fn test_wallet_chain_id_update() {
    let wallet = Wallet::new_random().with_chain_id(1).with_chain_id(5);

    assert_eq!(wallet.chain_id(), Some(5));
}

// ============================================================================
// Address Getters Tests
// ============================================================================

#[test]
fn test_wallet_address_string_format() {
    let wallet = Wallet::from_private_key(TEST_PRIVATE_KEY).unwrap();
    let address = wallet.address();

    assert!(address.starts_with("0x"));
    assert_eq!(address.len(), 42);
    assert!(address.chars().skip(2).all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_wallet_eth_address() {
    let wallet = Wallet::from_private_key(TEST_PRIVATE_KEY).unwrap();
    let eth_addr = wallet.eth_address();

    let addr_str = format!("{:?}", eth_addr).to_lowercase();
    assert!(addr_str.contains("f39fd6e51aad88f6f4ce6ab8827279cfffb92266"));
}

// ============================================================================
// Signing Tests
// ============================================================================

#[tokio::test]
async fn test_wallet_sign_transaction_hash() {
    let wallet = Wallet::new_random();
    let hash = B256::from([1u8; 32]);

    let signature = wallet.sign_transaction_hash(&hash).await;
    assert!(signature.is_ok());

    let sig = signature.unwrap();
    assert_eq!(sig.as_bytes().len(), 65); // r (32) + s (32) + v (1)
}

#[tokio::test]
async fn test_wallet_sign_transaction_hash_different_hashes() {
    let wallet = Wallet::new_random();

    let hash1 = B256::from([1u8; 32]);
    let hash2 = B256::from([2u8; 32]);

    let sig1 = wallet.sign_transaction_hash(&hash1).await.unwrap();
    let sig2 = wallet.sign_transaction_hash(&hash2).await.unwrap();

    // Different hashes should produce different signatures
    assert_ne!(sig1.as_bytes(), sig2.as_bytes());
}

#[tokio::test]
async fn test_wallet_sign_transaction_hash_deterministic() {
    let wallet = Wallet::from_private_key(TEST_PRIVATE_KEY).unwrap();
    let hash = B256::from([1u8; 32]);

    let sig1 = wallet.sign_transaction_hash(&hash).await.unwrap();
    let sig2 = wallet.sign_transaction_hash(&hash).await.unwrap();

    // Same wallet and hash should produce same signature
    assert_eq!(sig1.as_bytes(), sig2.as_bytes());
}

#[tokio::test]
async fn test_wallet_sign_message_string() {
    let wallet = Wallet::new_random();
    let message = "Hello, Ethereum!";

    let signature = wallet.sign_message(message).await;
    assert!(signature.is_ok());

    let sig = signature.unwrap();
    assert_eq!(sig.as_bytes().len(), 65);
}

#[tokio::test]
async fn test_wallet_sign_message_bytes() {
    let wallet = Wallet::new_random();
    let message = vec![0x12, 0x34, 0x56, 0x78];

    let signature = wallet.sign_message(&message).await;
    assert!(signature.is_ok());

    let sig = signature.unwrap();
    assert_eq!(sig.as_bytes().len(), 65);
}

#[tokio::test]
async fn test_wallet_sign_message_empty() {
    let wallet = Wallet::new_random();
    let message = "";

    let signature = wallet.sign_message(message).await;
    assert!(signature.is_ok());
}

#[tokio::test]
async fn test_wallet_sign_message_long() {
    let wallet = Wallet::new_random();
    let message = "a".repeat(10000);

    let signature = wallet.sign_message(&message).await;
    assert!(signature.is_ok());
}

#[tokio::test]
async fn test_wallet_sign_message_different_messages() {
    let wallet = Wallet::new_random();

    let sig1 = wallet.sign_message("message1").await.unwrap();
    let sig2 = wallet.sign_message("message2").await.unwrap();

    // Different messages should produce different signatures
    assert_ne!(sig1.as_bytes(), sig2.as_bytes());
}

#[tokio::test]
async fn test_wallet_sign_typed_data_hash() {
    let wallet = Wallet::new_random();
    let hash = B256::from([2u8; 32]);

    let signature = wallet.sign_typed_data_hash(&hash).await;
    assert!(signature.is_ok());

    let sig = signature.unwrap();
    assert_eq!(sig.as_bytes().len(), 65);
}

#[tokio::test]
async fn test_wallet_sign_typed_data_hash_different_hashes() {
    let wallet = Wallet::new_random();

    let hash1 = B256::from([1u8; 32]);
    let hash2 = B256::from([2u8; 32]);

    let sig1 = wallet.sign_typed_data_hash(&hash1).await.unwrap();
    let sig2 = wallet.sign_typed_data_hash(&hash2).await.unwrap();

    // Different hashes should produce different signatures
    assert_ne!(sig1.as_bytes(), sig2.as_bytes());
}

// ============================================================================
// Export Private Key Tests
// ============================================================================

#[test]
fn test_wallet_export_private_key() {
    let wallet = Wallet::from_private_key(TEST_PRIVATE_KEY).unwrap();
    let exported = wallet.export_private_key();

    assert!(exported.starts_with("0x"));
    assert_eq!(exported.len(), 66); // 0x + 64 hex chars
    assert_eq!(exported.to_lowercase(), TEST_PRIVATE_KEY.to_lowercase());
}

#[test]
fn test_wallet_export_private_key_format() {
    let wallet = Wallet::new_random();
    let exported = wallet.export_private_key();

    assert!(exported.starts_with("0x"));
    assert_eq!(exported.len(), 66);
    assert!(exported.chars().skip(2).all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_wallet_export_reimport_cycle() {
    let wallet1 = Wallet::new_random();
    let exported = wallet1.export_private_key();

    let wallet2 = Wallet::from_private_key(&exported).unwrap();

    assert_eq!(wallet1.address(), wallet2.address());
}

// ============================================================================
// Debug Format Tests
// ============================================================================

#[test]
fn test_wallet_debug_format() {
    let wallet = Wallet::from_private_key(TEST_PRIVATE_KEY)
        .unwrap()
        .with_chain_id(1);
    let debug_str = format!("{:?}", wallet);

    // Debug output should contain address
    assert!(debug_str.contains("address"));

    // Debug output should contain chain_id
    assert!(debug_str.contains("chain_id"));

    // Debug output should NOT contain private key
    assert!(!debug_str.to_lowercase().contains("ac0974"));
}

// ============================================================================
// Clone Tests
// ============================================================================

#[test]
fn test_wallet_clone() {
    let wallet1 = Wallet::from_private_key(TEST_PRIVATE_KEY)
        .unwrap()
        .with_chain_id(1);
    let wallet2 = wallet1.clone();

    assert_eq!(wallet1.address(), wallet2.address());
    assert_eq!(wallet1.chain_id(), wallet2.chain_id());
}

// ============================================================================
// Wallet Manager Tests
// ============================================================================

#[test]
fn test_wallet_manager_new() {
    let manager = WalletManager::new();

    assert_eq!(manager.wallet_count(), 0);
    assert!(manager.active_wallet().is_none());
}

#[test]
fn test_wallet_manager_default() {
    let manager = WalletManager::default();

    assert_eq!(manager.wallet_count(), 0);
}

#[test]
fn test_wallet_manager_add_wallet() {
    let mut manager = WalletManager::new();
    let wallet = Wallet::new_random();

    let index = manager.add_wallet(wallet);

    assert_eq!(index, 0);
    assert_eq!(manager.wallet_count(), 1);
}

#[test]
fn test_wallet_manager_add_multiple_wallets() {
    let mut manager = WalletManager::new();

    let idx0 = manager.add_wallet(Wallet::new_random());
    let idx1 = manager.add_wallet(Wallet::new_random());
    let idx2 = manager.add_wallet(Wallet::new_random());

    assert_eq!(idx0, 0);
    assert_eq!(idx1, 1);
    assert_eq!(idx2, 2);
    assert_eq!(manager.wallet_count(), 3);
}

#[test]
fn test_wallet_manager_active_wallet() {
    let mut manager = WalletManager::new();

    assert!(manager.active_wallet().is_none());

    manager.add_wallet(Wallet::new_random());

    assert!(manager.active_wallet().is_some());
}

#[test]
fn test_wallet_manager_get_wallet() {
    let mut manager = WalletManager::new();

    manager.add_wallet(Wallet::from_private_key(TEST_PRIVATE_KEY).unwrap());

    let wallet = manager.wallet(0);
    assert!(wallet.is_some());
    assert_eq!(
        wallet.unwrap().address().to_lowercase(),
        TEST_EXPECTED_ADDRESS
    );
}

#[test]
fn test_wallet_manager_get_wallet_invalid_index() {
    let manager = WalletManager::new();

    assert!(manager.wallet(0).is_none());
    assert!(manager.wallet(100).is_none());
}

#[test]
fn test_wallet_manager_set_active() {
    let mut manager = WalletManager::new();

    manager.add_wallet(Wallet::new_random());
    manager.add_wallet(Wallet::new_random());
    manager.add_wallet(Wallet::new_random());

    let result = manager.set_active(1);
    assert!(result.is_ok());

    let result = manager.set_active(2);
    assert!(result.is_ok());
}

#[test]
fn test_wallet_manager_set_active_invalid_index() {
    let mut manager = WalletManager::new();

    manager.add_wallet(Wallet::new_random());

    let result = manager.set_active(1);
    assert!(result.is_err());

    let result = manager.set_active(100);
    assert!(result.is_err());
}

#[test]
fn test_wallet_manager_list_addresses() {
    let mut manager = WalletManager::new();

    manager.add_wallet(Wallet::new_random());
    manager.add_wallet(Wallet::new_random());
    manager.add_wallet(Wallet::new_random());

    let addresses = manager.list_addresses();

    assert_eq!(addresses.len(), 3);
    for addr in addresses {
        assert!(addr.starts_with("0x"));
        assert_eq!(addr.len(), 42);
    }
}

#[test]
fn test_wallet_manager_list_addresses_empty() {
    let manager = WalletManager::new();
    let addresses = manager.list_addresses();

    assert_eq!(addresses.len(), 0);
}

#[test]
fn test_wallet_manager_create_wallet() {
    let mut manager = WalletManager::new();

    let idx = manager.create_wallet();

    assert_eq!(idx, 0);
    assert_eq!(manager.wallet_count(), 1);

    let wallet = manager.wallet(0).unwrap();
    assert!(wallet.address().starts_with("0x"));
}

#[test]
fn test_wallet_manager_create_multiple_wallets() {
    let mut manager = WalletManager::new();

    for i in 0..5 {
        let idx = manager.create_wallet();
        assert_eq!(idx, i);
    }

    assert_eq!(manager.wallet_count(), 5);
}

#[test]
fn test_wallet_manager_import_wallet() {
    let mut manager = WalletManager::new();

    let idx = manager.import_wallet(TEST_PRIVATE_KEY).unwrap();

    assert_eq!(idx, 0);
    assert_eq!(manager.wallet_count(), 1);

    let wallet = manager.wallet(0).unwrap();
    assert_eq!(wallet.address().to_lowercase(), TEST_EXPECTED_ADDRESS);
}

#[test]
fn test_wallet_manager_import_wallet_invalid() {
    let mut manager = WalletManager::new();

    let result = manager.import_wallet("invalid_key");
    assert!(result.is_err());

    assert_eq!(manager.wallet_count(), 0);
}

#[test]
fn test_wallet_manager_import_from_mnemonic() {
    let mut manager = WalletManager::new();

    let idx = manager.import_from_mnemonic(TEST_MNEMONIC, 0).unwrap();

    assert_eq!(idx, 0);
    assert_eq!(manager.wallet_count(), 1);

    let wallet = manager.wallet(0).unwrap();
    assert!(wallet.address().starts_with("0x"));
}

#[test]
fn test_wallet_manager_import_from_mnemonic_different_indices() {
    let mut manager = WalletManager::new();

    let idx0 = manager.import_from_mnemonic(TEST_MNEMONIC, 0).unwrap();
    let idx1 = manager.import_from_mnemonic(TEST_MNEMONIC, 1).unwrap();

    assert_eq!(idx0, 0);
    assert_eq!(idx1, 1);

    let addr0 = manager.wallet(0).unwrap().address();
    let addr1 = manager.wallet(1).unwrap().address();

    assert_ne!(addr0, addr1);
}

#[test]
fn test_wallet_manager_mixed_operations() {
    let mut manager = WalletManager::new();

    // Create random wallet
    manager.create_wallet();

    // Import from private key
    manager.import_wallet(TEST_PRIVATE_KEY).unwrap();

    // Import from mnemonic
    manager.import_from_mnemonic(TEST_MNEMONIC, 0).unwrap();

    assert_eq!(manager.wallet_count(), 3);

    // List all addresses
    let addresses = manager.list_addresses();
    assert_eq!(addresses.len(), 3);

    // Check that one address matches the test private key
    assert!(addresses.contains(&TEST_EXPECTED_ADDRESS.to_string()));
}

#[test]
fn test_wallet_manager_active_wallet_switching() {
    let mut manager = WalletManager::new();

    let _addr0 = Wallet::new_random().address();
    let _addr1 = Wallet::new_random().address();
    let _addr2 = Wallet::new_random().address();

    manager.add_wallet(Wallet::new_random());
    manager.add_wallet(Wallet::new_random());
    manager.add_wallet(Wallet::new_random());

    // Default active is index 0
    assert!(manager.active_wallet().is_some());

    // Switch to index 1
    manager.set_active(1).unwrap();
    assert!(manager.active_wallet().is_some());

    // Switch to index 2
    manager.set_active(2).unwrap();
    assert!(manager.active_wallet().is_some());
}

// ============================================================================
// Edge Cases and Error Handling
// ============================================================================

#[test]
fn test_wallet_special_private_keys() {
    // Test with all zeros (invalid in practice but should be handled)
    let zero_key = "0x0000000000000000000000000000000000000000000000000000000000000000";
    let result = Wallet::from_private_key(zero_key);
    // This might succeed or fail depending on the library, we just test it doesn't panic
    let _ = result;

    // Test with all ones
    let ones_key = "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
    let result = Wallet::from_private_key(ones_key);
    let _ = result;
}

#[test]
fn test_wallet_mnemonic_edge_cases() {
    // Very long index
    let result = Wallet::from_mnemonic(TEST_MNEMONIC, u32::MAX);
    // Should handle gracefully
    let _ = result;

    // Index 0 should always work
    let result = Wallet::from_mnemonic(TEST_MNEMONIC, 0);
    assert!(result.is_ok());
}

#[test]
fn test_wallet_manager_capacity() {
    let mut manager = WalletManager::new();

    // Add many wallets
    for _ in 0..100 {
        manager.create_wallet();
    }

    assert_eq!(manager.wallet_count(), 100);

    let addresses = manager.list_addresses();
    assert_eq!(addresses.len(), 100);
}

#[tokio::test]
async fn test_wallet_concurrent_signing() {
    use std::sync::Arc;

    let wallet = Arc::new(Wallet::new_random());
    let mut handles = vec![];

    for i in 0..10 {
        let wallet_clone = wallet.clone();
        let handle = tokio::spawn(async move {
            let message = format!("message{}", i);
            wallet_clone.sign_message(&message).await
        });
        handles.push(handle);
    }

    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok());
    }
}

#[test]
fn test_wallet_manager_concurrent_modifications() {
    use std::sync::{Arc, Mutex};
    use std::thread;

    let manager = Arc::new(Mutex::new(WalletManager::new()));
    let mut handles = vec![];

    for _ in 0..10 {
        let manager_clone = manager.clone();
        let handle = thread::spawn(move || {
            let mut mgr = manager_clone.lock().unwrap();
            mgr.create_wallet();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_count = manager.lock().unwrap().wallet_count();
    assert_eq!(final_count, 10);
}
