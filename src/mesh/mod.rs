//! Universal Mesh Infrastructure
//!
//! Core mesh networking components that provide the foundation for distributed
//! collaboration systems. This module contains universal primitives that can be
//! extended by context-specific plugins.

pub mod discovery;
pub mod events;
pub mod health;
pub mod manager;
pub mod node;
pub mod resource;
pub mod security;

// Re-export key types for convenience
pub use discovery::{
    MeshDiscovery, MeshNode, NodeCapabilities, TrustLevel, DiscoveryState
};
pub use events::{
    EventSystem, MeshEvent, EventType, EventPayload, EventPriority,
    NodeLifecycleType, CommunicationType, ResourceEventType, TopologyEventType,
    HealthEventType, SecurityEventType, PerformanceEventType, EventConfig,
    EventStatistics, EventProvider
};
pub use health::{
    HealthMonitor, HealthStatus, NodeHealthStatus, NodeHealthMetrics,
    HealthCheckResult, HealthIssue, HealthSeverity, PerformanceMetrics,
    HealthConfig, HealthEvent, HealthProvider
};
pub use manager::{
    MeshManager, LocalNode, RemoteNode, MeshConfig, MeshState,
    MeshMetrics, ConnectionState, TopologyChangeType
};
pub use node::{
    MeshNode as UniversalMeshNode, NodeInfo, NodeType, NodeCapability, NodeEndpoint,
    EndpointType, NodeVersion, NodeAnnouncement, NodeMetrics
};
pub use resource::{
    MeshResource, ResourceType, ResourceState, ResourceMetadata, QualityMetrics,
    CollaborationMetrics, ResourceInstance, InstanceState, ContextAdaptation,
    ModificationInfo, ModificationType, SyncStatus, SyncState, SyncConflict,
    ConflictType, ConflictDetails, ConflictSeverity, ConflictResolution,
    AccessControl, ContextAccess, Permission, PermissionType, InstancePermissions,
    VisibilityLevel, ConflictInfo, SessionStatus, CeremonyStatus
};
pub use security::{
    SecuritySystem, TrustRelationship, TrustEvent, TrustEventType,
    SharedCredentials, TrustVerificationMethod, TrustBoundaries,
    SecurityPolicies, AuthenticationPolicy, AuthorizationRule, EncryptionPolicy,
    AccessControlPolicy, MonitoringPolicy, SecurityEvent, SecurityEventFilter,
    SecuritySeverity, ResolutionStatus, SecurityConfig, SecurityProvider
};

use anyhow::Result;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use uuid::Uuid;

/// Universal mesh error types
#[derive(Debug, thiserror::Error)]
pub enum MeshError {
    /// Zenoh-related errors
    #[error("Zenoh error: {0}")]
    ZenohError(String),
    
    /// Discovery errors
    #[error("Discovery error: {0}")]
    DiscoveryError(String),
    
    /// Connection errors
    #[error("Connection error: {0}")]
    ConnectionError(String),
    
    /// Configuration errors
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    
    /// Node errors
    #[error("Node error: {0}")]
    NodeError(String),
    
    /// Generic mesh errors
    #[error("Mesh error: {0}")]
    Generic(String),
}

/// Universal mesh interface for different contexts
#[async_trait::async_trait]
pub trait MeshInterface {
    /// Start the mesh
    async fn start(&mut self) -> Result<(), MeshError>;
    
    /// Stop the mesh
    async fn stop(&mut self) -> Result<(), MeshError>;
    
    /// Get mesh metrics
    async fn get_metrics(&self) -> Result<MeshMetrics, MeshError>;
    
    /// Get all nodes
    async fn get_nodes(&self) -> Result<Vec<RemoteNode>, MeshError>;
    
    /// Broadcast an event
    async fn broadcast_event(&self, event: MeshEvent) -> Result<(), MeshError>;
}

/// Plugin interface for extending mesh functionality
#[async_trait::async_trait]
pub trait MeshPlugin: Send + Sync {
    /// Plugin name
    fn name(&self) -> &str;
    
    /// Plugin version
    fn version(&self) -> &str;
    
    /// Initialize the plugin
    async fn initialize(&mut self, config: &HashMap<String, serde_json::Value>) -> Result<()>;
    
    /// Handle mesh events
    async fn handle_event(&self, event: &MeshEvent) -> Result<()>;
    
    /// Cleanup plugin resources
    async fn cleanup(&mut self) -> Result<()>;
}

/// Plugin registry for managing mesh extensions
pub struct PluginRegistry {
    plugins: HashMap<String, Box<dyn MeshPlugin>>,
}

impl std::fmt::Debug for PluginRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PluginRegistry")
            .field("plugin_count", &self.plugins.len())
            .field("plugin_names", &self.get_plugin_names())
            .finish()
    }
}

