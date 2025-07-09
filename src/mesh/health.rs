//! Universal Mesh Health Monitoring
//!
//! Provides comprehensive health monitoring for mesh networks,
//! including node health, network partitions, and performance metrics.
//! This module contains universal health monitoring primitives that can be
//! extended by context-specific plugins.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;

/// Universal health status of a node
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    /// Node is healthy and responsive
    Healthy,
    
    /// Node is experiencing degraded performance
    Degraded {
        issues: Vec<HealthIssue>,
        severity: HealthSeverity,
    },
    
    /// Node is unhealthy but still reachable
    Unhealthy {
        issues: Vec<HealthIssue>,
        last_response: DateTime<Utc>,
    },
    
    /// Node is unreachable
    Unreachable {
        last_seen: DateTime<Utc>,
        attempts: u32,
    },
    
    /// Node status is unknown
    Unknown,
}

/// Universal health monitoring service for mesh networks
pub struct HealthMonitor {
    /// Local node ID
    local_node_id: Uuid,
    
    /// Health check interval
    check_interval: Duration,
    
    /// Node health status
    node_health: Arc<RwLock<HashMap<Uuid, NodeHealthStatus>>>,
    
    /// Performance metrics
    metrics: Arc<RwLock<PerformanceMetrics>>,
    
    /// Health check task handle
    task_handle: Option<tokio::task::JoinHandle<()>>,
    
    /// Running state
    is_running: Arc<RwLock<bool>>,
    
    /// Health configuration
    config: HealthConfig,
    
    /// Health providers for context-specific monitoring
    providers: Vec<Box<dyn HealthProvider>>,
}

/// Detailed health status for a node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeHealthStatus {
    /// Node ID
    pub node_id: Uuid,
    
    /// Current health status
    pub status: HealthStatus,
    
    /// Last health check timestamp
    pub last_check: DateTime<Utc>,
    
    /// Response time in milliseconds
    pub response_time_ms: f64,
    
    /// Health metrics
    pub metrics: NodeHealthMetrics,
    
    /// Health history (limited to recent entries)
    pub history: Vec<HealthCheckResult>,
    
    /// Context-specific health data
    pub context_data: HashMap<String, serde_json::Value>,
}

/// Universal health metrics for a node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeHealthMetrics {
    /// CPU usage percentage (0.0 to 100.0)
    pub cpu_usage: f64,
    
    /// Memory usage percentage (0.0 to 100.0)
    pub memory_usage: f64,
    
    /// Disk usage percentage (0.0 to 100.0)
    pub disk_usage: f64,
    
    /// Network latency in milliseconds
    pub network_latency: f64,
    
    /// Number of active connections
    pub active_connections: usize,
    
    /// Error rate (errors per minute)
    pub error_rate: f64,
    
    /// Uptime in seconds
    pub uptime_seconds: u64,
    
    /// Last update timestamp
    pub last_update: DateTime<Utc>,
    
    /// Context-specific metrics
    pub context_metrics: HashMap<String, serde_json::Value>,
}

/// Health check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResult {
    /// Check timestamp
    pub timestamp: DateTime<Utc>,
    
    /// Check result
    pub result: HealthCheckOutcome,
    
    /// Response time in milliseconds
    pub response_time_ms: f64,
    
    /// Any issues detected
    pub issues: Vec<HealthIssue>,
    
    /// Context-specific check data
    pub context_data: HashMap<String, serde_json::Value>,
}

/// Outcome of a health check
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthCheckOutcome {
    /// Health check succeeded
    Success,
    
    /// Health check failed
    Failed {
        error: String,
        error_code: String,
    },
    
    /// Health check timed out
    Timeout,
    
    /// Node was unreachable
    Unreachable,
}

/// Universal health issues that can affect nodes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthIssue {
    /// High CPU usage
    HighCpuUsage {
        current: f64,
        threshold: f64,
    },
    
    /// High memory usage
    HighMemoryUsage {
        current: f64,
        threshold: f64,
    },
    
    /// High disk usage
    HighDiskUsage {
        current: f64,
        threshold: f64,
    },
    
    /// High network latency
    HighLatency {
        current: f64,
        threshold: f64,
    },
    
    /// High error rate
    HighErrorRate {
        current: f64,
        threshold: f64,
    },
    
    /// Network connectivity issues
    NetworkConnectivity {
        description: String,
    },
    
    /// Resource exhaustion
    ResourceExhaustion {
        resource: String,
        description: String,
    },
    
    /// Service unavailable
    ServiceUnavailable {
        service: String,
        reason: String,
    },
    
    /// Custom health issue
    Custom {
        issue_type: String,
        description: String,
        severity: HealthSeverity,
    },
}

