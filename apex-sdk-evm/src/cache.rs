//! Caching layer for EVM queries
//!
//! This module provides:
//! - In-memory LRU cache
//! - Configurable TTL per cache type
//! - Automatic cache invalidation
//! - Cache statistics

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Cache entry with expiration
#[derive(Clone)]
struct CacheEntry<V> {
    value: V,
    inserted_at: Instant,
    ttl: Duration,
}

impl<V> CacheEntry<V> {
    fn new(value: V, ttl: Duration) -> Self {
        Self {
            value,
            inserted_at: Instant::now(),
            ttl,
        }
    }

    fn is_expired(&self) -> bool {
        self.inserted_at.elapsed() > self.ttl
    }
}

/// Cache statistics
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    /// Total number of cache hits
    pub hits: u64,
    /// Total number of cache misses
    pub misses: u64,
    /// Total number of cache sets
    pub sets: u64,
    /// Total number of cache evictions
    pub evictions: u64,
    /// Current number of entries
    pub entries: usize,
}

impl CacheStats {
    /// Calculate hit rate as a percentage
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            (self.hits as f64 / total as f64) * 100.0
        }
    }
}

/// Simple in-memory cache with TTL support
pub struct Cache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    store: Arc<RwLock<HashMap<K, CacheEntry<V>>>>,
    max_size: usize,
    stats: Arc<RwLock<CacheStats>>,
}

impl<K, V> Cache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    /// Create a new cache with a maximum size
    pub fn new(max_size: usize) -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
            max_size,
            stats: Arc::new(RwLock::new(CacheStats::default())),
        }
    }

    /// Get a value from the cache
    pub async fn get(&self, key: &K) -> Option<V> {
        let store = self.store.read().await;
        let mut stats = self.stats.write().await;

        if let Some(entry) = store.get(key) {
            if !entry.is_expired() {
                stats.hits += 1;
                return Some(entry.value.clone());
            }
        }

        stats.misses += 1;
        None
    }

    /// Set a value in the cache with TTL
    pub async fn set(&self, key: K, value: V, ttl: Duration) {
        let mut store = self.store.write().await;
        let mut stats = self.stats.write().await;

        // Check if we need to evict entries
        if store.len() >= self.max_size && !store.contains_key(&key) {
            // Evict expired entries first
            let expired_keys: Vec<K> = store
                .iter()
                .filter(|(_, entry)| entry.is_expired())
                .map(|(k, _)| k.clone())
                .collect();

            for k in expired_keys {
                store.remove(&k);
                stats.evictions += 1;
            }

            // If still at capacity, remove oldest entry
            if store.len() >= self.max_size {
                if let Some(oldest_key) = store
                    .iter()
                    .min_by_key(|(_, entry)| entry.inserted_at)
                    .map(|(k, _)| k.clone())
                {
                    store.remove(&oldest_key);
                    stats.evictions += 1;
                }
            }
        }

        store.insert(key, CacheEntry::new(value, ttl));
        stats.sets += 1;
        stats.entries = store.len();
    }

    /// Remove a value from the cache
    pub async fn remove(&self, key: &K) -> Option<V> {
        let mut store = self.store.write().await;
        let mut stats = self.stats.write().await;

        if let Some(entry) = store.remove(key) {
            stats.entries = store.len();
            Some(entry.value)
        } else {
            None
        }
    }

    /// Clear all entries from the cache
    pub async fn clear(&self) {
        let mut store = self.store.write().await;
        let mut stats = self.stats.write().await;

        store.clear();
        stats.entries = 0;
    }

    /// Get cache statistics
    pub async fn stats(&self) -> CacheStats {
        self.stats.read().await.clone()
    }

    /// Clean up expired entries
    pub async fn cleanup_expired(&self) {
        let mut store = self.store.write().await;
        let mut stats = self.stats.write().await;

        let expired_keys: Vec<K> = store
            .iter()
            .filter(|(_, entry)| entry.is_expired())
            .map(|(k, _)| k.clone())
            .collect();

        let count = expired_keys.len();
        for k in expired_keys {
            store.remove(&k);
        }

        if count > 0 {
            tracing::debug!("Cleaned up {} expired cache entries", count);
            stats.evictions += count as u64;
            stats.entries = store.len();
        }
    }

    /// Get the current number of entries
    pub async fn len(&self) -> usize {
        self.store.read().await.len()
    }

    /// Check if the cache is empty
    pub async fn is_empty(&self) -> bool {
        self.store.read().await.is_empty()
    }
}

/// Cache configuration for different query types
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// TTL for balance queries (default: 30 seconds)
    pub balance_ttl_secs: u64,
    /// TTL for transaction status queries (default: 5 minutes)
    pub transaction_status_ttl_secs: u64,
    /// TTL for block data (default: immutable, 1 hour)
    pub block_data_ttl_secs: u64,
    /// TTL for chain metadata (default: 1 hour)
    pub chain_metadata_ttl_secs: u64,
    /// Maximum cache size per type
    pub max_cache_size: usize,
    /// Cleanup interval in seconds
    pub cleanup_interval_secs: u64,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            balance_ttl_secs: 30,
            transaction_status_ttl_secs: 300,
            block_data_ttl_secs: 3600,
            chain_metadata_ttl_secs: 3600,
            max_cache_size: 10000,
            cleanup_interval_secs: 300,
        }
    }
}

/// Multi-tier cache for different types of EVM data
pub struct EvmCache {
    balance_cache: Cache<String, String>,
    tx_status_cache: Cache<String, String>,
    block_cache: Cache<u64, String>,
    config: CacheConfig,
}

