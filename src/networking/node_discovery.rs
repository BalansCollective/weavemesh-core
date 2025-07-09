//! Universal Node Discovery for WeaveMesh
//! 
//! This module handles discovery and registration of nodes in the mesh network,
//! enabling dynamic mesh formation and resource sharing across different contexts.

use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::networking::zenoh_integration::{ZenohSession, WeaveMeshMessage, MessageType, WeaveMeshTopics};

/// Universal node discovery and registration manager
/// 
/// Handles:
/// - Announcing this node's presence to the mesh
/// - Discovering other nodes in the mesh
/// - Maintaining a registry of known nodes
/// - Handling node join/leave events
/// - Providing node lookup and routing information
pub struct NodeDiscovery {
    /// This node's ID
    node_id: Uuid,
    
    /// Zenoh session for mesh communication
    zenoh_session: Arc<ZenohSession>,
    
    /// Registry of discovered nodes
    node_registry: Arc<RwLock<HashMap<Uuid, NodeInfo>>>,
    
    /// Discovery configuration
    config: DiscoveryConfig,
    
    /// Whether discovery is currently active
    is_active: Arc<RwLock<bool>>,
}

/// Configuration for node discovery
#[derive(Debug, Clone)]
pub struct DiscoveryConfig {
    /// How often to announce this node's presence (seconds)
    pub announcement_interval: u64,
    
    /// How long to wait for discovery responses (seconds)
    pub discovery_timeout: u64,
    
    /// How long to keep inactive nodes in registry (seconds)
    pub node_timeout: u64,
    
    /// Whether to enable debug logging
    pub debug: bool,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            announcement_interval: 30,
            discovery_timeout: 10,
            node_timeout: 300, // 5 minutes
            debug: false,
        }
    }
}

/// Information about a discovered node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    /// Node ID
    pub node_id: Uuid,
    
    /// Human-readable display name
    pub display_name: String,
    
    /// Context or organization ID
    pub context_id: String,
    
    /// Node capabilities
    pub capabilities: Vec<NodeCapability>,
    
    /// Network endpoints for direct communication
    pub endpoints: Vec<String>,
    
    /// When this node was first discovered
    pub discovered_at: DateTime<Utc>,
    
    /// When this node was last seen
    pub last_seen: DateTime<Utc>,
    
    /// Whether this node is currently online
    pub is_online: bool,
    
    /// Additional metadata about the node
    pub metadata: HashMap<String, String>,
}

/// Universal capabilities that a node can advertise
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NodeCapability {
    /// Can store and share resources
    ResourceStorage,
    
    /// Can provide Sacred Alliance validation
    SacredAllianceValidation,
    
    /// Can track attribution
    AttributionTracking,
    
    /// Can participate in collaborative workflows
    Collaboration,
    
    /// Can provide AI assistance
    AiAssistance,
    
    /// Can handle git operations
    GitIntegration,
    
    /// Can serve as a WebSocket bridge
    WebSocketBridge,
    
    /// Can provide mesh networking services
    MeshNetworking,
    
    /// Can provide security services
    SecurityServices,
    
    /// Can provide health monitoring
    HealthMonitoring,
    
    /// Context-specific capability
    ContextSpecific(String),
    
    /// Custom capability
    Custom(String),
}

/// Node announcement message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeAnnouncement {
    /// Node information
    pub node_info: NodeInfo,
    
    /// Type of announcement
    pub announcement_type: AnnouncementType,
    
    /// Timestamp of announcement
    pub timestamp: DateTime<Utc>,
}

/// Types of node announcements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnnouncementType {
    /// Node is joining the mesh
    Join,
    
    /// Node is leaving the mesh
    Leave,
    
    /// Periodic heartbeat
    Heartbeat,
    
    /// Node capabilities have changed
    CapabilityUpdate,
    
    /// Node context has changed
    ContextUpdate,
}

/// Discovery query for finding specific nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryQuery {
    /// Query ID for tracking responses
    pub query_id: String,
    
    /// Node making the query
    pub from_node: Uuid,
    
    /// Filter criteria
    pub filter: NodeFilter,
    
    /// Query timestamp
    pub timestamp: DateTime<Utc>,
}

