//! Comprehensive observability example showcasing all metrics features
//!
//! This example demonstrates:
//! - Telemetry initialization
//! - Metrics collection and recording
//! - Error categorization
//! - Performance profiling
//! - Health monitoring
//! - Metrics aggregation
//! - Prometheus server
//!
//! Run with: cargo run --example comprehensive_observability

use apex_sdk_core::metrics::MetricsCollector;
use apex_sdk_metrics::{
    categorize_error, init_telemetry, AggregatedMetrics, ComponentHealth, HealthChecker,
    HealthStatus, MetricsAggregator, MetricsServer, ObservabilityConfig, OperationType,
    PerformanceProfiler, TimeWindow,
};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Apex SDK Comprehensive Observability Example\n");

    println!("📊 Step 1: Initialize Telemetry");
    let config = ObservabilityConfig::new("apex-observability-demo")
        .with_environment("development")
        .with_prometheus_port(9090)
        .with_log_level("info")
        .with_console_output(true);

    let _telemetry = init_telemetry(config.clone())?;
    println!("✓ Telemetry initialized\n");

    println!("📊 Step 2: Create Observability Components");
    let collector = MetricsCollector::new();
    let profiler = PerformanceProfiler::new();
    let health_checker = HealthChecker::new();
    let aggregator = MetricsAggregator::with_time_window(TimeWindow::FiveMinutes);
    println!("✓ Components created\n");

    println!("📊 Step 3: Simulate Transaction Metrics");
    simulate_transactions(&collector, &profiler).await;
    println!("✓ Recorded {} transactions\n", 20);

    println!("📊 Step 4: Demonstrate Error Categorization");
    demonstrate_error_categorization();
    println!();

    println!("📊 Step 5: Monitor Component Health");
    monitor_health(&health_checker);
    let health_summary = health_checker.health_summary();
    println!("✓ Overall Status: {:?}", health_summary.status);
    println!(
        "✓ CPU Usage: {:.1}%",
        health_summary.resources.cpu_usage_percent
    );
    println!(
        "✓ Memory Usage: {:.1}%\n",
        health_summary.resources.memory_usage_percent
    );

    println!("📊 Step 6: Aggregate Metrics");
    let aggregated = aggregator.aggregate(&collector.get_metrics());
    display_aggregated_metrics(&aggregated);

    println!("📊 Step 7: Performance Statistics");
    display_performance_stats(&profiler);

    println!("📊 Step 8: Starting Prometheus Metrics Server");
    let server = MetricsServer::new(config.prometheus_port, collector.clone()).await?;
    println!(
        "✓ Metrics server listening on http://localhost:{}",
        config.prometheus_port
    );
    println!(
        "✓ Prometheus endpoint: http://localhost:{}/metrics",
        config.prometheus_port
    );
    println!(
        "✓ Health check: http://localhost:{}/health\n",
        config.prometheus_port
    );

    println!("📊 Observability Dashboard Ready!");
    println!("\nℹ️  Try these commands in another terminal:");
    println!("  curl http://localhost:9090/metrics     # View Prometheus metrics");
    println!("  curl http://localhost:9090/health      # Check health status");
    println!("\n  Press Ctrl+C to stop the server");

    server.start().await?;

    Ok(())
}

async fn simulate_transactions(collector: &MetricsCollector, profiler: &PerformanceProfiler) {
    let chains = ["ethereum", "polygon", "arbitrum"];
    let operations = [
        OperationType::TransactionSubmit,
        OperationType::BalanceQuery,
        OperationType::RpcRequest,
    ];

    for i in 0..20 {
        let chain = chains[i % chains.len()];
        let operation = operations[i % operations.len()];
        let tx_hash = format!("0x{:064x}", i);

        let mut span = profiler.start_span(operation);
        span.set_attribute("chain", chain);
        span.set_attribute("tx_hash", &tx_hash);

        sleep(Duration::from_millis(10 + (i as u64 % 50))).await;

        if i % 7 == 0 {
            collector.record_transaction_failure(chain, &tx_hash, "insufficient_gas");
            span.error("insufficient_gas");
        } else {
            collector.record_transaction_success(chain, &tx_hash);
            span.success();
        }

        if matches!(operation, OperationType::TransactionSubmit) {
            let gas_used = 21000 + (i as u64 * 1000);
            let gas_limit = 21000 + (i as u64 * 1200);
            collector.record_gas_usage(chain, gas_used, gas_limit);
        }

        collector.record_provider_response_time(
            chain,
            &format!("{:?}", operation),
            Duration::from_millis(10 + (i as u64 % 100)),
        );
    }
}

