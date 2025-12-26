use apex_sdk_types::{Address, Chain, ChainType, ValidationError};

// =============================================================================
// Additional Coverage Tests for Missing Lines
// =============================================================================

#[test]
fn test_chain_acala_methods() {
    assert_eq!(Chain::Acala.name(), "Acala");
    assert_eq!(Chain::Acala.chain_type(), ChainType::Substrate);
    assert!(Chain::Acala.supports_smart_contracts());
    assert!(!Chain::Acala.is_testnet());
    assert!(!Chain::Acala.is_layer2());
    assert_eq!(Chain::Acala.chain_id(), None);
    assert!(!Chain::Acala.default_endpoint().is_empty());
}

#[test]
fn test_chain_phala_methods() {
    assert_eq!(Chain::Phala.name(), "Phala");
    assert_eq!(Chain::Phala.chain_type(), ChainType::Substrate);
    assert!(Chain::Phala.supports_smart_contracts());
    assert!(!Chain::Phala.is_testnet());
    assert!(!Chain::Phala.is_layer2());
}

#[test]
fn test_chain_bifrost_methods() {
    assert_eq!(Chain::Bifrost.name(), "Bifrost");
    assert_eq!(Chain::Bifrost.chain_type(), ChainType::Substrate);
    assert!(!Chain::Bifrost.supports_smart_contracts());
    assert!(!Chain::Bifrost.is_testnet());
}

#[test]
fn test_chain_bsc_methods() {
    assert_eq!(Chain::BinanceSmartChain.name(), "Binance Smart Chain");
    assert_eq!(Chain::BinanceSmartChain.chain_type(), ChainType::Evm);
    assert_eq!(Chain::BinanceSmartChain.chain_id(), Some(56));
    assert!(Chain::BinanceSmartChain.supports_smart_contracts());
}

#[test]
fn test_chain_polygon_methods() {
    assert_eq!(Chain::Polygon.name(), "Polygon");
    assert_eq!(Chain::Polygon.chain_type(), ChainType::Evm);
    assert_eq!(Chain::Polygon.chain_id(), Some(137));
    assert!(!Chain::Polygon.is_layer2());
}

#[test]
fn test_chain_avalanche_methods() {
    assert_eq!(Chain::Avalanche.name(), "Avalanche");
    assert_eq!(Chain::Avalanche.chain_type(), ChainType::Evm);
    assert_eq!(Chain::Avalanche.chain_id(), Some(43114));
}

#[test]
fn test_chain_zksync_methods() {
    assert_eq!(Chain::ZkSync.name(), "zkSync");
    assert_eq!(Chain::ZkSync.chain_type(), ChainType::Evm);
    assert_eq!(Chain::ZkSync.chain_id(), Some(324));
    assert!(Chain::ZkSync.is_layer2());
}

#[test]
fn test_chain_base_methods() {
    assert_eq!(Chain::Base.name(), "Base");
    assert_eq!(Chain::Base.chain_type(), ChainType::Evm);
    assert_eq!(Chain::Base.chain_id(), Some(8453));
    assert!(Chain::Base.is_layer2());
}

#[test]
fn test_chain_kusama_methods() {
    assert_eq!(Chain::Kusama.name(), "Kusama");
    assert_eq!(Chain::Kusama.chain_type(), ChainType::Substrate);
    assert!(!Chain::Kusama.supports_smart_contracts());
    assert_eq!(Chain::Kusama.chain_id(), None);
}

#[test]
fn test_chain_paseo_methods() {
    assert_eq!(Chain::Paseo.name(), "Paseo");
    assert_eq!(Chain::Paseo.chain_type(), ChainType::Substrate);
    assert!(Chain::Paseo.is_testnet());
}

#[test]
fn test_chain_arbitrum_methods() {
    assert_eq!(Chain::Arbitrum.name(), "Arbitrum");
    assert_eq!(Chain::Arbitrum.chain_type(), ChainType::Evm);
    assert_eq!(Chain::Arbitrum.chain_id(), Some(42161));
    assert!(Chain::Arbitrum.is_layer2());
}

