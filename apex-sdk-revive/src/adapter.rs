use crate::{Error, Result};
use apex_sdk_core::{BlockInfo, ChainAdapter, Provider, SdkError};
use apex_sdk_types::{Address, TransactionStatus, TxStatus};
use async_trait::async_trait;
use subxt::dynamic::{At, Value};
use subxt::{OnlineClient, PolkadotConfig};

/// Adapter for interacting with pallet-revive on System Chains
pub struct ReviveAdapter {
    client: OnlineClient<PolkadotConfig>,
}

impl ReviveAdapter {
    /// Create a new adapter from a subxt client (internal use)
    pub fn new(client: OnlineClient<PolkadotConfig>) -> Self {
        Self { client }
    }

    /// Connect to a node with pallet-revive
    pub async fn connect(url: &str) -> Result<Self> {
        let client = OnlineClient::from_url(url)
            .await
            .map_err(|e| Error::Connection(e.to_string()))?;
        Ok(Self { client })
    }

    /// Get the underlying subxt client
    pub fn client(&self) -> &OnlineClient<PolkadotConfig> {
        &self.client
    }

    /// Check if connected to the node
    pub async fn is_connected(&self) -> bool {
        self.client.blocks().at_latest().await.is_ok()
    }
}

#[async_trait]
impl Provider for ReviveAdapter {
    async fn get_block_number(&self) -> std::result::Result<u64, SdkError> {
        let block = self
            .client
            .blocks()
            .at_latest()
            .await
            .map_err(|e| SdkError::NetworkError(e.to_string()))?;
        Ok(block.number() as u64)
    }

    async fn get_balance(&self, address: &Address) -> std::result::Result<u128, SdkError> {
        self.get_revive_balance(address).await.map_err(|e| e.into())
    }

    async fn get_transaction_count(&self, address: &Address) -> std::result::Result<u64, SdkError> {
        let address_bytes = match address {
            Address::Substrate(s) => s.as_bytes().to_vec(),
            Address::Evm(e) => hex::decode(e.trim_start_matches("0x"))
                .map_err(|e| SdkError::ProviderError(format!("Invalid EVM address: {}", e)))?,
        };

        let storage_address =
            subxt::dynamic::storage("System", "Account", vec![Value::from_bytes(address_bytes)]);

        let account_info = self
            .client
            .storage()
            .at_latest()
            .await
            .map_err(|e| SdkError::NetworkError(e.to_string()))?
            .fetch(&storage_address)
            .await
            .map_err(|e| SdkError::NetworkError(e.to_string()))?;

        let account_info_value = match account_info {
            Some(info) => info
                .to_value()
                .map_err(|e| SdkError::ProviderError(format!("Failed to decode value: {}", e)))?,
            None => return Ok(0),
        };

        let nonce = account_info_value
            .at("nonce")
            .and_then(|n| n.as_u128())
            .map(|n| n as u64)
            .ok_or_else(|| SdkError::ProviderError("Invalid nonce type or structure".into()))?;

        Ok(nonce)
    }

    async fn estimate_fee(&self, _tx: &[u8]) -> std::result::Result<u128, SdkError> {
        Ok(0)
    }

    async fn get_block(&self, block_number: u64) -> std::result::Result<BlockInfo, SdkError> {
        let block_number_val = Value::from(block_number as u128);
        let storage_address =
            subxt::dynamic::storage("System", "BlockHash", vec![block_number_val]);

        let block_hash_value = self
            .client
            .storage()
            .at_latest()
            .await
            .map_err(|e| SdkError::NetworkError(e.to_string()))?
            .fetch(&storage_address)
            .await
            .map_err(|e| SdkError::NetworkError(e.to_string()))?
            .ok_or_else(|| {
                SdkError::NetworkError(format!("Block {} not found in storage", block_number))
            })?;

        let block_hash_bytes = block_hash_value
            .to_value()
            .map_err(|e| SdkError::ProviderError(format!("Failed to decode block hash: {}", e)))?;

        let hash_vec = format!("{:?}", block_hash_bytes);
        let hash_vec = if let Some(stripped) = hash_vec.strip_prefix("0x") {
            hex::decode(stripped).map_err(|e| SdkError::ProviderError(e.to_string()))?
        } else {
            return Err(SdkError::ProviderError(
                "Block hash is not in expected hex format".into(),
            ));
        };

        if hash_vec.len() != 32 {
            return Err(SdkError::ProviderError(format!(
                "Invalid block hash length: {}",
                hash_vec.len()
            )));
        }

        let mut hash = [0u8; 32];
        hash.copy_from_slice(&hash_vec);
        let block_hash: subxt::utils::H256 = hash.into();

        let block = self
            .client
            .blocks()
            .at(block_hash)
            .await
            .map_err(|e| SdkError::NetworkError(e.to_string()))?;

        Ok(BlockInfo {
            number: block_number,
            hash: format!("0x{:x}", block.hash()),
            parent_hash: format!("0x{:x}", block.header().parent_hash),
            timestamp: 0,
            transactions: vec![],
            state_root: Some(format!("0x{:x}", block.header().state_root)),
            extrinsics_root: Some(format!("0x{:x}", block.header().extrinsics_root)),
            extrinsic_count: 0,
            event_count: None,
            is_finalized: false,
        })
    }

