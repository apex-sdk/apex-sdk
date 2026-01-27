//! Performance profiling and distributed tracing
//!
//! This module provides OpenTelemetry-based performance profiling with
//! automatic span tracking, operation timing, and distributed tracing support.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tracing::{info, warn};

/// Operation types for profiling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OperationType {
    /// Transaction submission
    TransactionSubmit,
    /// Transaction confirmation
    TransactionConfirm,
    /// Block query
    BlockQuery,
    /// Balance query
    BalanceQuery,
    /// Storage query
    StorageQuery,
    /// Contract call
    ContractCall,
    /// RPC request
    RpcRequest,
    /// Signature generation
    Signing,
    /// Fee estimation
    FeeEstimation,
    /// Nonce retrieval
    NonceRetrieval,
    /// Custom operation
    Custom,
}

impl std::fmt::Display for OperationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationType::TransactionSubmit => write!(f, "transaction_submit"),
            OperationType::TransactionConfirm => write!(f, "transaction_confirm"),
            OperationType::BlockQuery => write!(f, "block_query"),
            OperationType::BalanceQuery => write!(f, "balance_query"),
            OperationType::StorageQuery => write!(f, "storage_query"),
            OperationType::ContractCall => write!(f, "contract_call"),
            OperationType::RpcRequest => write!(f, "rpc_request"),
            OperationType::Signing => write!(f, "signing"),
            OperationType::FeeEstimation => write!(f, "fee_estimation"),
            OperationType::NonceRetrieval => write!(f, "nonce_retrieval"),
            OperationType::Custom => write!(f, "custom"),
        }
    }
}

/// Span context for distributed tracing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpanContext {
    /// Span ID
    pub span_id: String,
    /// Trace ID (for distributed tracing)
    pub trace_id: String,
    /// Parent span ID
    pub parent_span_id: Option<String>,
    /// Span attributes
    pub attributes: HashMap<String, String>,
}

impl SpanContext {
    /// Create a new span context
    pub fn new(operation: OperationType) -> Self {
        Self {
            span_id: uuid::Uuid::new_v4().to_string(),
            trace_id: uuid::Uuid::new_v4().to_string(),
            parent_span_id: None,
            attributes: {
                let mut attrs = HashMap::new();
                attrs.insert("operation".to_string(), operation.to_string());
                attrs
            },
        }
    }

    /// Create a child span context
    pub fn child(&self, operation: OperationType) -> Self {
        Self {
            span_id: uuid::Uuid::new_v4().to_string(),
            trace_id: self.trace_id.clone(),
            parent_span_id: Some(self.span_id.clone()),
            attributes: {
                let mut attrs = HashMap::new();
                attrs.insert("operation".to_string(), operation.to_string());
                attrs
            },
        }
    }

    /// Add an attribute to the span
    pub fn with_attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }
}

/// Performance span for tracking operation duration
pub struct OperationSpan {
    context: SpanContext,
    operation_type: OperationType,
    start_time: Instant,
    start_timestamp: u64,
    attributes: HashMap<String, String>,
    profiler: Arc<PerformanceProfiler>,
}

impl OperationSpan {
    /// Create a new operation span
    fn new(
        context: SpanContext,
        operation_type: OperationType,
        profiler: Arc<PerformanceProfiler>,
    ) -> Self {
        info!(
            span_id = %context.span_id,
            trace_id = %context.trace_id,
            operation = %operation_type,
            "Starting operation span"
        );

        Self {
            context,
            operation_type,
            start_time: Instant::now(),
            start_timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            attributes: HashMap::new(),
            profiler,
        }
    }

    /// Add an attribute to the span
    pub fn set_attribute(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.attributes.insert(key.into(), value.into());
    }

    /// Mark the span as successful
    pub fn success(mut self) {
        self.attributes
            .insert("status".to_string(), "success".to_string());
        drop(self);
    }

    /// Mark the span as failed with an error
    pub fn error(mut self, error: impl Into<String>) {
        self.attributes
            .insert("status".to_string(), "error".to_string());
        self.attributes.insert("error".to_string(), error.into());
        drop(self);
    }

    /// Get the span context
    pub fn context(&self) -> &SpanContext {
        &self.context
    }
}

