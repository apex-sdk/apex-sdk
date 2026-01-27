//! Metrics aggregation and statistical analysis
//!
//! This module provides powerful metrics aggregation capabilities including
//! statistical analysis, time-series data processing, and trend detection.

use apex_sdk_core::metrics::{Metric, MetricType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Time window for aggregation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeWindow {
    /// Last minute
    OneMinute,
    /// Last 5 minutes
    FiveMinutes,
    /// Last 15 minutes
    FifteenMinutes,
    /// Last hour
    OneHour,
    /// Last 24 hours
    OneDay,
    /// Last 7 days
    OneWeek,
}

impl TimeWindow {
    /// Get duration for the time window
    pub fn duration(&self) -> Duration {
        match self {
            TimeWindow::OneMinute => Duration::from_secs(60),
            TimeWindow::FiveMinutes => Duration::from_secs(300),
            TimeWindow::FifteenMinutes => Duration::from_secs(900),
            TimeWindow::OneHour => Duration::from_secs(3600),
            TimeWindow::OneDay => Duration::from_secs(86400),
            TimeWindow::OneWeek => Duration::from_secs(604800),
        }
    }

    /// Get seconds for the time window
    pub fn seconds(&self) -> u64 {
        self.duration().as_secs()
    }
}

/// Statistical snapshot of metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticalSnapshot {
    /// Metric name
    pub metric_name: String,
    /// Time window
    pub time_window: TimeWindow,
    /// Sample count
    pub count: usize,
    /// Sum of all values
    pub sum: f64,
    /// Mean value
    pub mean: f64,
    /// Median value
    pub median: f64,
    /// Minimum value
    pub min: f64,
    /// Maximum value
    pub max: f64,
    /// Standard deviation
    pub std_dev: f64,
    /// 50th percentile (same as median)
    pub p50: f64,
    /// 90th percentile
    pub p90: f64,
    /// 95th percentile
    pub p95: f64,
    /// 99th percentile
    pub p99: f64,
    /// Timestamp of snapshot
    pub timestamp: u64,
}

impl StatisticalSnapshot {
    /// Calculate statistical snapshot from metrics
    pub fn from_metrics(
        metrics: &[Metric],
        metric_name: &str,
        time_window: TimeWindow,
    ) -> Option<Self> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let cutoff_time = now.saturating_sub(time_window.seconds());

        let mut values: Vec<f64> = metrics
            .iter()
            .filter(|m| m.name == metric_name && m.timestamp >= cutoff_time)
            .map(|m| m.value)
            .collect();

        if values.is_empty() {
            return None;
        }

        values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let count = values.len();
        let sum: f64 = values.iter().sum();
        let mean = sum / count as f64;

        let variance = values.iter().map(|&v| (v - mean).powi(2)).sum::<f64>() / count as f64;
        let std_dev = variance.sqrt();

        Some(Self {
            metric_name: metric_name.to_string(),
            time_window,
            count,
            sum,
            mean,
            median: percentile(&values, 50.0),
            min: values.first().copied().unwrap_or(0.0),
            max: values.last().copied().unwrap_or(0.0),
            std_dev,
            p50: percentile(&values, 50.0),
            p90: percentile(&values, 90.0),
            p95: percentile(&values, 95.0),
            p99: percentile(&values, 99.0),
            timestamp: now,
        })
    }

    /// Calculate rate per second
    pub fn rate_per_second(&self) -> f64 {
        let window_seconds = self.time_window.seconds() as f64;
        if window_seconds > 0.0 {
            self.count as f64 / window_seconds
        } else {
            0.0
        }
    }

    /// Check if values are trending up
    pub fn is_trending_up(&self) -> bool {
        self.p95 > self.mean * 1.2
    }

    /// Check if values are trending down
    pub fn is_trending_down(&self) -> bool {
        self.p95 < self.mean * 0.8
    }
}

/// Aggregated metrics by label
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedMetrics {
    /// Metrics grouped by labels
    pub by_label: HashMap<String, HashMap<String, StatisticalSnapshot>>,
    /// Overall statistics
    pub overall: HashMap<String, StatisticalSnapshot>,
    /// Time window used for aggregation
    pub time_window: TimeWindow,
}

/// Metrics aggregator
pub struct MetricsAggregator {
    time_window: TimeWindow,
}

impl MetricsAggregator {
    /// Create a new metrics aggregator
    pub fn new() -> Self {
        Self {
            time_window: TimeWindow::FiveMinutes,
        }
    }

    /// Create an aggregator with a specific time window
    pub fn with_time_window(time_window: TimeWindow) -> Self {
        Self { time_window }
    }

