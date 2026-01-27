//! Telemetry and observability configuration
//!
//! This module provides comprehensive telemetry initialization with support for
//! OpenTelemetry, distributed tracing, and structured logging.

use crate::{MetricsError, Result};
use opentelemetry_sdk::trace::SdkTracerProvider;
use serde::{Deserialize, Serialize};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Observability configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityConfig {
    /// Service name for telemetry
    pub service_name: String,
    /// Service version
    pub service_version: String,
    /// Environment (development, staging, production)
    pub environment: String,
    /// Prometheus metrics port
    pub prometheus_port: u16,
    /// Enable OpenTelemetry tracing
    pub enable_tracing: bool,
    /// OpenTelemetry collector endpoint (optional)
    pub otlp_endpoint: Option<String>,
    /// Log level
    pub log_level: String,
    /// Enable JSON logging
    pub json_logs: bool,
    /// Enable console output
    pub console_output: bool,
}

impl ObservabilityConfig {
    /// Create a new observability configuration
    pub fn new(service_name: impl Into<String>) -> Self {
        Self {
            service_name: service_name.into(),
            service_version: env!("CARGO_PKG_VERSION").to_string(),
            environment: "development".to_string(),
            prometheus_port: 9090,
            enable_tracing: true,
            otlp_endpoint: None,
            log_level: "info".to_string(),
            json_logs: false,
            console_output: true,
        }
    }

    /// Set the service version
    pub fn with_service_version(mut self, version: impl Into<String>) -> Self {
        self.service_version = version.into();
        self
    }

    /// Set the environment
    pub fn with_environment(mut self, environment: impl Into<String>) -> Self {
        self.environment = environment.into();
        self
    }

    /// Set the Prometheus port
    pub fn with_prometheus_port(mut self, port: u16) -> Self {
        self.prometheus_port = port;
        self
    }

    /// Enable OpenTelemetry tracing
    pub fn with_tracing(mut self, enabled: bool) -> Self {
        self.enable_tracing = enabled;
        self
    }

    /// Set the OpenTelemetry collector endpoint
    pub fn with_otlp_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.otlp_endpoint = Some(endpoint.into());
        self
    }

    /// Set the log level
    pub fn with_log_level(mut self, level: impl Into<String>) -> Self {
        self.log_level = level.into();
        self
    }

    /// Enable JSON logging
    pub fn with_json_logs(mut self, enabled: bool) -> Self {
        self.json_logs = enabled;
        self
    }

    /// Enable console output
    pub fn with_console_output(mut self, enabled: bool) -> Self {
        self.console_output = enabled;
        self
    }
}

impl Default for ObservabilityConfig {
    fn default() -> Self {
        Self::new("apex-sdk")
    }
}

/// Telemetry layer for tracing integration
pub struct TelemetryLayer {
    tracer_provider: Option<SdkTracerProvider>,
}

impl TelemetryLayer {
    /// Create a new telemetry layer
    pub fn new(config: &ObservabilityConfig) -> Result<Self> {
        let tracer_provider = if config.enable_tracing {
            Some(Self::init_tracer(config)?)
        } else {
            None
        };

        Ok(Self { tracer_provider })
    }

    /// Initialize OpenTelemetry tracer
    fn init_tracer(config: &ObservabilityConfig) -> Result<SdkTracerProvider> {
        use opentelemetry::KeyValue;
        use opentelemetry_sdk::Resource;

        let resource = Resource::builder_empty()
            .with_service_name(config.service_name.clone())
            .with_attributes(vec![
                KeyValue::new("service.version", config.service_version.clone()),
                KeyValue::new("deployment.environment", config.environment.clone()),
            ])
            .build();

        let provider = SdkTracerProvider::builder().with_resource(resource).build();

        Ok(provider)
    }

    /// Get the tracer provider
    pub fn tracer_provider(&self) -> Option<&SdkTracerProvider> {
        self.tracer_provider.as_ref()
    }

    /// Shutdown telemetry
    pub fn shutdown(self) {
        if let Some(provider) = self.tracer_provider {
            if let Err(e) = provider.shutdown() {
                eprintln!("Error shutting down tracer provider: {}", e);
            }
        }
    }
}

/// Initialize telemetry and observability
pub fn init_telemetry(config: ObservabilityConfig) -> Result<TelemetryLayer> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::new(format!("apex_sdk={},apex_sdk_core={},apex_sdk_substrate={},apex_sdk_evm={},apex_sdk_metrics={}",
            config.log_level, config.log_level, config.log_level, config.log_level, config.log_level))
    });

    let telemetry = TelemetryLayer::new(&config)?;

    if config.console_output {
        if config.json_logs {
            let subscriber = tracing_subscriber::registry().with(env_filter).with(
                fmt::layer()
                    .json()
                    .with_current_span(true)
                    .with_span_list(true)
                    .with_target(true),
            );

            subscriber.try_init().map_err(|e| {
                MetricsError::TelemetryConfig(format!("Failed to initialize tracing: {}", e))
            })?;
        } else {
            let subscriber = tracing_subscriber::registry().with(env_filter).with(
                fmt::layer()
                    .with_target(true)
                    .with_thread_ids(true)
                    .with_file(true)
                    .with_line_number(true),
            );

            subscriber.try_init().map_err(|e| {
                MetricsError::TelemetryConfig(format!("Failed to initialize tracing: {}", e))
            })?;
        }
    } else {
        let subscriber = tracing_subscriber::registry().with(env_filter);

        subscriber.try_init().map_err(|e| {
            MetricsError::TelemetryConfig(format!("Failed to initialize tracing: {}", e))
        })?;
    }

    tracing::info!(
        service = %config.service_name,
        version = %config.service_version,
        environment = %config.environment,
        "Telemetry initialized"
    );

    Ok(telemetry)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_observability_config_builder() {
        let config = ObservabilityConfig::new("test-service")
            .with_service_version("1.0.0")
            .with_environment("production")
            .with_prometheus_port(8080)
            .with_log_level("debug")
            .with_json_logs(true);

        assert_eq!(config.service_name, "test-service");
        assert_eq!(config.service_version, "1.0.0");
        assert_eq!(config.environment, "production");
        assert_eq!(config.prometheus_port, 8080);
        assert_eq!(config.log_level, "debug");
        assert!(config.json_logs);
    }

    #[test]
    fn test_default_config() {
        let config = ObservabilityConfig::default();
        assert_eq!(config.service_name, "apex-sdk");
        assert_eq!(config.environment, "development");
        assert_eq!(config.prometheus_port, 9090);
        assert!(config.enable_tracing);
    }

    #[test]
    fn test_telemetry_layer_creation() {
        let config = ObservabilityConfig::default();
        let telemetry = TelemetryLayer::new(&config);
        assert!(telemetry.is_ok());
    }
}
