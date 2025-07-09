//! Universal Node Management
//! 
//! This module provides the core node interface for WeaveMesh,
//! representing any collaborative entity in the mesh - humans, AI agents,
//! systems, or hybrid combinations. Context-specific behaviors are
//! implemented through plugins.

use std::collections::HashMap;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Unique identifier for any node in the WeaveMesh
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(Uuid);

impl NodeId {
    /// Create a new random node ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Create a node ID from a string (for testing/debugging)
    pub fn from_string(s: &str) -> Result<Self, uuid::Error> {
        Ok(Self(Uuid::parse_str(s)?))
    }

    /// Get the string representation of the node ID
    pub fn as_string(&self) -> String {
        self.0.to_string()
    }
}

impl Default for NodeId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Type of entity that a node represents
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NodeType {
    /// Human individual or team
    Human,
    /// AI agent or system
    AI(AIType),
    /// System or service
    System(SystemType),
    /// Hybrid human-AI collaboration
    Hybrid,
}

/// Types of AI entities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AIType {
    /// Large Language Model
    LLM(String), // e.g., "gpt-4", "claude-3", "local-llama"
    /// Specialized AI agent
    Agent(String), // e.g., "code-analyzer", "medical-assistant"
    /// AI assistant or copilot
    Assistant,
    /// Autonomous system
    Autonomous,
}

/// Types of system entities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SystemType {
    /// Database or data store
    Database,
    /// API or web service
    API,
    /// IoT device or sensor
    IoTDevice,
    /// Legacy system
    Legacy,
    /// Cloud service
    Cloud,
}

/// Security classification levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum SecurityLevel {
    Public,
    Internal,
    Confidential,
    Secret,
    TopSecret,
    Custom(String),
}

/// Node roles for access control
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NodeRole {
    Individual,
    TeamLead,
    Administrator,
    Auditor,
    Service,
    Custom(String),
}

/// Capabilities that a node can advertise
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NodeCapability {
    // Universal capabilities
    ResourceStorage,
    AttributionTracking,
    Collaboration,
    
    // AI-specific capabilities
    NaturalLanguageProcessing,
    CodeGeneration,
    DataAnalysis,
    PatternRecognition,
    KnowledgeRetrieval,
    
    // System-specific capabilities
    DatabaseAccess,
    APIIntegration,
    SensorData,
    ComputeResources,
    
    // Domain-specific capabilities
    MedicalDataProcessing,
    LegalDocumentAnalysis,
    FinancialModeling,
    ScientificComputation,
    
    // Security capabilities
    Encryption,
    Authentication,
    AuditLogging,
    
    // Custom capability
    Custom(String),
}

/// Basic node configuration
#[derive(Debug, Clone)]
pub struct NodeConfig {
    /// Human-readable name for this node
    pub display_name: String,
    
    /// Organization or group identifier
    pub organization_id: String,
    
    /// Type of entity this node represents
    pub node_type: NodeType,
    
    /// Role of this node in the organization
    pub role: NodeRole,
    
    /// Security classification level
    pub security_level: SecurityLevel,
    
    /// Capabilities this node provides
    pub capabilities: Vec<NodeCapability>,
    
    /// Additional metadata
    pub metadata: HashMap<String, String>,
    
    /// Whether to enable debug logging
    pub debug_mode: bool,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            display_name: "WeaveMesh Node".to_string(),
            organization_id: "default-org".to_string(),
            node_type: NodeType::Human,
            role: NodeRole::Individual,
            security_level: SecurityLevel::Internal,
            capabilities: vec![
                NodeCapability::ResourceStorage,
                NodeCapability::AttributionTracking,
                NodeCapability::Collaboration,
            ],
            metadata: HashMap::new(),
            debug_mode: false,
        }
    }
}

/// Node information for discovery and networking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub node_id: NodeId,
    pub display_name: String,
    pub organization_id: String,
    pub node_type: NodeType,
    pub role: NodeRole,
    pub security_level: SecurityLevel,
    pub capabilities: Vec<NodeCapability>,
    pub metadata: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub is_active: bool,
}

/// Universal node trait for mesh participation
pub trait Node {
    /// Get the node ID
    fn id(&self) -> &NodeId;
    
    /// Get the display name for this node
    fn display_name(&self) -> &str;
    
    /// Get the organization ID for this node
    fn organization_id(&self) -> &str;
    
    /// Get the node type
    fn node_type(&self) -> &NodeType;
    
    /// Get the node role
    fn role(&self) -> &NodeRole;
    
    /// Get the security level
    fn security_level(&self) -> &SecurityLevel;
    
    /// Get the node capabilities
    fn capabilities(&self) -> &[NodeCapability];
    
