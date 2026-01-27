# Apex SDK Metrics

Comprehensive metrics, observability, and monitoring infrastructure for the Apex SDK.

## Features

### Core Capabilities

- **Operation-Specific Metrics**: Detailed tracking for every operation type (transactions, RPC calls, storage queries, etc.)
- **Advanced Error Categorization**: Automatic error classification by category, severity, and impact
- **Performance Profiling**: OpenTelemetry-based distributed tracing with span tracking
- **Prometheus Integration**: Production-ready HTTP server with Prometheus-compatible metrics endpoint
- **Health Monitoring**: Comprehensive component health checks and system resource monitoring
- **Statistical Aggregation**: Advanced metrics aggregation with percentiles, trends, and rate calculations

### Metrics Categories

1. **Transaction Metrics**
   - Success/failure rates by chain
   - Transaction latency (P50, P95, P99)
   - Gas usage and efficiency
   - Confirmation times

2. **RPC Metrics**
   - Provider response times by operation
   - Request rates and throughput
   - Error rates by provider

3. **Error Metrics**
   - Categorized by type, severity, and impact
   - Automatic remediation suggestions
   - Retryability classification

4. **System Metrics**
   - CPU and memory usage
   - Component health status
   - Uptime tracking

## Quick Start

### Basic Usage

```rust
use apex_sdk_metrics::{MetricsServer, ObservabilityConfig, init_telemetry};
use apex_sdk_core::metrics::MetricsCollector;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize telemetry
    let config = ObservabilityConfig::new("my-blockchain-app")
        .with_environment("production")
        .with_prometheus_port(9090)
        .with_log_level("info");

    let _telemetry = init_telemetry(config.clone())?;

    // Create metrics collector
    let collector = MetricsCollector::new();

    // Record some metrics
    collector.record_transaction_success("ethereum", "0x123...");
    collector.record_gas_usage("ethereum", 21000, 21000);

    // Start Prometheus metrics server
    let server = MetricsServer::new(9090, collector).await?;
    server.start().await?;

    // Metrics available at http://localhost:9090/metrics
    // Health check at http://localhost:9090/health

    Ok(())
}
```

### Error Categorization

```rust
use apex_sdk_metrics::categorize_error;

let classification = categorize_error("connection timeout", None);

println!("Category: {:?}", classification.category);
println!("Severity: {:?}", classification.severity);
println!("Is Retryable: {}", classification.is_retryable());
println!("Remediation: {:?}", classification.remediation);
```

### Performance Profiling

```rust
use apex_sdk_metrics::{PerformanceProfiler, OperationType};
use std::time::Duration;

let profiler = PerformanceProfiler::new();

// Profile a transaction submission
{
    let mut span = profiler.start_span(OperationType::TransactionSubmit);
    span.set_attribute("chain", "ethereum");
    span.set_attribute("tx_hash", "0x123...");

    // ... perform transaction ...

    span.success();  // Automatically records duration
}

// Get statistics
let stats = profiler.operation_stats(OperationType::TransactionSubmit);
println!("Success Rate: {:.2}%", stats.success_rate());
println!("P95 Latency: {:.3}s", stats.p95_duration_secs);
```

### Health Monitoring

```rust
use apex_sdk_metrics::{HealthChecker, ComponentHealth, HealthStatus};

let health_checker = HealthChecker::new();

// Update component health
health_checker.update_component(
    ComponentHealth::new("rpc-provider", HealthStatus::Healthy)
        .with_message("Connected to Ethereum mainnet")
        .with_metadata("endpoint", "https://eth.llamarpc.com")
);

// Get health summary
let summary = health_checker.health_summary();
println!("Overall Status: {:?}", summary.status);
println!("CPU Usage: {:.1}%", summary.resources.cpu_usage_percent);
println!("Memory Usage: {:.1}%", summary.resources.memory_usage_percent);
```

### Metrics Aggregation