/// Severity levels for health issues
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum HealthSeverity {
    /// Low severity - monitoring only
    Low,
    
    /// Medium severity - may affect performance
    Medium,
    
    /// High severity - significant impact
    High,
    
    /// Critical severity - immediate attention required
    Critical,
}

/// Performance metrics for the entire mesh
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Average response time across all nodes
    pub avg_response_time: f64,
    
    /// Total number of health checks performed
    pub total_health_checks: u64,
    
    /// Number of failed health checks
    pub failed_health_checks: u64,
    
    /// Health check success rate
    pub success_rate: f64,
    
    /// Average node health score
    pub avg_health_score: f64,
    
    /// Metrics collection period
    pub collection_period: Duration,
    
    /// Last metrics update
    pub last_update: DateTime<Utc>,
    
    /// Context-specific performance metrics
    pub context_metrics: HashMap<String, serde_json::Value>,
}

/// Health monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthConfig {
    /// Health check interval in seconds
    pub check_interval: u64,
    
    /// Health check timeout in seconds
    pub check_timeout: u64,
    
    /// Maximum number of health history entries to keep
    pub max_history_entries: usize,
    
    /// Stale health threshold in minutes
    pub stale_threshold_minutes: i64,
    
    /// CPU usage threshold for warnings
    pub cpu_warning_threshold: f64,
    
    /// Memory usage threshold for warnings
    pub memory_warning_threshold: f64,
    
    /// Disk usage threshold for warnings
    pub disk_warning_threshold: f64,
    
    /// Network latency threshold for warnings (ms)
    pub latency_warning_threshold: f64,
    
    /// Error rate threshold for warnings (errors per minute)
    pub error_rate_warning_threshold: f64,
    
    /// Enable automatic issue detection
    pub auto_issue_detection: bool,
    
    /// Context-specific configuration
    pub context_config: HashMap<String, serde_json::Value>,
}

impl Default for HealthConfig {
    fn default() -> Self {
        Self {
            check_interval: 30,
            check_timeout: 5,
            max_history_entries: 100,
            stale_threshold_minutes: 5,
            cpu_warning_threshold: 80.0,
            memory_warning_threshold: 85.0,
            disk_warning_threshold: 90.0,
            latency_warning_threshold: 1000.0,
            error_rate_warning_threshold: 10.0,
            auto_issue_detection: true,
            context_config: HashMap::new(),
        }
    }
}

/// Health event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthEvent {
    /// Node health status changed
    HealthStatusChanged {
        node_id: Uuid,
        old_status: HealthStatus,
        new_status: HealthStatus,
    },
    
    /// Health check completed
    HealthCheckCompleted {
        node_id: Uuid,
        result: HealthCheckResult,
    },
    
    /// Health issue detected
    IssueDetected {
        node_id: Uuid,
        issue: HealthIssue,
    },
    
    /// Health issue resolved
    IssueResolved {
        node_id: Uuid,
        issue_type: String,
    },
    
    /// Network partition detected
    NetworkPartitionDetected {
        unreachable_nodes: Vec<Uuid>,
        partition_ratio: f64,
    },
    
    /// Network partition resolved
    NetworkPartitionResolved,
}

impl HealthMonitor {
    /// Create a new health monitor
    pub fn new(
        local_node_id: Uuid,
        config: Option<HealthConfig>,
    ) -> Self {
        let config = config.unwrap_or_default();
        let check_interval = Duration::from_secs(config.check_interval);
        
        info!("Initializing health monitor for node: {}", local_node_id);
        
        Self {
            local_node_id,
            check_interval,
            node_health: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(RwLock::new(PerformanceMetrics {
                avg_response_time: 0.0,
                total_health_checks: 0,
                failed_health_checks: 0,
                success_rate: 1.0,
                avg_health_score: 1.0,
                collection_period: check_interval,
                last_update: Utc::now(),
                context_metrics: HashMap::new(),
            })),
            task_handle: None,
            is_running: Arc::new(RwLock::new(false)),
            config,
            providers: Vec::new(),
        }
    }
    
    /// Add a health provider for context-specific monitoring
    pub fn add_provider(&mut self, provider: Box<dyn HealthProvider>) {
        info!("Adding health provider: {}", provider.name());
        self.providers.push(provider);
    }
    
