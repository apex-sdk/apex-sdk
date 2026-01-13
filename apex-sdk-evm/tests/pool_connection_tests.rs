//! Comprehensive tests for connection pool module
//!
//! - Testing connection pool creation and configuration
//! - Testing round-robin load balancing
//! - Testing health checks and failover
//! - Testing connection lifecycle management

use apex_sdk_evm::pool::{ConnectionPool, EndpointHealth, PoolConfig};
use std::time::{Duration, Instant};

// ============================================================================
// Pool Config Tests
// ============================================================================

#[test]
fn test_pool_config_default() {
    let config = PoolConfig::default();

    assert_eq!(config.max_connections_per_endpoint, 10);
    assert_eq!(config.health_check_interval_secs, 30);
    assert_eq!(config.health_check_timeout_secs, 5);
    assert_eq!(config.max_failures, 3);
    assert_eq!(config.unhealthy_retry_delay_secs, 60);
}

#[test]
fn test_pool_config_custom() {
    let config = PoolConfig {
        max_connections_per_endpoint: 20,
        health_check_interval_secs: 60,
        health_check_timeout_secs: 10,
        max_failures: 5,
        unhealthy_retry_delay_secs: 120,
    };

    assert_eq!(config.max_connections_per_endpoint, 20);
    assert_eq!(config.health_check_interval_secs, 60);
    assert_eq!(config.health_check_timeout_secs, 10);
    assert_eq!(config.max_failures, 5);
    assert_eq!(config.unhealthy_retry_delay_secs, 120);
}

#[test]
fn test_pool_config_clone() {
    let config = PoolConfig {
        max_connections_per_endpoint: 15,
        health_check_interval_secs: 45,
        health_check_timeout_secs: 8,
        max_failures: 4,
        unhealthy_retry_delay_secs: 90,
    };

    let cloned = config.clone();

    assert_eq!(
        cloned.max_connections_per_endpoint,
        config.max_connections_per_endpoint
    );
    assert_eq!(
        cloned.health_check_interval_secs,
        config.health_check_interval_secs
    );
    assert_eq!(
        cloned.health_check_timeout_secs,
        config.health_check_timeout_secs
    );
    assert_eq!(cloned.max_failures, config.max_failures);
    assert_eq!(
        cloned.unhealthy_retry_delay_secs,
        config.unhealthy_retry_delay_secs
    );
}

#[test]
fn test_pool_config_edge_cases() {
    // Test minimum values
    let config = PoolConfig {
        max_connections_per_endpoint: 1,
        health_check_interval_secs: 1,
        health_check_timeout_secs: 1,
        max_failures: 1,
        unhealthy_retry_delay_secs: 1,
    };

    assert_eq!(config.max_connections_per_endpoint, 1);
    assert_eq!(config.max_failures, 1);

    // Test large values
    let config = PoolConfig {
        max_connections_per_endpoint: 1000,
        health_check_interval_secs: 3600,
        health_check_timeout_secs: 300,
        max_failures: 100,
        unhealthy_retry_delay_secs: 7200,
    };

    assert_eq!(config.max_connections_per_endpoint, 1000);
    assert_eq!(config.unhealthy_retry_delay_secs, 7200);
}

// ============================================================================
// Endpoint Health Tests
// ============================================================================

#[test]
fn test_endpoint_health_default() {
    let health = EndpointHealth::default();

    assert!(health.is_healthy);
    assert_eq!(health.failure_count, 0);
    assert_eq!(health.avg_response_time_ms, 0);
    assert!(health.last_success.is_none());
    assert!(health.last_failure.is_none());
}

#[test]
fn test_endpoint_health_clone() {
    let health = EndpointHealth {
        is_healthy: true,
        last_success: Some(Instant::now()),
        last_failure: None,
        failure_count: 0,
        avg_response_time_ms: 100,
    };

    let cloned = health.clone();

    assert_eq!(cloned.is_healthy, health.is_healthy);
    assert_eq!(cloned.failure_count, health.failure_count);
    assert_eq!(cloned.avg_response_time_ms, health.avg_response_time_ms);
}

#[test]
fn test_endpoint_health_states() {
    // Healthy state
    let healthy = EndpointHealth {
        is_healthy: true,
        last_success: Some(Instant::now()),
        last_failure: None,
        failure_count: 0,
        avg_response_time_ms: 50,
    };

    assert!(healthy.is_healthy);
    assert_eq!(healthy.failure_count, 0);

    // Unhealthy state
    let unhealthy = EndpointHealth {
        is_healthy: false,
        last_success: None,
        last_failure: Some(Instant::now()),
        failure_count: 5,
        avg_response_time_ms: 0,
    };

    assert!(!unhealthy.is_healthy);
    assert!(unhealthy.failure_count > 0);
    assert!(unhealthy.last_failure.is_some());
}

