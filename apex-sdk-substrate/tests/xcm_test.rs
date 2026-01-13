//! Comprehensive tests for XCM module
//!
//! These tests verify XCM functionality including:
//! - MultiLocation construction
//! - Asset representation
//! - Weight limits
//! - Junction types
//! - Network IDs

use apex_sdk_substrate::xcm::*;

#[test]
fn test_xcm_version_default() {
    let version = XcmVersion::default();
    assert_eq!(version, XcmVersion::V3);
}

#[test]
fn test_xcm_version_variants() {
    let v2 = XcmVersion::V2;
    let v3 = XcmVersion::V3;
    let v4 = XcmVersion::V4;

    assert_ne!(v2, v3);
    assert_ne!(v3, v4);
    assert_ne!(v2, v4);
}

#[test]
fn test_xcm_version_equality() {
    assert_eq!(XcmVersion::V2, XcmVersion::V2);
    assert_eq!(XcmVersion::V3, XcmVersion::V3);
    assert_eq!(XcmVersion::V4, XcmVersion::V4);
}

#[test]
fn test_xcm_transfer_type_variants() {
    let reserve = XcmTransferType::ReserveTransfer;
    let teleport = XcmTransferType::Teleport;
    let limited_reserve = XcmTransferType::LimitedReserveTransfer;
    let limited_teleport = XcmTransferType::LimitedTeleport;

    assert_eq!(reserve, XcmTransferType::ReserveTransfer);
    assert_eq!(teleport, XcmTransferType::Teleport);
    assert_eq!(limited_reserve, XcmTransferType::LimitedReserveTransfer);
    assert_eq!(limited_teleport, XcmTransferType::LimitedTeleport);
}

#[test]
fn test_multilocation_parent() {
    let location = MultiLocation::parent();

    assert_eq!(location.parents, 1);
    assert!(location.interior.is_empty());
    assert!(location.is_parent());
    assert!(!location.is_parachain());
}

#[test]
fn test_multilocation_parachain() {
    let location = MultiLocation::parachain(2000);

    assert_eq!(location.parents, 1);
    assert_eq!(location.interior.len(), 1);
    assert!(location.is_parachain());
    assert!(!location.is_parent());
    assert_eq!(location.parachain_id(), Some(2000));
}

#[test]
fn test_multilocation_account() {
    let account_id = [42u8; 32];
    let location = MultiLocation::account(account_id);

    assert_eq!(location.parents, 0);
    assert_eq!(location.interior.len(), 1);
    assert!(!location.is_parent());
    assert!(!location.is_parachain());

    match &location.interior[0] {
        Junction::AccountId32 { network: _, id } => {
            assert_eq!(*id, account_id);
        }
        _ => panic!("Expected AccountId32 junction"),
    }
}

#[test]
fn test_multilocation_parachain_account() {
    let para_id = 1000;
    let account_id = [99u8; 32];
    let location = MultiLocation::parachain_account(para_id, account_id);

    assert_eq!(location.parents, 1);
    assert_eq!(location.interior.len(), 2);
    assert_eq!(location.parachain_id(), Some(para_id));
}

#[test]
fn test_multilocation_new() {
    let location = MultiLocation::new(
        2,
        vec![
            Junction::Parachain(1000),
            Junction::AccountId32 {
                network: None,
                id: [1u8; 32],
            },
        ],
    );

    assert_eq!(location.parents, 2);
    assert_eq!(location.interior.len(), 2);
}

#[test]
fn test_multilocation_is_parent() {
    let parent = MultiLocation::parent();
    let not_parent = MultiLocation::parachain(1000);

    assert!(parent.is_parent());
    assert!(!not_parent.is_parent());
}

#[test]
fn test_multilocation_is_parachain() {
    let parachain = MultiLocation::parachain(2000);
    let not_parachain = MultiLocation::parent();

    assert!(parachain.is_parachain());
    assert!(!not_parachain.is_parachain());
}

#[test]
fn test_multilocation_parachain_id() {
    let location1 = MultiLocation::parachain(1000);
    let location2 = MultiLocation::parachain(2000);
    let location3 = MultiLocation::parent();

    assert_eq!(location1.parachain_id(), Some(1000));
    assert_eq!(location2.parachain_id(), Some(2000));
    assert_eq!(location3.parachain_id(), None);
}

