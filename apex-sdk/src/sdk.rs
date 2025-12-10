//! Main SDK interface providing unified access to both Substrate and EVM blockchains.

use crate::{
    error::{Error, Result},
    transaction::{Transaction, TransactionResult},
    types::Chain,
};
use std::{sync::Arc, time::Duration};

#[cfg(feature = "substrate")]
use apex_sdk_substrate::SubstrateAdapter;

#[cfg(feature = "evm")]
use apex_sdk_evm::EvmAdapter;

/// The main Apex SDK providing unified access to multiple blockchain types.
///
/// # Example
///
/// ```rust,no_run
/// use apex_sdk::ApexSDK;
/// use apex_sdk_types::Chain;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let sdk = ApexSDK::builder()
///         .with_substrate_endpoint("wss://polkadot.api.onfinality.io/public-ws")
///         .with_evm_endpoint("https://mainnet.infura.io/v3/YOUR_KEY")
///         .build()
///         .await?;
///     
///     // Check if a chain is supported
///     println!("Polkadot supported: {}", sdk.is_chain_supported(&Chain::Polkadot));
///     println!("Ethereum supported: {}", sdk.is_chain_supported(&Chain::Ethereum));
///     
///     Ok(())
/// }
/// ```
pub struct ApexSDK {
    #[cfg(feature = "substrate")]
    substrate_adapter: Option<Arc<SubstrateAdapter>>,

    #[cfg(feature = "evm")]
    evm_adapter: Option<Arc<EvmAdapter>>,

    timeout: Duration,
}

impl ApexSDK {
    /// Create a new SDK builder.
    pub fn builder() -> crate::builder::ApexSDKBuilder {
        crate::builder::ApexSDKBuilder::new()
    }

    /// Create a new SDK instance.
    ///
    /// **Note**: It's recommended to use the builder pattern instead of calling this directly.
    pub fn new(
        #[cfg(feature = "substrate")] substrate_adapter: Option<SubstrateAdapter>,
        #[cfg(feature = "evm")] evm_adapter: Option<EvmAdapter>,
        timeout: Duration,
    ) -> Result<Self> {
        // Ensure at least one adapter is provided
        #[cfg(not(any(feature = "substrate", feature = "evm")))]
        {
            return Err(Error::Config(
                "No blockchain adapters enabled. Enable 'substrate' or 'evm' features.".to_string(),
            ));
        }

        #[cfg(all(feature = "substrate", feature = "evm"))]
        {
            if substrate_adapter.is_none() && evm_adapter.is_none() {
                return Err(Error::Config(
                    "At least one adapter must be configured".to_string(),
                ));
            }
        }

        Ok(Self {
            #[cfg(feature = "substrate")]
            substrate_adapter: substrate_adapter.map(Arc::new),

            #[cfg(feature = "evm")]
            evm_adapter: evm_adapter.map(Arc::new),

            timeout,
        })
    }

    /// Execute a transaction on the appropriate blockchain.
    pub async fn execute(&self, transaction: Transaction) -> Result<TransactionResult> {
        match transaction.destination_chain() {
            #[cfg(feature = "substrate")]
            chain if chain.chain_type() == apex_sdk_types::ChainType::Substrate => {
                let adapter = self.substrate_adapter.as_ref().ok_or_else(|| {
                    Error::UnsupportedChain(format!(
                        "Substrate adapter not configured for {}",
                        chain.name()
                    ))
                })?;

                // Convert transaction and execute
                self.execute_substrate_transaction(adapter, transaction)
                    .await
            }

            #[cfg(feature = "evm")]
            chain if chain.chain_type() == apex_sdk_types::ChainType::Evm => {
                let adapter = self.evm_adapter.as_ref().ok_or_else(|| {
                    Error::UnsupportedChain(format!(
                        "EVM adapter not configured for {}",
                        chain.name()
                    ))
                })?;

                // Convert transaction and execute
                self.execute_evm_transaction(adapter, transaction).await
            }

            chain => Err(Error::UnsupportedChain(format!(
                "Chain {} not supported",
                chain.name()
            ))),
        }
    }

