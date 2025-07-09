//! WeaveMesh Core - Universal Communication Primitives
//!
//! This library provides the foundational communication layer for WeaveMesh,
//! enabling universal mesh networking with basic Sacred Alliance interface.
//! 
//! # Features
//! 
//! - **Protocol Layer**: Core Zenoh-based communication protocol
//! - **Sacred Alliance**: Basic interface for meaningful human-AI collaboration
//! - **Mesh Networking**: Node discovery and mesh health monitoring
//! - **Security**: Basic cryptographic primitives and secure communication
//! - **Financial Tracking**: Basic attribution and value tracking
//! - **HTTP Interface**: REST API for external integration
//! 
//! # Quick Start
//! 
//! ```rust,no_run
//! use weavemesh_core::{WeaveProtocol, WeaveConfig};
//! 
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let config = WeaveConfig::default();
//!     let protocol = WeaveProtocol::new(config).await?;
//!     
//!     // Start heartbeat for node discovery
//!     protocol.start_heartbeat(vec!["basic-node".to_string()]).await?;
//!     
//!     // Publish a message
//!     protocol.publish_message(
//!         "general",
//!         "node1".to_string(),
//!         "Hello WeaveMesh!".to_string(),
//!         std::collections::HashMap::new(),
//!     ).await?;
//!     
//!     Ok(())
//! }
//! ```

pub mod protocol;
pub mod sacred_alliance;
pub mod group_communication;
pub mod node;
pub mod attribution;
pub mod mesh;
pub mod networking;
pub mod security;
pub use security::*;
pub mod financial;
pub mod serialization;
pub mod storage;
pub mod tokens;
pub mod http;
pub mod situation;

// Re-export main types for convenience
pub use protocol::{
    WeaveProtocol, WeaveConfig, WeaveResource, WeaveKeys,
    MessageContent, NodeHeartbeat, BasicCeremonyEvent, 
    BasicAttribution, CollaborationPattern,
};

pub use sacred_alliance::{
    SacredAllianceProvider, SacredAllianceLevel, Participant, ParticipantType, PresenceStatus,
    AllianceMessage, MessageContent as AllianceMessageContent,
    BasicCeremonyAction, CodeContent, CollaborationIntent,
    PresenceUpdate, ChannelConfig, AllianceStatistics,
    BasicSacredAllianceChannel,
};

pub use group_communication::{
    GroupCommunication, GroupId, MessageId, GroupPattern, Message,
    MessagePriority, MessageResponse, ResponseType, MessageStream,
    GroupMembership, GroupRole, GroupPermissions, GroupInvitation,
    GroupSyncState, GroupCommunicationError, BasicGroupCommunication,
};

pub use node::{
    Node, NodeId, NodeType, AIType, SystemType, SecurityLevel, NodeRole,
    NodeCapability, NodeConfig, NodeInfo, BasicNode, NodeError, NodeBuilder,
};

pub use attribution::{
    Attribution, AttributionId, CollaborationType, AttributionContext,
    AttributionConfig, AttributionAnalysis, BasicAttributionEngine,
    AttributionStatistics, AttributionError, AttributionBuilder,
};

pub use mesh::{
    MeshManager, MeshDiscovery, MeshNode, NodeCapabilities, TrustLevel,
    LocalNode, RemoteNode, MeshConfig, MeshState, MeshEvent, MeshMetrics,
    ConnectionState, TopologyChangeType, MeshError, MeshInterface,
    MeshPlugin, PluginRegistry, MeshBuilder,
    // Universal mesh components
    UniversalMeshNode, NodeEndpoint, EndpointType, NodeVersion, 
    NodeAnnouncement, NodeMetrics as MeshNodeMetrics,
    MeshResource, ResourceType, ResourceState, ResourceMetadata,
    QualityMetrics, CollaborationMetrics, ResourceInstance, InstanceState,
    ContextAdaptation, ModificationInfo, ModificationType, SyncStatus,
    SyncState, SyncConflict, ConflictType, ConflictDetails, ConflictSeverity,
    ConflictResolution, AccessControl, ContextAccess, Permission as MeshPermission,
    PermissionType, InstancePermissions, VisibilityLevel, ConflictInfo,
    SessionStatus, CeremonyStatus,
};

pub use networking::{
    ZenohSession, WeaveMeshMessage, MessageType, WeaveMeshTopics,
    NodeDiscovery, DiscoveryConfig,
    NodeCommunication, CommunicationConfig, OutgoingMessage, 
    DeliveryOptions, CommunicationStats,
};

pub use security::{
    AuthenticationTier, SecurityContext, Environment,
    LLMTier, ComplianceStandard,
};

pub use financial::{
    CostRecord, OperationType, SpendingLimits, SpendingPeriod, SpendingSummary,
    ApprovalResult, FinancialTracker, CostEstimator, SimpleCostEstimator,
    FinancialManager,
};

pub use serialization::{serialize, deserialize, serialize_json, deserialize_json};