/// Filter criteria for node discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeFilter {
    /// Filter by context ID
    pub context_id: Option<String>,
    
    /// Filter by required capabilities
    pub required_capabilities: Vec<NodeCapability>,
    
    /// Filter by online status
    pub online_only: bool,
    
    /// Filter by node name pattern
    pub name_pattern: Option<String>,
    
    /// Filter by metadata key-value pairs
    pub metadata_filters: HashMap<String, String>,
}

impl NodeDiscovery {
    /// Create a new node discovery manager
    pub fn new(
        node_id: Uuid,
        zenoh_session: Arc<ZenohSession>,
        config: DiscoveryConfig,
    ) -> Self {
        Self {
            node_id,
            zenoh_session,
            node_registry: Arc::new(RwLock::new(HashMap::new())),
            config,
            is_active: Arc::new(RwLock::new(false)),
        }
    }
    
    /// Start the discovery process
    pub async fn start(
        &self,
        node_info: NodeInfo,
    ) -> Result<(), DiscoveryError> {
        // Mark as active
        *self.is_active.write().await = true;
        
        // Subscribe to discovery topics
        self.setup_subscriptions().await?;
        
        // Announce this node's presence
        self.announce_node(node_info, AnnouncementType::Join).await?;
        
        // Start periodic announcement task
        self.start_announcement_task().await;
        
        // Start cleanup task for inactive nodes
        self.start_cleanup_task().await;
        
        if self.config.debug {
            println!("Node discovery started for {}", self.node_id);
        }
        
        Ok(())
    }
    
    /// Stop the discovery process
    pub async fn stop(&self) -> Result<(), DiscoveryError> {
        // Mark as inactive
        *self.is_active.write().await = false;
        
        // Announce that we're leaving
        if let Some(node_info) = self.get_own_node_info().await {
            self.announce_node(node_info, AnnouncementType::Leave).await?;
        }
        
        // Clear the registry
        self.node_registry.write().await.clear();
        
        if self.config.debug {
            println!("Node discovery stopped for {}", self.node_id);
        }
        
        Ok(())
    }
    
    /// Get information about a specific node
    pub async fn get_node_info(&self, node_id: &Uuid) -> Option<NodeInfo> {
        self.node_registry.read().await.get(node_id).cloned()
    }
    
    /// Get all discovered nodes
    pub async fn get_all_nodes(&self) -> Vec<NodeInfo> {
        self.node_registry.read().await.values().cloned().collect()
    }
    
    /// Find nodes matching specific criteria
    pub async fn find_nodes(&self, filter: NodeFilter) -> Vec<NodeInfo> {
        let registry = self.node_registry.read().await;
        registry.values()
            .filter(|node| self.matches_filter(node, &filter))
            .cloned()
            .collect()
    }
    
    /// Query the mesh for nodes matching criteria
    pub async fn query_nodes(&self, filter: NodeFilter) -> Result<Vec<NodeInfo>, DiscoveryError> {
        let query = DiscoveryQuery {
            query_id: Uuid::new_v4().to_string(),
            from_node: self.node_id,
            filter,
            timestamp: Utc::now(),
        };
        
        let payload = serde_json::to_vec(&query)
            .map_err(|e| DiscoveryError::SerializationError(e.to_string()))?;
        
        // Broadcast the query
        self.zenoh_session.broadcast_message(
            MessageType::NodeDiscovery,
            payload,
        ).await.map_err(|e| DiscoveryError::NetworkError(e.to_string()))?;
        
        // Wait for responses (simplified - in practice we'd collect responses)
        tokio::time::sleep(tokio::time::Duration::from_secs(self.config.discovery_timeout)).await;
        
        // Return current registry matches
        Ok(self.find_nodes(query.filter).await)
    }
    
    /// Get nodes in the same context
    pub async fn get_context_nodes(&self, context_id: &str) -> Vec<NodeInfo> {
        let filter = NodeFilter {
            context_id: Some(context_id.to_string()),
            required_capabilities: Vec::new(),
            online_only: true,
            name_pattern: None,
            metadata_filters: HashMap::new(),
        };
        
        self.find_nodes(filter).await
    }
    
