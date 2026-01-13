//! Comprehensive tests for cache module
//!
//! - Testing cache hit/miss scenarios
//! - Testing TTL expiration and cleanup
//! - Testing eviction policies
//! - Testing cache statistics and metrics

use apex_sdk_evm::cache::{Cache, CacheConfig, CacheStats, EvmCache};
use std::sync::Arc;
use std::time::Duration;

// ============================================================================
// Cache Config Tests
// ============================================================================

#[test]
fn test_cache_config_default() {
    let config = CacheConfig::default();

    assert_eq!(config.balance_ttl_secs, 30);
    assert_eq!(config.transaction_status_ttl_secs, 300);
    assert_eq!(config.block_data_ttl_secs, 3600);
    assert_eq!(config.chain_metadata_ttl_secs, 3600);
    assert_eq!(config.max_cache_size, 10000);
    assert_eq!(config.cleanup_interval_secs, 300);
}

#[test]
fn test_cache_config_custom() {
    let config = CacheConfig {
        balance_ttl_secs: 60,
        transaction_status_ttl_secs: 600,
        block_data_ttl_secs: 7200,
        chain_metadata_ttl_secs: 7200,
        max_cache_size: 20000,
        cleanup_interval_secs: 600,
    };

    assert_eq!(config.balance_ttl_secs, 60);
    assert_eq!(config.transaction_status_ttl_secs, 600);
    assert_eq!(config.block_data_ttl_secs, 7200);
    assert_eq!(config.chain_metadata_ttl_secs, 7200);
    assert_eq!(config.max_cache_size, 20000);
    assert_eq!(config.cleanup_interval_secs, 600);
}

#[test]
fn test_cache_config_clone() {
    let config = CacheConfig {
        balance_ttl_secs: 45,
        transaction_status_ttl_secs: 450,
        block_data_ttl_secs: 5400,
        chain_metadata_ttl_secs: 5400,
        max_cache_size: 15000,
        cleanup_interval_secs: 450,
    };

    let cloned = config.clone();

    assert_eq!(cloned.balance_ttl_secs, config.balance_ttl_secs);
    assert_eq!(cloned.max_cache_size, config.max_cache_size);
    assert_eq!(cloned.cleanup_interval_secs, config.cleanup_interval_secs);
}

#[test]
fn test_cache_config_ttl_differences() {
    let config = CacheConfig::default();

    // Balance should have shortest TTL (most volatile)
    assert!(config.balance_ttl_secs < config.transaction_status_ttl_secs);

    // Block data should have longest TTL (immutable)
    assert!(config.block_data_ttl_secs > config.balance_ttl_secs);
    assert!(config.block_data_ttl_secs >= config.transaction_status_ttl_secs);
}

// ============================================================================
// Cache Stats Tests
// ============================================================================

#[test]
fn test_cache_stats_default() {
    let stats = CacheStats::default();

    assert_eq!(stats.hits, 0);
    assert_eq!(stats.misses, 0);
    assert_eq!(stats.sets, 0);
    assert_eq!(stats.evictions, 0);
    assert_eq!(stats.entries, 0);
}

#[test]
fn test_cache_stats_hit_rate_zero_total() {
    let stats = CacheStats::default();
    assert_eq!(stats.hit_rate(), 0.0);
}

#[test]
fn test_cache_stats_hit_rate_perfect() {
    let stats = CacheStats {
        hits: 100,
        misses: 0,
        sets: 100,
        evictions: 0,
        entries: 50,
    };

    assert_eq!(stats.hit_rate(), 100.0);
}

#[test]
fn test_cache_stats_hit_rate_fifty_percent() {
    let stats = CacheStats {
        hits: 50,
        misses: 50,
        sets: 50,
        evictions: 0,
        entries: 50,
    };

    assert_eq!(stats.hit_rate(), 50.0);
}

#[test]
fn test_cache_stats_hit_rate_various() {
    let test_cases = vec![
        (80, 20, 80.0),
        (90, 10, 90.0),
        (25, 75, 25.0),
        (1, 99, 1.0),
        (99, 1, 99.0),
    ];

    for (hits, misses, expected_rate) in test_cases {
        let stats = CacheStats {
            hits,
            misses,
            sets: hits,
            evictions: 0,
            entries: hits as usize,
        };

        assert_eq!(stats.hit_rate(), expected_rate);
    }
}