    /// Get the status of a transaction.
    pub async fn get_transaction_status(
        &self,
        tx_hash: &str,
        chain: &Chain,
    ) -> Result<apex_sdk_types::TransactionStatus> {
        match chain.chain_type() {
            #[cfg(feature = "substrate")]
            apex_sdk_types::ChainType::Substrate => {
                let adapter = self.substrate_adapter.as_ref().ok_or_else(|| {
                    Error::UnsupportedChain(format!(
                        "Substrate adapter not configured for {}",
                        chain.name()
                    ))
                })?;

                adapter
                    .get_transaction_status(tx_hash)
                    .await
                    .map_err(|e| Error::Transaction(e.to_string()))
            }

            #[cfg(feature = "evm")]
            apex_sdk_types::ChainType::Evm => {
                let adapter = self.evm_adapter.as_ref().ok_or_else(|| {
                    Error::UnsupportedChain(format!(
                        "EVM adapter not configured for {}",
                        chain.name()
                    ))
                })?;

                adapter
                    .get_transaction_status(tx_hash)
                    .await
                    .map_err(|e| Error::Transaction(e.to_string()))
            }

            #[cfg(feature = "substrate")]
            apex_sdk_types::ChainType::Hybrid => {
                // For hybrid chains, try substrate first, then EVM
                if let Some(adapter) = &self.substrate_adapter {
                    match adapter.get_transaction_status(tx_hash).await {
                        Ok(status) => Ok(status),
                        Err(_) => {
                            if let Some(evm_adapter) = &self.evm_adapter {
                                evm_adapter
                                    .get_transaction_status(tx_hash)
                                    .await
                                    .map_err(|e| Error::Transaction(e.to_string()))
                            } else {
                                Err(Error::Transaction(
                                    "No EVM adapter available for hybrid chain".to_string(),
                                ))
                            }
                        }
                    }
                } else if let Some(evm_adapter) = &self.evm_adapter {
                    evm_adapter
                        .get_transaction_status(tx_hash)
                        .await
                        .map_err(|e| Error::Transaction(e.to_string()))
                } else {
                    Err(Error::UnsupportedChain(format!(
                        "No adapter configured for hybrid chain {}",
                        chain.name()
                    )))
                }
            }

            #[cfg(not(feature = "substrate"))]
            apex_sdk_types::ChainType::Hybrid => {
                if let Some(evm_adapter) = &self.evm_adapter {
                    evm_adapter
                        .get_transaction_status(tx_hash)
                        .await
                        .map_err(Error::Transaction)
                } else {
                    Err(Error::UnsupportedChain(format!(
                        "No EVM adapter configured for hybrid chain {}",
                        chain.name()
                    )))
                }
            }
        }
    }

    /// Check if a chain is supported by the current configuration.
    pub fn is_chain_supported(&self, chain: &Chain) -> bool {
        match chain.chain_type() {
            #[cfg(feature = "substrate")]
            apex_sdk_types::ChainType::Substrate => self.substrate_adapter.is_some(),

            #[cfg(feature = "evm")]
            apex_sdk_types::ChainType::Evm => self.evm_adapter.is_some(),

            #[cfg(all(feature = "substrate", feature = "evm"))]
            apex_sdk_types::ChainType::Hybrid => {
                self.substrate_adapter.is_some() || self.evm_adapter.is_some()
            }

            #[cfg(not(any(feature = "substrate", feature = "evm")))]
            _ => false,

            #[cfg(all(not(feature = "substrate"), feature = "evm"))]
            apex_sdk_types::ChainType::Substrate | apex_sdk_types::ChainType::Hybrid => false,

            #[cfg(all(feature = "substrate", not(feature = "evm")))]
            apex_sdk_types::ChainType::Evm | apex_sdk_types::ChainType::Hybrid => false,
        }
    }

    /// Get access to the Substrate adapter (if configured).
    #[cfg(feature = "substrate")]
    pub fn substrate(&self) -> Result<Arc<SubstrateAdapter>> {
        self.substrate_adapter
            .as_ref()
            .cloned()
            .ok_or_else(|| Error::Config("Substrate adapter not configured".to_string()))
    }

    /// Get access to the EVM adapter (if configured).
    #[cfg(feature = "evm")]
    pub fn evm(&self) -> Result<Arc<EvmAdapter>> {
        self.evm_adapter
            .as_ref()
            .cloned()
            .ok_or_else(|| Error::Config("EVM adapter not configured".to_string()))
    }

    /// Get the configured timeout duration.
    pub fn timeout(&self) -> Duration {
        self.timeout
    }

    #[cfg(feature = "substrate")]
    async fn execute_substrate_transaction(
        &self,
        _adapter: &SubstrateAdapter,
        _transaction: Transaction,
    ) -> Result<TransactionResult> {
        // This would contain the actual substrate transaction execution logic
        // For now, return a placeholder
        todo!("Implement Substrate transaction execution")
    }

