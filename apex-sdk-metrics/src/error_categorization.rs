//! Advanced error categorization and classification system
//!
//! This module provides comprehensive error taxonomy and automatic categorization
//! for improved debugging, monitoring, and alerting.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Error severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ErrorSeverity {
    /// Critical errors requiring immediate attention
    Critical,
    /// High severity errors affecting core functionality
    High,
    /// Medium severity errors with workarounds available
    Medium,
    /// Low severity errors with minimal impact
    Low,
    /// Informational warnings
    Info,
}

/// Error categories for taxonomy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ErrorCategory {
    /// Network and connectivity errors
    Network,
    /// Transaction execution errors
    Transaction,
    /// Authentication and authorization errors
    Authentication,
    /// Resource exhaustion errors (gas, memory, etc.)
    ResourceExhaustion,
    /// Configuration errors
    Configuration,
    /// Data validation errors
    Validation,
    /// External service errors (RPC providers, etc.)
    ExternalService,
    /// Internal SDK errors
    Internal,
    /// Cryptographic operation errors
    Cryptography,
    /// Timeout errors
    Timeout,
    /// Rate limiting errors
    RateLimit,
    /// Chain-specific errors
    ChainSpecific,
}

/// Error impact assessment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ErrorImpact {
    /// Blocks all operations
    Blocking,
    /// Degrades performance but operations can continue
    Degraded,
    /// Retry may resolve the issue
    Retryable,
    /// No impact on other operations
    Isolated,
}

/// Comprehensive error classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorClassification {
    /// Error category
    pub category: ErrorCategory,
    /// Severity level
    pub severity: ErrorSeverity,
    /// Impact assessment
    pub impact: ErrorImpact,
    /// Human-readable description
    pub description: String,
    /// Suggested remediation actions
    pub remediation: Option<String>,
    /// Related error codes or types
    pub error_codes: Vec<String>,
    /// Metrics labels for this error
    pub labels: HashMap<String, String>,
}

impl ErrorClassification {
    /// Create a new error classification
    pub fn new(
        category: ErrorCategory,
        severity: ErrorSeverity,
        impact: ErrorImpact,
        description: impl Into<String>,
    ) -> Self {
        Self {
            category,
            severity,
            impact,
            description: description.into(),
            remediation: None,
            error_codes: Vec::new(),
            labels: HashMap::new(),
        }
    }

    /// Add remediation suggestion
    pub fn with_remediation(mut self, remediation: impl Into<String>) -> Self {
        self.remediation = Some(remediation.into());
        self
    }

    /// Add error codes
    pub fn with_error_codes(mut self, codes: Vec<String>) -> Self {
        self.error_codes = codes;
        self
    }

    /// Add a metric label
    pub fn with_label(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.labels.insert(key.into(), value.into());
        self
    }

    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(self.impact, ErrorImpact::Retryable)
    }

    /// Check if error is critical
    pub fn is_critical(&self) -> bool {
        matches!(self.severity, ErrorSeverity::Critical | ErrorSeverity::High)
    }

    /// Get Prometheus-compatible label map
    pub fn to_prometheus_labels(&self) -> HashMap<String, String> {
        let mut labels = self.labels.clone();
        labels.insert("category".to_string(), format!("{:?}", self.category));
        labels.insert("severity".to_string(), format!("{:?}", self.severity));
        labels.insert("impact".to_string(), format!("{:?}", self.impact));
        labels
    }
}