    /// Get nodes with specific capabilities
    pub async fn get_nodes_with_capabilities(&self, capabilities: Vec<NodeCapability>) -> Vec<NodeInfo> {
        let filter = NodeFilter {
            context_id: None,
            required_capabilities: capabilities,
            online_only: true,
            name_pattern: None,
            metadata_filters: HashMap::new(),
        };
        
        self.find_nodes(filter).await
    }
    
    /// Check if a node is currently online
    pub async fn is_node_online(&self, node_id: &Uuid) -> bool {
        if let Some(node_info) = self.get_node_info(node_id).await {
            node_info.is_online && 
            (Utc::now() - node_info.last_seen).num_seconds() < self.config.node_timeout as i64
        } else {
            false
        }
    }
    
    /// Update node capabilities
    pub async fn update_capabilities(&self, capabilities: Vec<NodeCapability>) -> Result<(), DiscoveryError> {
        if let Some(mut node_info) = self.get_own_node_info().await {
            node_info.capabilities = capabilities;
            self.announce_node(node_info, AnnouncementType::CapabilityUpdate).await?;
        }
        Ok(())
    }
    
    /// Update node metadata
    pub async fn update_metadata(&self, metadata: HashMap<String, String>) -> Result<(), DiscoveryError> {
        if let Some(mut node_info) = self.get_own_node_info().await {
            node_info.metadata = metadata;
            self.announce_node(node_info, AnnouncementType::CapabilityUpdate).await?;
        }
        Ok(())
    }
    
    /// Setup Zenoh subscriptions for discovery
    async fn setup_subscriptions(&self) -> Result<(), DiscoveryError> {
        // Subscribe to discovery announcements
        self.zenoh_session.subscribe(WeaveMeshTopics::NODE_DISCOVERY)
            .await
            .map_err(|e| DiscoveryError::NetworkError(e.to_string()))?;
        
        // Subscribe to direct messages for this node
        let direct_topic = WeaveMeshTopics::node_direct(self.node_id);
        self.zenoh_session.subscribe(&direct_topic)
            .await
            .map_err(|e| DiscoveryError::NetworkError(e.to_string()))?;
        
        // Subscribe to broadcast messages
        self.zenoh_session.subscribe(WeaveMeshTopics::BROADCAST)
            .await
            .map_err(|e| DiscoveryError::NetworkError(e.to_string()))?;
        
        // Set up message handler
        let node_registry = Arc::clone(&self.node_registry);
        let config = self.config.clone();
        
        self.zenoh_session.set_message_handler(move |message| {
            let registry = Arc::clone(&node_registry);
            let config = config.clone();
            
            tokio::spawn(async move {
                if let Err(e) = Self::handle_discovery_message(message, registry, config).await {
                    eprintln!("Error handling discovery message: {}", e);
                }
            });
            
            Ok(())
        }).await;
        
        Ok(())
    }
    
    /// Handle incoming discovery messages
    async fn handle_discovery_message(
        message: WeaveMeshMessage,
        node_registry: Arc<RwLock<HashMap<Uuid, NodeInfo>>>,
        config: DiscoveryConfig,
    ) -> Result<(), DiscoveryError> {
        match message.message_type {
            MessageType::NodeDiscovery => {
                if let Ok(announcement) = serde_json::from_slice::<NodeAnnouncement>(&message.payload) {
                    Self::handle_node_announcement(announcement, node_registry, config).await?;
                }
            }
            MessageType::Heartbeat => {
                // Update last seen time for the node
                if let Ok(node_id) = Uuid::parse_str(&message.from_node) {
                    let mut registry = node_registry.write().await;
                    if let Some(node_info) = registry.get_mut(&node_id) {
                        node_info.last_seen = Utc::now();
                        node_info.is_online = true;
                    }
                }
            }
            _ => {
                // Other message types handled elsewhere
            }
        }
        
        Ok(())
    }
    
