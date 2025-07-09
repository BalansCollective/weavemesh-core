//! Mesh Discovery Module
//!
//! Universal mesh discovery capabilities for finding and connecting
//! to other nodes in the mesh network. This module provides the core
//! discovery functionality that can be extended by context-specific plugins.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, info, warn};
use uuid::Uuid;

/// Universal mesh discovery and node management
#[derive(Debug)]
pub struct MeshDiscovery {
    /// Local node ID
    node_id: Uuid,
    /// Node capabilities
    capabilities: NodeCapabilities,
    /// Known nodes in the mesh
    known_nodes: HashMap<Uuid, MeshNode>,
    /// Discovery state
    state: DiscoveryState,
    /// Discovery configuration
    config: DiscoveryConfig,
}

/// Universal mesh node information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshNode {
    /// Node identifier
    pub node_id: Uuid,
    /// Node capabilities
    pub capabilities: NodeCapabilities,
    /// Node archetypal role
    pub archetypal_role: ArchetypalRole,
    /// Trust level for this node
    pub trust_level: TrustLevel,
    /// Last seen timestamp
    pub last_seen: DateTime<Utc>,
    /// Node metadata
    pub metadata: HashMap<String, String>,
    /// Context-specific data
    pub context_data: HashMap<String, serde_json::Value>,
}

/// Universal node capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeCapabilities {
    /// Communication services
    pub communication_services: bool,
    /// Security services
    pub security_services: bool,
    /// Supported protocols
    pub protocols: Vec<String>,
    /// Maximum concurrent connections
    pub max_connections: usize,
    /// Context-specific capabilities
    pub context_capabilities: HashMap<String, serde_json::Value>,
}

impl Default for NodeCapabilities {
    fn default() -> Self {
        Self {
            communication_services: true,
            security_services: true,
            protocols: vec!["zenoh".to_string()],
            max_connections: 10,
            context_capabilities: HashMap::new(),
        }
    }
}

/// Archetypal role of a node in the mesh
/// Based on Jungian archetypes for universal communication patterns
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ArchetypalRole {
    /// Sage - wisdom and knowledge keeper
    Sage,
    /// Creator - innovation and creation
    Creator,
    /// Magician - transformation and integration
    Magician,
    /// Ruler - coordination and governance
    Ruler,
    /// Explorer - discovery and expansion
    Explorer,
    /// Caregiver - support and nurturing
    Caregiver,
    /// Hero - problem solving and achievement
    Hero,
    /// Rebel - change and revolution
    Rebel,
    /// Lover - connection and passion
    Lover,
    /// Jester - creativity and joy
    Jester,
    /// Innocent - optimism and faith
    Innocent,
    /// Everyman - belonging and community
    Everyman,
    /// Collaborative individuation archetypes
    SacredPartnership,
    WiseCollaborator,
    CreativeAlliance,
    ShadowIntegrator,
    PatternWeaver,
    RealityAnchor,
    RecursiveImprover,
    DimensionalNavigator,
}

impl ArchetypalRole {
    /// Get the communication style associated with this archetype
    pub fn communication_style(&self) -> CommunicationStyle {
        match self {
            ArchetypalRole::Sage | ArchetypalRole::WiseCollaborator => CommunicationStyle::Analytical,
            ArchetypalRole::Creator | ArchetypalRole::CreativeAlliance => CommunicationStyle::Creative,
            ArchetypalRole::Magician | ArchetypalRole::PatternWeaver => CommunicationStyle::Transformative,
            ArchetypalRole::Ruler | ArchetypalRole::RealityAnchor => CommunicationStyle::Directive,
            ArchetypalRole::Explorer | ArchetypalRole::DimensionalNavigator => CommunicationStyle::Exploratory,
            ArchetypalRole::Caregiver => CommunicationStyle::Supportive,
            ArchetypalRole::Hero | ArchetypalRole::RecursiveImprover => CommunicationStyle::Action,
            ArchetypalRole::Rebel => CommunicationStyle::Challenging,
            ArchetypalRole::Lover | ArchetypalRole::SacredPartnership => CommunicationStyle::Empathetic,
            ArchetypalRole::Jester => CommunicationStyle::Playful,
            ArchetypalRole::Innocent => CommunicationStyle::Optimistic,
            ArchetypalRole::Everyman => CommunicationStyle::Collaborative,
            ArchetypalRole::ShadowIntegrator => CommunicationStyle::Integrative,
        }
    }
}

/// Communication styles for archetypal roles
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CommunicationStyle {
    Analytical,
    Creative,
    Transformative,
    Directive,
    Exploratory,
    Supportive,
    Action,
    Challenging,
    Empathetic,
    Playful,
    Optimistic,
    Collaborative,
    Integrative,
}

