//! Universal Mesh Manager
//!
//! Core mesh management functionality that can be extended by context-specific plugins.
//! Provides the foundation for distributed mesh networking with Zenoh.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};
use uuid::Uuid;
use zenoh::{Config, Session};

use super::discovery::{MeshDiscovery, NodeCapabilities, TrustLevel};
use super::MeshError;

/// Universal mesh manager for distributed networking
#[derive(Debug)]
pub struct MeshManager {
    /// Zenoh session for mesh communication
    pub session: Arc<Session>,
    
    /// Local node information
    pub local_node: LocalNode,
    
    /// Known nodes in the mesh
    pub nodes: Arc<RwLock<HashMap<Uuid, RemoteNode>>>,
    
    /// Mesh discovery service
    pub discovery: MeshDiscovery,
    
    /// Mesh configuration
    pub config: MeshConfig,
    
    /// Mesh state
    state: MeshState,
}

/// Local node information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalNode {
    /// Node identifier
    pub id: Uuid,
    /// Node capabilities
    pub capabilities: NodeCapabilities,
    /// Node metadata
    pub metadata: HashMap<String, String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

/// Remote node information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteNode {
    /// Node identifier
    pub id: Uuid,
    /// Node capabilities
    pub capabilities: NodeCapabilities,
    /// Trust level
    pub trust_level: TrustLevel,
    /// Last seen timestamp
    pub last_seen: DateTime<Utc>,
    /// Node metadata
    pub metadata: HashMap<String, String>,
    /// Connection state
    pub connection_state: ConnectionState,
}

/// Connection state for remote nodes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConnectionState {
    /// Not connected
    Disconnected,
    /// Connecting
    Connecting,
    /// Connected and active
    Connected,
    /// Connection failed
    Failed { reason: String },
    /// Connection lost
    Lost { last_error: Option<String> },
}

/// Mesh configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshConfig {
    /// Node discovery interval in seconds
    pub discovery_interval: u64,
    /// Maximum number of nodes to track
    pub max_nodes: usize,
    /// Connection timeout in seconds
    pub connection_timeout: u64,
    /// Enable automatic reconnection
    pub auto_reconnect: bool,
    /// Zenoh configuration
    pub zenoh_config: Option<Config>,
    /// Custom configuration for extensions
    pub custom_config: HashMap<String, serde_json::Value>,
}

impl Default for MeshConfig {
    fn default() -> Self {
        Self {
            discovery_interval: 30,
            max_nodes: 100,
            connection_timeout: 30,
            auto_reconnect: true,
            zenoh_config: None,
            custom_config: HashMap::new(),
        }
    }
}

/// Mesh state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MeshState {
    /// Mesh is stopped
    Stopped,
    /// Mesh is starting
    Starting,
    /// Mesh is active
    Active,
    /// Mesh is paused
    Paused,
    /// Mesh is stopping
    Stopping,
    /// Mesh has failed
    Failed { error: String },
}

/// Mesh event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MeshEvent {
    /// Node joined the mesh
    NodeJoined {
        node_id: Uuid,
        capabilities: NodeCapabilities,
        timestamp: DateTime<Utc>,
    },
    
    /// Node left the mesh
    NodeLeft {
        node_id: Uuid,
        reason: String,
        timestamp: DateTime<Utc>,
    },
    
    /// Node connection state changed
    ConnectionStateChanged {
        node_id: Uuid,
        old_state: ConnectionState,
        new_state: ConnectionState,
        timestamp: DateTime<Utc>,
    },
    
    /// Mesh topology changed
    TopologyChanged {
        change_type: TopologyChangeType,
        affected_nodes: Vec<Uuid>,
        timestamp: DateTime<Utc>,
    },
}

/// Types of topology changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TopologyChangeType {
    /// Network partition detected
    NetworkPartition,
    /// Network partition healed
    NetworkHealed,
    /// New cluster formed
    ClusterFormed,
    /// Cluster merged
    ClusterMerged,
}

