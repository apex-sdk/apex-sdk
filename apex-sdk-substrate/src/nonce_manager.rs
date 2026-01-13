//! Substrate Nonce Manager implementation
//!
//! This module provides robust nonce management for Substrate transactions with:
//! - Network nonce queries via System::Account storage
//! - Local nonce caching for pending transactions
//! - Reconciliation between local and network nonces
//! - Thread-safe concurrent access
//! - Reset capability for failed transactions

use crate::{Result, StorageClient};
use apex_sdk_core::{NonceManager as CoreNonceManager, SdkError};
use apex_sdk_types::Address;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{debug, warn};

/// Substrate nonce manager with local caching and network reconciliation
#[derive(Debug, Clone)]
pub struct SubstrateNonceManager {
    storage_client: Arc<StorageClient>,
    local_nonces: Arc<Mutex<HashMap<String, u64>>>,
}

impl SubstrateNonceManager {
    /// Create a new Substrate nonce manager
    pub fn new(storage_client: StorageClient) -> Self {
        Self {
            storage_client: Arc::new(storage_client),
            local_nonces: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Get nonce from network (via System::Account storage query)
    async fn get_network_nonce(&self, address: &str) -> Result<u64> {
        debug!("Fetching network nonce for address: {}", address);

        self.storage_client.get_nonce(address).await.map_err(|e| {
            warn!("Failed to fetch network nonce for {}: {}", address, e);
            e
        })
    }

    /// Update local nonce cache
    fn update_local_nonce(&self, address: &str, nonce: u64) {
        let mut local_nonces = self.local_nonces.lock().unwrap();
        local_nonces.insert(address.to_string(), nonce);
        debug!("Updated local nonce cache for {} to {}", address, nonce);
    }

    /// Get local nonce from cache
    fn get_local_nonce(&self, address: &str) -> Option<u64> {
        let local_nonces = self.local_nonces.lock().unwrap();
        local_nonces.get(address).copied()
    }

    /// Reconcile local and network nonces to determine next nonce to use
    ///
    /// This method:
    /// 1. Fetches current nonce from the network
    /// 2. Compares with locally tracked nonce
    /// 3. Returns the maximum (to handle pending transactions)
    /// 4. Updates local cache for next call
    async fn reconcile_nonce(&self, address: &str) -> Result<u64> {
        let network_nonce = self.get_network_nonce(address).await?;
        let local_nonce = self.get_local_nonce(address);

        let next_nonce = match local_nonce {
            Some(local) => {
                let reconciled = local.max(network_nonce);

                if local > network_nonce {
                    debug!(
                        "Local nonce {} ahead of network nonce {} for address {} (pending transactions)",
                        local, network_nonce, address
                    );
                } else if local < network_nonce {
                    debug!(
                        "Network nonce {} ahead of local nonce {} for address {} (transactions confirmed)",
                        network_nonce, local, address
                    );
                }

                reconciled
            }
            None => {
                debug!(
                    "First nonce request for {}, using network nonce: {}",
                    address, network_nonce
                );
                network_nonce
            }
        };

        self.update_local_nonce(address, next_nonce + 1);

        debug!(
            "Reconciled nonce for {}: network={}, local={:?}, returning={}",
            address, network_nonce, local_nonce, next_nonce
        );

        Ok(next_nonce)
    }

    /// Reset local nonce for an address to match network state
    ///
    /// This is useful after transaction failures or when recovering from errors.
    /// Call this method to resync with the chain's actual nonce.
    pub async fn reset_nonce(&self, address: &str) -> Result<()> {
        debug!("Resetting nonce for address: {}", address);

        let network_nonce = self.get_network_nonce(address).await?;
        self.update_local_nonce(address, network_nonce);

        debug!(
            "Reset local nonce for {} to network value: {}",
            address, network_nonce
        );

        Ok(())
    }

    /// Clear the entire nonce cache
    ///
    /// This forces all subsequent nonce requests to query the network.
    /// Useful for testing or after major state changes.
    pub fn clear_cache(&self) {
        let mut local_nonces = self.local_nonces.lock().unwrap();
        local_nonces.clear();
        debug!("Cleared all local nonce cache");
    }

    /// Get the current local nonce without querying the network
    ///
    /// Returns None if this address hasn't been cached yet.
    pub fn peek_local_nonce(&self, address: &str) -> Option<u64> {
        self.get_local_nonce(address)
    }
}

#[async_trait]
impl CoreNonceManager for SubstrateNonceManager {
    async fn get_next_nonce(&self, address: &Address) -> std::result::Result<u64, SdkError> {
        match address {
            Address::Substrate(addr) => self.reconcile_nonce(addr).await.map_err(SdkError::from),
            _ => Err(SdkError::ConfigError(
                "Invalid address type for Substrate nonce manager".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Metrics, SubstrateAdapter};

    #[tokio::test]
    async fn test_nonce_manager_basic() {
        if std::env::var("SUBSTRATE_INTEGRATION_TESTS").is_ok() {
            let substrate_url = std::env::var("SUBSTRATE_RPC_URL")
                .unwrap_or_else(|_| "ws://localhost:9944".to_string());

            let adapter = SubstrateAdapter::connect(&substrate_url)
                .await
                .expect("Should connect to Substrate node");

            let storage_client = StorageClient::new(adapter.client().clone(), Metrics::new());
            let nonce_manager = SubstrateNonceManager::new(storage_client);

            // Test with a known address (Alice on dev chains)
            let address = Address::substrate("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY");

            // First call should get network nonce
            let nonce1 = nonce_manager.get_next_nonce(&address).await;
            assert!(nonce1.is_ok(), "First nonce query should succeed");

            // Second call should increment local nonce
            let nonce2 = nonce_manager.get_next_nonce(&address).await;
            assert!(nonce2.is_ok(), "Second nonce query should succeed");
            assert_eq!(
                nonce2.unwrap(),
                nonce1.unwrap() + 1,
                "Second nonce should be incremented"
            );
        }
    }

    #[tokio::test]
    async fn test_local_nonce_management() {
        if std::env::var("SUBSTRATE_INTEGRATION_TESTS").is_ok() {
            let substrate_url = std::env::var("SUBSTRATE_RPC_URL")
                .unwrap_or_else(|_| "ws://localhost:9944".to_string());

            let adapter = SubstrateAdapter::connect(&substrate_url)
                .await
                .expect("Should connect to Substrate node");

            let storage_client = StorageClient::new(adapter.client().clone(), Metrics::new());
            let nonce_manager = SubstrateNonceManager::new(storage_client);

            let address_str = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";

            // Test local nonce storage
            nonce_manager.update_local_nonce(address_str, 42);
            assert_eq!(nonce_manager.get_local_nonce(address_str), Some(42));

            // Test peek
            assert_eq!(nonce_manager.peek_local_nonce(address_str), Some(42));

            // Test nonce reset
            let reset_result = nonce_manager.reset_nonce(address_str).await;
            assert!(reset_result.is_ok(), "Reset should succeed");

            // After reset, local nonce should match network
            let local = nonce_manager.get_local_nonce(address_str);
            assert!(local.is_some(), "Local nonce should be set after reset");
        }
    }

    #[tokio::test]
    async fn test_cache_operations() {
        if std::env::var("SUBSTRATE_INTEGRATION_TESTS").is_ok() {
            let substrate_url = std::env::var("SUBSTRATE_RPC_URL")
                .unwrap_or_else(|_| "ws://localhost:9944".to_string());

            let adapter = SubstrateAdapter::connect(&substrate_url)
                .await
                .expect("Should connect to Substrate node");

            let storage_client = StorageClient::new(adapter.client().clone(), Metrics::new());
            let nonce_manager = SubstrateNonceManager::new(storage_client);

            let address_str = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";

            // Set a local nonce
            nonce_manager.update_local_nonce(address_str, 100);
            assert_eq!(nonce_manager.peek_local_nonce(address_str), Some(100));

            // Clear cache
            nonce_manager.clear_cache();
            assert_eq!(
                nonce_manager.peek_local_nonce(address_str),
                None,
                "Cache should be empty after clear"
            );
        }
    }

    #[test]
    fn test_local_nonce_cache_without_network() {
        let substrate_url = "ws://localhost:9944".to_string();
        let metrics = Metrics::new();

        // This test doesn't require actual network connection
        // We just test the local cache operations
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            // Skip if we can't connect, this is just for cache testing
            if let Ok(adapter) = SubstrateAdapter::connect(&substrate_url).await {
                let storage_client = StorageClient::new(adapter.client().clone(), metrics);
                let nonce_manager = SubstrateNonceManager::new(storage_client);

                let address = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";

                // Test that cache starts empty
                assert_eq!(nonce_manager.peek_local_nonce(address), None);

                // Test update
                nonce_manager.update_local_nonce(address, 5);
                assert_eq!(nonce_manager.peek_local_nonce(address), Some(5));

                // Test clear
                nonce_manager.clear_cache();
                assert_eq!(nonce_manager.peek_local_nonce(address), None);
            }
        });
    }
}
