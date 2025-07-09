//! Universal Mesh Event System
//!
//! Provides a universal event system for mesh networks that can be extended
//! by context-specific plugins. This module contains universal event primitives
//! and a plugin-based architecture for context-specific event types.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;

/// Universal mesh event system
pub struct EventSystem {
    /// Local node ID
    local_node_id: Uuid,
    
    /// Event handlers for different patterns
    event_handlers: Arc<RwLock<HashMap<String, EventHandler>>>,
    
    /// Event history for pattern recognition
    event_history: Arc<RwLock<Vec<MeshEvent>>>,
    
    /// Event providers for context-specific events
    providers: Vec<Box<dyn EventProvider>>,
    
    /// Event configuration
    config: EventConfig,
    
    /// Running state
    is_running: Arc<RwLock<bool>>,
}

/// Handler for specific event types
pub struct EventHandler {
    /// Event pattern this handler responds to
    pub event_pattern: String,
    
    /// Handler function
    pub handler: Arc<dyn Fn(MeshEvent) -> Result<()> + Send + Sync>,
    
    /// Handler metadata
    pub metadata: HashMap<String, String>,
}

impl std::fmt::Debug for EventHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventHandler")
            .field("event_pattern", &self.event_pattern)
            .field("handler", &"<function>")
            .field("metadata", &self.metadata)
            .finish()
    }
}

/// Universal mesh event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshEvent {
    /// Event ID
    pub event_id: Uuid,
    
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
    
    /// Source node ID
    pub source_node: Uuid,
    
    /// Event type
    pub event_type: EventType,
    
    /// Event payload
    pub payload: EventPayload,
    
    /// Event metadata
    pub metadata: HashMap<String, String>,
    
    /// Event propagation path
    pub propagation_path: Vec<Uuid>,
    
    /// Event correlation ID for related events
    pub correlation_id: Option<Uuid>,
    
    /// Event priority
    pub priority: EventPriority,
}

/// Universal event types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EventType {
    /// Node lifecycle events
    NodeLifecycle {
        lifecycle_type: NodeLifecycleType,
    },
    
    /// Communication events
    Communication {
        communication_type: CommunicationType,
    },
    
    /// Resource management events
    Resource {
        resource_type: ResourceEventType,
    },
    
    /// Network topology events
    Topology {
        topology_type: TopologyEventType,
    },
    
    /// Health and monitoring events
    Health {
        health_type: HealthEventType,
    },
    
    /// Security events
    Security {
        security_type: SecurityEventType,
    },
    
    /// Performance events
    Performance {
        performance_type: PerformanceEventType,
    },
    
    /// Context-specific events (handled by providers)
    ContextSpecific {
        context: String,
        event_subtype: String,
        provider_data: serde_json::Value,
    },
    
    /// Generic events for extensibility
    Generic {
        category: String,
        subcategory: Option<String>,
    },
}

/// Node lifecycle event types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NodeLifecycleType {
    /// Node joining the mesh
    NodeJoined,
    
    /// Node leaving the mesh
    NodeLeft,
    
    /// Node capabilities updated
    CapabilitiesUpdated,
    
    /// Node status changed
    StatusChanged,
    
    /// Node configuration updated
    ConfigurationUpdated,
}

/// Communication event types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CommunicationType {
    /// Message sent
    MessageSent,
    
    /// Message received
    MessageReceived,
    
    /// Broadcast initiated
    BroadcastInitiated,
    
    /// Communication established
    CommunicationEstablished,
    
    /// Communication failed
    CommunicationFailed,
    
    /// Protocol negotiation
    ProtocolNegotiation,
}

/// Resource event types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ResourceEventType {
    /// Resource created
    ResourceCreated,
    
    /// Resource updated
    ResourceUpdated,
    
    /// Resource deleted
    ResourceDeleted,
    
    /// Resource accessed
    ResourceAccessed,
    
    /// Resource conflict
    ResourceConflict,
    
    /// Resource synchronized
    ResourceSynchronized,
}

/// Topology event types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TopologyEventType {
    /// Network partition detected
    NetworkPartition,
    
    /// Network partition resolved
    PartitionResolved,
    
    /// Topology changed
    TopologyChanged,
    
    /// Connection established
    ConnectionEstablished,
    
    /// Connection lost
    ConnectionLost,
    
    /// Route discovered
    RouteDiscovered,
}