#[test]
fn test_cache_stats_clone() {
    let stats = CacheStats {
        hits: 100,
        misses: 20,
        sets: 100,
        evictions: 5,
        entries: 95,
    };

    let cloned = stats.clone();

    assert_eq!(cloned.hits, stats.hits);
    assert_eq!(cloned.misses, stats.misses);
    assert_eq!(cloned.evictions, stats.evictions);
}

// ============================================================================
// Basic Cache Operations Tests
// ============================================================================

#[tokio::test]
async fn test_cache_basic_set_get() {
    let cache: Cache<String, String> = Cache::new(100);

    cache
        .set(
            "key1".to_string(),
            "value1".to_string(),
            Duration::from_secs(60),
        )
        .await;

    let value = cache.get(&"key1".to_string()).await;
    assert_eq!(value, Some("value1".to_string()));
}

#[tokio::test]
async fn test_cache_get_nonexistent() {
    let cache: Cache<String, String> = Cache::new(100);

    let value = cache.get(&"nonexistent".to_string()).await;
    assert_eq!(value, None);
}

#[tokio::test]
async fn test_cache_update_existing() {
    let cache: Cache<String, String> = Cache::new(100);

    cache
        .set(
            "key1".to_string(),
            "value1".to_string(),
            Duration::from_secs(60),
        )
        .await;

    cache
        .set(
            "key1".to_string(),
            "value2".to_string(),
            Duration::from_secs(60),
        )
        .await;

    let value = cache.get(&"key1".to_string()).await;
    assert_eq!(value, Some("value2".to_string()));
}

#[tokio::test]
async fn test_cache_remove() {
    let cache: Cache<String, String> = Cache::new(100);

    cache
        .set(
            "key1".to_string(),
            "value1".to_string(),
            Duration::from_secs(60),
        )
        .await;

    let removed = cache.remove(&"key1".to_string()).await;
    assert_eq!(removed, Some("value1".to_string()));

    let value = cache.get(&"key1".to_string()).await;
    assert_eq!(value, None);
}

#[tokio::test]
async fn test_cache_remove_nonexistent() {
    let cache: Cache<String, String> = Cache::new(100);

    let removed = cache.remove(&"nonexistent".to_string()).await;
    assert_eq!(removed, None);
}

#[tokio::test]
async fn test_cache_clear() {
    let cache: Cache<String, String> = Cache::new(100);

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
    cache
        .set(
            "key3".to_string(),
            "value3".to_string(),
            Duration::from_secs(60),
        )
        .await;

    cache.clear().await;

    assert!(cache.is_empty().await);
    assert_eq!(cache.len().await, 0);
}

#[tokio::test]
async fn test_cache_len_and_is_empty() {
    let cache: Cache<String, String> = Cache::new(100);

    assert!(cache.is_empty().await);
    assert_eq!(cache.len().await, 0);

    cache
        .set(
            "key1".to_string(),
            "value1".to_string(),
            Duration::from_secs(60),
        )
        .await;

    assert!(!cache.is_empty().await);
    assert_eq!(cache.len().await, 1);

    cache
        .set(
            "key2".to_string(),
            "value2".to_string(),
            Duration::from_secs(60),
        )
        .await;

    assert_eq!(cache.len().await, 2);
}

// ============================================================================
// TTL and Expiration Tests
// ============================================================================

#[tokio::test]
async fn test_cache_expiration_short_ttl() {
    let cache: Cache<String, String> = Cache::new(100);

    cache
        .set(
            "key1".to_string(),
            "value1".to_string(),
            Duration::from_millis(50),
        )
        .await;

    // Should be available immediately
    let value = cache.get(&"key1".to_string()).await;
    assert!(value.is_some());

    // Wait for expiration
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Should be expired
    let value = cache.get(&"key1".to_string()).await;
    assert!(value.is_none());
}

#[tokio::test]
async fn test_cache_expiration_long_ttl() {
    let cache: Cache<String, String> = Cache::new(100);

    cache
        .set(
            "key1".to_string(),
            "value1".to_string(),
            Duration::from_secs(60),
        )
        .await;

    // Should still be available after short wait
    tokio::time::sleep(Duration::from_millis(50)).await;

    let value = cache.get(&"key1".to_string()).await;
    assert_eq!(value, Some("value1".to_string()));
}