impl PluginRegistry {
    /// Create a new plugin registry
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }
    
    /// Register a plugin
    pub fn register_plugin(&mut self, plugin: Box<dyn MeshPlugin>) {
        let name = plugin.name().to_string();
        self.plugins.insert(name, plugin);
    }
    
    /// Get a plugin by name
    pub fn get_plugin(&self, name: &str) -> Option<&dyn MeshPlugin> {
        self.plugins.get(name).map(|p| p.as_ref())
    }
    
    /// Get all plugin names
    pub fn get_plugin_names(&self) -> Vec<&str> {
        self.plugins.keys().map(|s| s.as_str()).collect()
    }
    
    /// Initialize all plugins
    pub async fn initialize_all(&mut self, config: &HashMap<String, serde_json::Value>) -> Result<()> {
        for plugin in self.plugins.values_mut() {
            plugin.initialize(config).await?;
        }
        Ok(())
    }
    
    /// Handle event with all plugins
    pub async fn handle_event_all(&self, event: &MeshEvent) -> Result<()> {
        for plugin in self.plugins.values() {
            plugin.handle_event(event).await?;
        }
        Ok(())
    }
    
    /// Cleanup all plugins
    pub async fn cleanup_all(&mut self) -> Result<()> {
        for plugin in self.plugins.values_mut() {
            plugin.cleanup().await?;
        }
        Ok(())
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Mesh builder for creating configured mesh instances
#[derive(Debug)]
pub struct MeshBuilder {
    config: MeshConfig,
    plugins: PluginRegistry,
}

impl MeshBuilder {
    /// Create a new mesh builder
    pub fn new() -> Self {
        Self {
            config: MeshConfig::default(),
            plugins: PluginRegistry::new(),
        }
    }
    
    /// Set mesh configuration
    pub fn with_config(mut self, config: MeshConfig) -> Self {
        self.config = config;
        self
    }
    
    /// Add a plugin
    pub fn with_plugin(mut self, plugin: Box<dyn MeshPlugin>) -> Self {
        self.plugins.register_plugin(plugin);
        self
    }
    
    /// Set discovery interval
    pub fn with_discovery_interval(mut self, interval: u64) -> Self {
        self.config.discovery_interval = interval;
        self
    }
    
    /// Set maximum nodes
    pub fn with_max_nodes(mut self, max_nodes: usize) -> Self {
        self.config.max_nodes = max_nodes;
        self
    }
    
    /// Enable/disable auto reconnect
    pub fn with_auto_reconnect(mut self, auto_reconnect: bool) -> Self {
        self.config.auto_reconnect = auto_reconnect;
        self
    }
    
    /// Build the mesh manager
    pub async fn build(self) -> Result<MeshManager, MeshError> {
        MeshManager::new(self.config)
            .await
            .map_err(|e| MeshError::Generic(e.to_string()))
    }
}

impl Default for MeshBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility functions for mesh operations
pub mod utils {
    use super::*;
    
    /// Generate a unique node ID
    pub fn generate_node_id() -> Uuid {
        Uuid::new_v4()
    }
    
    /// Create default node capabilities
    pub fn default_capabilities() -> NodeCapabilities {
        NodeCapabilities::default()
    }
    
    /// Check if two nodes are compatible
    pub fn nodes_compatible(node1: &NodeCapabilities, node2: &NodeCapabilities) -> bool {
        // Basic compatibility check - both support communication
        node1.communication_services && node2.communication_services
    }
    
    /// Calculate trust score between nodes
    pub fn calculate_trust_score(trust_level: &TrustLevel) -> f64 {
        match trust_level {
            TrustLevel::Unknown => 0.0,
            TrustLevel::Basic => 0.2,
            TrustLevel::Verified => 0.5,
            TrustLevel::Trusted => 0.8,
            TrustLevel::HighlyTrusted => 1.0,
        }
    }
    
    /// Format node ID for display
    pub fn format_node_id(node_id: &Uuid) -> String {
        let id_str = node_id.to_string();
        format!("{}...{}", &id_str[0..8], &id_str[id_str.len()-8..])
    }
    
    /// Create a mesh event timestamp
    pub fn event_timestamp() -> DateTime<Utc> {
        Utc::now()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::utils::*;

    #[test]
    fn test_mesh_builder() {
        let builder = MeshBuilder::new()
            .with_discovery_interval(60)
            .with_max_nodes(50)
            .with_auto_reconnect(false);
        
        assert_eq!(builder.config.discovery_interval, 60);
        assert_eq!(builder.config.max_nodes, 50);
        assert!(!builder.config.auto_reconnect);
    }

    #[test]
    fn test_plugin_registry() {
        let mut registry = PluginRegistry::new();
        assert_eq!(registry.get_plugin_names().len(), 0);
        
        // Note: Would need a concrete plugin implementation to test registration
    }

    #[test]
    fn test_utils() {
        let node_id = generate_node_id();
        assert!(!node_id.is_nil());
        
        let capabilities = default_capabilities();
        assert!(capabilities.communication_services);
        
        let trust_score = calculate_trust_score(&TrustLevel::Trusted);
        assert_eq!(trust_score, 0.8);
        
        let formatted_id = format_node_id(&node_id);
        assert!(formatted_id.contains("..."));
    }

    #[test]
    fn test_nodes_compatibility() {
        let cap1 = NodeCapabilities {
            communication_services: true,
            security_services: true,
            protocols: vec!["zenoh".to_string()],
            max_connections: 10,
            context_capabilities: HashMap::new(),
        };
        
        let cap2 = NodeCapabilities {
            communication_services: true,
            security_services: false,
            protocols: vec!["zenoh".to_string()],
            max_connections: 5,
            context_capabilities: HashMap::new(),
        };
        
        assert!(nodes_compatible(&cap1, &cap2));
        
        let cap3 = NodeCapabilities {
            communication_services: false,
            security_services: true,
            protocols: vec!["zenoh".to_string()],
            max_connections: 10,
            context_capabilities: HashMap::new(),
        };
        
        assert!(!nodes_compatible(&cap1, &cap3));
    }
}