impl Drop for OperationSpan {
    fn drop(&mut self) {
        let duration = self.start_time.elapsed();
        let status = self
            .attributes
            .get("status")
            .map(String::as_str)
            .unwrap_or("unknown");

        info!(
            span_id = %self.context.span_id,
            trace_id = %self.context.trace_id,
            operation = %self.operation_type,
            duration_ms = duration.as_millis(),
            status = status,
            "Completed operation span"
        );

        let record = SpanRecord {
            context: self.context.clone(),
            operation_type: self.operation_type,
            duration,
            start_timestamp: self.start_timestamp,
            attributes: self.attributes.clone(),
        };

        self.profiler.record_span(record);
    }
}

/// Recorded span information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpanRecord {
    /// Span context
    pub context: SpanContext,
    /// Operation type
    pub operation_type: OperationType,
    /// Operation duration
    #[serde(with = "duration_serde")]
    pub duration: Duration,
    /// Start timestamp
    pub start_timestamp: u64,
    /// Span attributes
    pub attributes: HashMap<String, String>,
}

mod duration_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::time::Duration;

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        duration.as_secs_f64().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = f64::deserialize(deserializer)?;
        Ok(Duration::from_secs_f64(secs))
    }
}

impl SpanRecord {
    /// Check if the span represents a successful operation
    pub fn is_success(&self) -> bool {
        self.attributes
            .get("status")
            .map(|s| s == "success")
            .unwrap_or(false)
    }

    /// Check if the span represents a failed operation
    pub fn is_error(&self) -> bool {
        self.attributes
            .get("status")
            .map(|s| s == "error")
            .unwrap_or(false)
    }
}

/// Performance profiler for tracking operation performance
pub struct PerformanceProfiler {
    spans: Arc<Mutex<Vec<SpanRecord>>>,
}

impl PerformanceProfiler {
    /// Create a new performance profiler
    pub fn new() -> Self {
        Self {
            spans: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Start a new operation span
    pub fn start_span(&self, operation_type: OperationType) -> OperationSpan {
        let context = SpanContext::new(operation_type);
        OperationSpan::new(context, operation_type, Arc::new(self.clone()))
    }

    /// Start a new operation span with context
    pub fn start_span_with_context(
        &self,
        operation_type: OperationType,
        context: SpanContext,
    ) -> OperationSpan {
        OperationSpan::new(context, operation_type, Arc::new(self.clone()))
    }

    /// Record a completed span
    fn record_span(&self, record: SpanRecord) {
        if let Ok(mut spans) = self.spans.lock() {
            spans.push(record);

            if spans.len() > 10000 {
                warn!("Span buffer exceeds 10000 records, removing oldest 1000");
                spans.drain(0..1000);
            }
        }
    }

    /// Get all recorded spans
    pub fn get_spans(&self) -> Vec<SpanRecord> {
        self.spans
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone()
    }

    /// Get spans for a specific operation type
    pub fn get_spans_by_operation(&self, operation_type: OperationType) -> Vec<SpanRecord> {
        self.get_spans()
            .into_iter()
            .filter(|span| span.operation_type == operation_type)
            .collect()
    }

    /// Get performance statistics for an operation type
    pub fn operation_stats(&self, operation_type: OperationType) -> OperationStats {
        let spans = self.get_spans_by_operation(operation_type);

        if spans.is_empty() {
            return OperationStats::default();
        }

        let total_count = spans.len();
        let success_count = spans.iter().filter(|s| s.is_success()).count();
        let error_count = spans.iter().filter(|s| s.is_error()).count();

        let mut durations: Vec<f64> = spans.iter().map(|s| s.duration.as_secs_f64()).collect();
        durations.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let total_duration: f64 = durations.iter().sum();
        let mean_duration = total_duration / total_count as f64;

        let p50 = percentile(&durations, 50.0);
        let p95 = percentile(&durations, 95.0);
        let p99 = percentile(&durations, 99.0);

        OperationStats {
            operation_type,
            total_count,
            success_count,
            error_count,
            mean_duration_secs: mean_duration,
            p50_duration_secs: p50,
            p95_duration_secs: p95,
            p99_duration_secs: p99,
            min_duration_secs: durations.first().copied().unwrap_or(0.0),
            max_duration_secs: durations.last().copied().unwrap_or(0.0),
        }
    }

    /// Clear all recorded spans
    pub fn clear(&self) {
        if let Ok(mut spans) = self.spans.lock() {
            spans.clear();
        }
    }

    /// Get total span count
    pub fn span_count(&self) -> usize {
        self.spans
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .len()
    }
}

impl Clone for PerformanceProfiler {
    fn clone(&self) -> Self {
        Self {
            spans: Arc::clone(&self.spans),
        }
    }
}

impl Default for PerformanceProfiler {
    fn default() -> Self {
        Self::new()
    }
}

/// Operation performance statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationStats {
    /// Operation type
    pub operation_type: OperationType,
    /// Total operation count
    pub total_count: usize,
    /// Successful operation count
    pub success_count: usize,
    /// Failed operation count
    pub error_count: usize,
    /// Mean duration in seconds
    pub mean_duration_secs: f64,
    /// 50th percentile (median) duration
    pub p50_duration_secs: f64,
    /// 95th percentile duration
    pub p95_duration_secs: f64,
    /// 99th percentile duration
    pub p99_duration_secs: f64,
    /// Minimum duration
    pub min_duration_secs: f64,
    /// Maximum duration
    pub max_duration_secs: f64,
}

impl Default for OperationStats {
    fn default() -> Self {
        Self {
            operation_type: OperationType::Custom,
            total_count: 0,
            success_count: 0,
            error_count: 0,
            mean_duration_secs: 0.0,
            p50_duration_secs: 0.0,
            p95_duration_secs: 0.0,
            p99_duration_secs: 0.0,
            min_duration_secs: 0.0,
            max_duration_secs: 0.0,
        }
    }
}

impl OperationStats {
    /// Calculate success rate
    pub fn success_rate(&self) -> f64 {
        if self.total_count == 0 {
            return 0.0;
        }
        (self.success_count as f64 / self.total_count as f64) * 100.0
    }

