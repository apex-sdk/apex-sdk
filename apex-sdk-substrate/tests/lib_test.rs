//! Comprehensive tests for lib module
//!
//! These tests verify core adapter functionality including:
//! - Chain configuration
//! - Error handling
//! - Address validation
//! - Constants and type conversions

use apex_sdk_substrate::*;
use apex_sdk_types::Address;

#[test]
fn test_chain_config_polkadot() {
    let config = ChainConfig::polkadot();

    assert_eq!(config.name, "Polkadot");
    assert_eq!(config.ss58_prefix, 0);
    assert_eq!(config.token_symbol, "DOT");
    assert_eq!(config.token_decimals, 10);
    assert!(config.endpoint.starts_with("wss://"));
    assert!(config.endpoint.contains("polkadot"));
}

#[test]
fn test_chain_config_kusama() {
    let config = ChainConfig::kusama();

    assert_eq!(config.name, "Kusama");
    assert_eq!(config.ss58_prefix, 2);
    assert_eq!(config.token_symbol, "KSM");
    assert_eq!(config.token_decimals, 12);
    assert!(config.endpoint.starts_with("wss://"));
    assert!(config.endpoint.contains("kusama"));
}

#[test]
fn test_chain_config_westend() {
    let config = ChainConfig::westend();

    assert_eq!(config.name, "Westend");
    assert_eq!(config.ss58_prefix, 42);
    assert_eq!(config.token_symbol, "WND");
    assert_eq!(config.token_decimals, 12);
    assert!(config.endpoint.starts_with("wss://"));
    assert!(config.endpoint.contains("westend"));
}

#[test]
fn test_chain_config_paseo() {
    let config = ChainConfig::paseo();

    assert_eq!(config.name, "Paseo");
    assert_eq!(config.ss58_prefix, 42);
    assert_eq!(config.token_symbol, "PAS");
    assert_eq!(config.token_decimals, 10);
    assert_eq!(config.endpoint, "wss://paseo.rpc.amforc.com");
}

#[test]
fn test_chain_config_custom() {
    let config = ChainConfig::custom("MyChain", "wss://my.endpoint.com", 100);

    assert_eq!(config.name, "MyChain");
    assert_eq!(config.endpoint, "wss://my.endpoint.com");
    assert_eq!(config.ss58_prefix, 100);
    assert_eq!(config.token_symbol, "UNIT");
    assert_eq!(config.token_decimals, 12);
}

#[test]
fn test_chain_config_clone() {
    let config = ChainConfig::polkadot();
    let cloned = config.clone();

    assert_eq!(cloned.name, config.name);
    assert_eq!(cloned.endpoint, config.endpoint);
    assert_eq!(cloned.ss58_prefix, config.ss58_prefix);
    assert_eq!(cloned.token_symbol, config.token_symbol);
    assert_eq!(cloned.token_decimals, config.token_decimals);
}

#[test]
fn test_chain_config_debug() {
    let config = ChainConfig::polkadot();
    let debug_output = format!("{:?}", config);

    assert!(debug_output.contains("ChainConfig"));
    assert!(debug_output.contains("Polkadot"));
}

#[test]
fn test_error_connection() {
    let error = Error::Connection("Failed to connect".to_string());
    assert_eq!(error.to_string(), "Connection error: Failed to connect");
}

#[test]
fn test_error_transaction() {
    let error = Error::Transaction("Transaction failed".to_string());
    assert_eq!(error.to_string(), "Transaction error: Transaction failed");
}

#[test]
fn test_error_metadata() {
    let error = Error::Metadata("Invalid metadata".to_string());
    assert_eq!(error.to_string(), "Metadata error: Invalid metadata");
}

#[test]
fn test_error_storage() {
    let error = Error::Storage("Storage query failed".to_string());
    assert_eq!(error.to_string(), "Storage error: Storage query failed");
}

#[test]
fn test_error_wallet() {
    let error = Error::Wallet("Invalid wallet".to_string());
    assert_eq!(error.to_string(), "Wallet error: Invalid wallet");
}

#[test]
fn test_error_signature() {
    let error = Error::Signature("Invalid signature".to_string());
    assert_eq!(error.to_string(), "Signature error: Invalid signature");
}

#[test]
fn test_error_encoding() {
    let error = Error::Encoding("Encoding failed".to_string());
    assert_eq!(error.to_string(), "Encoding error: Encoding failed");
}

#[test]
fn test_error_other() {
    let error = Error::Other("Unknown error".to_string());
    assert_eq!(error.to_string(), "Other error: Unknown error");
}

#[test]
fn test_error_debug() {
    let error = Error::Connection("Test error".to_string());
    let debug_output = format!("{:?}", error);

    assert!(debug_output.contains("Connection"));
    assert!(debug_output.contains("Test error"));
}