#[test]
fn test_junction_parachain() {
    let junction = Junction::Parachain(1000);

    match junction {
        Junction::Parachain(id) => assert_eq!(id, 1000),
        _ => panic!("Expected Parachain junction"),
    }
}

#[test]
fn test_junction_account_id32() {
    let account = [55u8; 32];
    let junction = Junction::AccountId32 {
        network: None,
        id: account,
    };

    match junction {
        Junction::AccountId32 { network, id } => {
            assert!(network.is_none());
            assert_eq!(id, account);
        }
        _ => panic!("Expected AccountId32 junction"),
    }
}

#[test]
fn test_junction_account_id32_with_network() {
    let account = [77u8; 32];
    let junction = Junction::AccountId32 {
        network: Some(NetworkId::Polkadot),
        id: account,
    };

    match junction {
        Junction::AccountId32 { network, id } => {
            assert_eq!(network, Some(NetworkId::Polkadot));
            assert_eq!(id, account);
        }
        _ => panic!("Expected AccountId32 junction"),
    }
}

#[test]
fn test_junction_account_id20() {
    let key = [88u8; 20];
    let junction = Junction::AccountId20 { network: None, key };

    match junction {
        Junction::AccountId20 { network, key: k } => {
            assert!(network.is_none());
            assert_eq!(k, key);
        }
        _ => panic!("Expected AccountId20 junction"),
    }
}

#[test]
fn test_junction_general_index() {
    let junction = Junction::GeneralIndex(12345);

    match junction {
        Junction::GeneralIndex(index) => assert_eq!(index, 12345),
        _ => panic!("Expected GeneralIndex junction"),
    }
}

#[test]
fn test_junction_general_key() {
    let data = vec![1, 2, 3, 4, 5];
    let junction = Junction::GeneralKey { data: data.clone() };

    match junction {
        Junction::GeneralKey { data: d } => assert_eq!(d, data),
        _ => panic!("Expected GeneralKey junction"),
    }
}

#[test]
fn test_junction_pallet_instance() {
    let junction = Junction::PalletInstance(50);

    match junction {
        Junction::PalletInstance(instance) => assert_eq!(instance, 50),
        _ => panic!("Expected PalletInstance junction"),
    }
}

#[test]
fn test_network_id_variants() {
    let polkadot = NetworkId::Polkadot;
    let kusama = NetworkId::Kusama;
    let westend = NetworkId::Westend;
    let rococo = NetworkId::Rococo;

    assert_eq!(polkadot, NetworkId::Polkadot);
    assert_eq!(kusama, NetworkId::Kusama);
    assert_eq!(westend, NetworkId::Westend);
    assert_eq!(rococo, NetworkId::Rococo);
}

#[test]
fn test_network_id_by_genesis() {
    let genesis = [0u8; 32];
    let network = NetworkId::ByGenesis(genesis);

    match network {
        NetworkId::ByGenesis(g) => assert_eq!(g, genesis),
        _ => panic!("Expected ByGenesis variant"),
    }
}

#[test]
fn test_asset_id_concrete() {
    let location = MultiLocation::parent();
    let asset_id = AssetId::Concrete(location.clone());

    match asset_id {
        AssetId::Concrete(loc) => assert_eq!(loc, location),
        _ => panic!("Expected Concrete asset ID"),
    }
}

#[test]
fn test_asset_id_abstract() {
    let data = vec![1, 2, 3, 4];
    let asset_id = AssetId::Abstract(data.clone());

    match asset_id {
        AssetId::Abstract(d) => assert_eq!(d, data),
        _ => panic!("Expected Abstract asset ID"),
    }
}

#[test]
fn test_fungibility_fungible() {
    let fungibility = Fungibility::Fungible(1_000_000);

    match fungibility {
        Fungibility::Fungible(amount) => assert_eq!(amount, 1_000_000),
        _ => panic!("Expected Fungible variant"),
    }
}