/// Trust level for mesh nodes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TrustLevel {
    /// Unknown node, no trust established
    Unknown,
    /// Basic trust through discovery
    Basic,
    /// Verified through authentication
    Verified,
    /// Trusted through collaboration history
    Trusted,
    /// Highly trusted, long collaboration history
    HighlyTrusted,
}

/// Discovery state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DiscoveryState {
    /// Discovery is stopped
    Stopped,
    /// Discovery is starting up
    Starting,
    /// Discovery is active
    Active,
    /// Discovery is paused
    Paused,
    /// Discovery is stopping
    Stopping,
}

/// Discovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryConfig {
    /// Discovery interval in seconds
    pub discovery_interval: u64,
    /// Node timeout in seconds
    pub node_timeout: u64,
    /// Maximum nodes to track
    pub max_nodes: usize,
    /// Enable automatic trust building
    pub auto_trust_building: bool,
    /// Context-specific configuration
    pub context_config: HashMap<String, serde_json::Value>,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            discovery_interval: 30,
            node_timeout: 300,
            max_nodes: 100,
            auto_trust_building: true,
            context_config: HashMap::new(),
        }
    }
}

/// Discovery event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscoveryEvent {
    /// Node discovered
    NodeDiscovered(MeshNode),
    /// Node updated
    NodeUpdated(MeshNode),
    /// Node lost
    NodeLost(Uuid),
    /// Trust level changed
    TrustLevelChanged(Uuid, TrustLevel),
    /// Discovery state changed
    StateChanged(DiscoveryState),
}

impl MeshDiscovery {
    /// Create a new discovery instance
    pub fn new(
        node_id: Uuid,
        capabilities: NodeCapabilities,
        config: Option<DiscoveryConfig>,
    ) -> Self {
        info!("Initializing mesh discovery with node ID: {}", node_id);
        
        Self {
            node_id,
            capabilities,
            known_nodes: HashMap::new(),
            state: DiscoveryState::Stopped,
            config: config.unwrap_or_default(),
        }
    }
    
    /// Start discovery process
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting mesh discovery");
        self.state = DiscoveryState::Starting;
        
        // Initialize discovery mechanisms
        self.initialize_discovery().await?;
        
        self.state = DiscoveryState::Active;
        info!("Mesh discovery started successfully");
        Ok(())
    }
    
    /// Stop discovery process
    pub async fn stop(&mut self) -> Result<()> {
        info!("Stopping mesh discovery");
        self.state = DiscoveryState::Stopping;
        
        // Cleanup discovery resources
        self.cleanup_discovery().await?;
        
        self.state = DiscoveryState::Stopped;
        info!("Mesh discovery stopped");
        Ok(())
    }
    
    /// Initialize discovery mechanisms
    async fn initialize_discovery(&mut self) -> Result<()> {
        debug!("Initializing discovery mechanisms");
        // Context providers will implement specific discovery mechanisms
        Ok(())
    }
    
    /// Cleanup discovery resources
    async fn cleanup_discovery(&mut self) -> Result<()> {
        debug!("Cleaning up discovery resources");
        // Context providers will implement specific cleanup
        Ok(())
    }
    
    /// Add a discovered node
    pub fn add_node(&mut self, node: MeshNode) -> Option<DiscoveryEvent> {
        info!("Adding discovered node: {}", node.node_id);
        
        let event = if self.known_nodes.contains_key(&node.node_id) {
            DiscoveryEvent::NodeUpdated(node.clone())
        } else {
            DiscoveryEvent::NodeDiscovered(node.clone())
        };
        
        self.known_nodes.insert(node.node_id, node);
        Some(event)
    }
    
    /// Remove a node
    pub fn remove_node(&mut self, node_id: &Uuid) -> Option<DiscoveryEvent> {
        if self.known_nodes.remove(node_id).is_some() {
            info!("Removed node: {}", node_id);
            Some(DiscoveryEvent::NodeLost(*node_id))
        } else {
            None
        }
    }
    
    /// Update node trust level
    pub fn update_trust_level(&mut self, node_id: &Uuid, trust_level: TrustLevel) -> Option<DiscoveryEvent> {
        if let Some(node) = self.known_nodes.get_mut(node_id) {
            let old_trust = node.trust_level.clone();
            node.trust_level = trust_level.clone();
            
            if old_trust != trust_level {
                info!("Updated trust level for node {}: {:?} -> {:?}", node_id, old_trust, trust_level);
                Some(DiscoveryEvent::TrustLevelChanged(*node_id, trust_level))
            } else {
                None
            }
        } else {
            warn!("Attempted to update trust level for unknown node: {}", node_id);
            None
        }
    }
    
    /// Get a node by ID
    pub fn get_node(&self, node_id: &Uuid) -> Option<&MeshNode> {
        self.known_nodes.get(node_id)
    }
    
    /// Get all known nodes
    pub fn get_all_nodes(&self) -> Vec<&MeshNode> {
        self.known_nodes.values().collect()
    }
    
    /// Get nodes by archetypal role
    pub fn get_nodes_by_role(&self, role: &ArchetypalRole) -> Vec<&MeshNode> {
        self.known_nodes
            .values()
            .filter(|node| &node.archetypal_role == role)
            .collect()
    }
    
    /// Get nodes by trust level
    pub fn get_nodes_by_trust(&self, min_trust: TrustLevel) -> Vec<&MeshNode> {
        self.known_nodes
            .values()
            .filter(|node| node.trust_level >= min_trust)
            .collect()
    }
    
    /// Get discovery statistics
    pub fn get_statistics(&self) -> DiscoveryStatistics {
        let total_nodes = self.known_nodes.len();
        let mut trust_distribution = HashMap::new();
        let mut role_distribution = HashMap::new();
        
        for node in self.known_nodes.values() {
            *trust_distribution.entry(node.trust_level.clone()).or_insert(0) += 1;
            *role_distribution.entry(node.archetypal_role.clone()).or_insert(0) += 1;
        }
        
        DiscoveryStatistics {
            total_nodes,
            trust_distribution,
            role_distribution,
            state: self.state.clone(),
        }
    }
    
    /// Get current discovery state
    pub fn get_state(&self) -> &DiscoveryState {
        &self.state
    }
    
    /// Get discovery configuration
    pub fn get_config(&self) -> &DiscoveryConfig {
        &self.config
    }
    
    /// Update discovery configuration
    pub fn update_config(&mut self, config: DiscoveryConfig) {
        self.config = config;
        debug!("Updated discovery configuration");
    }
}

