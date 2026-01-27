//! # Apex SDK Metrics and Observability
//!
//! Comprehensive metrics collection, distributed tracing, and observability for the Apex SDK.
//!
//! ## Features
//!
//! - **Operation-specific metrics**: Track detailed metrics for every operation type
//! - **Error categorization**: Advanced error taxonomy with automatic categorization
//! - **Performance profiling**: OpenTelemetry-based distributed tracing and span tracking
//! - **Prometheus integration**: HTTP server with Prometheus-compatible metrics endpoint
//! - **Health checks**: Comprehensive health status monitoring
//! - **Metrics aggregation**: Statistical analysis and trend detection
//!
//! ## Example Usage
//!
//! ```rust,no_run
//! use apex_sdk_metrics::{MetricsServer, ObservabilityConfig};
//! use apex_sdk_core::metrics::MetricsCollector;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Initialize observability with default configuration
//! let config = ObservabilityConfig::default()
//!     .with_prometheus_port(9090);
//!
//! // Create metrics collector
//! let collector = MetricsCollector::new();
//!
//! // Start metrics server
//! let server = MetricsServer::new(9090, collector).await?;
//! server.start().await?;
//!
//! // Metrics are now available at http://localhost:9090/metrics
//! # Ok(())
//! # }
//! ```

pub mod aggregation;
pub mod error_categorization;
pub mod health;
pub mod profiling;
pub mod prometheus_exporter;
pub mod telemetry;

use std::sync::Arc;
use thiserror::Error;

pub use aggregation::{AggregatedMetrics, MetricsAggregator, StatisticalSnapshot, TimeWindow};
pub use error_categorization::{
    categorize_error, ErrorCategory, ErrorClassification, ErrorImpact, ErrorSeverity,
};
pub use health::{ComponentHealth, HealthChecker, HealthStatus};
pub use profiling::{OperationSpan, OperationType, PerformanceProfiler, SpanContext};
pub use prometheus_exporter::{MetricsServer, PrometheusRegistry};
pub use telemetry::{init_telemetry, ObservabilityConfig, TelemetryLayer};

/// Errors that can occur in the metrics system
#[derive(Error, Debug)]
pub enum MetricsError {
    #[error("Failed to initialize Prometheus registry: {0}")]
    PrometheusInit(String),

    #[error("Failed to start metrics server: {0}")]
    ServerStart(String),

    #[error("Failed to export metrics: {0}")]
    ExportFailed(String),

    #[error("Telemetry configuration error: {0}")]
    TelemetryConfig(String),

    #[error("Health check failed: {0}")]
    HealthCheck(String),

    #[error("Metrics aggregation error: {0}")]
    Aggregation(String),
}

/// Result type for metrics operations
pub type Result<T> = std::result::Result<T, MetricsError>;

/// Global metrics facade providing unified access to all observability features
#[derive(Clone)]
pub struct ObservabilityFacade {
    profiler: Arc<PerformanceProfiler>,
    health_checker: Arc<HealthChecker>,
    aggregator: Arc<MetricsAggregator>,
}

impl ObservabilityFacade {
    /// Create a new observability facade
    pub fn new() -> Self {
        Self {
            profiler: Arc::new(PerformanceProfiler::new()),
            health_checker: Arc::new(HealthChecker::new()),
            aggregator: Arc::new(MetricsAggregator::new()),
        }
    }

    /// Get the performance profiler
    pub fn profiler(&self) -> Arc<PerformanceProfiler> {
        Arc::clone(&self.profiler)
    }

    /// Get the health checker
    pub fn health_checker(&self) -> Arc<HealthChecker> {
        Arc::clone(&self.health_checker)
    }

    /// Get the metrics aggregator
    pub fn aggregator(&self) -> Arc<MetricsAggregator> {
        Arc::clone(&self.aggregator)
    }
}

impl Default for ObservabilityFacade {
    fn default() -> Self {
        Self::new()
    }
}