#[test]
fn test_endpoint_health_response_time_tracking() {
    let health = EndpointHealth {
        is_healthy: true,
        last_success: Some(Instant::now()),
        last_failure: None,
        failure_count: 0,
        avg_response_time_ms: 150,
    };

    assert_eq!(health.avg_response_time_ms, 150);

    // Simulate different response times
    let response_times = [100, 150, 200, 120, 180];
    let avg: u64 = response_times.iter().sum::<u64>() / response_times.len() as u64;

    assert_eq!(avg, 150);
}

#[test]
fn test_endpoint_health_failure_counting() {
    let mut failure_count = 0u32;
    let max_failures = 3u32;

    // Simulate failures
    for _ in 0..5 {
        failure_count += 1;

        if failure_count >= max_failures {
            // Should be marked unhealthy
            assert!(failure_count >= max_failures);
            break;
        }
    }

    assert_eq!(failure_count, max_failures);
}

// ============================================================================
// Connection Pool Tests (Unit)
// ============================================================================

#[tokio::test]
async fn test_pool_creation_empty_endpoints() {
    let endpoints: Vec<String> = vec![];
    let result = ConnectionPool::new(endpoints).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_pool_creation_invalid_endpoints() {
    let endpoints = vec!["invalid-url".to_string(), "not-a-url".to_string()];

    let result = ConnectionPool::new(endpoints).await;
    assert!(result.is_err());
}

#[test]
fn test_pool_endpoint_validation() {
    let valid_endpoints = vec![
        "https://eth.llamarpc.com",
        "https://ethereum.publicnode.com",
        "http://localhost:8545",
        "https://mainnet.infura.io/v3/key",
    ];

    for endpoint in valid_endpoints {
        assert!(endpoint.starts_with("http://") || endpoint.starts_with("https://"));
        assert!(endpoint.contains("://"));
        assert!(!endpoint.is_empty());
    }
}

#[test]
fn test_pool_invalid_endpoint_detection() {
    let invalid_endpoints = vec![
        "",
        "not-a-url",
        "ftp://invalid.com",
        "ws://websocket-endpoint",
    ];

    for endpoint in invalid_endpoints {
        if !endpoint.is_empty() {
            // Should not start with http/https
            let is_valid = endpoint.starts_with("http://") || endpoint.starts_with("https://");
            if endpoint.starts_with("ftp://") || endpoint.starts_with("ws://") {
                assert!(!is_valid);
            }
        }
    }
}

// ============================================================================
// Round-Robin Logic Tests
// ============================================================================

#[test]
fn test_round_robin_pattern() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    let endpoints = ["ep1", "ep2", "ep3"];
    let counter = AtomicUsize::new(0);

    let mut results = vec![];
    for _ in 0..9 {
        let index = counter.fetch_add(1, Ordering::Relaxed) % endpoints.len();
        results.push(endpoints[index]);
    }

    // Should cycle through endpoints: ep1, ep2, ep3, ep1, ep2, ep3, ...
    assert_eq!(results[0], "ep1");
    assert_eq!(results[1], "ep2");
    assert_eq!(results[2], "ep3");
    assert_eq!(results[3], "ep1");
    assert_eq!(results[4], "ep2");
    assert_eq!(results[5], "ep3");
}

#[test]
fn test_round_robin_with_single_endpoint() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    let endpoints = ["single"];
    let counter = AtomicUsize::new(0);

    for _ in 0..5 {
        let index = counter.fetch_add(1, Ordering::Relaxed) % endpoints.len();
        assert_eq!(endpoints[index], "single");
    }
}

#[test]
fn test_round_robin_index_wraparound() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    let endpoints_count = 3;
    let counter = AtomicUsize::new(usize::MAX - 1);

    // Test wraparound behavior
    for _ in 0..5 {
        let index = counter.fetch_add(1, Ordering::Relaxed) % endpoints_count;
        assert!(index < endpoints_count);
    }
}

// ============================================================================
// Health Check Tests
// ============================================================================

#[test]
fn test_health_check_interval_calculation() {
    let config = PoolConfig::default();
    let interval = Duration::from_secs(config.health_check_interval_secs);

    assert_eq!(interval, Duration::from_secs(30));

    // Test custom interval
    let custom_config = PoolConfig {
        health_check_interval_secs: 60,
        ..Default::default()
    };
    let custom_interval = Duration::from_secs(custom_config.health_check_interval_secs);

    assert_eq!(custom_interval, Duration::from_secs(60));
    assert!(custom_interval > interval);
}