#[test]
fn test_chain_optimism_methods() {
    assert_eq!(Chain::Optimism.name(), "Optimism");
    assert_eq!(Chain::Optimism.chain_type(), ChainType::Evm);
    assert_eq!(Chain::Optimism.chain_id(), Some(10));
    assert!(Chain::Optimism.is_layer2());
}

#[test]
fn test_chain_astar_methods() {
    assert_eq!(Chain::Astar.name(), "Astar");
    assert_eq!(Chain::Astar.chain_type(), ChainType::Hybrid);
    assert_eq!(Chain::Astar.chain_id(), Some(592));
    assert!(Chain::Astar.supports_smart_contracts());
}

#[test]
fn test_chain_from_str_all_aliases() {
    // Test all EVM aliases
    assert_eq!(
        Chain::from_str_case_insensitive("ethereum"),
        Some(Chain::Ethereum)
    );
    assert_eq!(
        Chain::from_str_case_insensitive("binance"),
        Some(Chain::BinanceSmartChain)
    );
    assert_eq!(
        Chain::from_str_case_insensitive("binancesmartchain"),
        Some(Chain::BinanceSmartChain)
    );
    assert_eq!(
        Chain::from_str_case_insensitive("polygon"),
        Some(Chain::Polygon)
    );
    assert_eq!(
        Chain::from_str_case_insensitive("avalanche"),
        Some(Chain::Avalanche)
    );
    assert_eq!(
        Chain::from_str_case_insensitive("arbitrum"),
        Some(Chain::Arbitrum)
    );
    assert_eq!(
        Chain::from_str_case_insensitive("optimism"),
        Some(Chain::Optimism)
    );
    assert_eq!(
        Chain::from_str_case_insensitive("zksync"),
        Some(Chain::ZkSync)
    );

    // Test Substrate chains
    assert_eq!(
        Chain::from_str_case_insensitive("acala"),
        Some(Chain::Acala)
    );
    assert_eq!(
        Chain::from_str_case_insensitive("phala"),
        Some(Chain::Phala)
    );
    assert_eq!(
        Chain::from_str_case_insensitive("bifrost"),
        Some(Chain::Bifrost)
    );
    assert_eq!(
        Chain::from_str_case_insensitive("westend"),
        Some(Chain::Westend)
    );
    assert_eq!(
        Chain::from_str_case_insensitive("paseo"),
        Some(Chain::Paseo)
    );
}

#[test]
fn test_all_chain_endpoints() {
    // Test all Substrate chain endpoints
    assert_eq!(
        Chain::Acala.default_endpoint(),
        "wss://acala.api.onfinality.io/public-ws"
    );
    assert_eq!(
        Chain::Phala.default_endpoint(),
        "wss://phala.api.onfinality.io/public-ws"
    );
    assert_eq!(
        Chain::Bifrost.default_endpoint(),
        "wss://bifrost-polkadot.api.onfinality.io/public-ws"
    );
    assert_eq!(
        Chain::Westend.default_endpoint(),
        "wss://westend-rpc.polkadot.io"
    );
    assert_eq!(
        Chain::Paseo.default_endpoint(),
        "wss://paseo.rpc.amforc.com"
    );

    // Test EVM L1 endpoints
    assert_eq!(
        Chain::Ethereum.default_endpoint(),
        "https://eth.llamarpc.com"
    );
    assert_eq!(
        Chain::BinanceSmartChain.default_endpoint(),
        "https://bsc.publicnode.com"
    );
    assert_eq!(Chain::Polygon.default_endpoint(), "https://polygon-rpc.com");
    assert_eq!(
        Chain::Avalanche.default_endpoint(),
        "https://api.avax.network/ext/bc/C/rpc"
    );

    // Test EVM L2 endpoints
    assert_eq!(
        Chain::Arbitrum.default_endpoint(),
        "https://arb1.arbitrum.io/rpc"
    );
    assert_eq!(
        Chain::Optimism.default_endpoint(),
        "https://mainnet.optimism.io"
    );
    assert_eq!(
        Chain::ZkSync.default_endpoint(),
        "https://mainnet.era.zksync.io"
    );
    assert_eq!(Chain::Base.default_endpoint(), "https://mainnet.base.org");

    // Test Hybrid endpoints
    assert_eq!(
        Chain::Astar.default_endpoint(),
        "wss://astar.api.onfinality.io/public-ws"
    );
}

