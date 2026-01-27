//! Health check and status monitoring
//!
//! This module provides comprehensive health checking for SDK components,
//! including RPC providers, metrics collection, and system resources.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use sysinfo::System;

/// Health status for a component
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    /// Component is healthy and operational
    Healthy,
    /// Component is degraded but operational
    Degraded,
    /// Component is unhealthy and non-operational
    Unhealthy,
    /// Component status is unknown
    Unknown,
}

/// Component health information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    /// Component name
    pub name: String,
    /// Health status
    pub status: HealthStatus,
    /// Status message
    pub message: Option<String>,
    /// Last check timestamp
    pub last_check: u64,
    /// Response time in milliseconds
    pub response_time_ms: Option<u64>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl ComponentHealth {
    /// Create a new component health status
    pub fn new(name: impl Into<String>, status: HealthStatus) -> Self {
        Self {
            name: name.into(),
            status,
            message: None,
            last_check: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            response_time_ms: None,
            metadata: HashMap::new(),
        }
    }

    /// Set status message
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Set response time
    pub fn with_response_time(mut self, duration: Duration) -> Self {
        self.response_time_ms = Some(duration.as_millis() as u64);
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Check if component is healthy
    pub fn is_healthy(&self) -> bool {
        matches!(self.status, HealthStatus::Healthy)
    }

    /// Check if component is operational (healthy or degraded)
    pub fn is_operational(&self) -> bool {
        matches!(self.status, HealthStatus::Healthy | HealthStatus::Degraded)
    }
}

/// System health summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthSummary {
    /// Overall system status
    pub status: HealthStatus,
    /// Individual component statuses
    pub components: Vec<ComponentHealth>,
    /// System uptime in seconds
    pub uptime_seconds: u64,
    /// Timestamp of health check
    pub timestamp: u64,
    /// System resource information
    pub resources: SystemResources,
}

impl HealthSummary {
    /// Check if all components are healthy
    pub fn all_healthy(&self) -> bool {
        self.components.iter().all(|c| c.is_healthy())
    }

    /// Get unhealthy components
    pub fn unhealthy_components(&self) -> Vec<&ComponentHealth> {
        self.components
            .iter()
            .filter(|c| !c.is_operational())
            .collect()
    }
}

/// System resource information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemResources {
    /// CPU usage percentage
    pub cpu_usage_percent: f32,
    /// Memory usage in bytes
    pub memory_used_bytes: u64,
    /// Total memory in bytes
    pub memory_total_bytes: u64,
    /// Memory usage percentage
    pub memory_usage_percent: f32,
}

/// Health checker for monitoring component health
pub struct HealthChecker {
    components: Arc<Mutex<HashMap<String, ComponentHealth>>>,
    start_time: SystemTime,
}

impl HealthChecker {
    /// Create a new health checker
    pub fn new() -> Self {
        Self {
            components: Arc::new(Mutex::new(HashMap::new())),
            start_time: SystemTime::now(),
        }
    }

    /// Register or update a component health status
    pub fn update_component(&self, health: ComponentHealth) {
        if let Ok(mut components) = self.components.lock() {
            components.insert(health.name.clone(), health);
        }
    }

    /// Get health status for a specific component
    pub fn get_component(&self, name: &str) -> Option<ComponentHealth> {
        self.components
            .lock()
            .ok()
            .and_then(|components| components.get(name).cloned())
    }

    /// Get health summary for all components
    pub fn health_summary(&self) -> HealthSummary {
        let components: Vec<ComponentHealth> = self
            .components
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .values()
            .cloned()
            .collect();

        let overall_status = if components.is_empty() {
            HealthStatus::Unknown
        } else if components.iter().all(|c| c.is_healthy()) {
            HealthStatus::Healthy
        } else if components
            .iter()
            .any(|c| matches!(c.status, HealthStatus::Unhealthy))
        {
            HealthStatus::Unhealthy
        } else {
            HealthStatus::Degraded
        };

        let uptime_seconds = self.start_time.elapsed().unwrap_or_default().as_secs();

        HealthSummary {
            status: overall_status,
            components,
            uptime_seconds,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            resources: Self::get_system_resources(),
        }
    }

    /// Get system resource information
    fn get_system_resources() -> SystemResources {
        let mut sys = System::new_all();
        sys.refresh_all();

        let cpu_usage = sys.global_cpu_usage();
        let memory_used = sys.used_memory();
        let memory_total = sys.total_memory();
        let memory_percent = if memory_total > 0 {
            (memory_used as f32 / memory_total as f32) * 100.0
        } else {
            0.0
        };

        SystemResources {
            cpu_usage_percent: cpu_usage,
            memory_used_bytes: memory_used,
            memory_total_bytes: memory_total,
            memory_usage_percent: memory_percent,
        }
    }

    /// Remove a component from monitoring
    pub fn remove_component(&self, name: &str) {
        if let Ok(mut components) = self.components.lock() {
            components.remove(name);
        }
    }

    /// Clear all component statuses
    pub fn clear(&self) {
        if let Ok(mut components) = self.components.lock() {
            components.clear();
        }
    }

    /// Get component count
    pub fn component_count(&self) -> usize {
        self.components
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .len()
    }
}

impl Default for HealthChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_health_creation() {
        let health = ComponentHealth::new("test-component", HealthStatus::Healthy)
            .with_message("All systems operational")
            .with_metadata("version", "1.0.0");

        assert_eq!(health.name, "test-component");
        assert_eq!(health.status, HealthStatus::Healthy);
        assert!(health.is_healthy());
        assert!(health.is_operational());
    }

    #[test]
    fn test_health_checker() {
        let checker = HealthChecker::new();

        let health1 = ComponentHealth::new("rpc-provider", HealthStatus::Healthy);
        let health2 = ComponentHealth::new("metrics-collector", HealthStatus::Degraded);

        checker.update_component(health1);
        checker.update_component(health2);

        assert_eq!(checker.component_count(), 2);

        let summary = checker.health_summary();
        assert_eq!(summary.status, HealthStatus::Degraded);
        assert_eq!(summary.components.len(), 2);
    }

    #[test]
    fn test_unhealthy_components() {
        let checker = HealthChecker::new();

        checker.update_component(ComponentHealth::new("comp1", HealthStatus::Healthy));
        checker.update_component(ComponentHealth::new("comp2", HealthStatus::Unhealthy));
        checker.update_component(ComponentHealth::new("comp3", HealthStatus::Degraded));

        let summary = checker.health_summary();
        let unhealthy = summary.unhealthy_components();

        assert_eq!(unhealthy.len(), 1);
        assert_eq!(unhealthy[0].name, "comp2");
    }

    #[test]
    fn test_system_resources() {
        let resources = HealthChecker::get_system_resources();
        assert!(resources.memory_total_bytes > 0);
        assert!(resources.memory_usage_percent >= 0.0);
        assert!(resources.cpu_usage_percent >= 0.0);
    }
}