impl EvmCache {
    /// Create a new EVM cache with default configuration
    pub fn new() -> Self {
        Self::with_config(CacheConfig::default())
    }

    /// Create a new EVM cache with custom configuration
    pub fn with_config(config: CacheConfig) -> Self {
        Self {
            balance_cache: Cache::new(config.max_cache_size),
            tx_status_cache: Cache::new(config.max_cache_size),
            block_cache: Cache::new(config.max_cache_size / 10), // Smaller block cache
            config,
        }
    }

    /// Get balance from cache
    pub async fn get_balance(&self, address: &str) -> Option<String> {
        self.balance_cache.get(&address.to_string()).await
    }

    /// Set balance in cache
    pub async fn set_balance(&self, address: &str, balance: String) {
        let ttl = Duration::from_secs(self.config.balance_ttl_secs);
        self.balance_cache
            .set(address.to_string(), balance, ttl)
            .await;
    }

    /// Get transaction status from cache
    pub async fn get_tx_status(&self, tx_hash: &str) -> Option<String> {
        self.tx_status_cache.get(&tx_hash.to_string()).await
    }

    /// Set transaction status in cache
    pub async fn set_tx_status(&self, tx_hash: &str, status: String) {
        let ttl = Duration::from_secs(self.config.transaction_status_ttl_secs);
        self.tx_status_cache
            .set(tx_hash.to_string(), status, ttl)
            .await;
    }

    /// Get block data from cache
    pub async fn get_block(&self, block_number: u64) -> Option<String> {
        self.block_cache.get(&block_number).await
    }

    /// Set block data in cache
    pub async fn set_block(&self, block_number: u64, data: String) {
        let ttl = Duration::from_secs(self.config.block_data_ttl_secs);
        self.block_cache.set(block_number, data, ttl).await;
    }

    /// Clear all caches
    pub async fn clear_all(&self) {
        self.balance_cache.clear().await;
        self.tx_status_cache.clear().await;
        self.block_cache.clear().await;
        tracing::info!("Cleared all caches");
    }

    /// Get statistics for all caches
    pub async fn stats(&self) -> HashMap<String, CacheStats> {
        let mut stats = HashMap::new();
        stats.insert("balance".to_string(), self.balance_cache.stats().await);
        stats.insert("tx_status".to_string(), self.tx_status_cache.stats().await);
        stats.insert("block".to_string(), self.block_cache.stats().await);
        stats
    }

    /// Run cleanup on all caches
    pub async fn cleanup(&self) {
        self.balance_cache.cleanup_expired().await;
        self.tx_status_cache.cleanup_expired().await;
        self.block_cache.cleanup_expired().await;
    }

    /// Start automatic cache cleanup in the background
    pub fn start_cleanup_task(self: Arc<Self>) {
        let cache = self.clone();
        let interval = Duration::from_secs(self.config.cleanup_interval_secs);
        let interval_secs = self.config.cleanup_interval_secs;

        tokio::spawn(async move {
            loop {
                tokio::time::sleep(interval).await;
                cache.cleanup().await;
            }
        });

        tracing::info!(
            "Started cache cleanup task with interval: {}s",
            interval_secs
        );
    }
}

impl Default for EvmCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_basic_operations() {
        let cache: Cache<String, String> = Cache::new(100);

        // Set a value
        cache
            .set(
                "key1".to_string(),
                "value1".to_string(),
                Duration::from_secs(60),
            )
            .await;

        // Get the value
        let value = cache.get(&"key1".to_string()).await;
        assert_eq!(value, Some("value1".to_string()));

        // Check stats
        let stats = cache.stats().await;
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.sets, 1);
    }

    #[tokio::test]
    async fn test_cache_expiration() {
        let cache: Cache<String, String> = Cache::new(100);

        // Set a value with very short TTL
        cache
            .set(
                "key1".to_string(),
                "value1".to_string(),
                Duration::from_millis(100),
            )
            .await;

        // Should be available immediately
        assert!(cache.get(&"key1".to_string()).await.is_some());

        // Wait for expiration
        tokio::time::sleep(Duration::from_millis(200)).await;

        // Should be expired
        assert!(cache.get(&"key1".to_string()).await.is_none());
    }

    #[tokio::test]
    async fn test_cache_eviction() {
        let cache: Cache<String, String> = Cache::new(2);

        // Fill cache to capacity
        cache
            .set(
                "key1".to_string(),
                "value1".to_string(),
                Duration::from_secs(60),
            )
            .await;
        cache
            .set(
                "key2".to_string(),
                "value2".to_string(),
                Duration::from_secs(60),
            )
            .await;

        // Add one more, should evict oldest
        cache
            .set(
                "key3".to_string(),
                "value3".to_string(),
                Duration::from_secs(60),
            )
            .await;

        let stats = cache.stats().await;
        assert!(stats.evictions > 0);
    }

    #[tokio::test]
    async fn test_evm_cache() {
        let cache = EvmCache::new();

        cache
            .set_balance("0x123", "1000000000000000000".to_string())
            .await;
        let balance = cache.get_balance("0x123").await;
        assert_eq!(balance, Some("1000000000000000000".to_string()));

        cache.set_tx_status("0xabc", "confirmed".to_string()).await;
        let status = cache.get_tx_status("0xabc").await;
        assert_eq!(status, Some("confirmed".to_string()));

        // Check stats
        let stats = cache.stats().await;
        assert!(stats.contains_key("balance"));
        assert!(stats.contains_key("tx_status"));
    }

    #[test]
    fn test_cache_stats_hit_rate() {
        let stats = CacheStats {
            hits: 80,
            misses: 20,
            ..Default::default()
        };

        assert_eq!(stats.hit_rate(), 80.0);
    }
}