/// Health event types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthEventType {
    /// Health status changed
    HealthStatusChanged,
    
    /// Health check completed
    HealthCheckCompleted,
    
    /// Health issue detected
    IssueDetected,
    
    /// Health issue resolved
    IssueResolved,
    
    /// Performance degradation
    PerformanceDegradation,
    
    /// Recovery completed
    RecoveryCompleted,
}

/// Security event types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SecurityEventType {
    /// Authentication attempt
    AuthenticationAttempt,
    
    /// Authorization check
    AuthorizationCheck,
    
    /// Security violation
    SecurityViolation,
    
    /// Trust level changed
    TrustLevelChanged,
    
    /// Encryption established
    EncryptionEstablished,
    
    /// Security audit
    SecurityAudit,
}

/// Performance event types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PerformanceEventType {
    /// Latency measurement
    LatencyMeasurement,
    
    /// Throughput measurement
    ThroughputMeasurement,
    
    /// Resource utilization
    ResourceUtilization,
    
    /// Performance threshold exceeded
    ThresholdExceeded,
    
    /// Performance optimization
    PerformanceOptimization,
    
    /// Benchmark completed
    BenchmarkCompleted,
}

/// Event payload containing specific event data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventPayload {
    /// Node lifecycle data
    NodeLifecycle {
        node_id: Uuid,
        node_info: Option<NodeInfo>,
        previous_state: Option<String>,
        new_state: String,
        reason: Option<String>,
    },
    
    /// Communication data
    Communication {
        participants: Vec<Uuid>,
        message_id: Option<Uuid>,
        protocol: String,
        status: String,
        error: Option<String>,
    },
    
    /// Resource data
    Resource {
        resource_id: String,
        resource_type: String,
        operation: String,
        affected_nodes: Vec<Uuid>,
        conflict_info: Option<ConflictInfo>,
    },
    
    /// Topology data
    Topology {
        affected_nodes: Vec<Uuid>,
        change_description: String,
        topology_metrics: Option<TopologyMetrics>,
    },
    
    /// Health data
    Health {
        node_id: Uuid,
        health_status: String,
        metrics: Option<HealthMetrics>,
        issues: Vec<String>,
    },
    
    /// Security data
    Security {
        principal: Option<String>,
        resource: Option<String>,
        action: String,
        result: String,
        risk_level: SecurityRiskLevel,
    },
    
    /// Performance data
    Performance {
        metric_name: String,
        metric_value: f64,
        metric_unit: String,
        threshold: Option<f64>,
        node_id: Option<Uuid>,
    },
    
    /// Context-specific data (handled by providers)
    ContextSpecific {
        context: String,
        data: serde_json::Value,
    },
    
    /// Generic data for extensibility
    Generic {
        data: HashMap<String, serde_json::Value>,
    },
}

/// Event priority levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventPriority {
    /// Low priority - informational
    Low,
    
    /// Normal priority - standard events
    Normal,
    
    /// High priority - important events
    High,
    
    /// Critical priority - urgent events
    Critical,
    
    /// Emergency priority - immediate attention required
    Emergency,
}

/// Security risk levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecurityRiskLevel {
    /// No risk
    None,
    
    /// Low risk
    Low,
    
    /// Medium risk
    Medium,
    
    /// High risk
    High,
    
    /// Critical risk
    Critical,
}

/// Node information for events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    /// Node name
    pub name: String,
    
    /// Node capabilities
    pub capabilities: Vec<String>,
    
    /// Node version
    pub version: String,
    
    /// Node metadata
    pub metadata: HashMap<String, String>,
}

/// Conflict information for resource events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictInfo {
    /// Conflicting nodes
    pub conflicting_nodes: Vec<Uuid>,
    
    /// Conflict type
    pub conflict_type: String,
    
    /// Conflict description
    pub description: String,
    
    /// Resolution strategy
    pub resolution_strategy: Option<String>,
}

/// Topology metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyMetrics {
    /// Number of nodes
    pub node_count: usize,
    
    /// Number of connections
    pub connection_count: usize,
    
    /// Network diameter
    pub network_diameter: Option<u32>,
    
    /// Clustering coefficient
    pub clustering_coefficient: Option<f64>,
}