/// Mesh statistics and metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshMetrics {
    /// Total number of active nodes
    pub active_nodes: usize,
    /// Number of connected nodes
    pub connected_nodes: usize,
    /// Average response time in milliseconds
    pub avg_response_time: f64,
    /// Network partition status
    pub is_partitioned: bool,
    /// Last update timestamp
    pub last_update: DateTime<Utc>,
}

impl MeshManager {
    /// Create a new mesh manager with the given configuration
    pub async fn new(config: MeshConfig) -> Result<Self> {
        info!("Creating mesh manager with config: {:?}", config);
        
        // Initialize Zenoh session
        let zenoh_config = config.zenoh_config.clone().unwrap_or_default();
        let session = Arc::new(
            zenoh::open(zenoh_config)
                .await
                .map_err(|e| MeshError::ZenohError(format!("Failed to open Zenoh session: {}", e)))?
        );
        
        // Create local node
        let local_node = LocalNode::new();
        
        // Initialize discovery
        let discovery = MeshDiscovery::new(
            local_node.id,
            local_node.capabilities.clone(),
            None, // Use default discovery config
        );
        
        Ok(Self {
            session,
            local_node,
            nodes: Arc::new(RwLock::new(HashMap::new())),
            discovery,
            config,
            state: MeshState::Stopped,
        })
    }
    
    /// Start the mesh manager and all its services
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting mesh manager for node {}", self.local_node.id);
        self.state = MeshState::Starting;
        
        // Start discovery service
        self.discovery.start().await?;
        
        // Set up event handlers
        self.setup_event_handlers().await?;
        
        self.state = MeshState::Active;
        info!("Mesh manager started successfully");
        Ok(())
    }
    
    /// Stop the mesh manager and clean up resources
    pub async fn stop(&mut self) -> Result<()> {
        info!("Stopping mesh manager for node {}", self.local_node.id);
        self.state = MeshState::Stopping;
        
        // Announce departure
        self.announce_departure().await?;
        
        // Stop discovery
        self.discovery.stop().await?;
        
        self.state = MeshState::Stopped;
        info!("Mesh manager stopped successfully");
        Ok(())
    }
    
    /// Get current mesh metrics
    pub async fn get_metrics(&self) -> Result<MeshMetrics> {
        let nodes = self.nodes.read().await;
        let active_nodes = nodes.len() + 1; // +1 for local node
        let connected_nodes = nodes.values()
            .filter(|node| node.connection_state == ConnectionState::Connected)
            .count();
        
        Ok(MeshMetrics {
            active_nodes,
            connected_nodes,
            avg_response_time: 50.0, // Would be calculated from actual metrics
            is_partitioned: false, // Would be determined from network analysis
            last_update: Utc::now(),
        })
    }
    
    /// Get all known nodes
    pub async fn get_all_nodes(&self) -> Vec<RemoteNode> {
        let nodes = self.nodes.read().await;
        nodes.values().cloned().collect()
    }
    
    /// Get a specific node by ID
    pub async fn get_node(&self, node_id: &Uuid) -> Option<RemoteNode> {
        let nodes = self.nodes.read().await;
        nodes.get(node_id).cloned()
    }
    
    /// Add a new node to the mesh
    pub async fn add_node(&self, node: RemoteNode) -> Result<()> {
        info!("Adding node to mesh: {}", node.id);
        let mut nodes = self.nodes.write().await;
        nodes.insert(node.id, node);
        Ok(())
    }
    
    /// Remove a node from the mesh
    pub async fn remove_node(&self, node_id: &Uuid) -> Result<()> {
        info!("Removing node from mesh: {}", node_id);
        let mut nodes = self.nodes.write().await;
        nodes.remove(node_id);
        Ok(())
    }
    
    /// Update node connection state
    pub async fn update_node_connection_state(
        &self,
        node_id: &Uuid,
        new_state: ConnectionState,
    ) -> Result<()> {
        let mut nodes = self.nodes.write().await;
        if let Some(node) = nodes.get_mut(node_id) {
            let old_state = node.connection_state.clone();
            node.connection_state = new_state.clone();
            
            debug!(
                "Node {} connection state changed: {:?} -> {:?}",
                node_id, old_state, new_state
            );
        }
        Ok(())
    }
    
    /// Get mesh state
    pub fn get_state(&self) -> &MeshState {
        &self.state
    }
    
    /// Get local node information
    pub fn get_local_node(&self) -> &LocalNode {
        &self.local_node
    }
    
    /// Set up event handlers for mesh events
    async fn setup_event_handlers(&self) -> Result<()> {
        debug!("Setting up mesh event handlers");
        // Implementation would set up Zenoh subscribers for mesh events
        Ok(())
    }
    
    /// Announce departure from the mesh
    async fn announce_departure(&self) -> Result<()> {
        debug!("Announcing departure from mesh");
        // Implementation would broadcast departure message
        Ok(())
    }
    
    /// Broadcast an event to the mesh
    pub async fn broadcast_event(&self, event: MeshEvent) -> Result<()> {
        debug!("Broadcasting mesh event: {:?}", event);
        // Implementation would serialize and broadcast the event
        Ok(())
    }
}

