//! EVM Signer implementation

use crate::{AlloyHttpProvider, Error};
use alloy::consensus::SignableTransaction;
use alloy::network::TransactionBuilder;
use alloy::primitives::{Address as EthAddress, Bytes, Signature as EthSignature, U256};
use alloy::providers::Provider;
use alloy::rpc::types::{Block, BlockNumberOrTag, TransactionRequest};
use alloy::signers::{local::PrivateKeySigner, Signer as AlloySigner};
use apex_sdk_core::{SdkError, Signer as CoreSigner};
use apex_sdk_types::Address;
use async_trait::async_trait;
use std::str::FromStr;

/// Decoded transaction metadata: (tx_type, to_address, value, optional_calldata)
type DecodedMetadata = (u8, EthAddress, U256, Option<Vec<u8>>);

/// EVM signer implementation
#[derive(Debug, Clone)]
pub struct EvmSigner {
    signer: PrivateKeySigner,
    address: Address,
    provider: Option<AlloyHttpProvider>,
}

impl EvmSigner {
    /// Create a new EVM signer from private key
    pub fn new(private_key: &str) -> Result<Self, Error> {
        let signer = PrivateKeySigner::from_str(private_key)
            .map_err(|e| Error::Other(format!("Invalid private key: {}", e)))?;

        let eth_address = signer.address();
        let address = Address::evm(format!("0x{:x}", eth_address));

        Ok(Self {
            signer,
            address,
            provider: None,
        })
    }

    /// Create a new random EVM signer
    pub fn random() -> Result<Self, Error> {
        let signer = PrivateKeySigner::random();
        let eth_address = signer.address();
        let address = Address::evm(format!("0x{:x}", eth_address));

        Ok(Self {
            signer,
            address,
            provider: None,
        })
    }

    /// Set the provider for this signer (needed for transaction building)
    pub fn with_provider(mut self, provider: AlloyHttpProvider) -> Self {
        self.provider = Some(provider);
        self
    }

    /// Get the underlying Alloy signer
    pub fn alloy_signer(&self) -> &PrivateKeySigner {
        &self.signer
    }

    /// Sign a message hash
    pub async fn sign_message(&self, message: &[u8]) -> Result<EthSignature, Error> {
        self.signer
            .sign_message(message)
            .await
            .map_err(|e| Error::Other(format!("Failed to sign message: {}", e)))
    }

    /// Decode transaction metadata
    fn decode_metadata(&self, metadata: &[u8]) -> Result<DecodedMetadata, Error> {
        if metadata.is_empty() {
            return Err(Error::Transaction("Empty metadata".to_string()));
        }

        let tx_type = metadata[0];

        match tx_type {
            0x00 => {
                // Native ETH transfer: [0x00, to_address (20 bytes), value (32 bytes)]
                if metadata.len() < 53 {
                    return Err(Error::Transaction(
                        "Invalid native transfer metadata".to_string(),
                    ));
                }
                let to_bytes: [u8; 20] = metadata[1..21]
                    .try_into()
                    .map_err(|_| Error::Transaction("Invalid address".to_string()))?;
                let to = EthAddress::from(to_bytes);
                let value = U256::from_be_slice(&metadata[21..53]);
                Ok((tx_type, to, value, None))
            }
            0x01 => {
                // ERC-20 transfer: [0x01, token_address (20 bytes), calldata]
                if metadata.len() < 21 {
                    return Err(Error::Transaction(
                        "Invalid ERC-20 transfer metadata".to_string(),
                    ));
                }
                let to_bytes: [u8; 20] = metadata[1..21]
                    .try_into()
                    .map_err(|_| Error::Transaction("Invalid token address".to_string()))?;
                let token_address = EthAddress::from(to_bytes);
                let calldata = metadata[21..].to_vec();
                Ok((tx_type, token_address, U256::ZERO, Some(calldata)))
            }
            0x02 => {
                // Generic contract call: [0x02, contract_address (20 bytes), value (32 bytes), calldata]
                if metadata.len() < 53 {
                    return Err(Error::Transaction(
                        "Invalid contract call metadata".to_string(),
                    ));
                }
                let to_bytes: [u8; 20] = metadata[1..21]
                    .try_into()
                    .map_err(|_| Error::Transaction("Invalid contract address".to_string()))?;
                let to = EthAddress::from(to_bytes);
                let value = U256::from_be_slice(&metadata[21..53]);
                let calldata = if metadata.len() > 53 {
                    Some(metadata[53..].to_vec())
                } else {
                    None
                };
                Ok((tx_type, to, value, calldata))
            }
            _ => Err(Error::Transaction(format!(
                "Unknown transaction type: {}",
                tx_type
            ))),
        }
    }