/// Discovery statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryStatistics {
    /// Total number of known nodes
    pub total_nodes: usize,
    /// Distribution of trust levels
    pub trust_distribution: HashMap<TrustLevel, usize>,
    /// Distribution of archetypal roles
    pub role_distribution: HashMap<ArchetypalRole, usize>,
    /// Current discovery state
    pub state: DiscoveryState,
}

/// Trait for context-specific discovery providers
pub trait DiscoveryProvider: Send + Sync {
    /// Initialize context-specific discovery
    async fn initialize(&mut self) -> Result<()>;
    
    /// Cleanup context-specific discovery
    async fn cleanup(&mut self) -> Result<()>;
    
    /// Handle discovery events
    async fn handle_event(&mut self, event: DiscoveryEvent) -> Result<()>;
    
    /// Get provider name
    fn name(&self) -> &str;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_discovery_creation() {
        let node_id = Uuid::new_v4();
        let capabilities = NodeCapabilities::default();
        let discovery = MeshDiscovery::new(node_id, capabilities, None);
        
        assert_eq!(discovery.node_id, node_id);
        assert_eq!(discovery.state, DiscoveryState::Stopped);
        assert_eq!(discovery.known_nodes.len(), 0);
    }
    
    #[test]
    fn test_node_management() {
        let node_id = Uuid::new_v4();
        let capabilities = NodeCapabilities::default();
        let mut discovery = MeshDiscovery::new(node_id, capabilities, None);
        
        let test_node = MeshNode {
            node_id: Uuid::new_v4(),
            capabilities: NodeCapabilities::default(),
            archetypal_role: ArchetypalRole::Sage,
            trust_level: TrustLevel::Basic,
            last_seen: Utc::now(),
            metadata: HashMap::new(),
            context_data: HashMap::new(),
        };
        
        // Add node
        let event = discovery.add_node(test_node.clone());
        assert!(matches!(event, Some(DiscoveryEvent::NodeDiscovered(_))));
        assert_eq!(discovery.known_nodes.len(), 1);
        
        // Update trust level
        let event = discovery.update_trust_level(&test_node.node_id, TrustLevel::Trusted);
        assert!(matches!(event, Some(DiscoveryEvent::TrustLevelChanged(_, TrustLevel::Trusted))));
        
        // Remove node
        let event = discovery.remove_node(&test_node.node_id);
        assert!(matches!(event, Some(DiscoveryEvent::NodeLost(_))));
        assert_eq!(discovery.known_nodes.len(), 0);
    }
    
    #[test]
    fn test_archetypal_role_communication_style() {
        assert_eq!(ArchetypalRole::Sage.communication_style(), CommunicationStyle::Analytical);
        assert_eq!(ArchetypalRole::Creator.communication_style(), CommunicationStyle::Creative);
        assert_eq!(ArchetypalRole::SacredPartnership.communication_style(), CommunicationStyle::Empathetic);
    }
}