#[test]
fn test_error_from_subxt() {
    let subxt_error = subxt::Error::Other("RPC error".to_string());
    let our_error: Error = subxt_error.into();

    match our_error {
        Error::Subxt(_) => {} // Expected
        _ => panic!("Expected Subxt error variant"),
    }
}

#[test]
fn test_address_substrate() {
    let addr = Address::substrate("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY");

    match addr {
        Address::Substrate(s) => {
            assert_eq!(s, "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY");
        }
        _ => panic!("Expected Substrate address"),
    }
}

#[test]
fn test_address_evm() {
    let addr = Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7");

    match addr {
        Address::Evm(s) => {
            assert_eq!(s, "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7");
        }
        _ => panic!("Expected EVM address"),
    }
}

#[test]
fn test_all_chain_configs() {
    let configs = [
        ChainConfig::polkadot(),
        ChainConfig::kusama(),
        ChainConfig::westend(),
        ChainConfig::paseo(),
    ];

    assert_eq!(configs[0].name, "Polkadot");
    assert_eq!(configs[1].name, "Kusama");
    assert_eq!(configs[2].name, "Westend");
    assert_eq!(configs[3].name, "Paseo");
}

#[test]
fn test_chain_config_ss58_prefixes() {
    assert_eq!(ChainConfig::polkadot().ss58_prefix, 0);
    assert_eq!(ChainConfig::kusama().ss58_prefix, 2);
    assert_eq!(ChainConfig::westend().ss58_prefix, 42);
    assert_eq!(ChainConfig::paseo().ss58_prefix, 42);
}

#[test]
fn test_chain_config_token_decimals() {
    assert_eq!(ChainConfig::polkadot().token_decimals, 10);
    assert_eq!(ChainConfig::kusama().token_decimals, 12);
    assert_eq!(ChainConfig::westend().token_decimals, 12);
    assert_eq!(ChainConfig::paseo().token_decimals, 10);
}

#[test]
fn test_chain_config_token_symbols() {
    assert_eq!(ChainConfig::polkadot().token_symbol, "DOT");
    assert_eq!(ChainConfig::kusama().token_symbol, "KSM");
    assert_eq!(ChainConfig::westend().token_symbol, "WND");
    assert_eq!(ChainConfig::paseo().token_symbol, "PAS");
}

#[test]
fn test_chain_config_endpoints() {
    let polkadot = ChainConfig::polkadot();
    let kusama = ChainConfig::kusama();
    let westend = ChainConfig::westend();

    assert!(polkadot.endpoint.starts_with("wss://"));
    assert!(kusama.endpoint.starts_with("wss://"));
    assert!(westend.endpoint.starts_with("wss://"));
}

#[test]
fn test_custom_chain_configs() {
    let custom1 = ChainConfig::custom("Chain1", "wss://endpoint1.com", 10);
    let custom2 = ChainConfig::custom("Chain2", "wss://endpoint2.com", 20);

    assert_eq!(custom1.name, "Chain1");
    assert_eq!(custom2.name, "Chain2");
    assert_eq!(custom1.ss58_prefix, 10);
    assert_eq!(custom2.ss58_prefix, 20);
}

#[test]
fn test_error_variants_creation() {
    let errors = [
        Error::Connection("test".to_string()),
        Error::Transaction("test".to_string()),
        Error::Metadata("test".to_string()),
        Error::Storage("test".to_string()),
        Error::Wallet("test".to_string()),
        Error::Signature("test".to_string()),
        Error::Encoding("test".to_string()),
        Error::Other("test".to_string()),
    ];

    assert_eq!(errors.len(), 8);
}

#[test]
fn test_error_display_formatting() {
    let error = Error::Connection("Connection timeout".to_string());
    let display = format!("{}", error);

    assert!(display.contains("Connection error"));
    assert!(display.contains("Connection timeout"));
}

#[test]
fn test_chain_config_various_ss58_prefixes() {
    let configs = [
        ChainConfig::custom("Test1", "wss://test1.com", 0),
        ChainConfig::custom("Test2", "wss://test2.com", 42),
        ChainConfig::custom("Test3", "wss://test3.com", 100),
        ChainConfig::custom("Test4", "wss://test4.com", 255),
    ];

    assert_eq!(configs[0].ss58_prefix, 0);
    assert_eq!(configs[1].ss58_prefix, 42);
    assert_eq!(configs[2].ss58_prefix, 100);
    assert_eq!(configs[3].ss58_prefix, 255);
}

#[test]
fn test_chain_config_endpoint_formats() {
    let wss = ChainConfig::custom("WSS", "wss://example.com", 0);
    let ws = ChainConfig::custom("WS", "ws://localhost:9944", 0);
    let custom = ChainConfig::custom("Custom", "custom://endpoint", 0);

    assert!(wss.endpoint.starts_with("wss://"));
    assert!(ws.endpoint.starts_with("ws://"));
    assert!(custom.endpoint.starts_with("custom://"));
}