fn demonstrate_error_categorization() {
    let errors = vec![
        (
            "connection timeout to RPC endpoint",
            "Network error with timeout",
        ),
        ("insufficient gas for transaction", "Resource exhaustion"),
        (
            "unauthorized access - invalid signature",
            "Authentication failure",
        ),
        ("rate limit exceeded - too many requests", "Rate limiting"),
        ("invalid nonce for transaction", "Transaction error"),
    ];

    for (error_msg, description) in errors {
        let classification = categorize_error(error_msg, None);
        println!("  Error: {}", description);
        println!("    Category: {:?}", classification.category);
        println!("    Severity: {:?}", classification.severity);
        println!("    Retryable: {}", classification.is_retryable());
        if let Some(remediation) = &classification.remediation {
            println!("    Remediation: {}", remediation);
        }
        println!();
    }
}

fn monitor_health(health_checker: &HealthChecker) {
    health_checker.update_component(
        ComponentHealth::new("ethereum-rpc", HealthStatus::Healthy)
            .with_message("Connected to Ethereum mainnet")
            .with_metadata("endpoint", "https://eth.llamarpc.com")
            .with_response_time(Duration::from_millis(150)),
    );

    health_checker.update_component(
        ComponentHealth::new("polygon-rpc", HealthStatus::Healthy)
            .with_message("Connected to Polygon mainnet")
            .with_metadata("endpoint", "https://polygon-rpc.com")
            .with_response_time(Duration::from_millis(120)),
    );

    health_checker.update_component(
        ComponentHealth::new("metrics-collector", HealthStatus::Healthy)
            .with_message("Collecting metrics successfully"),
    );
}

fn display_aggregated_metrics(aggregated: &AggregatedMetrics) {
    println!(
        "  Overall Statistics ({})",
        format_time_window(aggregated.time_window)
    );
    for (name, snapshot) in &aggregated.overall {
        println!("    {}", name);
        println!("      Count: {}", snapshot.count);
        println!("      Mean: {:.3}", snapshot.mean);
        println!("      P50: {:.3}", snapshot.p50);
        println!("      P95: {:.3}", snapshot.p95);
        println!("      P99: {:.3}", snapshot.p99);
        println!("      Rate: {:.2}/sec", snapshot.rate_per_second());
    }
    println!();
}

fn display_performance_stats(profiler: &PerformanceProfiler) {
    let operations = vec![
        OperationType::TransactionSubmit,
        OperationType::BalanceQuery,
        OperationType::RpcRequest,
    ];

    for operation in operations {
        let stats = profiler.operation_stats(operation);
        if stats.total_count > 0 {
            println!("  Operation: {:?}", operation);
            println!("    Total: {}", stats.total_count);
            println!("    Success Rate: {:.1}%", stats.success_rate());
            println!("    Error Rate: {:.1}%", stats.error_rate());
            println!("    Mean Duration: {:.3}s", stats.mean_duration_secs);
            println!("    P95 Duration: {:.3}s", stats.p95_duration_secs);
            println!("    P99 Duration: {:.3}s", stats.p99_duration_secs);
            println!();
        }
    }
}

fn format_time_window(window: TimeWindow) -> String {
    match window {
        TimeWindow::OneMinute => "last 1 minute".to_string(),
        TimeWindow::FiveMinutes => "last 5 minutes".to_string(),
        TimeWindow::FifteenMinutes => "last 15 minutes".to_string(),
        TimeWindow::OneHour => "last 1 hour".to_string(),
        TimeWindow::OneDay => "last 24 hours".to_string(),
        TimeWindow::OneWeek => "last 7 days".to_string(),
    }
}