    /// Start the health monitoring service
    pub async fn start(&mut self) -> Result<()> {
        let mut is_running = self.is_running.write().await;
        if *is_running {
            return Ok(());
        }
        
        *is_running = true;
        drop(is_running);
        
        // Initialize health providers
        for provider in &mut self.providers {
            provider.initialize(&self.config).await?;
        }
        
        // Start health check task
        let task_handle = self.start_health_check_task().await;
        self.task_handle = Some(task_handle);
        
        info!("Health monitor started for node {}", self.local_node_id);
        Ok(())
    }
    
    /// Stop the health monitoring service
    pub async fn stop(&mut self) -> Result<()> {
        let mut is_running = self.is_running.write().await;
        if !*is_running {
            return Ok(());
        }
        
        *is_running = false;
        drop(is_running);
        
        // Stop health check task
        if let Some(handle) = self.task_handle.take() {
            handle.abort();
        }
        
        // Cleanup health providers
        for provider in &mut self.providers {
            provider.cleanup().await?;
        }
        
        info!("Health monitor stopped for node {}", self.local_node_id);
        Ok(())
    }
    
    /// Get health status for a specific node
    pub async fn get_node_health(&self, node_id: Uuid) -> Option<NodeHealthStatus> {
        let health = self.node_health.read().await;
        health.get(&node_id).cloned()
    }
    
    /// Get health status for all nodes
    pub async fn get_all_health(&self) -> HashMap<Uuid, NodeHealthStatus> {
        let health = self.node_health.read().await;
        health.clone()
    }
    
    /// Update node health status
    pub async fn update_node_health(&self, status: NodeHealthStatus) -> Option<HealthEvent> {
        let mut health = self.node_health.write().await;
        
        let event = if let Some(existing) = health.get(&status.node_id) {
            if existing.status != status.status {
                Some(HealthEvent::HealthStatusChanged {
                    node_id: status.node_id,
                    old_status: existing.status.clone(),
                    new_status: status.status.clone(),
                })
            } else {
                None
            }
        } else {
            Some(HealthEvent::HealthStatusChanged {
                node_id: status.node_id,
                old_status: HealthStatus::Unknown,
                new_status: status.status.clone(),
            })
        };
        
        health.insert(status.node_id, status);
        event
    }
    
    /// Perform a health check on a specific node
    pub async fn check_node_health(&self, node_id: Uuid) -> Result<HealthCheckResult> {
        let start_time = std::time::Instant::now();
        
        // Use health providers to perform context-specific checks
        let mut issues = Vec::new();
        let mut context_data = HashMap::new();
        let mut outcome = HealthCheckOutcome::Success;
        
        for provider in &self.providers {
            match provider.check_node_health(node_id).await {
                Ok(result) => {
                    issues.extend(result.issues);
                    context_data.extend(result.context_data);
                    if result.result != HealthCheckOutcome::Success {
                        outcome = result.result;
                    }
                }
                Err(e) => {
                    warn!("Health provider {} failed: {}", provider.name(), e);
                    outcome = HealthCheckOutcome::Failed {
                        error: e.to_string(),
                        error_code: "PROVIDER_ERROR".to_string(),
                    };
                }
            }
        }
        
        let response_time = start_time.elapsed().as_millis() as f64;
        
        // Detect additional issues if auto-detection is enabled
        if self.config.auto_issue_detection {
            if let Some(node_status) = self.get_node_health(node_id).await {
                issues.extend(self.detect_metric_issues(&node_status.metrics));
            }
        }
        
        Ok(HealthCheckResult {
            timestamp: Utc::now(),
            result: outcome,
            response_time_ms: response_time,
            issues,
            context_data,
        })
    }
    
    /// Check if the network is partitioned
    pub async fn is_network_partitioned(&self) -> bool {
        let health = self.node_health.read().await;
        let total_nodes = health.len();
        
        if total_nodes == 0 {
            return false;
        }
        
        let unreachable_nodes = health.values()
            .filter(|status| matches!(status.status, HealthStatus::Unreachable { .. }))
            .count();
        
        // Consider partitioned if more than 30% of nodes are unreachable
        (unreachable_nodes as f64 / total_nodes as f64) > 0.3
    }
    
    /// Get performance metrics
    pub async fn get_performance_metrics(&self) -> PerformanceMetrics {
        let metrics = self.metrics.read().await;
        metrics.clone()
    }
    
    /// Get health configuration
    pub fn get_config(&self) -> &HealthConfig {
        &self.config
    }
    