/// Categorize an error based on its message and type
pub fn categorize_error(error_message: &str, _error_type: Option<&str>) -> ErrorClassification {
    let lower_msg = error_message.to_lowercase();

    // Network errors
    if lower_msg.contains("connection")
        || lower_msg.contains("network")
        || lower_msg.contains("timeout")
        || lower_msg.contains("unreachable")
    {
        return if lower_msg.contains("timeout") {
            ErrorClassification::new(
                ErrorCategory::Timeout,
                ErrorSeverity::Medium,
                ErrorImpact::Retryable,
                "Network operation timed out",
            )
            .with_remediation("Retry the operation with exponential backoff")
            .with_label("retryable", "true")
        } else {
            ErrorClassification::new(
                ErrorCategory::Network,
                ErrorSeverity::High,
                ErrorImpact::Blocking,
                "Network connectivity issue",
            )
            .with_remediation("Check network connection and RPC endpoint availability")
            .with_label("retryable", "true")
        };
    }

    // Transaction errors
    if lower_msg.contains("transaction")
        || lower_msg.contains("tx")
        || lower_msg.contains("nonce")
        || lower_msg.contains("gas")
    {
        if lower_msg.contains("insufficient") && lower_msg.contains("gas") {
            return ErrorClassification::new(
                ErrorCategory::ResourceExhaustion,
                ErrorSeverity::Medium,
                ErrorImpact::Isolated,
                "Insufficient gas for transaction",
            )
            .with_remediation("Increase gas limit or optimize transaction")
            .with_label("resource_type", "gas");
        }

        if lower_msg.contains("nonce") {
            return ErrorClassification::new(
                ErrorCategory::Transaction,
                ErrorSeverity::Medium,
                ErrorImpact::Retryable,
                "Nonce management error",
            )
            .with_remediation("Refresh nonce and retry transaction")
            .with_label("retryable", "true");
        }

        return ErrorClassification::new(
            ErrorCategory::Transaction,
            ErrorSeverity::Medium,
            ErrorImpact::Isolated,
            "Transaction execution failed",
        )
        .with_remediation("Review transaction parameters and chain state");
    }

    // Authentication errors
    if lower_msg.contains("unauthorized")
        || lower_msg.contains("forbidden")
        || lower_msg.contains("signature")
        || lower_msg.contains("authentication")
    {
        return ErrorClassification::new(
            ErrorCategory::Authentication,
            ErrorSeverity::High,
            ErrorImpact::Blocking,
            "Authentication or authorization failure",
        )
        .with_remediation("Verify credentials and permissions")
        .with_label("security_related", "true");
    }

    // Rate limiting
    if lower_msg.contains("rate limit")
        || lower_msg.contains("too many requests")
        || lower_msg.contains("429")
    {
        return ErrorClassification::new(
            ErrorCategory::RateLimit,
            ErrorSeverity::Medium,
            ErrorImpact::Retryable,
            "Rate limit exceeded",
        )
        .with_remediation("Implement exponential backoff and request throttling")
        .with_label("retryable", "true")
        .with_label("http_status", "429");
    }

    // Configuration errors
    if lower_msg.contains("config")
        || lower_msg.contains("invalid endpoint")
        || lower_msg.contains("unsupported")
    {
        return ErrorClassification::new(
            ErrorCategory::Configuration,
            ErrorSeverity::High,
            ErrorImpact::Blocking,
            "Configuration error",
        )
        .with_remediation("Review and correct configuration settings");
    }

    // Validation errors
    if lower_msg.contains("invalid")
        || lower_msg.contains("validation")
        || lower_msg.contains("malformed")
    {
        return ErrorClassification::new(
            ErrorCategory::Validation,
            ErrorSeverity::Medium,
            ErrorImpact::Isolated,
            "Data validation failed",
        )
        .with_remediation("Verify input data format and constraints");
    }

    // External service errors
    if lower_msg.contains("rpc") || lower_msg.contains("provider") || lower_msg.contains("node") {
        return ErrorClassification::new(
            ErrorCategory::ExternalService,
            ErrorSeverity::High,
            ErrorImpact::Degraded,
            "External service error",
        )
        .with_remediation("Check RPC provider status and consider fallback providers")
        .with_label("retryable", "true");
    }

    // Cryptography errors
    if lower_msg.contains("decrypt")
        || lower_msg.contains("encrypt")
        || lower_msg.contains("key")
        || lower_msg.contains("signing")
    {
        return ErrorClassification::new(
            ErrorCategory::Cryptography,
            ErrorSeverity::High,
            ErrorImpact::Blocking,
            "Cryptographic operation failed",
        )
        .with_remediation("Verify key material and cryptographic parameters")
        .with_label("security_related", "true");
    }

    // Default: Internal error
    ErrorClassification::new(
        ErrorCategory::Internal,
        ErrorSeverity::Medium,
        ErrorImpact::Isolated,
        format!("Internal error: {}", error_message),
    )
    .with_remediation("Review error details and SDK logs")
}

