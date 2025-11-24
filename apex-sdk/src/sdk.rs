//! Main ApexSDK implementation
//!
//! This module provides the core SDK struct and methods for interacting
//! with multiple blockchain networks through a unified interface.
//!
//! # Examples
//!
//! ```rust,no_run
//! use apex_sdk::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     // Initialize SDK with both Substrate and EVM support
//!     let sdk = ApexSDK::builder()
//!         .with_substrate_endpoint("wss://polkadot.api.onfinality.io/public-ws")
//!         .with_evm_endpoint("https://mainnet.infura.io/v3/YOUR_KEY")
//!         .build()
//!         .await?;
//!
//!     // Check chain support
//!     if sdk.is_chain_supported(&Chain::Ethereum) {
//!         println!("Ethereum is supported!");
//!     }
//!
//!     // Create and execute a transaction
//!     let tx = sdk.transaction()
//!         .from_evm_address("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7")
//!         .to_evm_address("0x1234567890123456789012345678901234567890")
//!         .amount(1000)
//!         .build()?;
//!
//!     let result = sdk.execute(tx).await?;
//!     println!("Transaction hash: {}", result.source_tx_hash);
//!
//!     Ok(())
//! }
//! ```

use crate::builder::ApexSDKBuilder;
use crate::error::{Error, Result};
use crate::transaction::{Transaction, TransactionBuilder, TransactionResult};
use apex_sdk_evm::EvmAdapter;
use apex_sdk_substrate::SubstrateAdapter;
use apex_sdk_types::{Chain, TransactionStatus};

/// Main Apex SDK struct providing unified interface to blockchain operations.
///
/// The `ApexSDK` is the primary entry point for interacting with multiple
/// blockchain networks. It manages connections to both Substrate-based and
/// EVM-compatible chains through adapter interfaces.
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust,no_run
/// use apex_sdk::prelude::*;
///
/// # #[tokio::main]
/// # async fn main() -> Result<()> {
/// let sdk = ApexSDK::builder()
///     .with_evm_endpoint("https://mainnet.infura.io/v3/YOUR_KEY")
///     .build()
///     .await?;
/// # Ok(())
/// # }
/// ```
pub struct ApexSDK {
    pub(crate) substrate_adapter: Option<SubstrateAdapter>,
    pub(crate) evm_adapter: Option<EvmAdapter>,
}