#[test]
fn test_fungibility_non_fungible() {
    let fungibility = Fungibility::NonFungible(42);

    match fungibility {
        Fungibility::NonFungible(instance) => assert_eq!(instance, 42),
        _ => panic!("Expected NonFungible variant"),
    }
}

#[test]
fn test_xcm_asset_fungible() {
    let location = MultiLocation::parent();
    let asset = XcmAsset::fungible(AssetId::Concrete(location), 5_000_000);

    match asset.fun {
        Fungibility::Fungible(amount) => assert_eq!(amount, 5_000_000),
        _ => panic!("Expected Fungible"),
    }
}

#[test]
fn test_xcm_asset_non_fungible() {
    let location = MultiLocation::parent();
    let asset = XcmAsset::non_fungible(AssetId::Concrete(location), 123);

    match asset.fun {
        Fungibility::NonFungible(instance) => assert_eq!(instance, 123),
        _ => panic!("Expected NonFungible"),
    }
}

#[test]
fn test_xcm_asset_native() {
    let amount = 10_000_000_000_000u128;
    let asset = XcmAsset::native(amount);

    match asset.fun {
        Fungibility::Fungible(a) => assert_eq!(a, amount),
        _ => panic!("Expected Fungible"),
    }

    match asset.id {
        AssetId::Concrete(location) => {
            assert_eq!(location.parents, 0);
            assert!(location.interior.is_empty());
        }
        _ => panic!("Expected Concrete asset"),
    }
}

#[test]
fn test_weight_limit_unlimited() {
    let limit = WeightLimit::Unlimited;

    match limit {
        WeightLimit::Unlimited => {} // Expected
        _ => panic!("Expected Unlimited variant"),
    }
}

#[test]
fn test_weight_limit_limited() {
    let limit = WeightLimit::Limited(1_000_000_000);

    match limit {
        WeightLimit::Limited(weight) => assert_eq!(weight, 1_000_000_000),
        _ => panic!("Expected Limited variant"),
    }
}

#[test]
fn test_weight_limit_default() {
    let limit = WeightLimit::default();

    match limit {
        WeightLimit::Limited(weight) => assert_eq!(weight, 5_000_000_000),
        _ => panic!("Expected Limited variant with default weight"),
    }
}

#[test]
fn test_xcm_config_default() {
    let config = XcmConfig::default();

    assert_eq!(config.version, XcmVersion::V3);
    assert!(config.fee_asset.is_none());

    match config.weight_limit {
        WeightLimit::Limited(_) => {} // Expected
        _ => panic!("Expected Limited weight limit"),
    }
}

#[test]
fn test_xcm_config_structure() {
    let config = XcmConfig {
        version: XcmVersion::V4,
        weight_limit: WeightLimit::Limited(10_000_000_000),
        fee_asset: Some(XcmAsset::native(1_000_000)),
    };

    assert_eq!(config.version, XcmVersion::V4);
    assert!(config.fee_asset.is_some());
}

#[test]
fn test_multilocation_clone() {
    let location = MultiLocation::parachain(1000);
    let cloned = location.clone();

    assert_eq!(cloned.parents, location.parents);
    assert_eq!(cloned.interior.len(), location.interior.len());
}

#[test]
fn test_junction_clone() {
    let junction = Junction::Parachain(2000);
    let cloned = junction.clone();

    match (junction, cloned) {
        (Junction::Parachain(id1), Junction::Parachain(id2)) => {
            assert_eq!(id1, id2);
        }
        _ => panic!("Clone failed"),
    }
}

#[test]
fn test_xcm_asset_clone() {
    let asset = XcmAsset::native(1_000_000);
    let cloned = asset.clone();

    match (asset.fun, cloned.fun) {
        (Fungibility::Fungible(a1), Fungibility::Fungible(a2)) => {
            assert_eq!(a1, a2);
        }
        _ => panic!("Clone failed"),
    }
}

#[test]
fn test_multilocation_debug() {
    let location = MultiLocation::parent();
    let debug_output = format!("{:?}", location);

    assert!(debug_output.contains("MultiLocation"));
}

#[test]
fn test_junction_debug() {
    let junction = Junction::Parachain(1000);
    let debug_output = format!("{:?}", junction);

    assert!(debug_output.contains("Parachain"));
}