    /// Check if the node has a specific capability
    fn has_capability(&self, capability: &NodeCapability) -> bool {
        self.capabilities().contains(capability)
    }
    
    /// Get node information for discovery
    fn get_node_info(&self) -> NodeInfo;
}

/// Basic node implementation
#[derive(Debug)]
pub struct BasicNode {
    /// Unique identifier for this node
    pub node_id: NodeId,
    
    /// Configuration for this node
    pub config: NodeConfig,
    
    /// Node creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// Last activity timestamp
    pub last_activity: DateTime<Utc>,
    
    /// Whether this node is currently active
    pub is_active: bool,
}

impl BasicNode {
    /// Create a new basic node
    pub fn new(config: NodeConfig) -> Self {
        let now = Utc::now();
        
        Self {
            node_id: NodeId::new(),
            config,
            created_at: now,
            last_activity: now,
            is_active: false,
        }
    }
    
    /// Start the node
    pub fn start(&mut self) -> Result<(), NodeError> {
        self.is_active = true;
        self.update_activity();
        
        if self.config.debug_mode {
            println!("BasicNode {} ({:?}) started", 
                self.display_name(), 
                self.node_type());
        }
        
        Ok(())
    }
    
    /// Stop the node
    pub fn stop(&mut self) -> Result<(), NodeError> {
        self.is_active = false;
        self.update_activity();
        
        if self.config.debug_mode {
            println!("BasicNode {} stopped", self.display_name());
        }
        
        Ok(())
    }
    
    /// Update last activity timestamp
    pub fn update_activity(&mut self) {
        self.last_activity = Utc::now();
    }
    
    /// Check if this node can collaborate with another node
    pub fn can_collaborate_with(&self, other: &dyn Node) -> bool {
        // Same organization
        if self.organization_id() == other.organization_id() {
            return true;
        }
        
        // Check security levels are compatible
        if self.security_level() <= other.security_level() {
            return true;
        }
        
        // Check for specific collaboration capabilities
        self.has_capability(&NodeCapability::Collaboration) &&
        other.has_capability(&NodeCapability::Collaboration)
    }
}

impl Node for BasicNode {
    fn id(&self) -> &NodeId {
        &self.node_id
    }
    
    fn display_name(&self) -> &str {
        &self.config.display_name
    }
    
    fn organization_id(&self) -> &str {
        &self.config.organization_id
    }
    
    fn node_type(&self) -> &NodeType {
        &self.config.node_type
    }
    
    fn role(&self) -> &NodeRole {
        &self.config.role
    }
    
    fn security_level(&self) -> &SecurityLevel {
        &self.config.security_level
    }
    
    fn capabilities(&self) -> &[NodeCapability] {
        &self.config.capabilities
    }
    
    fn get_node_info(&self) -> NodeInfo {
        NodeInfo {
            node_id: self.node_id.clone(),
            display_name: self.config.display_name.clone(),
            organization_id: self.config.organization_id.clone(),
            node_type: self.config.node_type.clone(),
            role: self.config.role.clone(),
            security_level: self.config.security_level.clone(),
            capabilities: self.config.capabilities.clone(),
            metadata: self.config.metadata.clone(),
            created_at: self.created_at,
            last_activity: self.last_activity,
            is_active: self.is_active,
        }
    }
}

/// Errors that can occur with nodes
#[derive(Debug, thiserror::Error)]
pub enum NodeError {
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Security error: {0}")]
    SecurityError(String),
    
    #[error("Node not active")]
    NodeNotActive,
    
    #[error("Insufficient permissions")]
    InsufficientPermissions,
    
    #[error("Capability not supported: {0}")]
    CapabilityNotSupported(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Generic node error: {0}")]
    Generic(String),
}

/// Node builder for easy configuration
pub struct NodeBuilder {
    config: NodeConfig,
}

impl Default for NodeBuilder {
    fn default() -> Self {
        Self {
            config: NodeConfig::default(),
        }
    }
}

impl NodeBuilder {
    /// Create a new node builder
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set the display name
    pub fn with_display_name(mut self, name: &str) -> Self {
        self.config.display_name = name.to_string();
        self
    }
    
    /// Set the organization ID
    pub fn with_organization(mut self, org_id: &str) -> Self {
        self.config.organization_id = org_id.to_string();
        self
    }
    
    /// Set the node type
    pub fn with_node_type(mut self, node_type: NodeType) -> Self {
        self.config.node_type = node_type;
        self
    }
    
    /// Set the node role
    pub fn with_role(mut self, role: NodeRole) -> Self {
        self.config.role = role;
        self
    }
    
    /// Set the security level
    pub fn with_security_level(mut self, level: SecurityLevel) -> Self {
        self.config.security_level = level;
        self
    }
    
