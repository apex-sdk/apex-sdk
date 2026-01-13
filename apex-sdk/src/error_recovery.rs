//! Error recovery and retry mechanisms.

use std::time::Duration;
use thiserror::Error;

/// Retry configuration
#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_attempts: usize,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            multiplier: 2.0,
        }
    }
}

impl RetryConfig {
    pub fn builder() -> RetryConfigBuilder {
        RetryConfigBuilder::default()
    }
}

/// Builder for retry configuration
#[derive(Debug, Default)]
pub struct RetryConfigBuilder {
    max_attempts: Option<usize>,
    initial_delay: Option<Duration>,
    max_delay: Option<Duration>,
    multiplier: Option<f64>,
}

impl RetryConfigBuilder {
    pub fn max_attempts(mut self, attempts: usize) -> Self {
        self.max_attempts = Some(attempts);
        self
    }

    pub fn initial_delay(mut self, delay: Duration) -> Self {
        self.initial_delay = Some(delay);
        self
    }

    pub fn max_delay(mut self, delay: Duration) -> Self {
        self.max_delay = Some(delay);
        self
    }

    pub fn multiplier(mut self, multiplier: f64) -> Self {
        self.multiplier = Some(multiplier);
        self
    }

    pub fn build(self) -> RetryConfig {
        let default = RetryConfig::default();
        RetryConfig {
            max_attempts: self.max_attempts.unwrap_or(default.max_attempts),
            initial_delay: self.initial_delay.unwrap_or(default.initial_delay),
            max_delay: self.max_delay.unwrap_or(default.max_delay),
            multiplier: self.multiplier.unwrap_or(default.multiplier),
        }
    }
}

/// Execute a function with retry logic
pub async fn with_retry<F, Fut, T, E>(mut f: F, config: RetryConfig) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
    E: std::fmt::Display,
{
    let mut last_error = None;
    let mut delay = config.initial_delay;

    for attempt in 1..=config.max_attempts {
        match f().await {
            Ok(result) => return Ok(result),
            Err(err) => {
                if !is_retryable(&err) {
                    return Err(err);
                }

                last_error = Some(err);

                if attempt < config.max_attempts {
                    tokio::time::sleep(delay).await;
                    delay = std::cmp::min(
                        Duration::from_millis(
                            (delay.as_millis() as f64 * config.multiplier) as u64,
                        ),
                        config.max_delay,
                    );
                }
            }
        }
    }

    Err(last_error.unwrap())
}

/// Check if an error is retryable
fn is_retryable<E: std::fmt::Display>(_error: &E) -> bool {
    // Simple implementation - in practice this would check error types
    true
}

/// Circuit breaker for preventing cascading failures
#[derive(Debug)]
pub struct CircuitBreaker {
    failure_threshold: usize,
    state: CircuitBreakerState,
    failure_count: usize,
}

#[derive(Debug, Clone, PartialEq)]
enum CircuitBreakerState {
    Closed,
    Open,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: usize, _recovery_timeout: Duration) -> Self {
        Self {
            failure_threshold,
            state: CircuitBreakerState::Closed,
            failure_count: 0,
        }
    }

    pub async fn execute<F, Fut, T, E>(&mut self, f: F) -> Result<T, CircuitBreakerError<E>>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, E>>,
    {
        match self.state {
            CircuitBreakerState::Open => Err(CircuitBreakerError::CircuitOpen),
            _ => match f().await {
                Ok(result) => {
                    self.on_success();
                    Ok(result)
                }
                Err(err) => {
                    self.on_failure();
                    Err(CircuitBreakerError::Execution(err))
                }
            },
        }
    }

    fn on_success(&mut self) {
        self.failure_count = 0;
        self.state = CircuitBreakerState::Closed;
    }

    fn on_failure(&mut self) {
        self.failure_count += 1;
        if self.failure_count >= self.failure_threshold {
            self.state = CircuitBreakerState::Open;
        }
    }

    pub fn is_open(&self) -> bool {
        matches!(self.state, CircuitBreakerState::Open)
    }
}

/// Circuit breaker error
#[derive(Debug, Error)]
pub enum CircuitBreakerError<E> {
    #[error("Circuit breaker is open")]
    CircuitOpen,
    #[error("Execution failed: {0}")]
    Execution(E),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retry_config_builder() {
        let config = RetryConfig::builder()
            .max_attempts(5)
            .initial_delay(Duration::from_millis(50))
            .build();

        assert_eq!(config.max_attempts, 5);
        assert_eq!(config.initial_delay, Duration::from_millis(50));
    }

    #[tokio::test]
    async fn test_with_retry_success() {
        let mut call_count = 0;
        let result = with_retry(
            || {
                call_count += 1;
                async { Ok::<i32, &'static str>(42) }
            },
            RetryConfig::default(),
        )
        .await;

        assert_eq!(result.unwrap(), 42);
        assert_eq!(call_count, 1);
    }

    #[test]
    fn test_circuit_breaker_opens_after_failures() {
        let mut breaker = CircuitBreaker::new(3, Duration::from_secs(10));

        // Simulate failures
        for _ in 0..3 {
            breaker.on_failure();
        }

        assert!(breaker.is_open());
    }
}
