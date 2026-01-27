# Metrics and Monitoring

Apex SDK provides comprehensive observability through the `apex-sdk-metrics` crate, offering production-ready metrics collection, distributed tracing, error categorization, and health monitoring.

## Overview

The metrics system is designed around several core principles:

- **Zero-overhead when disabled**: Minimal performance impact with opt-in collection
- **Production-ready**: Prometheus-compatible metrics with Grafana dashboards
- **Comprehensive**: Track transactions, RPC calls, errors, and system health
- **Intelligent**: Automatic error categorization and performance profiling
- **Standards-based**: OpenTelemetry for distributed tracing

## Quick Start

Add the metrics dependency to your `Cargo.toml`:

```toml
[dependencies]
apex-sdk-metrics = "0.1"
```

### Basic Setup

```rust
use apex_sdk_metrics::{MetricsServer, ObservabilityConfig, init_telemetry};
use apex_sdk_core::metrics::MetricsCollector;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize telemetry
    let config = ObservabilityConfig::new("my-app")
        .with_environment("production")
        .with_prometheus_port(9090);

    let _telemetry = init_telemetry(config.clone())?;

    // Create collector and start server
    let collector = MetricsCollector::new();
    let server = MetricsServer::new(9090, collector).await?;

    // Metrics available at http://localhost:9090/metrics
    server.start_background();

    Ok(())
}
```

## Core Features

### 1. Transaction Metrics

Track all blockchain transactions with detailed metrics:

```rust
use apex_sdk_core::metrics::MetricsCollector;

let collector = MetricsCollector::new();

// Record successful transaction
collector.record_transaction_success("ethereum", "0x123...");

// Record failed transaction
collector.record_transaction_failure("ethereum", "0x456...", "insufficient_gas");

// Record gas usage
collector.record_gas_usage("ethereum", 21000, 30000);
```

### 2. Error Categorization

Automatic intelligent error classification:

```rust
use apex_sdk_metrics::categorize_error;

let error = categorize_error("connection timeout to RPC", None);

// Provides structured error information
println!("Category: {:?}", error.category);      // Network
println!("Severity: {:?}", error.severity);      // Medium
println!("Retryable: {}", error.is_retryable()); // true
println!("Remediation: {:?}", error.remediation); // Suggested fix
```

**Error Categories:**
- Network
- Transaction
- Authentication
- ResourceExhaustion
- Configuration
- Validation
- ExternalService
- Timeout
- RateLimit
- Cryptography

### 3. Performance Profiling

Distributed tracing with automatic span tracking:

```rust
use apex_sdk_metrics::{PerformanceProfiler, OperationType};

let profiler = PerformanceProfiler::new();

// Profile an operation
{
    let mut span = profiler.start_span(OperationType::TransactionSubmit);
    span.set_attribute("chain", "ethereum");

    // ... perform operation ...

    span.success(); // Automatically records duration
}

// Get performance statistics
let stats = profiler.operation_stats(OperationType::TransactionSubmit);
println!("P95 Latency: {:.3}s", stats.p95_duration_secs);
println!("Success Rate: {:.1}%", stats.success_rate());
```

### 4. Health Monitoring

Monitor component health and system resources:

```rust
use apex_sdk_metrics::{HealthChecker, ComponentHealth, HealthStatus};

let health_checker = HealthChecker::new();

health_checker.update_component(
    ComponentHealth::new("ethereum-rpc", HealthStatus::Healthy)
        .with_message("Connected")
        .with_metadata("endpoint", "https://eth.llamarpc.com")
);

let summary = health_checker.health_summary();
println!("Status: {:?}", summary.status);
println!("CPU: {:.1}%", summary.resources.cpu_usage_percent);
```

### 5. Metrics Aggregation

Statistical analysis and trend detection:

```rust
use apex_sdk_metrics::{MetricsAggregator, TimeWindow};

let aggregator = MetricsAggregator::with_time_window(TimeWindow::FiveMinutes);
let aggregated = aggregator.aggregate(&collector.get_metrics());

// Access statistical snapshots
for (name, snapshot) in &aggregated.overall {
    println!("{}: P95={:.3}, trending_up={}",
        name, snapshot.p95, snapshot.is_trending_up());
}

// Calculate success rate
let success_rate = aggregator.calculate_success_rate(
    &collector.get_metrics(),
    Some("ethereum")
);
```