#[test]
fn test_substrate_address_for_all_chains() {
    // Test Westend
    let result = Address::substrate_for_chain(
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        &Chain::Westend,
    );
    // May succeed or fail depending on address format, just test it doesn't panic
    let _ = result;

    // Test Paseo
    let result = Address::substrate_for_chain(
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        &Chain::Paseo,
    );
    let _ = result;

    // Test Moonbeam
    let result = Address::substrate_for_chain(
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        &Chain::Moonbeam,
    );
    let _ = result;

    // Test Astar
    let result = Address::substrate_for_chain(
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        &Chain::Astar,
    );
    let _ = result;

    // Test Acala
    let result = Address::substrate_for_chain(
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        &Chain::Acala,
    );
    let _ = result;

    // Test Phala
    let result = Address::substrate_for_chain(
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        &Chain::Phala,
    );
    let _ = result;

    // Test Bifrost
    let result = Address::substrate_for_chain(
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        &Chain::Bifrost,
    );
    let _ = result;
}

#[test]
fn test_validation_error_display_all_variants() {
    let err1 = ValidationError::InvalidEvmAddress("test_addr".to_string());
    assert!(err1.to_string().contains("Invalid EVM address format"));
    assert!(err1.to_string().contains("test_addr"));

    let err2 = ValidationError::InvalidChecksum("test_checksum".to_string());
    assert!(err2
        .to_string()
        .contains("EIP-55 checksum validation failed"));
    assert!(err2.to_string().contains("test_checksum"));

    let err3 = ValidationError::InvalidChainId {
        chain: "TestChain".to_string(),
        expected: 1,
        actual: 2,
    };
    assert!(err3.to_string().contains("Invalid chain ID"));
    assert!(err3.to_string().contains("TestChain"));
    assert!(err3.to_string().contains("expected 1"));
    assert!(err3.to_string().contains("got 2"));

    let err4 = ValidationError::ChainIdNotFound("TestChain".to_string());
    assert!(err4.to_string().contains("Chain ID not available"));
    assert!(err4.to_string().contains("TestChain"));

    let err5 = ValidationError::InvalidSubstrateAddress("test_substrate".to_string());
    assert!(err5
        .to_string()
        .contains("Invalid Substrate SS58 address format"));
    assert!(err5.to_string().contains("test_substrate"));

    let err6 = ValidationError::InvalidSs58Checksum("test_ss58".to_string());
    assert!(err6.to_string().contains("SS58 checksum validation failed"));
    assert!(err6.to_string().contains("test_ss58"));
}

#[test]
fn test_validation_error_eq() {
    let err1 = ValidationError::InvalidEvmAddress("addr1".to_string());
    let err2 = ValidationError::InvalidEvmAddress("addr1".to_string());
    let err3 = ValidationError::InvalidEvmAddress("addr2".to_string());

    assert_eq!(err1, err2);
    assert_ne!(err1, err3);
}

#[test]
fn test_chain_type_hash() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(ChainType::Substrate);
    set.insert(ChainType::Evm);
    set.insert(ChainType::Hybrid);

    assert_eq!(set.len(), 3);
    assert!(set.contains(&ChainType::Substrate));
    assert!(set.contains(&ChainType::Evm));
    assert!(set.contains(&ChainType::Hybrid));
}

#[test]
fn test_chain_hash() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(Chain::Ethereum);
    set.insert(Chain::Polkadot);
    set.insert(Chain::Moonbeam);

    assert_eq!(set.len(), 3);
    assert!(set.contains(&Chain::Ethereum));
}

#[test]
fn test_address_debug() {
    let addr = Address::evm("0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed");
    let debug_str = format!("{:?}", addr);
    assert!(debug_str.contains("Evm"));
}

