//! Prometheus metrics exporter with HTTP server
//!
//! This module provides a production-ready Prometheus metrics server with
//! automatic metric registration, scraping endpoint, and integration with
//! the Apex SDK core metrics system.

use crate::{MetricsError, Result};
use apex_sdk_core::metrics::{Metric, MetricType, MetricsCollector};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use prometheus::{
    register_counter_vec_with_registry, register_gauge_vec_with_registry,
    register_histogram_vec_with_registry, CounterVec, Encoder, GaugeVec, HistogramVec, Registry,
    TextEncoder,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{error, info};

/// Prometheus metrics registry wrapper
pub struct PrometheusRegistry {
    registry: Registry,
    transaction_counter: CounterVec,
    transaction_duration: HistogramVec,
    gas_usage: GaugeVec,
    error_counter: CounterVec,
    rpc_duration: HistogramVec,
}

impl PrometheusRegistry {
    /// Create a new Prometheus registry with standard metrics
    pub fn new() -> Result<Self> {
        let registry = Registry::new();

        let transaction_counter = register_counter_vec_with_registry!(
            "apex_sdk_transactions_total",
            "Total number of transactions by chain and status",
            &["chain", "status"],
            registry
        )
        .map_err(|e| MetricsError::PrometheusInit(e.to_string()))?;

        let transaction_duration = register_histogram_vec_with_registry!(
            "apex_sdk_transaction_duration_seconds",
            "Transaction execution duration in seconds",
            &["chain", "operation"],
            vec![0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0, 5.0, 10.0, 30.0],
            registry
        )
        .map_err(|e| MetricsError::PrometheusInit(e.to_string()))?;

        let gas_usage = register_gauge_vec_with_registry!(
            "apex_sdk_gas_used",
            "Gas used for transactions",
            &["chain"],
            registry
        )
        .map_err(|e| MetricsError::PrometheusInit(e.to_string()))?;

        let error_counter = register_counter_vec_with_registry!(
            "apex_sdk_errors_total",
            "Total number of errors by type and operation",
            &["error_type", "operation", "category", "severity"],
            registry
        )
        .map_err(|e| MetricsError::PrometheusInit(e.to_string()))?;

        let rpc_duration = register_histogram_vec_with_registry!(
            "apex_sdk_rpc_duration_seconds",
            "RPC request duration in seconds",
            &["chain", "operation"],
            vec![0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0, 5.0],
            registry
        )
        .map_err(|e| MetricsError::PrometheusInit(e.to_string()))?;

        Ok(Self {
            registry,
            transaction_counter,
            transaction_duration,
            gas_usage,
            error_counter,
            rpc_duration,
        })
    }

    /// Update Prometheus metrics from SDK metrics
    pub fn update_from_sdk_metrics(&self, metrics: &[Metric]) {
        for metric in metrics {
            match metric.metric_type {
                MetricType::TransactionCount | MetricType::TransactionSuccessRate => {
                    if let (Some(chain), Some(status)) =
                        (metric.labels.get("chain"), metric.labels.get("status"))
                    {
                        self.transaction_counter
                            .with_label_values(&[chain, status])
                            .inc_by(metric.value);
                    }
                }

                MetricType::TransactionLatency => {
                    if let Some(chain) = metric.labels.get("chain") {
                        let default_operation = String::from("unknown");
                        let operation =
                            metric.labels.get("operation").unwrap_or(&default_operation);
                        self.transaction_duration
                            .with_label_values(&[chain, operation])
                            .observe(metric.value);
                    }
                }

                MetricType::GasUsage => {
                    if let Some(chain) = metric.labels.get("chain") {
                        self.gas_usage.with_label_values(&[chain]).set(metric.value);
                    }
                }

                MetricType::ErrorRate => {
                    let default_unknown = String::from("unknown");
                    let error_type = metric.labels.get("error_type").unwrap_or(&default_unknown);
                    let operation = metric.labels.get("operation").unwrap_or(&default_unknown);
                    let category = metric.labels.get("category").unwrap_or(&default_unknown);
                    let severity = metric.labels.get("severity").unwrap_or(&default_unknown);

                    self.error_counter
                        .with_label_values(&[error_type, operation, category, severity])
                        .inc_by(metric.value);
                }

                MetricType::ProviderResponseTime => {
                    if let Some(chain) = metric.labels.get("chain") {
                        let default_operation = String::from("unknown");
                        let operation =
                            metric.labels.get("operation").unwrap_or(&default_operation);
                        self.rpc_duration
                            .with_label_values(&[chain, operation])
                            .observe(metric.value);
                    }
                }

                _ => {}
            }
        }
    }

    /// Export all metrics in Prometheus text format
    pub fn export(&self) -> Result<String> {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut buffer = Vec::new();

        encoder
            .encode(&metric_families, &mut buffer)
            .map_err(|e| MetricsError::ExportFailed(e.to_string()))?;

        String::from_utf8(buffer).map_err(|e| MetricsError::ExportFailed(e.to_string()))
    }

    /// Get the underlying Prometheus registry
    pub fn registry(&self) -> &Registry {
        &self.registry
    }
}

impl Default for PrometheusRegistry {
    fn default() -> Self {
        Self::new().expect("Failed to create default Prometheus registry")
    }
}

/// Metrics server state
#[derive(Clone)]
struct ServerState {
    prometheus_registry: Arc<PrometheusRegistry>,
    sdk_metrics: Arc<MetricsCollector>,
}

/// Prometheus metrics HTTP server
pub struct MetricsServer {
    port: u16,
    state: ServerState,
}

impl MetricsServer {
    /// Create a new metrics server
    pub async fn new(port: u16, sdk_metrics: MetricsCollector) -> Result<Self> {
        let prometheus_registry = Arc::new(PrometheusRegistry::new()?);

        Ok(Self {
            port,
            state: ServerState {
                prometheus_registry,
                sdk_metrics: Arc::new(sdk_metrics),
            },
        })
    }

    /// Start the metrics server
    pub async fn start(self) -> Result<()> {
        let app = Router::new()
            .route("/metrics", get(metrics_handler))
            .route("/health", get(health_handler))
            .route("/ready", get(ready_handler))
            .with_state(self.state);

        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        let listener = TcpListener::bind(addr)
            .await
            .map_err(|e| MetricsError::ServerStart(e.to_string()))?;

        info!("Metrics server listening on http://{}", addr);
        info!("Prometheus metrics available at http://{}/metrics", addr);
        info!("Health check available at http://{}/health", addr);

        axum::serve(listener, app)
            .await
            .map_err(|e| MetricsError::ServerStart(e.to_string()))?;

        Ok(())
    }

    /// Start the metrics server in the background
    pub fn start_background(self) -> tokio::task::JoinHandle<Result<()>> {
        tokio::spawn(async move { self.start().await })
    }
}

async fn metrics_handler(State(state): State<ServerState>) -> Response {
    let sdk_metrics = state.sdk_metrics.get_metrics();

    state
        .prometheus_registry
        .update_from_sdk_metrics(&sdk_metrics);

    match state.prometheus_registry.export() {
        Ok(metrics) => (StatusCode::OK, metrics).into_response(),
        Err(e) => {
            error!("Failed to export metrics: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to export metrics: {}", e),
            )
                .into_response()
        }
    }
}

async fn health_handler() -> impl IntoResponse {
    (StatusCode::OK, "healthy")
}

async fn ready_handler() -> impl IntoResponse {
    (StatusCode::OK, "ready")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_prometheus_registry_creation() {
        let registry = PrometheusRegistry::new();
        assert!(registry.is_ok());
    }

    #[tokio::test]
    async fn test_metrics_export() {
        let registry = PrometheusRegistry::new().unwrap();
        let collector = MetricsCollector::new();

        collector.record_transaction_success("ethereum", "0x123");
        collector.record_gas_usage("ethereum", 21000, 21000);

        let metrics = collector.get_metrics();
        registry.update_from_sdk_metrics(&metrics);

        let exported = registry.export().unwrap();
        assert!(exported.contains("apex_sdk_transactions_total"));
        assert!(exported.contains("apex_sdk_gas_used"));
    }

    #[tokio::test]
    async fn test_metrics_server_creation() {
        let collector = MetricsCollector::new();
        let server = MetricsServer::new(0, collector).await;
        assert!(server.is_ok());
    }
}
