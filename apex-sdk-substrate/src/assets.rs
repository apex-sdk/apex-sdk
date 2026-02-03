use crate::{Result, SubstrateAdapter};
use apex_sdk_types::Address;
use subxt::dynamic::Value;
use subxt::ext::scale_value::Primitive;
use tracing::info;

/// High-level API for interacting with pallet-assets on Asset Hub
pub struct AssetManager<'a> {
    _adapter: &'a SubstrateAdapter,
}

impl<'a> AssetManager<'a> {
    pub fn new(adapter: &'a SubstrateAdapter) -> Self {
        Self { _adapter: adapter }
    }

    /// Create a new asset
    pub async fn create(
        &self,
        id: u32,
        admin: &Address,
        min_balance: u128,
    ) -> Result<subxt::tx::DynamicPayload> {
        info!("Preparing to create asset {} with admin {}", id, admin);

        // Convert Address to subxt Value for dynamic call
        let admin_val = Value::primitive(Primitive::String(admin.to_string()));

        let payload = subxt::dynamic::tx(
            "Assets",
            "create",
            vec![
                Value::unnamed_variant("u32", [Value::u128(id as u128)]),
                admin_val,
                Value::unnamed_variant("u128", [Value::u128(min_balance)]),
            ],
        );

        Ok(payload)
    }

    /// Set metadata for an asset
    pub async fn set_metadata(
        &self,
        id: u32,
        name: String,
        symbol: String,
        decimals: u8,
    ) -> Result<subxt::tx::DynamicPayload> {
        info!(
            "Preparing to set metadata for asset {}: {} ({})",
            id, name, symbol
        );

        let payload = subxt::dynamic::tx(
            "Assets",
            "set_metadata",
            vec![
                Value::unnamed_variant("u32", [Value::u128(id as u128)]),
                Value::string(name),
                Value::string(symbol),
                Value::u128(decimals as u128),
            ],
        );

        Ok(payload)
    }

    /// Mint assets to a beneficiary
    pub async fn mint(
        &self,
        id: u32,
        beneficiary: &Address,
        amount: u128,
    ) -> Result<subxt::tx::DynamicPayload> {
        info!(
            "Preparing to mint {} of asset {} to {}",
            amount, id, beneficiary
        );

        let beneficiary_val = Value::primitive(Primitive::String(beneficiary.to_string()));

        let payload = subxt::dynamic::tx(
            "Assets",
            "mint",
            vec![
                Value::unnamed_variant("u32", [Value::u128(id as u128)]),
                beneficiary_val,
                Value::unnamed_variant("u128", [Value::u128(amount)]),
            ],
        );

        Ok(payload)
    }

    /// Transfer assets to a target
    pub async fn transfer(
        &self,
        id: u32,
        target: &Address,
        amount: u128,
    ) -> Result<subxt::tx::DynamicPayload> {
        info!(
            "Preparing to transfer {} of asset {} to {}",
            amount, id, target
        );

        let target_val = Value::primitive(Primitive::String(target.to_string()));

        let payload = subxt::dynamic::tx(
            "Assets",
            "transfer",
            vec![
                Value::unnamed_variant("u32", [Value::u128(id as u128)]),
                target_val,
                Value::unnamed_variant("u128", [Value::u128(amount)]),
            ],
        );

        Ok(payload)
    }
}
