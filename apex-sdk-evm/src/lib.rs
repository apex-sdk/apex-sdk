//! # Apex SDK EVM Adapter
//!
//! EVM blockchain adapter for the Apex SDK, providing unified access to Ethereum
//! and EVM-compatible chains.
//!
//! ## Supported Networks
//!
//! - Ethereum Mainnet
//! - Binance Smart Chain (BSC)
//! - Polygon (Matic)
//! - Avalanche C-Chain
//! - And other EVM-compatible chains
//!
//! ## Features
//!
//! - **HTTP and WebSocket Support**: Flexible connection types
//! - **Transaction Management**: Send, track, and query transactions
//! - **Smart Contract Interaction**: Call and deploy contracts
//! - **Wallet Integration**: Built-in wallet and signing support
//! - **Connection Pooling**: Efficient resource management
//! - **Metrics Collection**: Performance monitoring
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use apex_sdk_evm::EvmAdapter;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Connect to Ethereum mainnet
//!     let adapter = EvmAdapter::connect("https://eth.llamarpc.com").await?;
//!
//!     // Get balance
//!     let balance = adapter.get_balance("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7").await?;
//!     println!("Balance: {} wei", balance);
//!
//!     Ok(())
//! }
//! ```

pub mod cache;
pub mod metrics;
pub mod pool;
pub mod transaction;
pub mod wallet;

use apex_sdk_types::{Address, TransactionStatus};
use async_trait::async_trait;
use thiserror::Error;

// Alloy imports
use alloy::primitives::{Address as EthAddress, B256, U256};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::types::TransactionReceipt;

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

/// Type alias for the complex Alloy provider type with all fillers
pub type AlloyHttpProvider = alloy::providers::fillers::FillProvider<
    alloy::providers::fillers::JoinFill<
        alloy::providers::Identity,
        alloy::providers::fillers::JoinFill<
            alloy::providers::fillers::GasFiller,
            alloy::providers::fillers::JoinFill<
                alloy::providers::fillers::BlobGasFiller,
                alloy::providers::fillers::JoinFill<
                    alloy::providers::fillers::NonceFiller,
                    alloy::providers::fillers::ChainIdFiller,
                >,
            >,
        >,
    >,
    alloy::providers::RootProvider<alloy::network::Ethereum>,
    alloy::network::Ethereum,
>;

/// Provider type that supports HTTP connections
/// Uses dynamic dispatch to support multiple transport types
#[derive(Clone)]
pub struct ProviderType {
    inner: AlloyHttpProvider,
}

impl ProviderType {
    /// Create a new ProviderType from an AlloyHttpProvider
    ///
    /// # Note
    /// This is primarily intended for testing purposes. In production code,
    /// use `EvmAdapter::connect()` to create a properly initialized provider.
    #[doc(hidden)]
    pub fn new(inner: AlloyHttpProvider) -> Self {
        Self { inner }
    }

    /// Get the current block number
    async fn get_block_number(&self) -> Result<u64, Error> {
        self.inner
            .get_block_number()
            .await
            .map_err(|e| Error::Connection(format!("Failed to get block number: {}", e)))
    }

    pub async fn get_transaction_receipt(
        &self,
        hash: B256,
    ) -> Result<Option<TransactionReceipt>, Error> {
        self.inner
            .get_transaction_receipt(hash)
            .await
            .map_err(|e| Error::Transaction(format!("Failed to get receipt: {}", e)))
    }

    async fn get_transaction(
        &self,
        hash: B256,
    ) -> Result<Option<alloy::rpc::types::Transaction>, Error> {
        self.inner
            .get_transaction_by_hash(hash)
            .await
            .map_err(|e| Error::Transaction(format!("Failed to get transaction: {}", e)))
    }

    async fn get_balance(&self, address: EthAddress) -> Result<U256, Error> {
        self.inner
            .get_balance(address)
            .await
            .map_err(|e| Error::Connection(format!("Failed to get balance: {}", e)))
    }

    pub async fn get_chain_id(&self) -> Result<u64, Error> {
        self.inner
            .get_chain_id()
            .await
            .map_err(|e| Error::Connection(format!("Failed to get chain ID: {}", e)))
    }
}