#[tokio::test]
async fn test_cache_different_ttls() {
    let cache: Cache<String, String> = Cache::new(100);

    cache
        .set(
            "short".to_string(),
            "value1".to_string(),
            Duration::from_millis(50),
        )
        .await;
    cache
        .set(
            "long".to_string(),
            "value2".to_string(),
            Duration::from_secs(60),
        )
        .await;

    tokio::time::sleep(Duration::from_millis(100)).await;

    // Short TTL should be expired
    assert!(cache.get(&"short".to_string()).await.is_none());

    // Long TTL should still be valid
    assert!(cache.get(&"long".to_string()).await.is_some());
}

#[tokio::test]
async fn test_cache_cleanup_expired() {
    let cache: Cache<String, String> = Cache::new(100);

    // Add entries with short TTL
    for i in 0..5 {
        cache
            .set(
                format!("key{}", i),
                format!("value{}", i),
                Duration::from_millis(50),
            )
            .await;
    }

    assert_eq!(cache.len().await, 5);

    // Wait for expiration
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Run cleanup
    cache.cleanup_expired().await;

    // All expired entries should be removed
    assert_eq!(cache.len().await, 0);
}

#[tokio::test]
async fn test_cache_cleanup_partial_expired() {
    let cache: Cache<String, String> = Cache::new(100);

    // Add entries with different TTLs
    cache
        .set(
            "expired1".to_string(),
            "v1".to_string(),
            Duration::from_millis(50),
        )
        .await;
    cache
        .set(
            "expired2".to_string(),
            "v2".to_string(),
            Duration::from_millis(50),
        )
        .await;
    cache
        .set(
            "valid1".to_string(),
            "v3".to_string(),
            Duration::from_secs(60),
        )
        .await;
    cache
        .set(
            "valid2".to_string(),
            "v4".to_string(),
            Duration::from_secs(60),
        )
        .await;

    assert_eq!(cache.len().await, 4);

    tokio::time::sleep(Duration::from_millis(100)).await;

    cache.cleanup_expired().await;

    // Only valid entries should remain
    assert_eq!(cache.len().await, 2);
}

// ============================================================================
// Eviction Tests
// ============================================================================