    /// Add a capability
    pub fn add_capability(mut self, capability: NodeCapability) -> Self {
        self.config.capabilities.push(capability);
        self
    }
    
    /// Set capabilities
    pub fn with_capabilities(mut self, capabilities: Vec<NodeCapability>) -> Self {
        self.config.capabilities = capabilities;
        self
    }
    
    /// Add metadata
    pub fn add_metadata(mut self, key: &str, value: &str) -> Self {
        self.config.metadata.insert(key.to_string(), value.to_string());
        self
    }
    
    /// Enable debug mode
    pub fn with_debug(mut self, debug: bool) -> Self {
        self.config.debug_mode = debug;
        self
    }
    
    /// Build the node
    pub fn build(self) -> BasicNode {
        BasicNode::new(self.config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_node_id_creation() {
        let id1 = NodeId::new();
        let id2 = NodeId::new();
        
        // IDs should be unique
        assert_ne!(id1, id2);
        
        // String conversion should work
        let id_str = id1.as_string();
        let id3 = NodeId::from_string(&id_str).unwrap();
        assert_eq!(id1, id3);
    }
    
    #[test]
    fn test_basic_node_creation() {
        let config = NodeConfig {
            display_name: "Test Node".to_string(),
            organization_id: "test-org".to_string(),
            node_type: NodeType::Human,
            role: NodeRole::Individual,
            ..Default::default()
        };
        
        let node = BasicNode::new(config);
        assert_eq!(node.display_name(), "Test Node");
        assert_eq!(node.organization_id(), "test-org");
        assert_eq!(node.node_type(), &NodeType::Human);
        assert!(!node.is_active);
    }
    
    #[test]
    fn test_node_builder() {
        let node = NodeBuilder::new()
            .with_display_name("AI Assistant")
            .with_organization("ai-lab")
            .with_node_type(NodeType::AI(AIType::Assistant))
            .with_role(NodeRole::Service)
            .add_capability(NodeCapability::NaturalLanguageProcessing)
            .add_capability(NodeCapability::CodeGeneration)
            .add_metadata("version", "1.0")
            .with_debug(true)
            .build();
        
        assert_eq!(node.display_name(), "AI Assistant");
        assert_eq!(node.organization_id(), "ai-lab");
        assert!(node.has_capability(&NodeCapability::NaturalLanguageProcessing));
        assert!(node.config.debug_mode);
    }
    
    #[test]
    fn test_node_capabilities() {
        let node = NodeBuilder::new()
            .add_capability(NodeCapability::NaturalLanguageProcessing)
            .add_capability(NodeCapability::CodeGeneration)
            .build();
        
        assert!(node.has_capability(&NodeCapability::NaturalLanguageProcessing));
        assert!(node.has_capability(&NodeCapability::CodeGeneration));
        assert!(!node.has_capability(&NodeCapability::DatabaseAccess));
    }
    
    #[test]
    fn test_node_collaboration() {
        let node1 = NodeBuilder::new()
            .with_organization("same-org")
            .with_security_level(SecurityLevel::Internal)
            .add_capability(NodeCapability::Collaboration)
            .build();
        
        let node2 = NodeBuilder::new()
            .with_organization("same-org")
            .with_security_level(SecurityLevel::Internal)
            .add_capability(NodeCapability::Collaboration)
            .build();
        
        assert!(node1.can_collaborate_with(&node2));
        assert!(node2.can_collaborate_with(&node1));
    }
    
    #[test]
    fn test_node_start_stop() {
        let mut node = NodeBuilder::new().build();
        
        assert!(!node.is_active);
        
        assert!(node.start().is_ok());
        assert!(node.is_active);
        
        assert!(node.stop().is_ok());
        assert!(!node.is_active);
    }
    
    #[test]
    fn test_node_types() {
        let human_node = NodeBuilder::new()
            .with_node_type(NodeType::Human)
            .build();
        assert_eq!(human_node.node_type(), &NodeType::Human);
        
        let ai_node = NodeBuilder::new()
            .with_node_type(NodeType::AI(AIType::LLM("gpt-4".to_string())))
            .build();
        assert_eq!(ai_node.node_type(), &NodeType::AI(AIType::LLM("gpt-4".to_string())));
        
        let system_node = NodeBuilder::new()
            .with_node_type(NodeType::System(SystemType::Database))
            .build();
        assert_eq!(system_node.node_type(), &NodeType::System(SystemType::Database));
    }
    
    #[test]
    fn test_security_levels() {
        assert!(SecurityLevel::Public < SecurityLevel::Internal);
        assert!(SecurityLevel::Internal < SecurityLevel::Confidential);
        assert!(SecurityLevel::Confidential < SecurityLevel::Secret);
        assert!(SecurityLevel::Secret < SecurityLevel::TopSecret);
    }
}
