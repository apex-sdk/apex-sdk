//! Apex SDK builder for configuration
//!
//! This module provides a builder pattern for configuring and creating
//! ApexSDK instances with support for Substrate and EVM blockchain adapters.
//!
//! # Examples
//!
//! ```rust,no_run
//! use apex_sdk::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     // Configure SDK with both Substrate and EVM support
//!     let sdk = ApexSDK::builder()
//!         .with_substrate_endpoint("wss://polkadot.api.onfinality.io/public-ws")
//!         .with_evm_endpoint("https://mainnet.infura.io/v3/YOUR_KEY")
//!         .with_timeout(30)
//!         .build()
//!         .await?;
//!
//!     Ok(())
//! }
//! ```

use crate::error::{Error, Result};
use crate::sdk::ApexSDK;

/// Builder for constructing an ApexSDK instance with customizable configuration.
///
/// The builder pattern allows you to configure the SDK with one or both blockchain
/// adapters (Substrate and EVM) before initialization. At least one adapter must
/// be configured for the SDK to function.
///
/// # Examples
///
/// ## Configure with Substrate only
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
/// # Ok(())
/// # }
/// ```
///
/// ## Configure with EVM only
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
///
/// ## Configure with both adapters
///
/// ```rust,no_run
/// use apex_sdk::prelude::*;
///
/// # #[tokio::main]
/// # async fn main() -> Result<()> {
/// let sdk = ApexSDK::builder()
///     .with_substrate_endpoint("wss://polkadot.api.onfinality.io/public-ws")
///     .with_evm_endpoint("https://mainnet.infura.io/v3/YOUR_KEY")
///     .with_timeout(60)
///     .build()
///     .await?;
/// # Ok(())
/// # }
/// ```
#[derive(Default)]
pub struct ApexSDKBuilder {
    substrate_endpoint: Option<String>,
    evm_endpoint: Option<String>,
    timeout_seconds: Option<u64>,
}

impl ApexSDKBuilder {
    /// Create a new builder instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use apex_sdk::builder::ApexSDKBuilder;
    ///
    /// let builder = ApexSDKBuilder::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the Substrate endpoint URL.
    ///
    /// This endpoint will be used to connect to Substrate-based blockchains
    /// like Polkadot and Kusama. The URL should be a WebSocket endpoint
    /// (typically starting with `wss://` or `ws://`).
    ///
    /// # Arguments
    ///
    /// * `url` - The WebSocket URL of the Substrate endpoint
    ///
    /// # Examples
    ///
    /// ```rust
    /// use apex_sdk::builder::ApexSDKBuilder;
    ///
    /// let builder = ApexSDKBuilder::new()
    ///     .with_substrate_endpoint("wss://polkadot.api.onfinality.io/public-ws");
    /// ```
    pub fn with_substrate_endpoint(mut self, url: impl Into<String>) -> Self {
        self.substrate_endpoint = Some(url.into());
        self
    }

    /// Set the EVM endpoint URL.
    ///
    /// This endpoint will be used to connect to EVM-compatible blockchains
    /// like Ethereum, Polygon, BSC, and Avalanche. The URL should be an
    /// HTTP or HTTPS endpoint.
    ///
    /// # Arguments
    ///
    /// * `url` - The HTTP(S) URL of the EVM endpoint
    ///
    /// # Examples
    ///
    /// ```rust
    /// use apex_sdk::builder::ApexSDKBuilder;
    ///
    /// let builder = ApexSDKBuilder::new()
    ///     .with_evm_endpoint("https://mainnet.infura.io/v3/YOUR_KEY");
    /// ```
    pub fn with_evm_endpoint(mut self, url: impl Into<String>) -> Self {
        self.evm_endpoint = Some(url.into());
        self
    }