    /// Calculate error rate
    pub fn error_rate(&self) -> f64 {
        if self.total_count == 0 {
            return 0.0;
        }
        (self.error_count as f64 / self.total_count as f64) * 100.0
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
    use std::thread;

    #[test]
    fn test_span_context_creation() {
        let context = SpanContext::new(OperationType::TransactionSubmit);
        assert!(!context.span_id.is_empty());
        assert!(!context.trace_id.is_empty());
        assert_eq!(context.parent_span_id, None);
    }

    #[test]
    fn test_child_span_context() {
        let parent = SpanContext::new(OperationType::TransactionSubmit);
        let child = parent.child(OperationType::Signing);

        assert_eq!(child.trace_id, parent.trace_id);
        assert_eq!(child.parent_span_id, Some(parent.span_id.clone()));
    }

    #[test]
    fn test_operation_span_recording() {
        let profiler = PerformanceProfiler::new();
        {
            let span = profiler.start_span(OperationType::RpcRequest);
            thread::sleep(Duration::from_millis(10));
            span.success();
        }

        assert_eq!(profiler.span_count(), 1);
        let spans = profiler.get_spans();
        assert!(spans[0].duration.as_millis() >= 10);
    }

    #[test]
    fn test_operation_stats() {
        let profiler = PerformanceProfiler::new();

        for i in 0..100 {
            let span = profiler.start_span(OperationType::BlockQuery);
            thread::sleep(Duration::from_millis(i % 10));
            if i % 10 == 0 {
                span.error("test error");
            } else {
                span.success();
            }
        }

        let stats = profiler.operation_stats(OperationType::BlockQuery);
        assert_eq!(stats.total_count, 100);
        assert_eq!(stats.success_count, 90);
        assert_eq!(stats.error_count, 10);
        assert_eq!(stats.success_rate(), 90.0);
    }

    #[test]
    fn test_span_attributes() {
        let profiler = PerformanceProfiler::new();
        {
            let mut span = profiler.start_span(OperationType::ContractCall);
            span.set_attribute("contract_address", "0x123");
            span.set_attribute("method", "transfer");
            span.success();
        }

        let spans = profiler.get_spans();
        assert_eq!(
            spans[0].attributes.get("contract_address").unwrap(),
            "0x123"
        );
        assert_eq!(spans[0].attributes.get("method").unwrap(), "transfer");
    }
}