/// Health metrics for events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMetrics {
    /// CPU usage percentage
    pub cpu_usage: f64,
    
    /// Memory usage percentage
    pub memory_usage: f64,
    
    /// Network latency in milliseconds
    pub network_latency: f64,
    
    /// Error rate
    pub error_rate: f64,
}

/// Event system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventConfig {
    /// Maximum events to keep in history
    pub max_history_size: usize,
    
    /// Event retention period in hours
    pub retention_period_hours: u64,
    
    /// Enable event persistence
    pub enable_persistence: bool,
    
    /// Event batch size for processing
    pub batch_size: usize,
    
    /// Event processing interval in milliseconds
    pub processing_interval_ms: u64,
    
    /// Context-specific configuration
    pub context_config: HashMap<String, serde_json::Value>,
}

impl Default for EventConfig {
    fn default() -> Self {
        Self {
            max_history_size: 10000,
            retention_period_hours: 24,
            enable_persistence: false,
            batch_size: 100,
            processing_interval_ms: 1000,
            context_config: HashMap::new(),
        }
    }
}

/// Event statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventStatistics {
    /// Total events processed
    pub total_events: u64,
    
    /// Events per minute
    pub events_per_minute: f64,
    
    /// Events by type
    pub events_by_type: HashMap<String, u64>,
    
    /// Events by priority
    pub events_by_priority: HashMap<String, u64>,
    
    /// Average processing time in milliseconds
    pub avg_processing_time_ms: f64,
    
    /// Last update timestamp
    pub last_update: DateTime<Utc>,
}

impl EventSystem {
    /// Create a new event system
    pub fn new(
        local_node_id: Uuid,
        config: Option<EventConfig>,
    ) -> Self {
        let config = config.unwrap_or_default();
        
        info!("Initializing event system for node: {}", local_node_id);
        
        Self {
            local_node_id,
            event_handlers: Arc::new(RwLock::new(HashMap::new())),
            event_history: Arc::new(RwLock::new(Vec::new())),
            providers: Vec::new(),
            config,
            is_running: Arc::new(RwLock::new(false)),
        }
    }
    
    /// Add an event provider for context-specific events
    pub fn add_provider(&mut self, provider: Box<dyn EventProvider>) {
        info!("Adding event provider: {}", provider.name());
        self.providers.push(provider);
    }
    
    /// Start the event system
    pub async fn start(&mut self) -> Result<()> {
        let mut is_running = self.is_running.write().await;
        if *is_running {
            return Ok(());
        }
        
        *is_running = true;
        drop(is_running);
        
        // Initialize event providers
        for provider in &mut self.providers {
            provider.initialize(&self.config).await?;
        }
        
        info!("Event system started for node {}", self.local_node_id);
        Ok(())
    }
    
    /// Stop the event system
    pub async fn stop(&mut self) -> Result<()> {
        let mut is_running = self.is_running.write().await;
        if !*is_running {
            return Ok(());
        }
        
        *is_running = false;
        drop(is_running);
        
        // Cleanup event providers
        for provider in &mut self.providers {
            provider.cleanup().await?;
        }
        
        info!("Event system stopped for node {}", self.local_node_id);
        Ok(())
    }
    
    /// Publish an event
    pub async fn publish_event(&self, event: MeshEvent) -> Result<()> {
        // Add to event history
        let mut history = self.event_history.write().await;
        history.push(event.clone());
        
        // Maintain history size limit
        if history.len() > self.config.max_history_size {
            let excess = history.len() - self.config.max_history_size;
            history.drain(0..excess);
        }
        drop(history);
        
        // Process with event handlers
        let handlers = self.event_handlers.read().await;
        for (pattern, handler) in handlers.iter() {
            if event.matches_pattern(pattern) {
                if let Err(e) = (handler.handler)(event.clone()) {
                    warn!("Event handler failed for pattern {}: {}", pattern, e);
                }
            }
        }
        drop(handlers);
        
        // Process with event providers
        for provider in &self.providers {
            if let Err(e) = provider.handle_event(&event).await {
                warn!("Event provider {} failed: {}", provider.name(), e);
            }
        }
        
        debug!("Published event: {} ({})", event.event_id, event.event_type.category());
        Ok(())
    }
    