/// EVM blockchain adapter
pub struct EvmAdapter {
    endpoint: String,
    provider: ProviderType,
    connected: bool,
}

impl EvmAdapter {
    /// Get the endpoint URL this adapter is connected to
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }
}

impl EvmAdapter {
    /// Get a reference to the Alloy provider
    pub fn provider(&self) -> &ProviderType {
        &self.provider
    }

    /// Create a transaction executor for this adapter
    pub fn transaction_executor(&self) -> transaction::TransactionExecutor {
        transaction::TransactionExecutor::new(self.provider.clone())
    }
}

impl EvmAdapter {
    /// Connect to an EVM node via HTTP
    pub async fn connect(endpoint: &str) -> Result<Self, Error> {
        tracing::info!("Connecting to EVM endpoint: {}", endpoint);

        // HTTP connection
        tracing::debug!("Using HTTP connection");
        let parsed_url = endpoint
            .parse()
            .map_err(|e| Error::Connection(format!("Invalid URL: {}", e)))?;
        let inner = ProviderBuilder::new().connect_http(parsed_url);
        let provider = ProviderType { inner };

        // Verify connection by getting chain ID
        let chain_id = provider.get_chain_id().await?;
        tracing::info!("Connected to chain ID: {}", chain_id);

        Ok(Self {
            endpoint: endpoint.to_string(),
            provider,
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

        // Parse transaction hash
        let hash: B256 = tx_hash
            .parse()
            .map_err(|e| Error::Transaction(format!("Invalid hash format: {}", e)))?;

        // Query transaction receipt
        match self.provider.get_transaction_receipt(hash).await? {
            Some(receipt) => {
                // Get current block number for confirmations
                let current_block = self.provider.get_block_number().await?;

                let _confirmations = if let Some(block_number) = receipt.block_number {
                    current_block.saturating_sub(block_number) as u32
                } else {
                    0
                };

                // Check if transaction succeeded (status is bool in Alloy)
                if receipt.status() {
                    Ok(TransactionStatus::Confirmed {
                        block_hash: receipt
                            .block_hash
                            .map(|h| format!("{:?}", h))
                            .unwrap_or_default(),
                        block_number: receipt.block_number,
                    })
                } else {
                    Ok(TransactionStatus::Failed {
                        error: "Transaction reverted".to_string(),
                    })
                }
            }
            None => {
                // Transaction not found in a block - check if it's in mempool
                match self.provider.get_transaction(hash).await? {
                    Some(_) => Ok(TransactionStatus::Pending),
                    None => Ok(TransactionStatus::Unknown),
                }
            }
        }
    }

    /// Get balance of an address in wei
    pub async fn get_balance(&self, address: &str) -> Result<U256, Error> {
        if !self.connected {
            return Err(Error::Connection("Not connected".to_string()));
        }

        tracing::debug!("Getting balance for address: {}", address);

        // Parse address
        let addr: EthAddress = address
            .parse()
            .map_err(|e| Error::InvalidAddress(format!("Invalid address format: {}", e)))?;

        // Query balance at latest block
        self.provider.get_balance(addr).await
    }

    /// Get balance of an address in a human-readable format (ETH)
    pub async fn get_balance_eth(&self, address: &str) -> Result<String, Error> {
        let balance_wei = self.get_balance(address).await?;

        // Convert wei to ETH (1 ETH = 10^18 wei)
        let eth_divisor = U256::from(10_u64.pow(18));
        let eth_value = balance_wei / eth_divisor;
        let remainder = balance_wei % eth_divisor;

        // Format with up to 18 decimal places
        Ok(format!("{}.{:018}", eth_value, remainder))
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
    pub fn contract(&self, address: &str) -> Result<ContractInfo, Error> {
        if !self.connected {
            return Err(Error::Connection("Not connected".to_string()));
        }

        if !self.validate_address(&Address::evm(address)) {
            return Err(Error::InvalidAddress(address.to_string()));
        }

        Ok(ContractInfo {
            address: address.to_string(),
        })
    }
}

/// Contract information and interaction
pub struct ContractInfo {
    address: String,
}

impl ContractInfo {
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
    use apex_sdk_core::ChainAdapter;
    use apex_sdk_types::Address;

    #[test]
    fn test_invalid_url_format() {
        // This doesn't require async or network
        let url = url::Url::parse("not-a-valid-url");
        assert!(url.is_err(), "Expected invalid URL to fail parsing");
    }

    #[test]
    fn test_address_validation_invalid() {
        // Create a mock adapter for testing without network
        let adapter = create_mock_adapter();

        let invalid_addr = Address::evm("invalid");
        assert!(!adapter.validate_address(&invalid_addr));

        let invalid_addr2 = Address::evm("0x123");
        assert!(!adapter.validate_address(&invalid_addr2));

        let invalid_addr3 = Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7G"); // Contains G
        assert!(!adapter.validate_address(&invalid_addr3));

        let substrate_addr = Address::substrate("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY");
        assert!(!adapter.validate_address(&substrate_addr));
    }

    #[test]
    fn test_address_validation_valid() {
        let adapter = create_mock_adapter();

        let valid_addr = Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7");
        assert!(adapter.validate_address(&valid_addr));

        let valid_addr2 = Address::evm("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045"); // vitalik.eth
        assert!(adapter.validate_address(&valid_addr2));

        let zero_addr = Address::evm("0x0000000000000000000000000000000000000000");
        assert!(adapter.validate_address(&zero_addr));
    }

    #[test]
    fn test_endpoint_getter() {
        let adapter = create_mock_adapter();
        assert_eq!(adapter.endpoint(), "https://mock.endpoint");
    }

    #[test]
    fn test_chain_adapter_trait_implementation() {
        let adapter = create_mock_adapter();
        assert_eq!(adapter.chain_name(), "EVM");

        let valid_addr = Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7");
        assert!(adapter.validate_address(&valid_addr));
    }

    #[test]
    fn test_contract_creation_valid_address() {
        let adapter = create_mock_adapter();
        let contract_result = adapter.contract("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7");

        assert!(contract_result.is_ok());
        let contract = contract_result.unwrap();
        assert_eq!(
            contract.address(),
            "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7"
        );
    }

    #[test]
    fn test_contract_creation_invalid_address() {
        let adapter = create_mock_adapter();
        let contract_result = adapter.contract("invalid_address");

        assert!(contract_result.is_err());
        if let Err(Error::InvalidAddress(addr)) = contract_result {
            assert_eq!(addr, "invalid_address");
        } else {
            panic!("Expected InvalidAddress error");
        }
    }

    #[test]
    fn test_transaction_executor_creation() {
        let adapter = create_mock_adapter();
        let _executor = adapter.transaction_executor();
        // Just test that we can create the executor without errors
    }

    #[test]
    fn test_provider_access() {
        let adapter = create_mock_adapter();
        let _provider = adapter.provider();
    }

    #[test]
    fn test_transaction_hash_validation() {
        let adapter = create_mock_adapter();

        let rt = tokio::runtime::Runtime::new().unwrap();

        // Too short hash
        let result = rt.block_on(adapter.get_transaction_status("0x123"));
        assert!(matches!(result, Err(Error::Transaction(_))));

        // Missing 0x prefix
        let result = rt.block_on(adapter.get_transaction_status(
            "5c504ed432cb51138bcf09aa5e8a410dd4a1e204ef84bfed1be16dfba1b22060",
        ));
        assert!(matches!(result, Err(Error::Transaction(_))));

        // Invalid hex character
        let result = rt.block_on(adapter.get_transaction_status(
            "0x5c504ed432cb51138bcf09aa5e8a410dd4a1e204ef84bfed1be16dfba1b22060G",
        ));
        assert!(matches!(result, Err(Error::Transaction(_))));
    }

    #[test]
    fn test_get_balance_disconnected() {
        let adapter = create_disconnected_adapter();
        let rt = tokio::runtime::Runtime::new().unwrap();

        let result = rt.block_on(adapter.get_balance("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7"));
        assert!(matches!(result, Err(Error::Connection(_))));
    }

    #[test]
    fn test_get_transaction_status_disconnected() {
        let adapter = create_disconnected_adapter();
        let rt = tokio::runtime::Runtime::new().unwrap();

        let result = rt.block_on(adapter.get_transaction_status(
            "0x5c504ed432cb51138bcf09aa5e8a410dd4a1e204ef84bfed1be16dfba1b22060",
        ));
        assert!(matches!(result, Err(Error::Connection(_))));
    }

    #[test]
    fn test_contract_creation_disconnected() {
        let adapter = create_disconnected_adapter();
        let result = adapter.contract("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7");
        assert!(matches!(result, Err(Error::Connection(_))));
    }

    #[test]
    fn test_error_display_messages() {
        let connection_err = Error::Connection("Test connection error".to_string());
        assert_eq!(
            connection_err.to_string(),
            "Connection error: Test connection error"
        );

        let transaction_err = Error::Transaction("Test transaction error".to_string());
        assert_eq!(
            transaction_err.to_string(),
            "Transaction error: Test transaction error"
        );

        let contract_err = Error::Contract("Test contract error".to_string());
        assert_eq!(
            contract_err.to_string(),
            "Contract error: Test contract error"
        );

        let address_err = Error::InvalidAddress("Test invalid address".to_string());
        assert_eq!(
            address_err.to_string(),
            "Invalid address: Test invalid address"
        );

        let other_err = Error::Other("Test other error".to_string());
        assert_eq!(other_err.to_string(), "Other error: Test other error");
    }

    #[test]
    fn test_balance_address_parsing_error() {
        let adapter = create_mock_adapter();
        let rt = tokio::runtime::Runtime::new().unwrap();

        let result = rt.block_on(adapter.get_balance("invalid_address"));
        assert!(matches!(result, Err(Error::InvalidAddress(_))));
    }

    // Integration tests (require network connection)
    #[tokio::test]
    #[ignore] // Requires network connection
    async fn test_evm_adapter_connect() {
        let adapter = EvmAdapter::connect("https://eth.llamarpc.com").await;
        assert!(adapter.is_ok());
    }

    #[tokio::test]
    #[ignore] // Requires network connection
    async fn test_address_validation_with_network() {
        let adapter = EvmAdapter::connect("https://eth.llamarpc.com")
            .await
            .unwrap();

        let valid_addr = Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7");
        assert!(adapter.validate_address(&valid_addr));

        let invalid_addr = Address::evm("invalid");
        assert!(!adapter.validate_address(&invalid_addr));
    }

    #[tokio::test]
    #[ignore] // Requires network connection
    async fn test_transaction_status_with_network() {
        let adapter = EvmAdapter::connect("https://eth.llamarpc.com")
            .await
            .unwrap();

        let result = adapter
            .get_transaction_status(
                "0x5c504ed432cb51138bcf09aa5e8a410dd4a1e204ef84bfed1be16dfba1b22060",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore] // Requires network connection
    async fn test_get_balance_with_network() {
        let adapter = EvmAdapter::connect("https://eth.llamarpc.com")
            .await
            .unwrap();

        let result = adapter
            .get_balance("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore] // Requires network connection
    async fn test_get_balance_eth_format_with_network() {
        let adapter = EvmAdapter::connect("https://eth.llamarpc.com")
            .await
            .unwrap();

        let result = adapter
            .get_balance_eth("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7")
            .await;
        assert!(result.is_ok());

        let eth_balance = result.unwrap();
        assert!(eth_balance.contains(".")); // Should have decimal point
    }

    #[tokio::test]
    #[ignore] // Requires network connection
    async fn test_invalid_endpoint_connection() {
        let result = EvmAdapter::connect("https://invalid.endpoint.that.does.not.exist").await;
        assert!(result.is_err());
    }

    // Helper functions for creating test adapters
    fn create_mock_adapter() -> EvmAdapter {
        // Create a mock adapter for testing without network
        let provider = create_mock_provider();
        EvmAdapter {
            endpoint: "https://mock.endpoint".to_string(),
            provider,
            connected: true,
        }
    }

    fn create_disconnected_adapter() -> EvmAdapter {
        let provider = create_mock_provider();
        EvmAdapter {
            endpoint: "https://mock.endpoint".to_string(),
            provider,
            connected: false,
        }
    }

    fn create_mock_provider() -> ProviderType {
        // For unit testing, we create a minimal provider that won't make network calls
        // This is a simplified version that will panic if actually used for network operations
        // but allows us to test the adapter logic
        let inner = ProviderBuilder::new().connect_http("http://localhost:8545".parse().unwrap());
        ProviderType { inner }
    }
}