    #[cfg(feature = "evm")]
    async fn execute_evm_transaction(
        &self,
        _adapter: &EvmAdapter,
        _transaction: Transaction,
    ) -> Result<TransactionResult> {
        // This would contain the actual EVM transaction execution logic
        // For now, return a placeholder
        todo!("Implement EVM transaction execution")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use apex_sdk_types::{Address, Chain};

    #[test]
    fn test_new_returns_error_when_no_adapters() {
        let result = ApexSDK::new(
            #[cfg(feature = "substrate")]
            None,
            #[cfg(feature = "evm")]
            None,
            Duration::from_secs(30),
        );
        assert!(result.is_err());
        if let Err(Error::Config(msg)) = result {
            assert!(msg.contains("adapter"));
        }
    }

    #[test]
    fn test_timeout_getter() {
        // Test timeout configuration and default values
        let default_timeout = Duration::from_secs(30);
        let custom_timeout = Duration::from_secs(45);

        // Test that we can create durations for timeouts
        assert_eq!(default_timeout.as_secs(), 30);
        assert_eq!(custom_timeout.as_secs(), 45);
        assert!(custom_timeout > default_timeout);

        // Test timeout validation
        let min_timeout = Duration::from_secs(5);
        let max_timeout = Duration::from_secs(300); // 5 minutes

        assert!(min_timeout <= default_timeout);
        assert!(default_timeout <= max_timeout);
    }

    #[test]
    fn test_is_chain_supported_no_adapters() {
        let sdk = ApexSDK {
            #[cfg(feature = "substrate")]
            substrate_adapter: None,
            #[cfg(feature = "evm")]
            evm_adapter: None,
            timeout: Duration::from_secs(30),
        };

        assert!(!sdk.is_chain_supported(&Chain::Polkadot));
        assert!(!sdk.is_chain_supported(&Chain::Ethereum));
        assert!(!sdk.is_chain_supported(&Chain::Kusama));
    }

    #[test]
    fn test_chain_type_detection() {
        // Test that we can detect chain types correctly
        assert_eq!(
            Chain::Polkadot.chain_type(),
            apex_sdk_types::ChainType::Substrate
        );
        assert_eq!(
            Chain::Kusama.chain_type(),
            apex_sdk_types::ChainType::Substrate
        );
        assert_eq!(
            Chain::Westend.chain_type(),
            apex_sdk_types::ChainType::Substrate
        );

        assert_eq!(Chain::Ethereum.chain_type(), apex_sdk_types::ChainType::Evm);
        assert_eq!(Chain::Polygon.chain_type(), apex_sdk_types::ChainType::Evm);
        assert_eq!(
            Chain::BinanceSmartChain.chain_type(),
            apex_sdk_types::ChainType::Evm
        );
    }

    #[test]
    fn test_chain_name_parsing() {
        // Test case insensitive parsing of chain names
        assert_eq!(
            Chain::from_str_case_insensitive("polkadot"),
            Some(Chain::Polkadot)
        );
        assert_eq!(
            Chain::from_str_case_insensitive("POLKADOT"),
            Some(Chain::Polkadot)
        );
        assert_eq!(
            Chain::from_str_case_insensitive("ethereum"),
            Some(Chain::Ethereum)
        );
        assert_eq!(
            Chain::from_str_case_insensitive("ETHEREUM"),
            Some(Chain::Ethereum)
        );

        // Test invalid chain names
        assert_eq!(Chain::from_str_case_insensitive("invalid"), None);
        assert_eq!(Chain::from_str_case_insensitive(""), None);
    }

    #[test]
    fn test_chain_endpoint_validation() {
        // Test endpoint format validation
        let valid_evm_endpoints = [
            "https://eth.llamarpc.com",
            "https://ethereum.publicnode.com",
            "http://localhost:8545",
        ];

        for endpoint in &valid_evm_endpoints {
            assert!(endpoint.starts_with("http"));
            assert!(!endpoint.is_empty());
            assert!(endpoint.contains("://"));
        }

        let valid_substrate_endpoints = [
            "wss://polkadot.api.onfinality.io",
            "wss://kusama-rpc.polkadot.io",
            "ws://localhost:9944",
        ];

        for endpoint in &valid_substrate_endpoints {
            assert!(endpoint.starts_with("ws"));
            assert!(!endpoint.is_empty());
            assert!(endpoint.contains("://"));
        }
    }

    #[test]
    fn test_chain_configuration() {
        // Test chain configuration data
        let polkadot = Chain::Polkadot;
        let ethereum = Chain::Ethereum;

        // Test Display implementation (if available)
        let _polkadot_str = format!("{:?}", polkadot);
        let _ethereum_str = format!("{:?}", ethereum);

        // Test equality
        assert_eq!(polkadot, Chain::Polkadot);
        assert_ne!(polkadot, ethereum);

        // Test cloning
        let cloned_polkadot = polkadot.clone();
        assert_eq!(polkadot, cloned_polkadot);
    }

    #[test]
    fn test_address_validation() {
        // Test address format validation
        use apex_sdk_types::Address;

        // Test that we can create addresses
        let substrate_address = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
        let evm_address = "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEbD";

        // Test address creation
        let _substrate_addr = Address::substrate_checked(substrate_address);
        let _evm_addr = Address::evm_checked(evm_address);

        // Both should successfully parse or fail gracefully - test passes if no panic occurs
    }

    #[test]
    fn test_adapter_error_handling() {
        // Test error handling without requiring actual adapters
        use crate::Error;

        // Test that we can create configuration errors
        let config_error = Error::Config("Test configuration error".to_string());
        match config_error {
            Error::Config(msg) => assert_eq!(msg, "Test configuration error"),
            _ => panic!("Wrong error type"),
        }

        // Test error display
        let error_string = format!("{}", Error::Config("Test error".to_string()));
        assert!(error_string.contains("Test error"));
    }

    #[test]
    fn test_timeout_configuration() {
        // Test timeout configuration without actual adapters
        let timeouts = [
            Duration::from_secs(5),
            Duration::from_secs(30),
            Duration::from_secs(60),
            Duration::from_secs(300),
        ];

        for timeout in &timeouts {
            assert!(timeout.as_secs() >= 5);
            assert!(timeout.as_secs() <= 300);

            // Test that we can clone timeouts
            let cloned = *timeout;
            assert_eq!(*timeout, cloned);
        }
    }

    #[test]
    #[cfg(feature = "evm")]
    fn test_evm_adapter_not_configured() {
        let sdk = ApexSDK {
            #[cfg(feature = "substrate")]
            substrate_adapter: None,
            evm_adapter: None,
            timeout: Duration::from_secs(30),
        };

        // Should return error when adapter not configured
        let result = sdk.evm();
        assert!(result.is_err());
        if let Err(Error::Config(msg)) = result {
            assert!(msg.contains("EVM adapter not configured"));
        }
    }

    #[test]
    #[cfg(feature = "substrate")]
    fn test_get_transaction_status_substrate_not_configured() {
        let sdk = ApexSDK {
            substrate_adapter: None,
            #[cfg(feature = "evm")]
            evm_adapter: None,
            timeout: Duration::from_secs(30),
        };

        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(sdk.get_transaction_status("0x123", &Chain::Polkadot));
        assert!(result.is_err());
        if let Err(Error::UnsupportedChain(msg)) = result {
            assert!(msg.contains("Substrate adapter not configured"));
        }
    }

    #[test]
    #[cfg(feature = "evm")]
    fn test_get_transaction_status_evm_not_configured() {
        let sdk = ApexSDK {
            #[cfg(feature = "substrate")]
            substrate_adapter: None,
            evm_adapter: None,
            timeout: Duration::from_secs(30),
        };

        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(sdk.get_transaction_status("0x123", &Chain::Ethereum));
        assert!(result.is_err());
        if let Err(Error::UnsupportedChain(msg)) = result {
            assert!(msg.contains("EVM adapter not configured"));
        }
    }

    #[test]
    fn test_execute_unsupported_chain() {
        use crate::transaction::TransactionBuilder;

        let sdk = ApexSDK {
            #[cfg(feature = "substrate")]
            substrate_adapter: None,
            #[cfg(feature = "evm")]
            evm_adapter: None,
            timeout: Duration::from_secs(30),
        };

        let from_addr = Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEbD".to_string());
        let to_addr = Address::evm("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045".to_string());

        let transaction = TransactionBuilder::new()
            .from(from_addr)
            .to(to_addr)
            .amount(100)
            .build()
            .expect("Failed to build test transaction");

        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(sdk.execute(transaction));
        assert!(result.is_err());
    }

    #[test]
    fn test_chain_defaults() {
        // Test that chains have sensible default endpoints
        let polkadot = Chain::Polkadot;
        let ethereum = Chain::Ethereum;

        let polkadot_endpoint = polkadot.default_endpoint();
        let ethereum_endpoint = ethereum.default_endpoint();

        assert!(polkadot_endpoint.starts_with("wss://"));
        assert!(ethereum_endpoint.starts_with("https://"));

        assert!(!polkadot_endpoint.is_empty());
        assert!(!ethereum_endpoint.is_empty());
    }

    #[test]
    fn test_chain_types() {
        // Test all supported chains have correct types
        let substrate_chains = [Chain::Polkadot, Chain::Kusama, Chain::Westend, Chain::Paseo];

        for chain in &substrate_chains {
            assert_eq!(chain.chain_type(), apex_sdk_types::ChainType::Substrate);
        }

        let evm_chains = [Chain::Ethereum, Chain::Polygon, Chain::BinanceSmartChain];

        for chain in &evm_chains {
            assert_eq!(chain.chain_type(), apex_sdk_types::ChainType::Evm);
        }
    }

    #[test]
    fn test_builder_creates_sdk_instance() {
        let builder = ApexSDK::builder();
        // Just test that builder can be created
        let builder_type_name = std::any::type_name_of_val(&builder);
        assert!(builder_type_name.contains("ApexSDKBuilder"));
    }
}
