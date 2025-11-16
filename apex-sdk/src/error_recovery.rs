//! Error recovery strategies for Apex SDK
//!
//! This module provides automatic retry logic and error recovery strategies
//! for transient failures in blockchain operations.

use crate::error::{Error, Result};
use rand;
use std::time::Duration;
use tokio::time::sleep;

/// Retry configuration for error recovery
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_retries: u32,
    /// Initial backoff duration
    pub initial_backoff: Duration,
    /// Maximum backoff duration
    pub max_backoff: Duration,
    /// Backoff multiplier
    pub multiplier: f64,
    /// Whether to use jitter to avoid thundering herd
    pub use_jitter: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_backoff: Duration::from_millis(1000),
            max_backoff: Duration::from_secs(30),
            multiplier: 2.0,
            use_jitter: true,
        }
    }
}

impl RetryConfig {
    /// Create a new retry configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Set maximum retries
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Set initial backoff
    pub fn with_initial_backoff(mut self, backoff: Duration) -> Self {
        self.initial_backoff = backoff;
        self
    }

    /// Set maximum backoff
    pub fn with_max_backoff(mut self, max_backoff: Duration) -> Self {
        self.max_backoff = max_backoff;
        self
    }

    /// Set backoff multiplier
    pub fn with_multiplier(mut self, multiplier: f64) -> Self {
        self.multiplier = multiplier;
        self
    }

    /// Enable or disable jitter
    pub fn with_jitter(mut self, use_jitter: bool) -> Self {
        self.use_jitter = use_jitter;
        self
    }
}

/// Determines if an error is retryable
pub fn is_retryable(error: &Error) -> bool {
    match error {
        Error::Connection(_, _) => true,
        Error::Transaction(msg, _) => {
            // Retry on timeout or network errors
            msg.contains("timeout")
                || msg.contains("network")
                || msg.contains("connection")
                || msg.contains("unavailable")
        }
        Error::Substrate(_) => false, // Chain-specific errors are typically not retryable
        Error::Evm(_) => false,
        Error::Config(_, _) => false,
        Error::UnsupportedChain(_) => false,
        Error::InvalidAddress(_) => false,
        Error::Serialization(_) => false,
        Error::Other(msg) => msg.contains("temporary") || msg.contains("timeout"),
    }
}

/// Execute an async operation with automatic retry logic
pub async fn with_retry<F, Fut, T>(config: &RetryConfig, operation: F) -> Result<T>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    let mut attempt = 0;
    let mut backoff = config.initial_backoff;

    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(error) => {
                if !is_retryable(&error) || attempt >= config.max_retries {
                    return Err(error);
                }

                attempt += 1;

                // Calculate backoff with jitter
                let delay = if config.use_jitter {
                    let jitter = rand::random::<f64>() * 0.3; // +/- 30% jitter
                    let multiplier = 1.0 + (jitter - 0.15);
                    Duration::from_millis((backoff.as_millis() as f64 * multiplier) as u64)
                } else {
                    backoff
                };

                let delay = delay.min(config.max_backoff);

                tracing::warn!(
                    "Operation failed (attempt {}/{}): {}. Retrying in {:?}",
                    attempt,
                    config.max_retries,
                    error,
                    delay
                );

                sleep(delay).await;

                // Exponential backoff
                backoff =
                    Duration::from_millis((backoff.as_millis() as f64 * config.multiplier) as u64)
                        .min(config.max_backoff);
            }
        }
    }
}

/// Circuit breaker for preventing cascading failures
#[derive(Debug)]
pub struct CircuitBreaker {
    failure_threshold: u32,
    success_threshold: u32,
    timeout: Duration,
    state: CircuitState,
    failure_count: u32,
    success_count: u32,
    last_failure_time: Option<std::time::Instant>,
}

#[derive(Debug, Clone, PartialEq)]
enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

