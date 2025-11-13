//! Substrate blockchain adapter

use apex_sdk_types::{Address, TransactionStatus};
use async_trait::async_trait;
use thiserror::Error;

/// Substrate adapter error
#[derive(Error, Debug)]
pub enum Error {
    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Transaction error: {0}")]
    Transaction(String),

    #[error("Metadata error: {0}")]
    Metadata(String),

    #[error("Other error: {0}")]
    Other(String),
}

/// Substrate blockchain adapter
pub struct SubstrateAdapter {
    #[allow(dead_code)]
    endpoint: String,
    connected: bool,
}

impl SubstrateAdapter {
    /// Connect to a Substrate node
    pub async fn connect(endpoint: &str) -> Result<Self, Error> {
        tracing::info!("Connecting to Substrate endpoint: {}", endpoint);

        // In a real implementation, this would establish WebSocket connection
        // using subxt or similar library
        Ok(Self {
            endpoint: endpoint.to_string(),
            connected: true,
        })
    }

    /// Get transaction status
    pub async fn get_transaction_status(&self, tx_hash: &str) -> Result<TransactionStatus, Error> {
        if !self.connected {
            return Err(Error::Connection("Not connected".to_string()));
        }

        tracing::debug!("Getting transaction status for: {}", tx_hash);

        // Mock implementation - in real scenario, this would query the chain
        Ok(TransactionStatus::Confirmed {
            block_number: 12345,
            confirmations: 10,
        })
    }

    /// Validate a Substrate address (SS58 format)
    pub fn validate_address(&self, address: &Address) -> bool {
        match address {
            Address::Substrate(addr) => {
                // Basic validation - real implementation would use SS58 codec
                !addr.is_empty() && addr.len() >= 47
            }
            _ => false,
        }
    }

    /// Get pallet information
    pub fn pallet(&self, name: &str) -> Result<PalletInfo<'_>, Error> {
        if !self.connected {
            return Err(Error::Connection("Not connected".to_string()));
        }

        Ok(PalletInfo {
            name: name.to_string(),
            adapter: self,
        })
    }
}

/// Pallet information and interaction
pub struct PalletInfo<'a> {
    name: String,
    #[allow(dead_code)]
    adapter: &'a SubstrateAdapter,
}

impl<'a> PalletInfo<'a> {
    /// Get the pallet name
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[async_trait]
impl apex_sdk_core::ChainAdapter for SubstrateAdapter {
    async fn get_transaction_status(&self, tx_hash: &str) -> Result<TransactionStatus, String> {
        self.get_transaction_status(tx_hash)
            .await
            .map_err(|e| e.to_string())
    }

    fn validate_address(&self, address: &Address) -> bool {
        self.validate_address(address)
    }

    fn chain_name(&self) -> &str {
        "Substrate"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_substrate_adapter_connect() {
        let adapter = SubstrateAdapter::connect("wss://rpc.polkadot.io").await;
        assert!(adapter.is_ok());
    }

    #[test]
    fn test_address_validation() {
        let adapter = SubstrateAdapter {
            endpoint: "test".to_string(),
            connected: true,
        };

        let valid_addr = Address::substrate("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY");
        assert!(adapter.validate_address(&valid_addr));

        let invalid_addr = Address::substrate("invalid");
        assert!(!adapter.validate_address(&invalid_addr));
    }
}