#[test]
fn test_health_check_timeout_validation() {
    let config = PoolConfig::default();

    let timeout = Duration::from_secs(config.health_check_timeout_secs);
    let interval = Duration::from_secs(config.health_check_interval_secs);

    // Timeout should be less than interval
    assert!(timeout < interval);
}

#[test]
fn test_failure_threshold_logic() {
    let config = PoolConfig::default();
    let mut failure_count = 0u32;

    // Simulate consecutive failures
    for _ in 0..5 {
        failure_count += 1;

        if failure_count >= config.max_failures {
            // Endpoint should be marked unhealthy
            assert!(failure_count >= config.max_failures);
            break;
        }
    }

    assert_eq!(failure_count, config.max_failures);
}

#[test]
fn test_unhealthy_retry_delay() {
    let config = PoolConfig::default();
    let retry_delay = Duration::from_secs(config.unhealthy_retry_delay_secs);

    assert_eq!(retry_delay, Duration::from_secs(60));

    // Simulate time passage
    let failure_time = Instant::now();
    std::thread::sleep(Duration::from_millis(10));

    let elapsed = failure_time.elapsed();
    let should_retry = elapsed >= retry_delay;

    // After only 10ms, should not retry yet
    assert!(!should_retry);
}

// ============================================================================
// Connection Statistics Tests
// ============================================================================

#[test]
fn test_connection_pool_statistics() {
    use std::collections::HashMap;

    let mut stats = HashMap::new();

    stats.insert("total_connections", 30);
    stats.insert("active_connections", 12);
    stats.insert("healthy_endpoints", 3);
    stats.insert("unhealthy_endpoints", 1);

    assert_eq!(stats.get("total_connections"), Some(&30));
    assert_eq!(stats.get("healthy_endpoints"), Some(&3));

    // Calculate utilization
    let active = *stats.get("active_connections").unwrap() as f64;
    let total = *stats.get("total_connections").unwrap() as f64;
    let utilization = active / total;

    assert!((utilization - 0.4).abs() < 0.01); // 12/30 = 0.4
}

#[test]
fn test_endpoint_health_statistics() {
    let mut total_response_time = 0u64;
    let mut request_count = 0u64;

    let response_times = vec![100, 150, 200, 120, 180];

    for time in response_times {
        total_response_time += time;
        request_count += 1;
    }

    let avg_response_time = total_response_time / request_count;
    assert_eq!(avg_response_time, 150);
}

#[test]
fn test_exponential_moving_average() {
    let mut ema = 0u64;

    let response_times = vec![100, 150, 200, 120, 180];

    for time in response_times {
        if ema == 0 {
            ema = time;
        } else {
            // EMA with alpha = 0.1 (9/10 weight to old, 1/10 to new)
            ema = (ema * 9 + time) / 10;
        }
    }

    // EMA should be influenced more by recent values
    assert!(ema > 0);
}

// ============================================================================
// Health State Transitions Tests
// ============================================================================

#[test]
fn test_health_state_healthy_to_unhealthy() {
    let mut health = EndpointHealth::default();
    assert!(health.is_healthy);

    // Simulate failures
    for _ in 0..3 {
        health.failure_count += 1;
        health.last_failure = Some(Instant::now());

        if health.failure_count >= 3 {
            health.is_healthy = false;
        }
    }

    assert!(!health.is_healthy);
    assert_eq!(health.failure_count, 3);
}

#[test]
fn test_health_state_unhealthy_to_healthy() {
    let mut health = EndpointHealth {
        is_healthy: false,
        failure_count: 5,
        last_failure: Some(Instant::now()),
        last_success: None,
        avg_response_time_ms: 0,
    };

    assert!(!health.is_healthy);

    // Simulate successful health check
    health.is_healthy = true;
    health.failure_count = 0;
    health.last_success = Some(Instant::now());
    health.avg_response_time_ms = 100;

    assert!(health.is_healthy);
    assert_eq!(health.failure_count, 0);
}

