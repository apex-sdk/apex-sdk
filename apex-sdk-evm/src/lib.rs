//! EVM blockchain adapter

use apex_sdk_types::{Address, TransactionStatus};
use async_trait::async_trait;
use thiserror::Error;

/// EVM adapter error
#[derive(Error, Debug)]
pub enum Error {
    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Transaction error: {0}")]
    Transaction(String),

    #[error("Contract error: {0}")]
    Contract(String),

    #[error("Invalid address: {0}")]
    InvalidAddress(String),

    #[error("Other error: {0}")]
    Other(String),
}

/// EVM blockchain adapter
pub struct EvmAdapter {
    #[allow(dead_code)]
    endpoint: String,
    connected: bool,
}

impl EvmAdapter {
    /// Connect to an EVM node
    pub async fn connect(endpoint: &str) -> Result<Self, Error> {
        tracing::info!("Connecting to EVM endpoint: {}", endpoint);

        // In a real implementation, this would establish HTTP/WebSocket connection
        // using ethers-rs or similar library
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

        // Validate tx hash format (0x + 64 hex chars)
        if !tx_hash.starts_with("0x") || tx_hash.len() != 66 {
            return Err(Error::Transaction("Invalid transaction hash".to_string()));
        }

        // Mock implementation - in real scenario, this would query the chain
        Ok(TransactionStatus::Confirmed {
            block_number: 18000000,
            confirmations: 12,
        })
    }

    /// Validate an EVM address (0x + 40 hex chars)
    pub fn validate_address(&self, address: &Address) -> bool {
        match address {
            Address::Evm(addr) => {
                addr.starts_with("0x")
                    && addr.len() == 42
                    && addr[2..].chars().all(|c| c.is_ascii_hexdigit())
            }
            _ => false,
        }
    }

    /// Get contract instance
    pub fn contract(&self, address: &str) -> Result<ContractInfo<'_>, Error> {
        if !self.connected {
            return Err(Error::Connection("Not connected".to_string()));
        }

        if !self.validate_address(&Address::evm(address)) {
            return Err(Error::InvalidAddress(address.to_string()));
        }

        Ok(ContractInfo {
            address: address.to_string(),
            adapter: self,
        })
    }
}

/// Contract information and interaction
pub struct ContractInfo<'a> {
    address: String,
    #[allow(dead_code)]
    adapter: &'a EvmAdapter,
}

impl<'a> ContractInfo<'a> {
    /// Get the contract address
    pub fn address(&self) -> &str {
        &self.address
    }
}

#[async_trait]
impl apex_sdk_core::ChainAdapter for EvmAdapter {
    async fn get_transaction_status(&self, tx_hash: &str) -> Result<TransactionStatus, String> {
        self.get_transaction_status(tx_hash)
            .await
            .map_err(|e| e.to_string())
    }

    fn validate_address(&self, address: &Address) -> bool {
        self.validate_address(address)
    }

    fn chain_name(&self) -> &str {
        "EVM"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_evm_adapter_connect() {
        let adapter = EvmAdapter::connect("https://mainnet.infura.io/v3/test").await;
        assert!(adapter.is_ok());
    }

    #[test]
    fn test_address_validation() {
        let adapter = EvmAdapter {
            endpoint: "test".to_string(),
            connected: true,
        };

        let valid_addr = Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7");
        assert!(adapter.validate_address(&valid_addr));

        let invalid_addr = Address::evm("invalid");
        assert!(!adapter.validate_address(&invalid_addr));

        let invalid_addr2 = Address::evm("0x123");
        assert!(!adapter.validate_address(&invalid_addr2));
    }

    #[tokio::test]
    async fn test_transaction_status() {
        let adapter = EvmAdapter::connect("https://test").await.unwrap();

        let result = adapter
            .get_transaction_status(
                "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
            )
            .await;
        assert!(result.is_ok());

        let invalid_result = adapter.get_transaction_status("invalid").await;
        assert!(invalid_result.is_err());
    }
}