## Prometheus Integration

### Metrics Endpoint

Start the metrics server to expose a Prometheus-compatible endpoint:

```rust
let server = MetricsServer::new(9090, collector).await?;
server.start().await?;
```

Available endpoints:
- `http://localhost:9090/metrics` - Prometheus metrics
- `http://localhost:9090/health` - Health check
- `http://localhost:9090/ready` - Readiness probe

### Exported Metrics

| Metric | Type | Description |
|--------|------|-------------|
| `apex_sdk_transactions_total` | Counter | Total transactions |
| `apex_sdk_transaction_duration_seconds` | Histogram | Transaction latency |
| `apex_sdk_gas_used` | Gauge | Gas consumption |
| `apex_sdk_errors_total` | Counter | Error counts |
| `apex_sdk_rpc_duration_seconds` | Histogram | RPC response time |

### Prometheus Configuration

```yaml
scrape_configs:
  - job_name: 'apex-sdk'
    scrape_interval: 15s
    static_configs:
      - targets: ['localhost:9090']
```

## Grafana Dashboards

Pre-built dashboards are available in `apex-sdk-metrics/dashboards/`:

### Apex SDK Overview
- Transaction success rates by chain
- P95 latency trends
- Gas usage monitoring
- Error rate by category
- RPC performance

### Performance Deep Dive
- Operation duration heatmaps
- Throughput analysis
- Confirmation time tracking
- Slow operation detection

### Importing Dashboards

1. Open Grafana → Dashboards → Import
2. Upload `apex-sdk-overview.json` or `apex-sdk-performance.json`
3. Select Prometheus data source
4. Click Import

## Configuration

### ObservabilityConfig

```rust
ObservabilityConfig::new("service-name")
    .with_service_version("1.0.0")
    .with_environment("production")
    .with_prometheus_port(9090)
    .with_log_level("info")           // trace, debug, info, warn, error
    .with_json_logs(true)              // Structured JSON logging
    .with_tracing(true)                // OpenTelemetry tracing
```

### Environment Variables

```bash
# Override log level
export RUST_LOG=info,apex_sdk=debug

# Disable metrics collection (minimal overhead)
export APEX_METRICS_ENABLED=false
```

## Production Deployment

### Docker Compose

```yaml
version: '3.8'
services:
  prometheus:
    image: prom/prometheus
    volumes:
      - ./prometheus.yml:/etc/prometheus/prometheus.yml
    ports:
      - "9091:9090"

  grafana:
    image: grafana/grafana
    volumes:
      - ./dashboards:/etc/grafana/provisioning/dashboards
    ports:
      - "3000:3000"

  app:
    build: .
    ports:
      - "9090:9090"  # Metrics
```

### Best Practices

1. **Use appropriate time windows** for aggregation (1m for real-time, 1h for trends)
2. **Set up alerts** on critical metrics (error rate, P95 latency)
3. **Monitor health checks** for all external dependencies
4. **Profile critical paths** with PerformanceProfiler
5. **Review dashboards regularly** to identify bottlenecks
6. **Categorize errors** for better debugging and alerting

## Performance Impact

The metrics system is designed for production use:

- **Memory**: ~1MB per 10,000 data points
- **CPU**: <1% overhead for recording
- **Network**: Pull-based (no constant traffic)
- **Latency**: <1μs per metric recording

## Examples

See `apex-sdk-metrics/examples/` for complete examples:

- `comprehensive_observability.rs` - Full feature demonstration
- Run with: `cargo run --example comprehensive_observability`

## Troubleshooting

### Metrics not appearing

1. Verify server is running: `curl http://localhost:9090/health`
2. Check Prometheus scrape config
3. Ensure collector is being used (not dropped)

### High memory usage

1. Reduce metric retention in Prometheus
2. Use shorter time windows for aggregation
3. Clear metrics periodically: `collector.clear()`

### Missing labels

Ensure labels are added before recording:
```rust
let mut metric = Metric::new(MetricType::TransactionCount, "tx", 1.0);
metric = metric.with_label("chain", "ethereum");
collector.record(metric);
```

## API Reference

Full API documentation available at: [docs.rs/apex-sdk-metrics](https://docs.rs/apex-sdk-metrics)

## Contributing

Found a bug or have a feature request? [Open an issue](https://github.com/apex-sdk/apex-sdk/issues) on GitHub.