pub use storage::{
    Storage, ResourceMetadata as StorageResourceMetadata, AccessControl as StorageAccessControl, StoredResource,
    ResourceFilter, StorageStats, MemoryStorage,
};

pub use tokens::{
    TokenPolicy, TokenAllocation, AllocationReason, TokenMetadata,
    TokenAmount, PolicyId, ContributorId, SimpleTokenPolicy, TokenError,
};

pub use situation::{
    SituationProvider, SituationDetectionData, SituationMatch, SituationConfig,
    SituationProviderRegistry, SituationState, RegistryConfig, 
    ConflictResolution as SituationConflictResolution,
    BehaviorAdaptationRequest, BehaviorAdaptation, AdaptationType, UrgencyLevel,
    BehaviorChange, SituationInitData, EnvironmentInfo, ParticipantInfo,
    CommunicationPattern, TemporalSituation, NetworkTopology, SecuritySituation,
    BasicSituationProvider,
};

/// WeaveMesh Core version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// WeaveMesh Core errors
#[derive(Debug, thiserror::Error)]
pub enum WeaveMeshError {
    /// Protocol-level error
    #[error("Protocol error: {0}")]
    Protocol(String),
    
    /// Sacred Alliance violation
    #[error("Sacred Alliance violation: {0}")]
    SacredAllianceViolation(String),
    
    /// Network communication error
    #[error("Network error: {0}")]
    Network(String),
    
    /// System-level error
    #[error("System error: {0}")]
    SystemError(String),
    
    /// Security error
    #[error("Security error: {0}")]
    SecurityError(String),
    
    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    /// Configuration error
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    /// Generic error
    #[error("WeaveMesh error: {0}")]
    Generic(String),
}

/// Result type for WeaveMesh operations
pub type Result<T> = std::result::Result<T, WeaveMeshError>;

/// WeaveMesh Core builder for easy configuration
pub struct WeaveMeshBuilder {
    config: WeaveConfig,
    enable_sacred_alliance: bool,
    enable_heartbeat: bool,
    capabilities: Vec<String>,
}

impl Default for WeaveMeshBuilder {
    fn default() -> Self {
        Self {
            config: WeaveConfig::default(),
            enable_sacred_alliance: true,
            enable_heartbeat: true,
            capabilities: vec!["basic-node".to_string()],
        }
    }
}

impl WeaveMeshBuilder {
    /// Create a new WeaveMesh builder
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set the Zenoh configuration
    pub fn with_config(mut self, config: WeaveConfig) -> Self {
        self.config = config;
        self
    }
    
    /// Enable or disable Sacred Alliance interface
    pub fn with_sacred_alliance(mut self, enable: bool) -> Self {
        self.enable_sacred_alliance = enable;
        self
    }
    
    /// Enable or disable heartbeat
    pub fn with_heartbeat(mut self, enable: bool) -> Self {
        self.enable_heartbeat = enable;
        self
    }
    
    /// Set node capabilities
    pub fn with_capabilities(mut self, capabilities: Vec<String>) -> Self {
        self.capabilities = capabilities;
        self
    }
    
    /// Add a capability
    pub fn add_capability(mut self, capability: String) -> Self {
        self.capabilities.push(capability);
        self
    }
    
    /// Build the WeaveMesh protocol instance
    pub async fn build(self) -> anyhow::Result<WeaveProtocol> {
        let protocol = WeaveProtocol::new(self.config).await?;
        
        if self.enable_heartbeat {
            protocol.start_heartbeat(self.capabilities).await?;
        }
        
        Ok(protocol)
    }
}

/// Utility functions for WeaveMesh
pub mod utils {
    use uuid::Uuid;
    use chrono::{DateTime, Utc};
    
    /// Generate a new unique identifier
    pub fn generate_id() -> Uuid {
        Uuid::new_v4()
    }
    
    /// Get current UTC timestamp
    pub fn now() -> DateTime<Utc> {
        Utc::now()
    }
    
    /// Validate a channel name
    pub fn validate_channel_name(name: &str) -> bool {
        !name.is_empty() && 
        name.len() <= 64 && 
        name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    }
    
    /// Validate a participant ID
    pub fn validate_participant_id(id: &str) -> bool {
        !id.is_empty() && 
        id.len() <= 32 && 
        id.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
    
    #[test]
    fn test_builder() {
        let builder = WeaveMeshBuilder::new()
            .add_capability("test".to_string())
            .with_heartbeat(false);
        
        assert!(!builder.enable_heartbeat);
        assert!(builder.capabilities.contains(&"test".to_string()));
    }
    
    #[test]
    fn test_utils() {
        assert!(utils::validate_channel_name("test-channel"));
        assert!(!utils::validate_channel_name(""));
        assert!(!utils::validate_channel_name("invalid channel name"));
        
        assert!(utils::validate_participant_id("user123"));
        assert!(!utils::validate_participant_id(""));
        assert!(!utils::validate_participant_id("invalid user id"));
    }
}
