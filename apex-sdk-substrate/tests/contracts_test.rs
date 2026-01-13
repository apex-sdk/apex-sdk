//! Comprehensive tests for contracts module
//!
//! These tests verify contract interaction functionality including:
//! - Contract metadata parsing
//! - Contract call building
//! - Gas limit calculations
//! - Storage deposit handling

use apex_sdk_substrate::contracts::*;

#[test]
fn test_gas_limit_new() {
    let limit = GasLimit::new(1_000_000, 2_000_000);
    assert_eq!(limit.ref_time, 1_000_000);
    assert_eq!(limit.proof_size, 2_000_000);
}

#[test]
fn test_gas_limit_default_call() {
    let limit = GasLimit::default_call();
    assert_eq!(limit.ref_time, 1_000_000_000_000);
    assert_eq!(limit.proof_size, 3_145_728);
}

#[test]
fn test_gas_limit_default_deploy() {
    let limit = GasLimit::default_deploy();
    assert_eq!(limit.ref_time, 5_000_000_000_000);
    assert_eq!(limit.proof_size, 10_485_760);
}

#[test]
fn test_gas_limit_encode_decode() {
    use parity_scale_codec::{Decode, Encode};

    let limit = GasLimit::new(123456, 789012);
    let encoded = limit.encode();
    let decoded = GasLimit::decode(&mut &encoded[..]).unwrap();

    assert_eq!(decoded.ref_time, 123456);
    assert_eq!(decoded.proof_size, 789012);
}

#[test]
fn test_storage_deposit_limit_no_limit() {
    let limit = StorageDepositLimit::NoLimit;
    match limit {
        StorageDepositLimit::NoLimit => {} // Expected
        _ => panic!("Expected NoLimit variant"),
    }
}

#[test]
fn test_storage_deposit_limit_limited() {
    let limit = StorageDepositLimit::Limited(1_000_000);
    match limit {
        StorageDepositLimit::Limited(amount) => assert_eq!(amount, 1_000_000),
        _ => panic!("Expected Limited variant"),
    }
}

#[test]
fn test_contract_call_builder_new() {
    let address = [1u8; 32];
    let selector = [0x12, 0x34, 0x56, 0x78];

    let builder = ContractCallBuilder::new(address, selector);
    let call_data = builder.build_call_data();

    assert_eq!(call_data.len(), 4);
    assert_eq!(&call_data[0..4], &selector);
}

#[test]
fn test_contract_call_builder_with_args() {
    let address = [2u8; 32];
    let selector = [0xAB, 0xCD, 0xEF, 0x01];
    let args = vec![0x11, 0x22, 0x33, 0x44];

    let builder = ContractCallBuilder::new(address, selector).args(&args);

    let call_data = builder.build_call_data();

    assert_eq!(call_data.len(), 8);
    assert_eq!(&call_data[0..4], &selector);
    assert_eq!(&call_data[4..8], &args[..]);
}

#[test]
fn test_contract_call_builder_with_gas_limit() {
    let address = [3u8; 32];
    let selector = [0x00, 0x11, 0x22, 0x33];
    let custom_gas = GasLimit::new(500_000, 1_000_000);

    let builder = ContractCallBuilder::new(address, selector).gas_limit(custom_gas);

    let call_data = builder.build_call_data();
    assert_eq!(&call_data[0..4], &selector);
}

#[test]
fn test_contract_call_builder_with_storage_deposit() {
    let address = [4u8; 32];
    let selector = [0xFF, 0xEE, 0xDD, 0xCC];

    let builder = ContractCallBuilder::new(address, selector)
        .storage_deposit(StorageDepositLimit::Limited(5_000_000));

    let call_data = builder.build_call_data();
    assert_eq!(&call_data[0..4], &selector);
}

#[test]
fn test_contract_call_builder_with_value() {
    let address = [5u8; 32];
    let selector = [0x01, 0x02, 0x03, 0x04];

    let builder = ContractCallBuilder::new(address, selector).value(1_000_000_000_000);

    let call_data = builder.build_call_data();
    assert_eq!(&call_data[0..4], &selector);
}

#[test]
fn test_contract_call_builder_chaining() {
    let address = [6u8; 32];
    let selector = [0xAA, 0xBB, 0xCC, 0xDD];
    let args = vec![1, 2, 3, 4, 5];

    let builder = ContractCallBuilder::new(address, selector)
        .args(&args)
        .gas_limit(GasLimit::new(1_000_000, 2_000_000))
        .storage_deposit(StorageDepositLimit::Limited(10_000_000))
        .value(500_000);

    let call_data = builder.build_call_data();
    assert_eq!(call_data.len(), 9);
    assert_eq!(&call_data[0..4], &selector);
    assert_eq!(&call_data[4..9], &args[..]);
}