    /// Aggregate metrics
    pub fn aggregate(&self, metrics: &[Metric]) -> AggregatedMetrics {
        let mut overall = HashMap::new();
        let mut by_label: HashMap<String, HashMap<String, StatisticalSnapshot>> = HashMap::new();

        let metric_names: Vec<String> = metrics
            .iter()
            .map(|m| m.name.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        for name in &metric_names {
            if let Some(snapshot) =
                StatisticalSnapshot::from_metrics(metrics, name, self.time_window)
            {
                overall.insert(name.clone(), snapshot);
            }
        }

        for (label_key, label_values) in Self::extract_label_combinations(metrics) {
            let mut label_stats = HashMap::new();

            for label_value in label_values {
                for name in &metric_names {
                    let filtered_metrics: Vec<Metric> = metrics
                        .iter()
                        .filter(|m| {
                            m.name == *name
                                && m.labels
                                    .get(&label_key)
                                    .map(|v| v == &label_value)
                                    .unwrap_or(false)
                        })
                        .cloned()
                        .collect();

                    if let Some(snapshot) =
                        StatisticalSnapshot::from_metrics(&filtered_metrics, name, self.time_window)
                    {
                        let key = format!("{}:{}", label_value, name);
                        label_stats.insert(key, snapshot);
                    }
                }
            }

            if !label_stats.is_empty() {
                by_label.insert(label_key, label_stats);
            }
        }

        AggregatedMetrics {
            by_label,
            overall,
            time_window: self.time_window,
        }
    }

    /// Aggregate metrics by type
    pub fn aggregate_by_type(
        &self,
        metrics: &[Metric],
        metric_type: MetricType,
    ) -> HashMap<String, StatisticalSnapshot> {
        let filtered: Vec<Metric> = metrics
            .iter()
            .filter(|m| m.metric_type == metric_type)
            .cloned()
            .collect();

        let mut result = HashMap::new();
        let metric_names: Vec<String> = filtered
            .iter()
            .map(|m| m.name.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        for name in metric_names {
            if let Some(snapshot) =
                StatisticalSnapshot::from_metrics(&filtered, &name, self.time_window)
            {
                result.insert(name, snapshot);
            }
        }

        result
    }

    /// Extract label combinations from metrics
    fn extract_label_combinations(metrics: &[Metric]) -> HashMap<String, Vec<String>> {
        let mut combinations: HashMap<String, std::collections::HashSet<String>> = HashMap::new();

        for metric in metrics {
            for (key, value) in &metric.labels {
                combinations
                    .entry(key.clone())
                    .or_default()
                    .insert(value.clone());
            }
        }

        combinations
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect()
    }

    /// Calculate success rate from transaction metrics
    pub fn calculate_success_rate(&self, metrics: &[Metric], chain: Option<&str>) -> f64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let cutoff_time = now.saturating_sub(self.time_window.seconds());

        let (success_count, failure_count) = metrics
            .iter()
            .filter(|m| {
                m.timestamp >= cutoff_time
                    && m.metric_type == MetricType::TransactionSuccessRate
                    && chain
                        .map(|c| m.labels.get("chain").map(|v| v == c).unwrap_or(false))
                        .unwrap_or(true)
            })
            .fold((0.0, 0.0), |(success, failure), m| {
                match m.labels.get("status").map(String::as_str) {
                    Some("success") => (success + m.value, failure),
                    Some("failure") => (success, failure + m.value),
                    _ => (success, failure),
                }
            });

        let total = success_count + failure_count;
        if total > 0.0 {
            (success_count / total) * 100.0
        } else {
            0.0
        }
    }
}

impl Default for MetricsAggregator {
    fn default() -> Self {
        Self::new()
    }
}

fn percentile(sorted_data: &[f64], p: f64) -> f64 {
    if sorted_data.is_empty() {
        return 0.0;
    }

    let index = (p / 100.0 * (sorted_data.len() - 1) as f64).round() as usize;
    sorted_data[index.min(sorted_data.len() - 1)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_metrics() -> Vec<Metric> {
        let mut metrics = Vec::new();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        for i in 0..100 {
            let mut metric = Metric::new(
                MetricType::TransactionLatency,
                "tx_duration",
                (i as f64) * 0.1,
            );
            metric.timestamp = now - (i * 2);
            metric
                .labels
                .insert("chain".to_string(), "ethereum".to_string());
            metrics.push(metric);
        }

        metrics
    }

    #[test]
    fn test_statistical_snapshot() {
        let metrics = create_test_metrics();
        let snapshot =
            StatisticalSnapshot::from_metrics(&metrics, "tx_duration", TimeWindow::FiveMinutes);

        assert!(snapshot.is_some());
        let snapshot = snapshot.unwrap();
        assert!(snapshot.count > 0);
        assert!(snapshot.mean > 0.0);
        assert!(snapshot.min >= 0.0);
        assert!(snapshot.max > snapshot.min);
    }

    #[test]
    fn test_metrics_aggregator() {
        let metrics = create_test_metrics();
        let aggregator = MetricsAggregator::new();
        let aggregated = aggregator.aggregate(&metrics);

        assert!(!aggregated.overall.is_empty());
        assert!(aggregated.overall.contains_key("tx_duration"));
    }

    #[test]
    fn test_aggregate_by_type() {
        let metrics = create_test_metrics();
        let aggregator = MetricsAggregator::new();
        let by_type = aggregator.aggregate_by_type(&metrics, MetricType::TransactionLatency);

        assert!(!by_type.is_empty());
        assert!(by_type.contains_key("tx_duration"));
    }

    #[test]
    fn test_success_rate_calculation() {
        let mut metrics = Vec::new();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        for i in 0..90 {
            let mut metric = Metric::new(MetricType::TransactionSuccessRate, "tx_result", 1.0);
            metric.timestamp = now - i;
            metric
                .labels
                .insert("chain".to_string(), "ethereum".to_string());
            metric
                .labels
                .insert("status".to_string(), "success".to_string());
            metrics.push(metric);
        }

        for i in 0..10 {
            let mut metric = Metric::new(MetricType::TransactionSuccessRate, "tx_result", 1.0);
            metric.timestamp = now - i;
            metric
                .labels
                .insert("chain".to_string(), "ethereum".to_string());
            metric
                .labels
                .insert("status".to_string(), "failure".to_string());
            metrics.push(metric);
        }

        let aggregator = MetricsAggregator::new();
        let success_rate = aggregator.calculate_success_rate(&metrics, Some("ethereum"));

        assert_eq!(success_rate, 90.0);
    }

    #[test]
    fn test_time_window_duration() {
        assert_eq!(TimeWindow::OneMinute.seconds(), 60);
        assert_eq!(TimeWindow::FiveMinutes.seconds(), 300);
        assert_eq!(TimeWindow::OneHour.seconds(), 3600);
    }
}