impl CircuitBreaker {
    /// Create a new circuit breaker
    pub fn new(failure_threshold: u32, timeout: Duration) -> Self {
        Self {
            failure_threshold,
            success_threshold: 2,
            timeout,
            state: CircuitState::Closed,
            failure_count: 0,
            success_count: 0,
            last_failure_time: None,
        }
    }

    /// Execute an operation through the circuit breaker
    pub async fn call<F, Fut, T>(&mut self, operation: F) -> Result<T>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        // Check if circuit should transition from Open to HalfOpen
        if self.state == CircuitState::Open {
            if let Some(last_failure) = self.last_failure_time {
                if last_failure.elapsed() > self.timeout {
                    self.state = CircuitState::HalfOpen;
                    self.success_count = 0;
                } else {
                    return Err(Error::connection("Circuit breaker is open"));
                }
            }
        }

        match operation().await {
            Ok(result) => {
                self.on_success();
                Ok(result)
            }
            Err(error) => {
                self.on_failure();
                Err(error)
            }
        }
    }

    fn on_success(&mut self) {
        self.failure_count = 0;

        if self.state == CircuitState::HalfOpen {
            self.success_count += 1;
            if self.success_count >= self.success_threshold {
                self.state = CircuitState::Closed;
            }
        }
    }

    fn on_failure(&mut self) {
        self.failure_count += 1;
        self.last_failure_time = Some(std::time::Instant::now());

        if self.failure_count >= self.failure_threshold {
            self.state = CircuitState::Open;
        }
    }

    /// Check if the circuit is open
    pub fn is_open(&self) -> bool {
        self.state == CircuitState::Open
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retry_config_builder() {
        let config = RetryConfig::new()
            .with_max_retries(5)
            .with_initial_backoff(Duration::from_millis(500))
            .with_multiplier(1.5);

        assert_eq!(config.max_retries, 5);
        assert_eq!(config.initial_backoff, Duration::from_millis(500));
        assert_eq!(config.multiplier, 1.5);
    }

    #[test]
    fn test_is_retryable() {
        assert!(is_retryable(&Error::connection("test")));
        assert!(is_retryable(&Error::transaction("timeout error")));
        assert!(!is_retryable(&Error::InvalidAddress("test".to_string())));
        assert!(!is_retryable(&Error::config("test")));
    }

    #[tokio::test]
    async fn test_with_retry_success() {
        let config = RetryConfig::new().with_max_retries(3);

        let result = with_retry(&config, || async { Ok::<_, Error>(42) }).await;

        assert_eq!(result.unwrap(), 42);
    }

    #[tokio::test]
    async fn test_with_retry_non_retryable_error() {
        let config = RetryConfig::new().with_max_retries(3);

        let result = with_retry(&config, || async {
            Err::<i32, _>(Error::InvalidAddress("test".to_string()))
        })
        .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_circuit_breaker_opens_after_failures() {
        let mut breaker = CircuitBreaker::new(2, Duration::from_secs(1));

        // First failure
        let _ = breaker
            .call(|| async { Err::<(), _>(Error::connection("test")) })
            .await;
        assert!(!breaker.is_open());

        // Second failure - circuit should open
        let _ = breaker
            .call(|| async { Err::<(), _>(Error::connection("test")) })
            .await;
        assert!(breaker.is_open());

        // Subsequent calls should fail immediately
        let result = breaker.call(|| async { Ok(()) }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_circuit_breaker_success_closes() {
        let mut breaker = CircuitBreaker::new(1, Duration::from_millis(100));

        // Trigger failure
        let _ = breaker
            .call(|| async { Err::<(), _>(Error::connection("test")) })
            .await;
        assert!(breaker.is_open());

        // Wait for timeout
        tokio::time::sleep(Duration::from_millis(150)).await;

        // Circuit should be half-open, success should close it
        let _ = breaker.call(|| async { Ok::<_, Error>(()) }).await;
        let _ = breaker.call(|| async { Ok::<_, Error>(()) }).await;

        // Should be closed now
        assert!(!breaker.is_open());
    }
}