    /// Handle node announcement
    async fn handle_node_announcement(
        announcement: NodeAnnouncement,
        node_registry: Arc<RwLock<HashMap<Uuid, NodeInfo>>>,
        config: DiscoveryConfig,
    ) -> Result<(), DiscoveryError> {
        let mut registry = node_registry.write().await;
        
        match announcement.announcement_type {
            AnnouncementType::Join | AnnouncementType::Heartbeat | 
            AnnouncementType::CapabilityUpdate | AnnouncementType::ContextUpdate => {
                let mut node_info = announcement.node_info;
                node_info.last_seen = Utc::now();
                node_info.is_online = true;
                
                let node_id = node_info.node_id;
                
                if let Some(existing) = registry.get(&node_id) {
                    // Update existing node info
                    let mut updated = existing.clone();
                    updated.display_name = node_info.display_name;
                    updated.context_id = node_info.context_id;
                    updated.capabilities = node_info.capabilities;
                    updated.endpoints = node_info.endpoints;
                    updated.last_seen = node_info.last_seen;
                    updated.is_online = true;
                    updated.metadata = node_info.metadata;
                    
                    registry.insert(node_id, updated);
                } else {
                    // Add new node
                    registry.insert(node_id, node_info);
                }
                
                if config.debug {
                    println!("Node {} announced: {:?}", 
                        node_id, 
                        announcement.announcement_type);
                }
            }
            AnnouncementType::Leave => {
                if let Some(mut node_info) = registry.get_mut(&announcement.node_info.node_id) {
                    node_info.is_online = false;
                    node_info.last_seen = Utc::now();
                }
                
                if config.debug {
                    println!("Node {} left the mesh", announcement.node_info.node_id);
                }
            }
        }
        
        Ok(())
    }
    
    /// Announce this node to the mesh
    async fn announce_node(
        &self,
        node_info: NodeInfo,
        announcement_type: AnnouncementType,
    ) -> Result<(), DiscoveryError> {
        let announcement = NodeAnnouncement {
            node_info,
            announcement_type,
            timestamp: Utc::now(),
        };
        
        let payload = serde_json::to_vec(&announcement)
            .map_err(|e| DiscoveryError::SerializationError(e.to_string()))?;
        
        self.zenoh_session.broadcast_message(
            MessageType::NodeDiscovery,
            payload,
        ).await.map_err(|e| DiscoveryError::NetworkError(e.to_string()))?;
        
        Ok(())
    }
    
    /// Start periodic announcement task
    async fn start_announcement_task(&self) {
        let zenoh_session = Arc::clone(&self.zenoh_session);
        let is_active = Arc::clone(&self.is_active);
        let interval = self.config.announcement_interval;
        let node_id = self.node_id;
        
        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(
                tokio::time::Duration::from_secs(interval)
            );
            
            while *is_active.read().await {
                interval_timer.tick().await;
                
                if *is_active.read().await {
                    // Send heartbeat
                    let _ = zenoh_session.broadcast_message(
                        MessageType::Heartbeat,
                        Vec::new(),
                    ).await;
                }
            }
        });
    }
    
    /// Start cleanup task for inactive nodes
    async fn start_cleanup_task(&self) {
        let node_registry = Arc::clone(&self.node_registry);
        let is_active = Arc::clone(&self.is_active);
        let timeout = self.config.node_timeout;
        let debug = self.config.debug;
        
        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(
                tokio::time::Duration::from_secs(60) // Check every minute
            );
            
            while *is_active.read().await {
                interval_timer.tick().await;
                
                if *is_active.read().await {
                    let mut registry = node_registry.write().await;
                    let now = Utc::now();
                    let mut to_remove = Vec::new();
                    
                    for (node_id, node_info) in registry.iter_mut() {
                        let seconds_since_seen = (now - node_info.last_seen).num_seconds();
                        
                        if seconds_since_seen > timeout as i64 {
                            if node_info.is_online {
                                node_info.is_online = false;
                                if debug {
                                    println!("Node {} marked as offline", node_id);
                                }
                            }
                            
                            // Remove nodes that have been offline for too long
                            if seconds_since_seen > (timeout * 2) as i64 {
                                to_remove.push(*node_id);
                            }
                        }
                    }
                    
                    for node_id in to_remove {
                        registry.remove(&node_id);
                        if debug {
                            println!("Node {} removed from registry", node_id);
                        }
                    }
                }
            }
        });
    }
    
    /// Get this node's own information
    async fn get_own_node_info(&self) -> Option<NodeInfo> {
        self.node_registry.read().await.get(&self.node_id).cloned()
    }
    
    /// Check if a node matches the given filter
    fn matches_filter(&self, node: &NodeInfo, filter: &NodeFilter) -> bool {
        // Check context ID
        if let Some(ref context_id) = filter.context_id {
            if &node.context_id != context_id {
                return false;
            }
        }
        
        // Check online status
        if filter.online_only && !node.is_online {
            return false;
        }
        
        // Check required capabilities
        for required_cap in &filter.required_capabilities {
            if !node.capabilities.contains(required_cap) {
                return false;
            }
        }
        
        // Check name pattern
        if let Some(ref pattern) = filter.name_pattern {
            if !node.display_name.to_lowercase().contains(&pattern.to_lowercase()) {
                return false;
            }
        }
        
        // Check metadata filters
        for (key, value) in &filter.metadata_filters {
            if let Some(node_value) = node.metadata.get(key) {
                if node_value != value {
                    return false;
                }
            } else {
                return false;
            }
        }
        
        true
    }
}