#[test]
fn test_network_id_debug() {
    let network = NetworkId::Polkadot;
    let debug_output = format!("{:?}", network);

    assert!(debug_output.contains("Polkadot"));
}

#[test]
fn test_xcm_version_debug() {
    let version = XcmVersion::V3;
    let debug_output = format!("{:?}", version);

    assert!(debug_output.contains("V3"));
}

#[test]
fn test_multilocation_various_parents() {
    let loc0 = MultiLocation::new(0, vec![]);
    let loc1 = MultiLocation::new(1, vec![]);
    let loc2 = MultiLocation::new(2, vec![]);

    assert_eq!(loc0.parents, 0);
    assert_eq!(loc1.parents, 1);
    assert_eq!(loc2.parents, 2);
}

#[test]
fn test_multilocation_complex() {
    let location = MultiLocation::new(
        1,
        vec![
            Junction::Parachain(2000),
            Junction::PalletInstance(50),
            Junction::GeneralIndex(100),
        ],
    );

    assert_eq!(location.parents, 1);
    assert_eq!(location.interior.len(), 3);
}

#[test]
fn test_xcm_asset_various_amounts() {
    let assets = [
        XcmAsset::native(1_000_000_000_000),
        XcmAsset::native(500_000_000_000),
        XcmAsset::native(100_000_000),
    ];

    match assets[0].fun {
        Fungibility::Fungible(amount) => assert_eq!(amount, 1_000_000_000_000),
        _ => panic!("Expected Fungible"),
    }

    match assets[1].fun {
        Fungibility::Fungible(amount) => assert_eq!(amount, 500_000_000_000),
        _ => panic!("Expected Fungible"),
    }

    match assets[2].fun {
        Fungibility::Fungible(amount) => assert_eq!(amount, 100_000_000),
        _ => panic!("Expected Fungible"),
    }
}

#[test]
fn test_weight_limit_various_values() {
    let limits = [
        WeightLimit::Limited(1_000_000_000),
        WeightLimit::Limited(5_000_000_000),
        WeightLimit::Limited(10_000_000_000),
        WeightLimit::Unlimited,
    ];

    assert_eq!(limits.len(), 4);
}

#[test]
fn test_parachain_ids() {
    let parachains = [
        MultiLocation::parachain(1000),
        MultiLocation::parachain(2000),
        MultiLocation::parachain(2094), // Interlay
        MultiLocation::parachain(2006), // Astar
    ];

    assert_eq!(parachains[0].parachain_id(), Some(1000));
    assert_eq!(parachains[1].parachain_id(), Some(2000));
    assert_eq!(parachains[2].parachain_id(), Some(2094));
    assert_eq!(parachains[3].parachain_id(), Some(2006));
}

#[test]
fn test_all_network_ids() {
    let networks = [
        NetworkId::Polkadot,
        NetworkId::Kusama,
        NetworkId::Westend,
        NetworkId::Rococo,
    ];

    assert_eq!(networks.len(), 4);
}

#[test]
fn test_xcm_config_clone() {
    let config = XcmConfig::default();
    let cloned = config.clone();

    assert_eq!(cloned.version, config.version);
}

#[test]
fn test_weight_limit_copy() {
    let limit1 = WeightLimit::Limited(1_000_000);
    let limit2 = limit1;

    match (limit1, limit2) {
        (WeightLimit::Limited(w1), WeightLimit::Limited(w2)) => {
            assert_eq!(w1, w2);
        }
        _ => panic!("Copy failed"),
    }
}

#[test]
fn test_network_id_copy() {
    let network1 = NetworkId::Polkadot;
    let network2 = network1;

    assert_eq!(network1, network2);
}

#[test]
fn test_xcm_transfer_type_copy() {
    let transfer1 = XcmTransferType::ReserveTransfer;
    let transfer2 = transfer1;

    assert_eq!(transfer1, transfer2);
}

#[test]
fn test_xcm_version_copy() {
    let version1 = XcmVersion::V3;
    let version2 = version1;

    assert_eq!(version1, version2);
}