    /// Update health configuration
    pub fn update_config(&mut self, config: HealthConfig) {
        self.config = config;
        self.check_interval = Duration::from_secs(self.config.check_interval);
        debug!("Updated health monitor configuration");
    }
    
    /// Start the health check task
    async fn start_health_check_task(&self) -> tokio::task::JoinHandle<()> {
        let local_node_id = self.local_node_id;
        let check_interval = self.check_interval;
        let node_health = self.node_health.clone();
        let metrics = self.metrics.clone();
        let is_running = self.is_running.clone();
        let config = self.config.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(check_interval);
            
            while *is_running.read().await {
                interval.tick().await;
                
                // Update metrics
                if let Err(e) = Self::update_metrics(&node_health, &metrics).await {
                    warn!("Failed to update metrics: {}", e);
                }
                
                // Clean up stale health records
                Self::cleanup_stale_health(&node_health, &config).await;
            }
        })
    }
    
    /// Update performance metrics
    async fn update_metrics(
        node_health: &Arc<RwLock<HashMap<Uuid, NodeHealthStatus>>>,
        metrics: &Arc<RwLock<PerformanceMetrics>>,
    ) -> Result<()> {
        let health = node_health.read().await;
        let mut metrics = metrics.write().await;
        
        if !health.is_empty() {
            let total_response_time: f64 = health.values()
                .map(|status| status.response_time_ms)
                .sum();
            
            metrics.avg_response_time = total_response_time / health.len() as f64;
            
            let health_scores: Vec<f64> = health.values()
                .map(|status| status.status.score())
                .collect();
            
            if !health_scores.is_empty() {
                metrics.avg_health_score = health_scores.iter().sum::<f64>() / health_scores.len() as f64;
            }
        }
        
        metrics.last_update = Utc::now();
        Ok(())
    }
    
    /// Clean up stale health records
    async fn cleanup_stale_health(
        node_health: &Arc<RwLock<HashMap<Uuid, NodeHealthStatus>>>,
        config: &HealthConfig,
    ) {
        let mut health = node_health.write().await;
        let stale_threshold = chrono::Duration::minutes(config.stale_threshold_minutes);
        let now = Utc::now();
        
        health.retain(|_, status| {
            now.signed_duration_since(status.last_check) < stale_threshold
        });
    }
    
    /// Detect issues based on node metrics
    fn detect_metric_issues(&self, metrics: &NodeHealthMetrics) -> Vec<HealthIssue> {
        let mut issues = Vec::new();
        
        if metrics.cpu_usage > self.config.cpu_warning_threshold {
            issues.push(HealthIssue::HighCpuUsage {
                current: metrics.cpu_usage,
                threshold: self.config.cpu_warning_threshold,
            });
        }
        
        if metrics.memory_usage > self.config.memory_warning_threshold {
            issues.push(HealthIssue::HighMemoryUsage {
                current: metrics.memory_usage,
                threshold: self.config.memory_warning_threshold,
            });
        }
        
        if metrics.disk_usage > self.config.disk_warning_threshold {
            issues.push(HealthIssue::HighDiskUsage {
                current: metrics.disk_usage,
                threshold: self.config.disk_warning_threshold,
            });
        }
        
        if metrics.network_latency > self.config.latency_warning_threshold {
            issues.push(HealthIssue::HighLatency {
                current: metrics.network_latency,
                threshold: self.config.latency_warning_threshold,
            });
        }
        
        if metrics.error_rate > self.config.error_rate_warning_threshold {
            issues.push(HealthIssue::HighErrorRate {
                current: metrics.error_rate,
                threshold: self.config.error_rate_warning_threshold,
            });
        }
        
        issues
    }
}

impl HealthStatus {
    /// Get a numeric health score (0.0 to 1.0)
    pub fn score(&self) -> f64 {
        match self {
            HealthStatus::Healthy => 1.0,
            HealthStatus::Degraded { severity, .. } => match severity {
                HealthSeverity::Low => 0.8,
                HealthSeverity::Medium => 0.6,
                HealthSeverity::High => 0.4,
                HealthSeverity::Critical => 0.2,
            },
            HealthStatus::Unhealthy { .. } => 0.3,
            HealthStatus::Unreachable { .. } => 0.0,
            HealthStatus::Unknown => 0.5,
        }
    }
    
    /// Check if the status indicates the node is available
    pub fn is_available(&self) -> bool {
        matches!(self, HealthStatus::Healthy | HealthStatus::Degraded { .. })
    }
    