    /// Register an event handler
    pub async fn register_handler<F>(&self, pattern: String, handler: F) -> Result<()>
    where
        F: Fn(MeshEvent) -> Result<()> + Send + Sync + 'static,
    {
        let event_handler = EventHandler {
            event_pattern: pattern.clone(),
            handler: Arc::new(handler),
            metadata: HashMap::new(),
        };
        
        let mut handlers = self.event_handlers.write().await;
        handlers.insert(pattern.clone(), event_handler);
        
        info!("Registered event handler for pattern: {}", pattern);
        Ok(())
    }
    
    /// Get event history matching a pattern
    pub async fn get_event_history(&self, pattern: Option<&str>) -> Vec<MeshEvent> {
        let history = self.event_history.read().await;
        match pattern {
            Some(p) => history.iter()
                .filter(|event| event.matches_pattern(p))
                .cloned()
                .collect(),
            None => history.clone(),
        }
    }
    
    /// Get event statistics
    pub async fn get_statistics(&self) -> EventStatistics {
        let history = self.event_history.read().await;
        let now = Utc::now();
        let one_minute_ago = now - chrono::Duration::minutes(1);
        
        let recent_events: Vec<_> = history.iter()
            .filter(|event| event.timestamp > one_minute_ago)
            .collect();
        
        let mut events_by_type = HashMap::new();
        let mut events_by_priority = HashMap::new();
        
        for event in &*history {
            let type_key = event.event_type.category().to_string();
            *events_by_type.entry(type_key).or_insert(0) += 1;
            
            let priority_key = format!("{:?}", event.priority);
            *events_by_priority.entry(priority_key).or_insert(0) += 1;
        }
        
        EventStatistics {
            total_events: history.len() as u64,
            events_per_minute: recent_events.len() as f64,
            events_by_type,
            events_by_priority,
            avg_processing_time_ms: 1.0, // Would be calculated from actual processing times
            last_update: now,
        }
    }
    
    /// Create a node lifecycle event
    pub fn create_node_event(
        &self,
        lifecycle_type: NodeLifecycleType,
        node_id: Uuid,
        node_info: Option<NodeInfo>,
        reason: Option<String>,
    ) -> MeshEvent {
        MeshEvent {
            event_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            source_node: self.local_node_id,
            event_type: EventType::NodeLifecycle { lifecycle_type },
            payload: EventPayload::NodeLifecycle {
                node_id,
                node_info,
                previous_state: None,
                new_state: "active".to_string(),
                reason,
            },
            metadata: HashMap::new(),
            propagation_path: vec![self.local_node_id],
            correlation_id: None,
            priority: EventPriority::Normal,
        }
    }
    
    /// Create a communication event
    pub fn create_communication_event(
        &self,
        communication_type: CommunicationType,
        participants: Vec<Uuid>,
        protocol: String,
    ) -> MeshEvent {
        MeshEvent {
            event_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            source_node: self.local_node_id,
            event_type: EventType::Communication { communication_type },
            payload: EventPayload::Communication {
                participants,
                message_id: None,
                protocol,
                status: "success".to_string(),
                error: None,
            },
            metadata: HashMap::new(),
            propagation_path: vec![self.local_node_id],
            correlation_id: None,
            priority: EventPriority::Normal,
        }
    }
    
    /// Create a health event
    pub fn create_health_event(
        &self,
        health_type: HealthEventType,
        node_id: Uuid,
        health_status: String,
        metrics: Option<HealthMetrics>,
    ) -> MeshEvent {
        MeshEvent {
            event_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            source_node: self.local_node_id,
            event_type: EventType::Health { health_type },
            payload: EventPayload::Health {
                node_id,
                health_status,
                metrics,
                issues: Vec::new(),
            },
            metadata: HashMap::new(),
            propagation_path: vec![self.local_node_id],
            correlation_id: None,
            priority: EventPriority::Normal,
        }
    }
    
    /// Get event configuration
    pub fn get_config(&self) -> &EventConfig {
        &self.config
    }
    
    /// Update event configuration
    pub fn update_config(&mut self, config: EventConfig) {
        self.config = config;
        debug!("Updated event system configuration");
    }
}