    async fn health_check(&self) -> std::result::Result<(), SdkError> {
        self.get_block_number().await.map(|_| ())
    }
}

#[async_trait]
impl ChainAdapter for ReviveAdapter {
    async fn get_transaction_status(
        &self,
        tx_hash: &str,
    ) -> std::result::Result<TransactionStatus, String> {
        self.get_transaction_status_async(tx_hash)
            .await
            .map_err(|e| e.to_string())
    }

    fn validate_address(&self, address: &Address) -> bool {
        match address {
            Address::Evm(_) => true,
            Address::Substrate(_) => true,
        }
    }

    fn chain_name(&self) -> &str {
        "Revive"
    }
}

impl ReviveAdapter {
    /// Get transaction status by hash
    pub async fn get_transaction_status_async(&self, tx_hash: &str) -> Result<TransactionStatus> {
        let hash_vec = if let Some(stripped) = tx_hash.strip_prefix("0x") {
            hex::decode(stripped).map_err(|e| Error::Other(format!("Invalid hash: {}", e)))?
        } else {
            hex::decode(tx_hash).map_err(|e| Error::Other(format!("Invalid hash: {}", e)))?
        };

        if hash_vec.len() != 32 {
            return Err(Error::Other("Transaction hash must be 32 bytes".into()));
        }

        let mut hash = [0u8; 32];
        hash.copy_from_slice(&hash_vec);

        // Try to find the transaction in the last few blocks
        let best_block = self
            .client
            .blocks()
            .at_latest()
            .await
            .map_err(|e| Error::Connection(e.to_string()))?;

        let mut current_block_hash = best_block.hash();

        // Check last 10 blocks by following parent hashes
        for _ in 0..10 {
            let block = self.client.blocks().at(current_block_hash).await;
            if let Ok(block) = block {
                let block_number = block.number() as u64;
                let extrinsics = block.extrinsics().await;
                if let Ok(extrinsics) = extrinsics {
                    for ext_details in extrinsics.iter() {
                        if ext_details.hash().0 == hash {
                            // Found it!
                            return Ok(TransactionStatus {
                                hash: tx_hash.to_string(),
                                status: TxStatus::Confirmed,
                                block_number: Some(block_number),
                                block_hash: Some(format!("0x{:x}", block.hash())),
                                ..Default::default()
                            });
                        }
                    }
                }
                current_block_hash = block.header().parent_hash;
            } else {
                break;
            }
        }

        Ok(TransactionStatus::pending(tx_hash.to_string()))
    }

    /// Get the balance of an address (native currency)
    pub async fn get_revive_balance(&self, address: &Address) -> Result<u128> {
        let address_bytes = match address {
            Address::Substrate(_s) => address.as_str().as_bytes().to_vec(),
            Address::Evm(e) => hex::decode(e.trim_start_matches("0x"))
                .map_err(|e| Error::Other(format!("Invalid EVM address: {}", e)))?,
        };

        let storage_address =
            subxt::dynamic::storage("System", "Account", vec![Value::from_bytes(address_bytes)]);

        let account_info = self
            .client
            .storage()
            .at_latest()
            .await
            .map_err(|e| Error::Connection(e.to_string()))?
            .fetch(&storage_address)
            .await
            .map_err(|e| Error::Connection(e.to_string()))?;

        let account_info_value = match account_info {
            Some(info) => info
                .to_value()
                .map_err(|e| Error::Other(format!("Failed to decode value: {}", e)))?,
            None => return Ok(0),
        };

        let balance = account_info_value
            .at("data")
            .and_then(|data| data.at("free"))
            .and_then(|free| free.as_u128())
            .ok_or(Error::Other("Invalid balance type or structure".into()))?;

        Ok(balance)
    }
}