impl ApexSDK {
    /// Create a new builder for configuring the SDK.
    ///
    /// This is the recommended way to create an ApexSDK instance.
    /// Use the builder pattern to configure which blockchain adapters
    /// you need before initialization.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use apex_sdk::prelude::*;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// let sdk = ApexSDK::builder()
    ///     .with_substrate_endpoint("wss://polkadot.api.onfinality.io/public-ws")
    ///     .with_evm_endpoint("https://mainnet.infura.io/v3/YOUR_KEY")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn builder() -> ApexSDKBuilder {
        ApexSDKBuilder::new()
    }

    /// Create a new SDK instance with default configuration.
    ///
    /// # Note
    ///
    /// This method will always return an error. Use `ApexSDK::builder()`
    /// instead to properly configure the SDK.
    ///
    /// # Errors
    ///
    /// Always returns a configuration error directing you to use the builder.
    pub async fn new() -> Result<Self> {
        Err(Error::config("Use ApexSDK::builder() to configure the SDK"))
    }

    /// Get a reference to the Substrate adapter.
    ///
    /// # Errors
    ///
    /// Returns an error if the Substrate adapter was not configured
    /// during SDK initialization.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use apex_sdk::prelude::*;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// let sdk = ApexSDK::builder()
    ///     .with_substrate_endpoint("wss://polkadot.api.onfinality.io/public-ws")
    ///     .build()
    ///     .await?;
    ///
    /// let substrate = sdk.substrate()?;
    /// // Use the Substrate adapter...
    /// # Ok(())
    /// # }
    /// ```
    pub fn substrate(&self) -> Result<&SubstrateAdapter> {
        self.substrate_adapter
            .as_ref()
            .ok_or_else(|| Error::config("Substrate adapter not configured"))
    }

    /// Get a reference to the EVM adapter.
    ///
    /// # Errors
    ///
    /// Returns an error if the EVM adapter was not configured
    /// during SDK initialization.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use apex_sdk::prelude::*;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// let sdk = ApexSDK::builder()
    ///     .with_evm_endpoint("https://mainnet.infura.io/v3/YOUR_KEY")
    ///     .build()
    ///     .await?;
    ///
    /// let evm = sdk.evm()?;
    /// // Use the EVM adapter...
    /// # Ok(())
    /// # }
    /// ```
    pub fn evm(&self) -> Result<&EvmAdapter> {
        self.evm_adapter
            .as_ref()
            .ok_or_else(|| Error::config("EVM adapter not configured"))
    }

    /// Check if a specific blockchain is supported by the current SDK configuration.
    ///
    /// Returns `true` if the chain is supported, `false` otherwise. Support
    /// depends on which adapters were configured during SDK initialization.
    ///
    /// # Arguments
    ///
    /// * `chain` - The blockchain to check for support
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use apex_sdk::prelude::*;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// let sdk = ApexSDK::builder()
    ///     .with_evm_endpoint("https://mainnet.infura.io/v3/YOUR_KEY")
    ///     .build()
    ///     .await?;
    ///
    /// if sdk.is_chain_supported(&Chain::Ethereum) {
    ///     println!("Ethereum is supported!");
    /// }
    ///
    /// if !sdk.is_chain_supported(&Chain::Polkadot) {
    ///     println!("Polkadot is not supported (no Substrate adapter)");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_chain_supported(&self, chain: &Chain) -> bool {
        match chain {
            Chain::Polkadot
            | Chain::Kusama
            | Chain::Acala
            | Chain::Phala
            | Chain::Bifrost
            | Chain::Westend
            | Chain::Paseo => self.substrate_adapter.is_some(),
            Chain::Ethereum
            | Chain::Polygon
            | Chain::BinanceSmartChain
            | Chain::Avalanche
            | Chain::Arbitrum
            | Chain::Optimism
            | Chain::ZkSync
            | Chain::Base => self.evm_adapter.is_some(),
            Chain::Moonbeam | Chain::Astar => {
                self.substrate_adapter.is_some() && self.evm_adapter.is_some()
            }
        }
    }

    /// Get the status of a transaction
    pub async fn get_transaction_status(
        &self,
        chain: &Chain,
        tx_hash: &str,
    ) -> Result<TransactionStatus> {
        match chain {
            Chain::Polkadot
            | Chain::Kusama
            | Chain::Acala
            | Chain::Phala
            | Chain::Bifrost
            | Chain::Westend
            | Chain::Paseo => self
                .substrate()?
                .get_transaction_status(tx_hash)
                .await
                .map_err(Error::Substrate),
            Chain::Ethereum
            | Chain::Polygon
            | Chain::BinanceSmartChain
            | Chain::Avalanche
            | Chain::Arbitrum
            | Chain::Optimism
            | Chain::ZkSync
            | Chain::Base => self
                .evm()?
                .get_transaction_status(tx_hash)
                .await
                .map_err(Error::Evm),
            Chain::Moonbeam | Chain::Astar => {
                // Try EVM first for hybrid chains
                self.evm()?
                    .get_transaction_status(tx_hash)
                    .await
                    .map_err(Error::Evm)
            }
        }
    }

    /// Create a new transaction builder
    pub fn transaction(&self) -> TransactionBuilder {
        TransactionBuilder::new()
    }

    /// Prepare and validate a transaction
    ///
    /// This method validates that the required adapters are configured and
    /// prepares the transaction for execution. Returns a transaction result
    /// with status set to Pending, indicating the transaction is ready to
    /// be signed and broadcast.
    ///
    /// Note: Actual transaction signing and broadcasting requires a signer.
    /// Use the substrate or EVM adapter directly with a wallet for full
    /// transaction execution.
    pub async fn execute(&self, transaction: Transaction) -> Result<TransactionResult> {
        tracing::info!(
            "Preparing transaction from {:?} to {:?}",
            transaction.source_chain,
            transaction.destination_chain
        );

        // Validate that the required adapters are configured
        match transaction.source_chain {
            Chain::Polkadot
            | Chain::Kusama
            | Chain::Acala
            | Chain::Phala
            | Chain::Bifrost
            | Chain::Westend
            | Chain::Paseo => {
                self.substrate()?;
            }
            Chain::Ethereum
            | Chain::Polygon
            | Chain::BinanceSmartChain
            | Chain::Avalanche
            | Chain::Arbitrum
            | Chain::Optimism
            | Chain::ZkSync
            | Chain::Base => {
                self.evm()?;
            }
            Chain::Moonbeam | Chain::Astar => {
                self.substrate()?;
                self.evm()?;
            }
        }

        // Compute transaction hash using the transaction's hash method
        let source_tx_hash = transaction.hash();

        // For cross-chain transactions, generate a destination hash
        let destination_tx_hash = if transaction.is_cross_chain() {
            // Create a modified hash for the destination chain
            let mut dest_tx = transaction.clone();
            std::mem::swap(&mut dest_tx.from, &mut dest_tx.to);
            Some(dest_tx.hash())
        } else {
            None
        };

        // Return transaction as pending - ready for signing
        Ok(TransactionResult {
            source_tx_hash,
            destination_tx_hash,
            status: TransactionStatus::Pending,
            block_number: None,
            gas_used: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_builder_requires_adapter() {
        let result = ApexSDK::builder().build().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_new_returns_error() {
        let result = ApexSDK::new().await;
        assert!(result.is_err());
        match result {
            Err(Error::Config(_, _)) => {}
            _ => panic!("Expected Config error"),
        }
    }

    #[tokio::test]
    #[ignore] // Requires network connection
    async fn test_substrate_adapter_not_configured() {
        let sdk = ApexSDK::builder()
            .with_evm_endpoint("https://eth.llamarpc.com")
            .build()
            .await
            .unwrap();

        let result = sdk.substrate();
        assert!(result.is_err());
        match result {
            Err(Error::Config(msg, _)) => {
                assert!(msg.contains("Substrate adapter not configured"));
            }
            _ => panic!("Expected Config error"),
        }
    }

    #[tokio::test]
    #[ignore] // Requires network connection
    async fn test_evm_adapter_not_configured() {
        let sdk = ApexSDK::builder()
            .with_substrate_endpoint("wss://test")
            .build()
            .await
            .unwrap();

        let result = sdk.evm();
        assert!(result.is_err());
        match result {
            Err(Error::Config(msg, _)) => {
                assert!(msg.contains("EVM adapter not configured"));
            }
            _ => panic!("Expected Config error"),
        }
    }

    #[tokio::test]
    #[ignore] // Requires network connection
    async fn test_is_chain_supported_substrate_only() {
        let sdk = ApexSDK::builder()
            .with_substrate_endpoint("wss://test")
            .build()
            .await
            .unwrap();

        assert!(sdk.is_chain_supported(&Chain::Polkadot));
        assert!(sdk.is_chain_supported(&Chain::Kusama));
        assert!(!sdk.is_chain_supported(&Chain::Ethereum));
        assert!(!sdk.is_chain_supported(&Chain::Polygon));
        assert!(!sdk.is_chain_supported(&Chain::Moonbeam)); // Requires both adapters
    }

    #[tokio::test]
    #[ignore] // Requires network connection
    async fn test_is_chain_supported_evm_only() {
        let sdk = ApexSDK::builder()
            .with_evm_endpoint("https://eth.llamarpc.com")
            .build()
            .await
            .unwrap();

        assert!(!sdk.is_chain_supported(&Chain::Polkadot));
        assert!(!sdk.is_chain_supported(&Chain::Kusama));
        assert!(sdk.is_chain_supported(&Chain::Ethereum));
        assert!(sdk.is_chain_supported(&Chain::Polygon));
        assert!(!sdk.is_chain_supported(&Chain::Moonbeam)); // Requires both adapters
    }

    #[tokio::test]
    #[ignore] // Requires network connection
    async fn test_is_chain_supported_both_adapters() {
        let sdk = ApexSDK::builder()
            .with_substrate_endpoint("wss://rpc.polkadot.io")
            .with_evm_endpoint("https://eth.llamarpc.com")
            .build()
            .await
            .unwrap();

        assert!(sdk.is_chain_supported(&Chain::Polkadot));
        assert!(sdk.is_chain_supported(&Chain::Ethereum));
        assert!(sdk.is_chain_supported(&Chain::Moonbeam));
        assert!(sdk.is_chain_supported(&Chain::Astar));
    }

    #[tokio::test]
    #[ignore] // Requires network connection
    async fn test_transaction_builder() {
        let sdk = ApexSDK::builder()
            .with_evm_endpoint("https://eth.llamarpc.com")
            .build()
            .await
            .unwrap();

        let tx = sdk
            .transaction()
            .from_evm_address("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7")
            .to_evm_address("0x1234567890123456789012345678901234567890")
            .amount(1000)
            .build();

        assert!(tx.is_ok());
    }

    #[tokio::test]
    #[ignore] // Requires network connection
    async fn test_execute_transaction() {
        let sdk = ApexSDK::builder()
            .with_evm_endpoint("https://eth.llamarpc.com")
            .build()
            .await
            .unwrap();

        let tx = sdk
            .transaction()
            .from_evm_address("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7")
            .to_evm_address("0x1234567890123456789012345678901234567890")
            .amount(1000)
            .build()
            .unwrap();

        let result = sdk.execute(tx).await;
        assert!(result.is_ok());

        let tx_result = result.unwrap();
        assert!(!tx_result.source_tx_hash.is_empty());
        assert!(tx_result.source_tx_hash.starts_with("0x"));
        assert!(tx_result.destination_tx_hash.is_none());
        assert!(matches!(tx_result.status, TransactionStatus::Pending));
    }

    #[tokio::test]
    #[ignore] // Requires network connection
    async fn test_execute_cross_chain_transaction() {
        let sdk = ApexSDK::builder()
            .with_substrate_endpoint("wss://rpc.polkadot.io")
            .with_evm_endpoint("https://eth.llamarpc.com")
            .build()
            .await
            .unwrap();

        let tx = sdk
            .transaction()
            .from_substrate_account("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY")
            .to_evm_address("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7")
            .amount(1000)
            .build()
            .unwrap();

        assert!(tx.is_cross_chain());

        let result = sdk.execute(tx).await;
        assert!(result.is_ok());

        let tx_result = result.unwrap();
        assert!(!tx_result.source_tx_hash.is_empty());
        assert!(tx_result.source_tx_hash.starts_with("0x"));
        assert!(tx_result.destination_tx_hash.is_some());
        assert!(matches!(tx_result.status, TransactionStatus::Pending));
    }
}