impl EventType {
    /// Get the event category for routing
    pub fn category(&self) -> &'static str {
        match self {
            EventType::NodeLifecycle { .. } => "node",
            EventType::Communication { .. } => "communication",
            EventType::Resource { .. } => "resource",
            EventType::Topology { .. } => "topology",
            EventType::Health { .. } => "health",
            EventType::Security { .. } => "security",
            EventType::Performance { .. } => "performance",
            EventType::ContextSpecific { context, .. } => {
                // Return context as category for context-specific events
                // This is a bit of a hack since we can't return a dynamic string
                // In practice, context-specific events would be handled by providers
                "context"
            },
            EventType::Generic { category, .. } => "generic",
        }
    }
}

impl MeshEvent {
    /// Check if this event matches a given pattern
    pub fn matches_pattern(&self, pattern: &str) -> bool {
        let event_category = self.event_type.category();
        pattern == "*" || 
        pattern == event_category ||
        pattern.starts_with(event_category) ||
        self.metadata.get("pattern").map_or(false, |p| p == pattern)
    }
    
    /// Add correlation ID to link related events
    pub fn with_correlation_id(mut self, correlation_id: Uuid) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }
    
    /// Add metadata to the event
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
    
    /// Set event priority
    pub fn with_priority(mut self, priority: EventPriority) -> Self {
        self.priority = priority;
        self
    }
    
    /// Add node to propagation path
    pub fn add_to_propagation_path(mut self, node_id: Uuid) -> Self {
        if !self.propagation_path.contains(&node_id) {
            self.propagation_path.push(node_id);
        }
        self
    }
}

impl Default for EventPriority {
    fn default() -> Self {
        EventPriority::Normal
    }
}

impl Default for SecurityRiskLevel {
    fn default() -> Self {
        SecurityRiskLevel::None
    }
}

/// Trait for context-specific event providers
#[async_trait::async_trait]
pub trait EventProvider: Send + Sync {
    /// Provider name
    fn name(&self) -> &str;
    
    /// Initialize the event provider
    async fn initialize(&mut self, config: &EventConfig) -> Result<()>;
    
    /// Cleanup provider resources
    async fn cleanup(&mut self) -> Result<()>;
    
    /// Handle an event
    async fn handle_event(&self, event: &MeshEvent) -> Result<()>;
    
    /// Create context-specific events
    async fn create_context_event(&self, context_data: serde_json::Value) -> Result<MeshEvent>;
    
    /// Get provider-specific event patterns
    fn get_event_patterns(&self) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_pattern_matching() {
        let event = MeshEvent {
            event_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            source_node: Uuid::new_v4(),
            event_type: EventType::NodeLifecycle {
                lifecycle_type: NodeLifecycleType::NodeJoined,
            },
            payload: EventPayload::Generic { data: HashMap::new() },
            metadata: HashMap::new(),
            propagation_path: Vec::new(),
            correlation_id: None,
            priority: EventPriority::Normal,
        };
        
        assert!(event.matches_pattern("*"));
        assert!(event.matches_pattern("node"));
        assert!(!event.matches_pattern("communication"));
    }

    #[test]
    fn test_event_category() {
        let event_type = EventType::Communication {
            communication_type: CommunicationType::MessageSent,
        };
        
        assert_eq!(event_type.category(), "communication");
    }

    #[test]
    fn test_event_priority_ordering() {
        assert!(EventPriority::Emergency > EventPriority::Critical);
        assert!(EventPriority::Critical > EventPriority::High);
        assert!(EventPriority::High > EventPriority::Normal);
        assert!(EventPriority::Normal > EventPriority::Low);
    }

    #[tokio::test]
    async fn test_event_system_creation() {
        let node_id = Uuid::new_v4();
        let event_system = EventSystem::new(node_id, None);
        
        assert_eq!(event_system.local_node_id, node_id);
        assert!(!*event_system.is_running.read().await);
        assert_eq!(event_system.providers.len(), 0);
    }

    #[tokio::test]
    async fn test_event_publishing() {
        let node_id = Uuid::new_v4();
        let event_system = EventSystem::new(node_id, None);
        
        let event = event_system.create_node_event(
            NodeLifecycleType::NodeJoined,
            node_id,
            None,
            None,
        );
        
        event_system.publish_event(event.clone()).await.unwrap();
        
        let history = event_system.get_event_history(None).await;
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].event_id, event.event_id);
    }
}
