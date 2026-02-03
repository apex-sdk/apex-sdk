use crate::{Result, SubstrateAdapter};
use apex_sdk_types::Address;
use subxt::dynamic::Value;
use tracing::info;

/// High-level API for interacting with pallet-nfts on Asset Hub
pub struct NftManager<'a> {
    _adapter: &'a SubstrateAdapter,
}

impl<'a> NftManager<'a> {
    pub fn new(adapter: &'a SubstrateAdapter) -> Self {
        Self { _adapter: adapter }
    }

    /// Create a new NFT collection
    pub async fn create_collection(
        &self,
        collection_id: u32,
        owner: &Address,
        metadata: String,
    ) -> Result<subxt::tx::DynamicPayload> {
        info!(
            "Preparing to create NFT collection {} for {}",
            collection_id, owner
        );
        let owner_val = Value::string(owner.to_string());
        let payload = subxt::dynamic::tx(
            "Nfts",
            "create_collection",
            vec![
                Value::u128(collection_id as u128),
                owner_val,
                Value::string(metadata),
            ],
        );
        Ok(payload)
    }

    /// Mint a new NFT item
    pub async fn mint(
        &self,
        collection_id: u32,
        item_id: u32,
        owner: &Address,
        metadata: String,
    ) -> Result<subxt::tx::DynamicPayload> {
        info!(
            "Preparing to mint NFT item {} in collection {} for {}",
            item_id, collection_id, owner
        );
        let owner_val = Value::string(owner.to_string());
        let payload = subxt::dynamic::tx(
            "Nfts",
            "mint",
            vec![
                Value::u128(collection_id as u128),
                Value::u128(item_id as u128),
                owner_val,
                Value::string(metadata),
            ],
        );
        Ok(payload)
    }

    /// Transfer an NFT item
    pub async fn transfer(
        &self,
        collection_id: u32,
        item_id: u32,
        to: &Address,
    ) -> Result<subxt::tx::DynamicPayload> {
        info!(
            "Preparing to transfer NFT item {} in collection {} to {}",
            item_id, collection_id, to
        );
        let to_val = Value::string(to.to_string());
        let payload = subxt::dynamic::tx(
            "Nfts",
            "transfer",
            vec![
                Value::u128(collection_id as u128),
                Value::u128(item_id as u128),
                to_val,
            ],
        );
        Ok(payload)
    }
}