#[test]
fn test_contract_metadata_parsing_minimal() {
    let json = r#"{
        "spec": {
            "constructors": [],
            "messages": [],
            "events": []
        },
        "storage": {
            "root": {
                "key": "0x00",
                "ty": 0
            }
        },
        "types": []
    }"#;

    let result = parse_metadata(json);
    assert!(result.is_ok());

    let metadata = result.unwrap();
    assert_eq!(metadata.spec.constructors.len(), 0);
    assert_eq!(metadata.spec.messages.len(), 0);
    assert_eq!(metadata.spec.events.len(), 0);
}

#[test]
fn test_contract_metadata_parsing_with_constructor() {
    let json = r#"{
        "spec": {
            "constructors": [{
                "label": "new",
                "selector": [155, 174, 157, 94],
                "args": [],
                "docs": ["Creates a new contract instance"]
            }],
            "messages": [],
            "events": []
        },
        "storage": {
            "root": {
                "key": "0x00",
                "ty": 0
            }
        },
        "types": []
    }"#;

    let result = parse_metadata(json);
    assert!(result.is_ok());

    let metadata = result.unwrap();
    assert_eq!(metadata.spec.constructors.len(), 1);
    assert_eq!(metadata.spec.constructors[0].label, "new");
    assert_eq!(metadata.spec.constructors[0].selector, [155, 174, 157, 94]);
}

#[test]
fn test_contract_metadata_parsing_with_message() {
    let json = r#"{
        "spec": {
            "constructors": [],
            "messages": [{
                "label": "get_value",
                "selector": [17, 34, 51, 68],
                "args": [],
                "return_type": {
                    "ty": 1,
                    "display_name": ["u128"]
                },
                "mutates": false,
                "payable": false,
                "docs": ["Returns the current value"]
            }],
            "events": []
        },
        "storage": {
            "root": {
                "key": "0x00",
                "ty": 0
            }
        },
        "types": []
    }"#;

    let result = parse_metadata(json);
    assert!(result.is_ok());

    let metadata = result.unwrap();
    assert_eq!(metadata.spec.messages.len(), 1);
    assert_eq!(metadata.spec.messages[0].label, "get_value");
    assert!(!metadata.spec.messages[0].mutates);
    assert!(!metadata.spec.messages[0].payable);
}

#[test]
fn test_contract_metadata_parsing_with_event() {
    let json = r#"{
        "spec": {
            "constructors": [],
            "messages": [],
            "events": [{
                "label": "Transfer",
                "args": [{
                    "label": "from",
                    "type_ref": {
                        "ty": 1,
                        "display_name": ["AccountId"]
                    },
                    "indexed": true
                }, {
                    "label": "to",
                    "type_ref": {
                        "ty": 1,
                        "display_name": ["AccountId"]
                    },
                    "indexed": true
                }, {
                    "label": "value",
                    "type_ref": {
                        "ty": 2,
                        "display_name": ["u128"]
                    },
                    "indexed": false
                }],
                "docs": ["Transfer event"]
            }]
        },
        "storage": {
            "root": {
                "key": "0x00",
                "ty": 0
            }
        },
        "types": []
    }"#;

    let result = parse_metadata(json);
    assert!(result.is_ok());

    let metadata = result.unwrap();
    assert_eq!(metadata.spec.events.len(), 1);
    assert_eq!(metadata.spec.events[0].label, "Transfer");
    assert_eq!(metadata.spec.events[0].args.len(), 3);
    assert!(metadata.spec.events[0].args[0].indexed);
    assert!(metadata.spec.events[0].args[1].indexed);
    assert!(!metadata.spec.events[0].args[2].indexed);
}

#[test]
fn test_contract_metadata_parsing_invalid_json() {
    let json = r#"{ invalid json }"#;

    let result = parse_metadata(json);
    assert!(result.is_err());
}

#[test]
fn test_contract_metadata_parsing_missing_fields() {
    let json = r#"{
        "spec": {
            "constructors": []
        }
    }"#;

    let result = parse_metadata(json);
    assert!(result.is_err());
}

#[test]
fn test_message_arg_structure() {
    let arg = MessageArg {
        label: "amount".to_string(),
        type_ref: TypeRef {
            ty: 1,
            display_name: vec!["u128".to_string()],
        },
    };

    assert_eq!(arg.label, "amount");
    assert_eq!(arg.type_ref.ty, 1);
    assert_eq!(arg.type_ref.display_name[0], "u128");
}