#[tokio::test]
async fn test_cache_eviction_on_capacity() {
    let cache: Cache<String, String> = Cache::new(2);

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

    assert_eq!(cache.len().await, 2);

    // Adding third item should trigger eviction
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
async fn test_cache_eviction_removes_oldest() {
    let cache: Cache<String, String> = Cache::new(2);

    cache
        .set(
            "key1".to_string(),
            "value1".to_string(),
            Duration::from_secs(60),
        )
        .await;
    tokio::time::sleep(Duration::from_millis(10)).await;

    cache
        .set(
            "key2".to_string(),
            "value2".to_string(),
            Duration::from_secs(60),
        )
        .await;
    tokio::time::sleep(Duration::from_millis(10)).await;

    cache
        .set(
            "key3".to_string(),
            "value3".to_string(),
            Duration::from_secs(60),
        )
        .await;

    // key1 (oldest) should be evicted
    assert!(cache.get(&"key1".to_string()).await.is_none());
    assert!(cache.get(&"key2".to_string()).await.is_some());
    assert!(cache.get(&"key3".to_string()).await.is_some());
}

#[tokio::test]
async fn test_cache_eviction_prefers_expired() {
    let cache: Cache<String, String> = Cache::new(2);

    // Add entries
    cache
        .set(
            "expired".to_string(),
            "v1".to_string(),
            Duration::from_millis(50),
        )
        .await;
    cache
        .set(
            "valid".to_string(),
            "v2".to_string(),
            Duration::from_secs(60),
        )
        .await;

    // Wait for first to expire
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Add new entry - should evict expired one
    cache
        .set("new".to_string(), "v3".to_string(), Duration::from_secs(60))
        .await;

    // Expired should be gone
    assert!(cache.get(&"expired".to_string()).await.is_none());

    // Valid entries should remain
    assert!(cache.get(&"valid".to_string()).await.is_some());
    assert!(cache.get(&"new".to_string()).await.is_some());
}

// ============================================================================
// Statistics Tracking Tests
// ============================================================================

#[tokio::test]
async fn test_cache_stats_hits_and_misses() {
    let cache: Cache<String, String> = Cache::new(100);

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

    // Generate hits
    cache.get(&"key1".to_string()).await;
    cache.get(&"key2".to_string()).await;
    cache.get(&"key1".to_string()).await;

    // Generate misses
    cache.get(&"nonexistent1".to_string()).await;
    cache.get(&"nonexistent2".to_string()).await;

    let stats = cache.stats().await;
    assert_eq!(stats.hits, 3);
    assert_eq!(stats.misses, 2);
    assert_eq!(stats.sets, 2);
    assert_eq!(stats.entries, 2);
}

#[tokio::test]
async fn test_cache_stats_sets_counter() {
    let cache: Cache<String, String> = Cache::new(100);

    for i in 0..10 {
        cache
            .set(
                format!("key{}", i),
                format!("value{}", i),
                Duration::from_secs(60),
            )
            .await;
    }

    let stats = cache.stats().await;
    assert_eq!(stats.sets, 10);
    assert_eq!(stats.entries, 10);
}

#[tokio::test]
async fn test_cache_stats_evictions_counter() {
    let cache: Cache<String, String> = Cache::new(2);

    // Add more items than capacity
    for i in 0..5 {
        cache
            .set(
                format!("key{}", i),
                format!("value{}", i),
                Duration::from_secs(60),
            )
            .await;
    }

    let stats = cache.stats().await;
    assert!(stats.evictions >= 3); // At least 3 evictions
}

// ============================================================================
// EVM Cache Tests
// ============================================================================

#[tokio::test]
async fn test_evm_cache_balance() {
    let cache = EvmCache::new();

    cache.set_balance("0x123", "1000000".to_string()).await;

    let balance = cache.get_balance("0x123").await;
    assert_eq!(balance, Some("1000000".to_string()));
}

#[tokio::test]
async fn test_evm_cache_transaction_status() {
    let cache = EvmCache::new();

    cache.set_tx_status("0xabc", "confirmed".to_string()).await;

    let status = cache.get_tx_status("0xabc").await;
    assert_eq!(status, Some("confirmed".to_string()));
}

#[tokio::test]
async fn test_evm_cache_block_data() {
    let cache = EvmCache::new();

    cache.set_block(12345, "block_data".to_string()).await;

    let block = cache.get_block(12345).await;
    assert_eq!(block, Some("block_data".to_string()));
}

#[tokio::test]
async fn test_evm_cache_multiple_caches() {
    let cache = EvmCache::new();

    cache.set_balance("0x123", "1000".to_string()).await;
    cache.set_tx_status("0xabc", "pending".to_string()).await;
    cache.set_block(100, "data".to_string()).await;

    assert!(cache.get_balance("0x123").await.is_some());
    assert!(cache.get_tx_status("0xabc").await.is_some());
    assert!(cache.get_block(100).await.is_some());
}

#[tokio::test]
async fn test_evm_cache_clear_all() {
    let cache = EvmCache::new();

    cache.set_balance("0x123", "1000".to_string()).await;
    cache.set_tx_status("0xabc", "confirmed".to_string()).await;
    cache.set_block(100, "data".to_string()).await;

    cache.clear_all().await;

    assert!(cache.get_balance("0x123").await.is_none());
    assert!(cache.get_tx_status("0xabc").await.is_none());
    assert!(cache.get_block(100).await.is_none());
}

#[tokio::test]
async fn test_evm_cache_stats() {
    let cache = EvmCache::new();

    cache.set_balance("0x123", "1000".to_string()).await;
    cache.set_balance("0x456", "2000".to_string()).await;

    cache.get_balance("0x123").await;
    cache.get_balance("0x456").await;
    cache.get_balance("0x789").await; // Miss

    let stats = cache.stats().await;

    assert!(stats.contains_key("balance"));
    let balance_stats = &stats["balance"];
    assert_eq!(balance_stats.hits, 2);
    assert_eq!(balance_stats.misses, 1);
}

#[tokio::test]
async fn test_evm_cache_with_config() {
    let config = CacheConfig {
        balance_ttl_secs: 10,
        transaction_status_ttl_secs: 20,
        block_data_ttl_secs: 30,
        chain_metadata_ttl_secs: 30,
        max_cache_size: 100,
        cleanup_interval_secs: 60,
    };

    let cache = EvmCache::with_config(config);

    cache.set_balance("0x123", "1000".to_string()).await;

    let balance = cache.get_balance("0x123").await;
    assert!(balance.is_some());
}

#[tokio::test]
async fn test_evm_cache_cleanup() {
    let cache = EvmCache::new();

    cache.set_balance("0x123", "1000".to_string()).await;
    cache.set_tx_status("0xabc", "confirmed".to_string()).await;

    cache.cleanup().await;

    // Cleanup shouldn't remove non-expired entries
    assert!(cache.get_balance("0x123").await.is_some());
    assert!(cache.get_tx_status("0xabc").await.is_some());
}

#[tokio::test]
async fn test_evm_cache_default() {
    let cache1 = EvmCache::new();
    let cache2 = EvmCache::default();

    cache1.set_balance("0x123", "1000".to_string()).await;
    cache2.set_balance("0x456", "2000".to_string()).await;

    assert!(cache1.get_balance("0x123").await.is_some());
    assert!(cache2.get_balance("0x456").await.is_some());
}

// ============================================================================
// Concurrent Access Tests
// ============================================================================

#[tokio::test]
async fn test_cache_concurrent_reads() {
    let cache: Cache<String, String> = Cache::new(100);
    cache
        .set(
            "key1".to_string(),
            "value1".to_string(),
            Duration::from_secs(60),
        )
        .await;

    let cache = Arc::new(cache);
    let mut handles = vec![];

    for _ in 0..10 {
        let cache_clone = cache.clone();
        let handle = tokio::spawn(async move { cache_clone.get(&"key1".to_string()).await });
        handles.push(handle);
    }

    for handle in handles {
        let result = handle.await.unwrap();
        assert_eq!(result, Some("value1".to_string()));
    }
}

#[tokio::test]
async fn test_cache_concurrent_writes() {
    let cache: Cache<String, String> = Cache::new(100);
    let cache = Arc::new(cache);
    let mut handles = vec![];

    for i in 0..10 {
        let cache_clone = cache.clone();
        let handle = tokio::spawn(async move {
            cache_clone
                .set(
                    format!("key{}", i),
                    format!("value{}", i),
                    Duration::from_secs(60),
                )
                .await;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    let len = cache.len().await;
    assert_eq!(len, 10);
}

// ============================================================================
// Edge Cases
// ============================================================================

#[tokio::test]
async fn test_cache_zero_capacity() {
    let cache: Cache<String, String> = Cache::new(0);

    cache
        .set(
            "key1".to_string(),
            "value1".to_string(),
            Duration::from_secs(60),
        )
        .await;

    // With zero capacity, cache might still accept items but not enforce limits
    // This tests that the cache doesn't panic with zero capacity
    let stats = cache.stats().await;
    // Just verify we can get stats without panicking
    assert!(stats.sets > 0);
}

#[tokio::test]
async fn test_cache_very_large_capacity() {
    let cache: Cache<String, String> = Cache::new(1_000_000);

    for i in 0..100 {
        cache
            .set(
                format!("key{}", i),
                format!("value{}", i),
                Duration::from_secs(60),
            )
            .await;
    }

    assert_eq!(cache.len().await, 100);
}

#[tokio::test]
async fn test_cache_zero_ttl() {
    let cache: Cache<String, String> = Cache::new(100);

    cache
        .set(
            "key1".to_string(),
            "value1".to_string(),
            Duration::from_secs(0),
        )
        .await;

    // Zero TTL should immediately expire
    let value = cache.get(&"key1".to_string()).await;
    assert!(value.is_none());
}

#[tokio::test]
async fn test_cache_very_long_ttl() {
    let cache: Cache<String, String> = Cache::new(100);

    cache
        .set(
            "key1".to_string(),
            "value1".to_string(),
            Duration::from_secs(3600 * 24 * 365), // 1 year
        )
        .await;

    let value = cache.get(&"key1".to_string()).await;
    assert!(value.is_some());
}

#[tokio::test]
async fn test_cache_special_characters_in_keys() {
    let cache: Cache<String, String> = Cache::new(100);

    let special_keys = vec![
        "key with spaces",
        "key-with-dashes",
        "key_with_underscores",
        "key.with.dots",
        "key/with/slashes",
        "key@with#special$chars",
    ];

    for key in special_keys {
        cache
            .set(
                key.to_string(),
                "value".to_string(),
                Duration::from_secs(60),
            )
            .await;
        let value = cache.get(&key.to_string()).await;
        assert!(value.is_some());
    }
}

#[tokio::test]
async fn test_cache_empty_string_key() {
    let cache: Cache<String, String> = Cache::new(100);

    cache
        .set("".to_string(), "value".to_string(), Duration::from_secs(60))
        .await;

    let value = cache.get(&"".to_string()).await;
    assert_eq!(value, Some("value".to_string()));
}

#[tokio::test]
async fn test_cache_empty_string_value() {
    let cache: Cache<String, String> = Cache::new(100);

    cache
        .set("key".to_string(), "".to_string(), Duration::from_secs(60))
        .await;

    let value = cache.get(&"key".to_string()).await;
    assert_eq!(value, Some("".to_string()));
}
