//! Builder pattern implementation for creating Apex SDK instances.

use crate::{
    error::{Error, Result},
    sdk::ApexSDK,
};
use std::time::Duration;

#[cfg(feature = "substrate")]
use apex_sdk_substrate::SubstrateAdapter;

#[cfg(feature = "evm")]
use apex_sdk_evm::EvmAdapter;

/// Builder for creating an ApexSDK instance with configuration.
///
/// # Example
///
/// ```rust,no_run
/// use apex_sdk::prelude::*;
/// use std::time::Duration;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let sdk = ApexSDKBuilder::new()
///         .with_substrate_endpoint("wss://polkadot.api.onfinality.io/public-ws")
///         .with_evm_endpoint("https://mainnet.infura.io/v3/YOUR_KEY")
///         .with_timeout(Duration::from_secs(60))
///         .build()
///         .await?;
///     
///     Ok(())
/// }
/// ```
#[derive(Debug, Default)]
pub struct ApexSDKBuilder {
    #[cfg(feature = "substrate")]
    substrate_endpoint: Option<String>,

    #[cfg(feature = "evm")]
    evm_endpoint: Option<String>,

    timeout: Option<Duration>,
}

impl ApexSDKBuilder {
    /// Create a new SDK builder.
    ///
    /// # Example
    ///
    /// ```rust
    /// use apex_sdk::ApexSDKBuilder;
    ///
    /// let builder = ApexSDKBuilder::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Configure the Substrate WebSocket endpoint.
    ///
    /// # Example
    ///
    /// ```rust
    /// use apex_sdk::ApexSDKBuilder;
    ///
    /// let builder = ApexSDKBuilder::new()
    ///     .with_substrate_endpoint("wss://polkadot.api.onfinality.io/public-ws");
    /// ```
    #[cfg(feature = "substrate")]
    pub fn with_substrate_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.substrate_endpoint = Some(endpoint.into());
        self
    }

    /// Configure the EVM HTTP/WebSocket endpoint.
    ///
    /// # Example
    ///
    /// ```rust
    /// use apex_sdk::ApexSDKBuilder;
    ///
    /// let builder = ApexSDKBuilder::new()
    ///     .with_evm_endpoint("https://mainnet.infura.io/v3/YOUR_KEY");
    /// ```
    #[cfg(feature = "evm")]
    pub fn with_evm_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.evm_endpoint = Some(endpoint.into());
        self
    }

    /// Set the timeout for operations.
    ///
    /// # Example
    ///
    /// ```rust
    /// use apex_sdk::ApexSDKBuilder;
    /// use std::time::Duration;
    ///
    /// let builder = ApexSDKBuilder::new()
    ///     .with_timeout(Duration::from_secs(60));
    /// ```
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Build the ApexSDK instance.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - No adapters are configured
    /// - Connection to any configured endpoint fails
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use apex_sdk::ApexSDKBuilder;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let sdk = ApexSDKBuilder::new()
    ///         .with_substrate_endpoint("wss://polkadot.api.onfinality.io/public-ws")
    ///         .build()
    ///         .await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn build(self) -> Result<ApexSDK> {
        let timeout = self.timeout.unwrap_or(Duration::from_secs(30));

        #[cfg(feature = "substrate")]
        let substrate_adapter = if let Some(endpoint) = self.substrate_endpoint {
            Some(
                SubstrateAdapter::connect(&endpoint)
                    .await
                    .map_err(|e| Error::Connection(e.to_string()))?,
            )
        } else {
            None
        };

        #[cfg(feature = "evm")]
        let evm_adapter = if let Some(endpoint) = self.evm_endpoint {
            Some(
                EvmAdapter::connect(&endpoint)
                    .await
                    .map_err(|e| Error::Connection(e.to_string()))?,
            )
        } else {
            None
        };

        // Ensure at least one adapter is configured
        #[cfg(all(feature = "substrate", feature = "evm"))]
        {
            if substrate_adapter.is_none() && evm_adapter.is_none() {
                return Err(Error::Config(
                    "At least one blockchain adapter must be configured".to_string(),
                ));
            }
        }

        #[cfg(all(feature = "substrate", not(feature = "evm")))]
        {
            if substrate_adapter.is_none() {
                return Err(Error::Config(
                    "Substrate adapter must be configured when EVM feature is disabled".to_string(),
                ));
            }
        }

        #[cfg(all(not(feature = "substrate"), feature = "evm"))]
        {
            if evm_adapter.is_none() {
                return Err(Error::Config(
                    "EVM adapter must be configured when Substrate feature is disabled".to_string(),
                ));
            }
        }

        ApexSDK::new(
            #[cfg(feature = "substrate")]
            substrate_adapter,
            #[cfg(feature = "evm")]
            evm_adapter,
            timeout,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_new_creates_default_builder() {
        let builder = ApexSDKBuilder::new();
        assert!(builder.timeout.is_none());

        #[cfg(feature = "substrate")]
        assert!(builder.substrate_endpoint.is_none());

        #[cfg(feature = "evm")]
        assert!(builder.evm_endpoint.is_none());
    }

    #[test]
    fn test_builder_default_trait() {
        let builder = ApexSDKBuilder::default();
        assert!(builder.timeout.is_none());
    }

    #[test]
    fn test_builder_chaining() {
        let builder = ApexSDKBuilder::new().with_timeout(Duration::from_secs(60));

        assert_eq!(builder.timeout, Some(Duration::from_secs(60)));
    }

    #[cfg(feature = "substrate")]
    #[test]
    fn test_builder_with_substrate_endpoint() {
        let endpoint = "wss://polkadot.api.onfinality.io/public-ws";
        let builder = ApexSDKBuilder::new().with_substrate_endpoint(endpoint);

        assert_eq!(builder.substrate_endpoint, Some(endpoint.to_string()));
    }

    #[cfg(feature = "evm")]
    #[test]
    fn test_builder_with_evm_endpoint() {
        let endpoint = "https://mainnet.infura.io/v3/YOUR_KEY";
        let builder = ApexSDKBuilder::new().with_evm_endpoint(endpoint);

        assert_eq!(builder.evm_endpoint, Some(endpoint.to_string()));
    }

    #[test]
    fn test_builder_with_timeout() {
        let timeout = Duration::from_secs(45);
        let builder = ApexSDKBuilder::new().with_timeout(timeout);

        assert_eq!(builder.timeout, Some(timeout));
    }

    #[tokio::test]
    async fn test_builder_requires_at_least_one_adapter() {
        let result = ApexSDKBuilder::new().build().await;

        assert!(result.is_err());
        if let Err(Error::Config(msg)) = result {
            assert!(msg.contains("adapter"));
        }
    }
}