#[test]
fn test_constructor_spec_structure() {
    let constructor = ConstructorSpec {
        label: "new".to_string(),
        selector: [0x9B, 0xAE, 0x9D, 0x5E],
        args: vec![],
        docs: vec!["Creates a new instance".to_string()],
    };

    assert_eq!(constructor.label, "new");
    assert_eq!(constructor.selector, [0x9B, 0xAE, 0x9D, 0x5E]);
    assert_eq!(constructor.args.len(), 0);
    assert_eq!(constructor.docs.len(), 1);
}

#[test]
fn test_message_spec_structure() {
    let message = MessageSpec {
        label: "transfer".to_string(),
        selector: [0x84, 0xA1, 0x5D, 0xA1],
        args: vec![],
        return_type: None,
        mutates: true,
        payable: false,
        docs: vec!["Transfers tokens".to_string()],
    };

    assert_eq!(message.label, "transfer");
    assert!(message.mutates);
    assert!(!message.payable);
    assert!(message.return_type.is_none());
}

#[test]
fn test_event_spec_structure() {
    let event = EventSpec {
        label: "Approval".to_string(),
        args: vec![],
        docs: vec!["Approval event".to_string()],
    };

    assert_eq!(event.label, "Approval");
    assert_eq!(event.args.len(), 0);
    assert_eq!(event.docs.len(), 1);
}

#[test]
fn test_storage_layout_structure() {
    let layout = StorageLayout {
        root: LayoutKey {
            key: "0x00".to_string(),
            ty: 0,
        },
    };

    assert_eq!(layout.root.key, "0x00");
    assert_eq!(layout.root.ty, 0);
}

#[test]
fn test_type_def_variant_composite() {
    let variant = TypeDefVariant::Composite {
        fields: vec![Field {
            name: Some("value".to_string()),
            ty: 1,
        }],
    };

    match variant {
        TypeDefVariant::Composite { fields } => {
            assert_eq!(fields.len(), 1);
            assert_eq!(fields[0].name, Some("value".to_string()));
        }
        _ => panic!("Expected Composite variant"),
    }
}

#[test]
fn test_type_def_variant_sequence() {
    let variant = TypeDefVariant::Sequence { type_param: 5 };

    match variant {
        TypeDefVariant::Sequence { type_param } => {
            assert_eq!(type_param, 5);
        }
        _ => panic!("Expected Sequence variant"),
    }
}

#[test]
fn test_type_def_variant_array() {
    let variant = TypeDefVariant::Array {
        len: 10,
        type_param: 3,
    };

    match variant {
        TypeDefVariant::Array { len, type_param } => {
            assert_eq!(len, 10);
            assert_eq!(type_param, 3);
        }
        _ => panic!("Expected Array variant"),
    }
}

#[test]
fn test_type_def_variant_tuple() {
    let variant = TypeDefVariant::Tuple {
        fields: vec![1, 2, 3],
    };

    match variant {
        TypeDefVariant::Tuple { fields } => {
            assert_eq!(fields.len(), 3);
            assert_eq!(fields, vec![1, 2, 3]);
        }
        _ => panic!("Expected Tuple variant"),
    }
}

#[test]
fn test_type_def_variant_primitive() {
    let variant = TypeDefVariant::Primitive {
        primitive: "u128".to_string(),
    };

    match variant {
        TypeDefVariant::Primitive { primitive } => {
            assert_eq!(primitive, "u128");
        }
        _ => panic!("Expected Primitive variant"),
    }
}

#[test]
fn test_multiple_selectors() {
    // Test different selector patterns
    let selectors = vec![
        [0x00, 0x00, 0x00, 0x00],
        [0xFF, 0xFF, 0xFF, 0xFF],
        [0x12, 0x34, 0x56, 0x78],
        [0xAB, 0xCD, 0xEF, 0x01],
    ];

    for selector in selectors {
        let builder = ContractCallBuilder::new([0u8; 32], selector);
        let call_data = builder.build_call_data();
        assert_eq!(&call_data[0..4], &selector);
    }
}

#[test]
fn test_empty_args() {
    let address = [7u8; 32];
    let selector = [0x11, 0x22, 0x33, 0x44];

    let builder = ContractCallBuilder::new(address, selector).args(&[]);

    let call_data = builder.build_call_data();
    assert_eq!(call_data.len(), 4);
    assert_eq!(&call_data, &selector);
}

#[test]
fn test_large_args() {
    let address = [8u8; 32];
    let selector = [0xAA, 0xBB, 0xCC, 0xDD];
    let args = vec![0u8; 1024]; // 1KB of arguments

    let builder = ContractCallBuilder::new(address, selector).args(&args);

    let call_data = builder.build_call_data();
    assert_eq!(call_data.len(), 1028);
    assert_eq!(&call_data[0..4], &selector);
}