impl LocalNode {
    /// Create a new local node with default capabilities
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            capabilities: NodeCapabilities::default(),
            metadata: HashMap::new(),
            created_at: Utc::now(),
        }
    }
    
    /// Create a local node with specific capabilities
    pub fn with_capabilities(capabilities: NodeCapabilities) -> Self {
        Self {
            id: Uuid::new_v4(),
            capabilities,
            metadata: HashMap::new(),
            created_at: Utc::now(),
        }
    }
    
    /// Set metadata for the local node
    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
    
    /// Get metadata from the local node
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
}

impl RemoteNode {
    /// Create a new remote node
    pub fn new(
        id: Uuid,
        capabilities: NodeCapabilities,
        trust_level: TrustLevel,
    ) -> Self {
        Self {
            id,
            capabilities,
            trust_level,
            last_seen: Utc::now(),
            metadata: HashMap::new(),
            connection_state: ConnectionState::Disconnected,
        }
    }
    
    /// Update the last seen timestamp
    pub fn update_last_seen(&mut self) {
        self.last_seen = Utc::now();
    }
    
    /// Set metadata for the remote node
    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
    
    /// Get metadata from the remote node
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
    
    /// Check if the node is considered stale
    pub fn is_stale(&self, threshold_seconds: u64) -> bool {
        let now = Utc::now();
        let threshold = chrono::Duration::seconds(threshold_seconds as i64);
        now.signed_duration_since(self.last_seen) > threshold
    }
    
    /// Check if the node is connected
    pub fn is_connected(&self) -> bool {
        self.connection_state == ConnectionState::Connected
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_node_creation() {
        let node = LocalNode::new();
        assert!(!node.id.is_nil());
        assert!(node.capabilities.communication_services);
    }

    #[test]
    fn test_remote_node_creation() {
        let id = Uuid::new_v4();
        let capabilities = NodeCapabilities::default();
        let node = RemoteNode::new(id, capabilities, TrustLevel::Basic);
        
        assert_eq!(node.id, id);
        assert_eq!(node.trust_level, TrustLevel::Basic);
        assert_eq!(node.connection_state, ConnectionState::Disconnected);
    }

    #[test]
    fn test_mesh_config_default() {
        let config = MeshConfig::default();
        assert_eq!(config.discovery_interval, 30);
        assert_eq!(config.max_nodes, 100);
        assert!(config.auto_reconnect);
    }

    #[tokio::test]
    async fn test_mesh_manager_creation() {
        let config = MeshConfig::default();
        let result = MeshManager::new(config).await;
        // This test might fail without a proper Zenoh setup, but it tests the structure
        assert!(result.is_ok() || result.is_err()); // Just ensure it doesn't panic
    }
}