#[test]
fn test_multiple_rpc_endpoints_details() {
    let polkadot_endpoints = Chain::Polkadot.rpc_endpoints();
    assert!(polkadot_endpoints.contains(&"wss://polkadot.api.onfinality.io/public-ws"));
    assert!(polkadot_endpoints.contains(&"wss://rpc.ibp.network/polkadot"));
    assert!(polkadot_endpoints.contains(&"wss://polkadot.dotters.network"));

    let kusama_endpoints = Chain::Kusama.rpc_endpoints();
    assert!(kusama_endpoints.contains(&"wss://kusama.api.onfinality.io/public-ws"));
    assert!(kusama_endpoints.contains(&"wss://rpc.ibp.network/kusama"));
    assert!(kusama_endpoints.contains(&"wss://kusama.dotters.network"));

    let westend_endpoints = Chain::Westend.rpc_endpoints();
    assert!(westend_endpoints.contains(&"wss://westend-rpc.polkadot.io"));
    assert!(westend_endpoints.contains(&"wss://rpc.ibp.network/westend"));
    assert!(westend_endpoints.contains(&"wss://westend.dotters.network"));
}

#[test]
fn test_address_eq_trait() {
    let addr1 = Address::substrate("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY");
    let addr2 = Address::substrate("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY");
    let addr3 = Address::substrate("5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty");

    assert_eq!(addr1, addr2);
    assert_ne!(addr1, addr3);

    let evm1 = Address::evm("0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed");
    let evm2 = Address::evm("0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed");
    assert_eq!(evm1, evm2);

    // Different types should not be equal
    assert_ne!(addr1, evm1);
}

#[test]
fn test_move_address_edge_cases() {
    // Test all zeros
    let zeros = "0x0000000000000000000000000000000000000000000000000000000000000000";
    let addr = Address::move_addr(zeros);
    assert!(addr.validate().is_ok());

    // Test all Fs
    let all_f = "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF";
    let addr = Address::move_addr(all_f);
    assert!(addr.validate().is_ok());

    // Test lowercase
    let lowercase = "0xabcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789";
    let addr = Address::move_addr(lowercase);
    assert!(addr.validate().is_ok());

    // Test uppercase
    let uppercase = "0xABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789";
    let addr = Address::move_addr(uppercase);
    assert!(addr.validate().is_ok());
}

#[test]
fn test_evm_address_with_numbers() {
    // Address with all numbers in hex part
    let addr = Address::evm("0x1234567890123456789012345678901234567890");
    assert!(addr.validate().is_ok());
}

#[test]
fn test_chain_validate_all_evm_chains() {
    assert!(Chain::Ethereum.validate_chain_id(1).is_ok());
    assert!(Chain::BinanceSmartChain.validate_chain_id(56).is_ok());
    assert!(Chain::Polygon.validate_chain_id(137).is_ok());
    assert!(Chain::Avalanche.validate_chain_id(43114).is_ok());
    assert!(Chain::Arbitrum.validate_chain_id(42161).is_ok());
    assert!(Chain::Optimism.validate_chain_id(10).is_ok());
    assert!(Chain::ZkSync.validate_chain_id(324).is_ok());
    assert!(Chain::Base.validate_chain_id(8453).is_ok());
    assert!(Chain::Moonbeam.validate_chain_id(1284).is_ok());
    assert!(Chain::Astar.validate_chain_id(592).is_ok());
}

#[test]
fn test_chain_validate_wrong_chain_ids() {
    assert!(Chain::Ethereum.validate_chain_id(56).is_err());
    assert!(Chain::BinanceSmartChain.validate_chain_id(1).is_err());
    assert!(Chain::Polygon.validate_chain_id(1).is_err());
    assert!(Chain::Avalanche.validate_chain_id(1).is_err());
    assert!(Chain::Arbitrum.validate_chain_id(1).is_err());
    assert!(Chain::Optimism.validate_chain_id(1).is_err());
    assert!(Chain::ZkSync.validate_chain_id(1).is_err());
    assert!(Chain::Base.validate_chain_id(1).is_err());
}

#[test]
fn test_all_substrate_chains_no_chain_id() {
    assert!(Chain::Polkadot.validate_chain_id(0).is_err());
    assert!(Chain::Kusama.validate_chain_id(0).is_err());
    assert!(Chain::Acala.validate_chain_id(0).is_err());
    assert!(Chain::Phala.validate_chain_id(0).is_err());
    assert!(Chain::Bifrost.validate_chain_id(0).is_err());
    assert!(Chain::Westend.validate_chain_id(0).is_err());
    assert!(Chain::Paseo.validate_chain_id(0).is_err());
}