/// Error statistics tracker
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ErrorStatistics {
    /// Total error count
    pub total_errors: u64,
    /// Errors by category
    pub by_category: HashMap<ErrorCategory, u64>,
    /// Errors by severity
    pub by_severity: HashMap<ErrorSeverity, u64>,
    /// Retryable error count
    pub retryable_errors: u64,
    /// Critical error count
    pub critical_errors: u64,
}

impl ErrorStatistics {
    /// Create new error statistics
    pub fn new() -> Self {
        Self::default()
    }

    /// Record an error classification
    pub fn record(&mut self, classification: &ErrorClassification) {
        self.total_errors += 1;
        *self.by_category.entry(classification.category).or_insert(0) += 1;
        *self.by_severity.entry(classification.severity).or_insert(0) += 1;

        if classification.is_retryable() {
            self.retryable_errors += 1;
        }

        if classification.is_critical() {
            self.critical_errors += 1;
        }
    }

    /// Get error rate for a category
    pub fn category_rate(&self, category: ErrorCategory) -> f64 {
        if self.total_errors == 0 {
            return 0.0;
        }
        let count = self.by_category.get(&category).unwrap_or(&0);
        (*count as f64 / self.total_errors as f64) * 100.0
    }

    /// Get critical error percentage
    pub fn critical_error_rate(&self) -> f64 {
        if self.total_errors == 0 {
            return 0.0;
        }
        (self.critical_errors as f64 / self.total_errors as f64) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_error_categorization() {
        let classification = categorize_error("connection timeout", None);
        assert_eq!(classification.category, ErrorCategory::Timeout);
        assert_eq!(classification.severity, ErrorSeverity::Medium);
        assert!(classification.is_retryable());
    }

    #[test]
    fn test_gas_error_categorization() {
        let classification = categorize_error("insufficient gas for transaction", None);
        assert_eq!(classification.category, ErrorCategory::ResourceExhaustion);
        assert_eq!(classification.impact, ErrorImpact::Isolated);
    }

    #[test]
    fn test_authentication_error_categorization() {
        let classification = categorize_error("unauthorized access", None);
        assert_eq!(classification.category, ErrorCategory::Authentication);
        assert_eq!(classification.severity, ErrorSeverity::High);
        assert!(!classification.is_retryable());
    }

    #[test]
    fn test_rate_limit_categorization() {
        let classification = categorize_error("rate limit exceeded - too many requests", None);
        assert_eq!(classification.category, ErrorCategory::RateLimit);
        assert!(classification.is_retryable());
    }

    #[test]
    fn test_error_statistics() {
        let mut stats = ErrorStatistics::new();

        let network_err = categorize_error("connection timeout", None);
        let gas_err = categorize_error("insufficient gas", None);

        stats.record(&network_err);
        stats.record(&gas_err);

        assert_eq!(stats.total_errors, 2);
        assert_eq!(stats.retryable_errors, 1);
    }

    #[test]
    fn test_prometheus_labels() {
        let classification = ErrorClassification::new(
            ErrorCategory::Transaction,
            ErrorSeverity::High,
            ErrorImpact::Blocking,
            "Test error",
        )
        .with_label("chain", "ethereum");

        let labels = classification.to_prometheus_labels();
        assert_eq!(labels.get("chain").unwrap(), "ethereum");
        assert_eq!(labels.get("category").unwrap(), "Transaction");
        assert_eq!(labels.get("severity").unwrap(), "High");
    }
}