/// Errors that can occur during node discovery
#[derive(Debug, thiserror::Error)]
pub enum DiscoveryError {
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Node not found: {0}")]
    NodeNotFound(String),
    
    #[error("Discovery not active")]
    NotActive,
    
    #[error("Invalid node information")]
    InvalidNodeInfo,
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
}

/// Utility functions for node discovery
pub mod utils {
    use super::*;
    
    /// Create default node capabilities for a basic node
    pub fn default_node_capabilities() -> Vec<NodeCapability> {
        vec![
            NodeCapability::ResourceStorage,
            NodeCapability::Collaboration,
            NodeCapability::MeshNetworking,
        ]
    }
    
    /// Create a basic node info structure
    pub fn create_basic_node_info(
        node_id: Uuid,
        display_name: String,
        context_id: String,
    ) -> NodeInfo {
        NodeInfo {
            node_id,
            display_name,
            context_id,
            capabilities: default_node_capabilities(),
            endpoints: Vec::new(),
            discovered_at: Utc::now(),
            last_seen: Utc::now(),
            is_online: true,
            metadata: HashMap::new(),
        }
    }
    
    /// Check if two nodes are in the same context
    pub fn same_context(node1: &NodeInfo, node2: &NodeInfo) -> bool {
        node1.context_id == node2.context_id
    }
    
    /// Check if a node has a specific capability
    pub fn has_capability(node: &NodeInfo, capability: &NodeCapability) -> bool {
        node.capabilities.contains(capability)
    }
    