#[test]
fn test_result_type_ok() {
    let result: Result<u32> = Ok(42);
    assert!(result.is_ok());
    if let Ok(value) = result {
        assert_eq!(value, 42);
    }
}

#[test]
fn test_result_type_err() {
    let result: Result<u32> = Err(Error::Other("error".to_string()));
    assert!(result.is_err());
}

#[test]
fn test_multiple_error_messages() {
    let errors = [
        Error::Connection("Connection refused".to_string()),
        Error::Connection("Timeout".to_string()),
        Error::Connection("Network unreachable".to_string()),
    ];

    assert!(errors[0].to_string().contains("Connection refused"));
    assert!(errors[1].to_string().contains("Timeout"));
    assert!(errors[2].to_string().contains("Network unreachable"));
}

#[test]
fn test_chain_config_name_variations() {
    let configs = [
        ChainConfig::custom("Polkadot", "wss://test.com", 0),
        ChainConfig::custom("polkadot", "wss://test.com", 0),
        ChainConfig::custom("POLKADOT", "wss://test.com", 0),
    ];

    assert_eq!(configs[0].name, "Polkadot");
    assert_eq!(configs[1].name, "polkadot");
    assert_eq!(configs[2].name, "POLKADOT");
}

#[test]
fn test_error_empty_messages() {
    let error = Error::Other("".to_string());
    assert_eq!(error.to_string(), "Other error: ");
}

#[test]
fn test_error_long_messages() {
    let long_message = "A".repeat(1000);
    let error = Error::Other(long_message.clone());
    assert!(error.to_string().contains(&long_message));
}

#[test]
fn test_chain_config_empty_name() {
    let config = ChainConfig::custom("", "wss://test.com", 0);
    assert_eq!(config.name, "");
}

#[test]
fn test_chain_config_long_name() {
    let long_name = "MyVeryLongChainName".repeat(10);
    let config = ChainConfig::custom(&long_name, "wss://test.com", 0);
    assert_eq!(config.name, long_name);
}

#[test]
fn test_chain_config_special_characters() {
    let config = ChainConfig::custom("My-Chain_123", "wss://test.com", 0);
    assert_eq!(config.name, "My-Chain_123");
}

#[test]
fn test_all_error_types() {
    let connection_err = Error::Connection("test".to_string());
    let transaction_err = Error::Transaction("test".to_string());
    let metadata_err = Error::Metadata("test".to_string());
    let storage_err = Error::Storage("test".to_string());
    let wallet_err = Error::Wallet("test".to_string());
    let signature_err = Error::Signature("test".to_string());
    let encoding_err = Error::Encoding("test".to_string());
    let other_err = Error::Other("test".to_string());

    // Verify all error types can be created and displayed
    assert!(connection_err.to_string().contains("Connection error"));
    assert!(transaction_err.to_string().contains("Transaction error"));
    assert!(metadata_err.to_string().contains("Metadata error"));
    assert!(storage_err.to_string().contains("Storage error"));
    assert!(wallet_err.to_string().contains("Wallet error"));
    assert!(signature_err.to_string().contains("Signature error"));
    assert!(encoding_err.to_string().contains("Encoding error"));
    assert!(other_err.to_string().contains("Other error"));
}

#[test]
fn test_known_substrate_addresses() {
    // Well-known Substrate addresses
    let addresses = vec![
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", // Ilara
        "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty", // Bob
        "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y", // Charlie
    ];

    for addr in addresses {
        let address = Address::substrate(addr);
        match address {
            Address::Substrate(s) => assert_eq!(s, addr),
            _ => panic!("Expected Substrate address"),
        }
    }
}

#[test]
fn test_chain_config_realistic_endpoints() {
    let configs = [
        ChainConfig::custom("Polkadot", "wss://rpc.polkadot.io", 0),
        ChainConfig::custom("Kusama", "wss://kusama-rpc.polkadot.io", 2),
        ChainConfig::custom("Local", "ws://127.0.0.1:9944", 42),
    ];

    assert_eq!(configs[0].endpoint, "wss://rpc.polkadot.io");
    assert_eq!(configs[1].endpoint, "wss://kusama-rpc.polkadot.io");
    assert_eq!(configs[2].endpoint, "ws://127.0.0.1:9944");
}

#[test]
fn test_chain_config_unicode_names() {
    let config = ChainConfig::custom("链", "wss://test.com", 0);
    assert_eq!(config.name, "链");
}

#[test]
fn test_error_nested_messages() {
    let inner_error = "Inner error message";
    let error = Error::Connection(format!("Outer error: {}", inner_error));
    assert!(error.to_string().contains("Inner error message"));
}

#[test]
fn test_chain_decimals_range() {
    // Test various decimal values
    let decimals = vec![0u8, 6, 10, 12, 18];

    for d in decimals {
        let mut config = ChainConfig::polkadot();
        config.token_decimals = d;
        assert_eq!(config.token_decimals, d);
    }
}