```rust
use apex_sdk_metrics::{MetricsAggregator, TimeWindow};
use apex_sdk_core::metrics::MetricsCollector;

let collector = MetricsCollector::new();
let aggregator = MetricsAggregator::with_time_window(TimeWindow::FiveMinutes);

// ... record metrics ...

// Aggregate metrics
let aggregated = aggregator.aggregate(&collector.get_metrics());

// Get success rate
let success_rate = aggregator.calculate_success_rate(
    &collector.get_metrics(),
    Some("ethereum")
);
println!("Success Rate: {:.2}%", success_rate);

// Access statistical snapshots
for (name, snapshot) in &aggregated.overall {
    println!("{}: mean={:.3}, p95={:.3}, p99={:.3}",
        name, snapshot.mean, snapshot.p95, snapshot.p99);
}
```

## Grafana Dashboards

Pre-built Grafana dashboard templates are available in the `dashboards/` directory:

- **apex-sdk-overview.json**: High-level metrics overview with success rates, latency, and error rates
- **apex-sdk-performance.json**: Deep-dive into performance metrics with heatmaps and percentiles

### Importing Dashboards

1. Open Grafana
2. Go to Dashboards → Import
3. Upload the JSON file from `dashboards/`
4. Select your Prometheus data source
5. Click Import

## Prometheus Metrics

The following metrics are exported at the `/metrics` endpoint:

| Metric | Type | Description | Labels |
|--------|------|-------------|--------|
| `apex_sdk_transactions_total` | Counter | Total transactions by status | `chain`, `status` |
| `apex_sdk_transaction_duration_seconds` | Histogram | Transaction execution duration | `chain`, `operation` |
| `apex_sdk_gas_used` | Gauge | Gas used for transactions | `chain` |
| `apex_sdk_errors_total` | Counter | Total errors by type | `error_type`, `operation`, `category`, `severity` |
| `apex_sdk_rpc_duration_seconds` | Histogram | RPC request duration | `chain`, `operation` |

## Configuration

### ObservabilityConfig Options

```rust
let config = ObservabilityConfig::new("service-name")
    .with_service_version("1.0.0")
    .with_environment("production")  // or "development", "staging"
    .with_prometheus_port(9090)
    .with_tracing(true)              // Enable OpenTelemetry tracing
    .with_log_level("info")          // trace, debug, info, warn, error
    .with_json_logs(false)           // Enable JSON structured logging
    .with_console_output(true);      // Enable console output
```

## Error Categories

The error categorization system classifies errors into the following categories:

- **Network**: Connection failures, timeouts, unreachable endpoints
- **Transaction**: Transaction execution failures, nonce issues
- **Authentication**: Unauthorized access, signature failures
- **ResourceExhaustion**: Insufficient gas, memory limits
- **Configuration**: Invalid configuration, unsupported options
- **Validation**: Data validation failures, malformed inputs
- **ExternalService**: RPC provider errors, node failures
- **Internal**: SDK internal errors
- **Cryptography**: Encryption/decryption failures, key issues
- **Timeout**: Operation timeouts
- **RateLimit**: API rate limiting (HTTP 429)
- **ChainSpecific**: Chain-specific errors

## Best Practices

1. **Initialize telemetry early** in your application startup
2. **Use structured labels** for better metric filtering and aggregation
3. **Profile critical operations** with the PerformanceProfiler
4. **Monitor health status** of all external dependencies
5. **Set up alerts** based on error rates and latency percentiles
6. **Use time windows** appropriate for your use case (1m for real-time, 1h for trends)
7. **Review Grafana dashboards** regularly to identify performance bottlenecks

## Production Deployment

### Docker Compose Example

```yaml
version: '3.8'
services:
  prometheus:
    image: prom/prometheus:latest
    ports:
      - "9091:9090"
    volumes:
      - ./prometheus.yml:/etc/prometheus/prometheus.yml

  grafana:
    image: grafana/grafana:latest
    ports:
      - "3000:3000"
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=admin
    volumes:
      - ./dashboards:/etc/grafana/provisioning/dashboards

  apex-app:
    build: .
    ports:
      - "9090:9090"  # Metrics endpoint
```

### Prometheus Configuration

```yaml
scrape_configs:
  - job_name: 'apex-sdk'
    scrape_interval: 15s
    static_configs:
      - targets: ['apex-app:9090']
```

## Performance Impact

The metrics system is designed for minimal performance overhead:

- **Memory**: ~1MB for 10,000 metric data points
- **CPU**: <1% overhead for metric recording
- **Network**: Metrics served on-demand (pull model)
- **Storage**: Configurable retention in Prometheus

## License

Apache-2.0