    /// Get EIP-1559 fees from the provider
    async fn get_eip1559_fees(&self) -> Result<(U256, U256), Error> {
        let provider = self
            .provider
            .as_ref()
            .ok_or_else(|| Error::Other("Provider not set".to_string()))?;

        let block: Block = provider
            .get_block_by_number(BlockNumberOrTag::Latest)
            .await
            .map_err(|e| Error::Connection(format!("Failed to get block: {}", e)))?
            .ok_or_else(|| Error::Connection("No latest block".to_string()))?;

        let base_fee = block
            .header
            .base_fee_per_gas
            .map(U256::from)
            .ok_or_else(|| Error::Other("EIP-1559 not supported".to_string()))?;

        let priority_fee = U256::from(2_000_000_000u64); // 2 gwei

        Ok((base_fee, priority_fee))
    }

    /// Build a proper EVM transaction from metadata
    async fn build_transaction(&self, metadata: &[u8]) -> Result<TransactionRequest, Error> {
        let provider = self
            .provider
            .as_ref()
            .ok_or_else(|| Error::Other("Provider not set".to_string()))?;

        let (_tx_type, to, value, data) = self.decode_metadata(metadata)?;
        let from = self.signer.address();

        // Get nonce
        let nonce = provider
            .get_transaction_count(from)
            .await
            .map_err(|e| Error::Connection(format!("Failed to get nonce: {}", e)))?;

        // Build transaction request
        let mut tx = TransactionRequest::default()
            .with_from(from)
            .with_to(to)
            .with_value(value)
            .with_nonce(nonce);

        if let Some(call_data) = data {
            tx = tx.with_input(Bytes::from(call_data));
        }

        // Get chain ID
        let chain_id = provider
            .get_chain_id()
            .await
            .map_err(|e| Error::Connection(format!("Failed to get chain ID: {}", e)))?;
        tx = tx.with_chain_id(chain_id);

        // Estimate gas
        let estimated_gas = provider
            .estimate_gas(tx.clone())
            .await
            .map_err(|e| Error::Transaction(format!("Gas estimation failed: {}", e)))?;
        let gas_limit = ((estimated_gas as f64) * 1.2) as u64; // 20% buffer
        tx = tx.with_gas_limit(gas_limit);

        // Try to get EIP-1559 fees, fallback to legacy if not supported
        match self.get_eip1559_fees().await {
            Ok((base_fee, priority_fee)) => {
                let max_fee = base_fee * U256::from(2) + priority_fee;
                tx = tx
                    .with_max_fee_per_gas(max_fee.to::<u128>())
                    .with_max_priority_fee_per_gas(priority_fee.to::<u128>());
            }
            Err(_) => {
                // Fallback to legacy gas price
                let gas_price = provider
                    .get_gas_price()
                    .await
                    .map_err(|e| Error::Connection(format!("Failed to get gas price: {}", e)))?;
                tx = tx.with_gas_price(gas_price);
            }
        }

        Ok(tx)
    }
}

#[async_trait]
impl CoreSigner for EvmSigner {
    async fn sign_transaction(&self, tx: &[u8]) -> Result<Vec<u8>, SdkError> {
        if tx.is_empty() {
            return Err(SdkError::SignerError(
                "Cannot sign empty transaction".to_string(),
            ));
        }

        // If provider is not set, fall back to simple signing (for backward compatibility)
        if self.provider.is_none() {
            tracing::warn!("Provider not set for EvmSigner, using legacy signing method");
            let signature = self
                .signer
                .sign_message(tx)
                .await
                .map_err(|e| Error::Other(format!("Failed to sign transaction: {}", e)))?;

            let mut signed_tx = Vec::new();
            signed_tx.extend_from_slice(tx);
            let sig_bytes = signature.as_bytes();
            signed_tx.extend_from_slice(&sig_bytes);
            return Ok(signed_tx);
        }

        // Build proper EVM transaction from metadata
        let tx_request = self.build_transaction(tx).await?;

        // Build typed transaction
        let typed_tx = tx_request
            .build_typed_tx()
            .map_err(|e| Error::Transaction(format!("Failed to build transaction: {:?}", e)))?;

        // Get signature hash
        let signature_hash = typed_tx.signature_hash();

        // Sign the transaction hash
        let signature = self
            .signer
            .sign_hash(&signature_hash)
            .await
            .map_err(|e| Error::Other(format!("Failed to sign transaction: {}", e)))?;

        // Create signed transaction
        let signed_tx = typed_tx.into_signed(signature);

        // RLP encode the signed transaction
        let mut encoded = Vec::new();
        signed_tx.rlp_encode(&mut encoded);

        Ok(encoded)
    }

    fn address(&self) -> Address {
        self.address.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_evm_signer_creation() {
        let signer = EvmSigner::random();
        assert!(signer.is_ok());

        let signer = signer.unwrap();
        let message = b"test message";
        let signature = signer.sign_transaction(message).await;
        assert!(signature.is_ok());
    }

    #[tokio::test]
    async fn test_evm_signer_from_private_key() {
        // Test private key (do not use in production)
        let private_key = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
        let signer = EvmSigner::new(private_key);
        assert!(signer.is_ok());

        let signer = signer.unwrap();
        let expected_address = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266";
        assert_eq!(
            signer.address().to_string().to_lowercase(),
            expected_address
        );
    }
}