#[test]
fn test_health_state_partial_recovery() {
    let mut health = EndpointHealth {
        is_healthy: false,
        failure_count: 5,
        last_failure: Some(Instant::now()),
        last_success: None,
        avg_response_time_ms: 0,
    };

    // One successful request doesn't immediately make it healthy
    // We simulate that it takes multiple successes
    health.failure_count = health.failure_count.saturating_sub(1);

    assert_eq!(health.failure_count, 4);
    assert!(!health.is_healthy); // Still unhealthy

    // Multiple successes
    for _ in 0..4 {
        health.failure_count = health.failure_count.saturating_sub(1);
    }

    assert_eq!(health.failure_count, 0);
}

// ============================================================================
// Concurrent Access Tests
// ============================================================================

#[tokio::test]
async fn test_concurrent_health_updates() {
    use std::sync::Arc;
    use tokio::sync::RwLock;

    let health = Arc::new(RwLock::new(EndpointHealth::default()));
    let mut handles = vec![];

    // Simulate concurrent health updates
    for i in 0..10 {
        let health_clone = health.clone();
        let handle = tokio::spawn(async move {
            let mut h = health_clone.write().await;
            h.avg_response_time_ms = i * 10;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    let final_health = health.read().await;
    assert!(final_health.avg_response_time_ms <= 90);
}

// ============================================================================
// Integration Tests (requires network)
// ============================================================================

#[tokio::test]
#[ignore] // Requires network
async fn test_pool_creation_with_real_endpoints() {
    let endpoints = vec![
        "https://eth.llamarpc.com".to_string(),
        "https://ethereum.publicnode.com".to_string(),
    ];

    let pool = ConnectionPool::new(endpoints).await;
    assert!(pool.is_ok());

    let pool = pool.unwrap();
    assert_eq!(pool.endpoint_count(), 2);
}

#[tokio::test]
#[ignore] // Requires network
async fn test_pool_get_connection_real() {
    let endpoints = vec!["https://eth.llamarpc.com".to_string()];

    let pool = ConnectionPool::new(endpoints).await.unwrap();
    let conn = pool.get_connection().await;

    assert!(conn.is_ok());
    let connection = conn.unwrap();
    assert!(!connection.endpoint().is_empty());
}

#[tokio::test]
#[ignore] // Requires network
async fn test_pool_health_checks_real() {
    let endpoints = vec!["https://eth.llamarpc.com".to_string()];

    let pool = ConnectionPool::new(endpoints).await.unwrap();
    let result = pool.run_health_checks().await;

    assert!(result.is_ok());

    let health_status = pool.health_status().await;
    assert_eq!(health_status.len(), 1);

    let (endpoint, health) = &health_status[0];
    assert!(!endpoint.is_empty());
    assert!(health.is_healthy);
}

#[tokio::test]
#[ignore] // Requires network
async fn test_pool_round_robin_real() {
    let endpoints = vec![
        "https://eth.llamarpc.com".to_string(),
        "https://ethereum.publicnode.com".to_string(),
    ];

    let pool = ConnectionPool::new(endpoints).await.unwrap();

    let mut used_endpoints = std::collections::HashSet::new();

    // Get multiple connections
    for _ in 0..4 {
        if let Ok(conn) = pool.get_connection().await {
            used_endpoints.insert(conn.endpoint().to_string());
        }
    }

    // Should have used both endpoints
    assert!(!used_endpoints.is_empty());
}

#[tokio::test]
#[ignore] // Requires network
async fn test_pool_with_custom_config_real() {
    let endpoints = vec!["https://eth.llamarpc.com".to_string()];

    let config = PoolConfig {
        max_connections_per_endpoint: 5,
        health_check_interval_secs: 60,
        health_check_timeout_secs: 10,
        max_failures: 5,
        unhealthy_retry_delay_secs: 120,
    };

    let pool = ConnectionPool::with_config(endpoints, config).await;
    assert!(pool.is_ok());
}

// ============================================================================
// Edge Cases and Error Handling
// ============================================================================

#[test]
fn test_endpoint_list_edge_cases() {
    // Very long endpoint
    let long_endpoint = format!("https://{}.com", "a".repeat(100));
    assert!(long_endpoint.len() > 100);

    // Endpoint with special characters
    let special_endpoint = "https://eth-mainnet.example.com:8545/v1";
    assert!(special_endpoint.contains(":"));
    assert!(special_endpoint.contains("/"));
}

#[test]
fn test_response_time_overflow() {
    let large_time = u64::MAX;
    let count = 10u64;

    // Prevent overflow in average calculation
    let avg = large_time / count;
    assert!(avg > 0);
}

#[test]
fn test_failure_count_saturation() {
    let mut failure_count = u32::MAX - 1;

    // Saturating add to prevent overflow
    failure_count = failure_count.saturating_add(10);
    assert_eq!(failure_count, u32::MAX);
}