    /// Check if the status indicates a critical condition
    pub fn is_critical(&self) -> bool {
        match self {
            HealthStatus::Degraded { severity, .. } => *severity == HealthSeverity::Critical,
            HealthStatus::Unhealthy { .. } | HealthStatus::Unreachable { .. } => true,
            _ => false,
        }
    }
}

impl Default for NodeHealthMetrics {
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_usage: 0.0,
            disk_usage: 0.0,
            network_latency: 0.0,
            active_connections: 0,
            error_rate: 0.0,
            uptime_seconds: 0,
            last_update: Utc::now(),
            context_metrics: HashMap::new(),
        }
    }
}

/// Trait for context-specific health providers
#[async_trait::async_trait]
pub trait HealthProvider: Send + Sync {
    /// Provider name
    fn name(&self) -> &str;
    
    /// Initialize the health provider
    async fn initialize(&mut self, config: &HealthConfig) -> Result<()>;
    
    /// Cleanup provider resources
    async fn cleanup(&mut self) -> Result<()>;
    
    /// Perform a health check on a node
    async fn check_node_health(&self, node_id: Uuid) -> Result<HealthCheckResult>;
    
    /// Handle health events
    async fn handle_event(&self, event: &HealthEvent) -> Result<()>;
    
    /// Get provider-specific metrics
    async fn get_metrics(&self) -> Result<HashMap<String, serde_json::Value>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_status_score() {
        assert_eq!(HealthStatus::Healthy.score(), 1.0);
        assert_eq!(HealthStatus::Unreachable { last_seen: Utc::now(), attempts: 3 }.score(), 0.0);
        assert_eq!(HealthStatus::Unknown.score(), 0.5);
        
        let degraded = HealthStatus::Degraded {
            issues: Vec::new(),
            severity: HealthSeverity::Medium,
        };
        assert_eq!(degraded.score(), 0.6);
    }

    #[test]
    fn test_health_status_availability() {
        assert!(HealthStatus::Healthy.is_available());
        assert!(HealthStatus::Degraded {
            issues: Vec::new(),
            severity: HealthSeverity::Low,
        }.is_available());
        assert!(!HealthStatus::Unreachable { last_seen: Utc::now(), attempts: 3 }.is_available());
        assert!(!HealthStatus::Unhealthy { issues: Vec::new(), last_response: Utc::now() }.is_available());
    }

    #[test]
    fn test_health_status_critical() {
        assert!(!HealthStatus::Healthy.is_critical());
        assert!(HealthStatus::Degraded {
            issues: Vec::new(),
            severity: HealthSeverity::Critical,
        }.is_critical());
        assert!(HealthStatus::Unreachable { last_seen: Utc::now(), attempts: 3 }.is_critical());
    }

    #[test]
    fn test_health_metrics_default() {
        let metrics = NodeHealthMetrics::default();
        assert_eq!(metrics.cpu_usage, 0.0);
        assert_eq!(metrics.memory_usage, 0.0);
        assert_eq!(metrics.active_connections, 0);
        assert!(metrics.context_metrics.is_empty());
    }

    #[test]
    fn test_health_config_default() {
        let config = HealthConfig::default();
        assert_eq!(config.check_interval, 30);
        assert_eq!(config.cpu_warning_threshold, 80.0);
        assert!(config.auto_issue_detection);
    }

    #[tokio::test]
    async fn test_health_monitor_creation() {
        let node_id = Uuid::new_v4();
        let monitor = HealthMonitor::new(node_id, None);
        
        assert_eq!(monitor.local_node_id, node_id);
        assert!(!*monitor.is_running.read().await);
        assert_eq!(monitor.providers.len(), 0);
    }

    #[tokio::test]
    async fn test_health_monitor_node_management() {
        let node_id = Uuid::new_v4();
        let monitor = HealthMonitor::new(node_id, None);
        
        let test_status = NodeHealthStatus {
            node_id: Uuid::new_v4(),
            status: HealthStatus::Healthy,
            last_check: Utc::now(),
            response_time_ms: 50.0,
            metrics: NodeHealthMetrics::default(),
            history: Vec::new(),
            context_data: HashMap::new(),
        };
        
        // Update node health
        let event = monitor.update_node_health(test_status.clone()).await;
        assert!(event.is_some());
        
        // Get node health
        let retrieved = monitor.get_node_health(test_status.node_id).await;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().node_id, test_status.node_id);
    }
}