    /// Get nodes by capability
    pub fn filter_by_capability<'a>(nodes: &'a [NodeInfo], capability: &NodeCapability) -> Vec<&'a NodeInfo> {
        nodes.iter()
            .filter(|node| has_capability(node, capability))
            .collect()
    }
    
    /// Calculate node uptime in seconds
    pub fn calculate_uptime(node: &NodeInfo) -> i64 {
        (Utc::now() - node.discovered_at).num_seconds()
    }
    
    /// Check if a node is recently active
    pub fn is_recently_active(node: &NodeInfo, threshold_seconds: i64) -> bool {
        (Utc::now() - node.last_seen).num_seconds() < threshold_seconds
    }
    
    /// Create a discovery filter for online nodes only
    pub fn online_nodes_filter() -> NodeFilter {
        NodeFilter {
            context_id: None,
            required_capabilities: Vec::new(),
            online_only: true,
            name_pattern: None,
            metadata_filters: HashMap::new(),
        }
    }
    
    /// Create a discovery filter for nodes with specific capabilities
    pub fn capability_filter(capabilities: Vec<NodeCapability>) -> NodeFilter {
        NodeFilter {
            context_id: None,
            required_capabilities: capabilities,
            online_only: true,
            name_pattern: None,
            metadata_filters: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::utils::*;
    
    #[test]
    fn test_node_filter_matching() {
        let node_info = NodeInfo {
            node_id: Uuid::new_v4(),
            display_name: "Test Node".to_string(),
            context_id: "test-context".to_string(),
            capabilities: vec![NodeCapability::ResourceStorage, NodeCapability::Collaboration],
            endpoints: vec!["tcp/127.0.0.1:8080".to_string()],
            discovered_at: Utc::now(),
            last_seen: Utc::now(),
            is_online: true,
            metadata: HashMap::new(),
        };
        
        // Test context filter
        let context_filter = NodeFilter {
            context_id: Some("test-context".to_string()),
            required_capabilities: Vec::new(),
            online_only: false,
            name_pattern: None,
            metadata_filters: HashMap::new(),
        };
        
        let discovery = NodeDiscovery {
            node_id: Uuid::new_v4(),
            zenoh_session: Arc::new(unsafe { std::mem::zeroed() }), // Mock for test
            node_registry: Arc::new(RwLock::new(HashMap::new())),
            config: DiscoveryConfig::default(),
            is_active: Arc::new(RwLock::new(false)),
        };
        
        assert!(discovery.matches_filter(&node_info, &context_filter));
        
        // Test capability filter
        let capability_filter = capability_filter(vec![NodeCapability::ResourceStorage]);
        assert!(discovery.matches_filter(&node_info, &capability_filter));
        
        // Test name pattern filter
        let name_filter = NodeFilter {
            context_id: None,
            required_capabilities: Vec::new(),
            online_only: false,
            name_pattern: Some("test".to_string()),
            metadata_filters: HashMap::new(),
        };
        
        assert!(discovery.matches_filter(&node_info, &name_filter));
    }
    
    #[test]
    fn test_node_capabilities() {
        let capabilities = vec![
            NodeCapability::ResourceStorage,
            NodeCapability::SacredAllianceValidation,
            NodeCapability::Custom("special-feature".to_string()),
        ];
        
        assert!(capabilities.contains(&NodeCapability::ResourceStorage));
        assert!(!capabilities.contains(&NodeCapability::AiAssistance));
    }
    
    #[test]
    fn test_node_info_creation() {
        let node_id = Uuid::new_v4();
        let node_info = create_basic_node_info(
            node_id,
            "Test Node".to_string(),
            "test-context".to_string(),
        );
        
        assert_eq!(node_info.node_id, node_id);
        assert_eq!(node_info.display_name, "Test Node");
        assert_eq!(node_info.context_id, "test-context");
        assert!(node_info.is_online);
        assert!(!node_info.capabilities.is_empty());
    }
    
    #[test]
    fn test_utility_functions() {
        let node1 = create_basic_node_info(
            Uuid::new_v4(),
            "Node 1".to_string(),
            "context-a".to_string(),
        );
        
        let node2 = create_basic_node_info(
            Uuid::new_v4(),
            "Node 2".to_string(),
            "context-a".to_string(),
        );
        
        let node3 = create_basic_node_info(
            Uuid::new_v4(),
            "Node 3".to_string(),
            "context-b".to_string(),
        );
        
        assert!(same_context(&node1, &node2));
        assert!(!same_context(&node1, &node3));
        
        assert!(has_capability(&node1, &NodeCapability::ResourceStorage));
        assert!(!has_capability(&node1, &NodeCapability::AiAssistance));
        
        let nodes = vec![node1, node2, node3];
        let storage_nodes = filter_by_capability(&nodes, &NodeCapability::ResourceStorage);
        assert_eq!(storage_nodes.len(), 3); // All have ResourceStorage by default
        
        let uptime = calculate_uptime(&nodes[0]);
        assert!(uptime >= 0);
        
        assert!(is_recently_active(&nodes[0], 60)); // Should be active within last minute
    }
    
    #[test]
    fn test_filter_creation() {
        let online_filter = online_nodes_filter();
        assert!(online_filter.online_only);
        assert!(online_filter.required_capabilities.is_empty());
        
        let cap_filter = capability_filter(vec![NodeCapability::AiAssistance]);
        assert_eq!(cap_filter.required_capabilities.len(), 1);
        assert!(cap_filter.online_only);
    }
}