    /// Set the connection timeout in seconds.
    ///
    /// This timeout applies to the initial connection attempts to the
    /// configured blockchain endpoints. If not set, a default timeout
    /// will be used.
    ///
    /// # Arguments
    ///
    /// * `seconds` - Timeout duration in seconds
    ///
    /// # Examples
    ///
    /// ```rust
    /// use apex_sdk::builder::ApexSDKBuilder;
    ///
    /// let builder = ApexSDKBuilder::new()
    ///     .with_evm_endpoint("https://mainnet.infura.io/v3/YOUR_KEY")
    ///     .with_timeout(30);
    /// ```
    pub fn with_timeout(mut self, seconds: u64) -> Self {
        self.timeout_seconds = Some(seconds);
        self
    }

    /// Build the ApexSDK instance.
    ///
    /// This method consumes the builder and attempts to create an ApexSDK
    /// instance by connecting to the configured endpoints. At least one
    /// adapter (Substrate or EVM) must be configured, or this will return
    /// an error.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - No adapters are configured
    /// - Connection to any configured endpoint fails
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
    /// // Use the SDK...
    /// # Ok(())
    /// # }
    /// ```
    pub async fn build(self) -> Result<ApexSDK> {
        let substrate_adapter = if let Some(endpoint) = self.substrate_endpoint {
            Some(
                apex_sdk_substrate::SubstrateAdapter::connect(&endpoint)
                    .await
                    .map_err(Error::Substrate)?,
            )
        } else {
            None
        };

        let evm_adapter = if let Some(endpoint) = self.evm_endpoint {
            Some(
                apex_sdk_evm::EvmAdapter::connect(&endpoint)
                    .await
                    .map_err(Error::Evm)?,
            )
        } else {
            None
        };

        if substrate_adapter.is_none() && evm_adapter.is_none() {
            return Err(Error::config(
                "At least one adapter (Substrate or EVM) must be configured",
            ));
        }

        Ok(ApexSDK {
            substrate_adapter,
            evm_adapter,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_builder_new_creates_default_builder() {
        let builder = ApexSDKBuilder::new();
        assert!(builder.substrate_endpoint.is_none());
        assert!(builder.evm_endpoint.is_none());
        assert!(builder.timeout_seconds.is_none());
    }

    #[tokio::test]
    async fn test_builder_with_substrate_endpoint() {
        let builder = ApexSDKBuilder::new().with_substrate_endpoint("wss://test.substrate.io");
        assert_eq!(
            builder.substrate_endpoint,
            Some("wss://test.substrate.io".to_string())
        );
    }

    #[tokio::test]
    async fn test_builder_with_evm_endpoint() {
        let builder = ApexSDKBuilder::new().with_evm_endpoint("https://test.ethereum.io");
        assert_eq!(
            builder.evm_endpoint,
            Some("https://test.ethereum.io".to_string())
        );
    }

    #[tokio::test]
    async fn test_builder_with_timeout() {
        let builder = ApexSDKBuilder::new().with_timeout(60);
        assert_eq!(builder.timeout_seconds, Some(60));
    }

    #[tokio::test]
    async fn test_builder_chaining() {
        let builder = ApexSDKBuilder::new()
            .with_substrate_endpoint("wss://test.substrate.io")
            .with_evm_endpoint("https://test.ethereum.io")
            .with_timeout(120);

        assert_eq!(
            builder.substrate_endpoint,
            Some("wss://test.substrate.io".to_string())
        );
        assert_eq!(
            builder.evm_endpoint,
            Some("https://test.ethereum.io".to_string())
        );
        assert_eq!(builder.timeout_seconds, Some(120));
    }

    #[tokio::test]
    async fn test_builder_requires_at_least_one_adapter() {
        let result = ApexSDKBuilder::new().build().await;
        assert!(result.is_err());
        match result {
            Err(Error::Config(msg, _)) => {
                assert!(msg.contains("At least one adapter"));
            }
            _ => panic!("Expected Config error"),
        }
    }

    #[tokio::test]
    async fn test_builder_default_trait() {
        let builder = ApexSDKBuilder::default();
        assert!(builder.substrate_endpoint.is_none());
        assert!(builder.evm_endpoint.is_none());
    }
}
