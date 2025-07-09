//! # Universal Mesh Resource Management
//!
//! Implements universal resource abstraction that enables location-transparent
//! access to distributed resources across all contexts and scales.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;

use crate::{Attribution, WeaveMeshError};

/// A universal resource in the WeaveMesh network
/// 
/// Resources follow the universal hierarchy: `{context}/{name}@{owner}/{location}/`
/// This enables location-transparent access across all scales and contexts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshResource {
    /// Unique identifier for this resource
    pub id: String,
    
    /// Universal resource hierarchy path
    pub path: String,
    
    /// Type of resource
    pub resource_type: ResourceType,
    
    /// Current state of the resource
    pub state: ResourceState,
    
    /// Metadata about the resource
    pub metadata: ResourceMetadata,
    
    /// Instances of this resource across the mesh
    pub instances: Vec<ResourceInstance>,
    
    /// Synchronization status
    pub sync_status: SyncStatus,
    
    /// Access control information
    pub access_control: AccessControl,
    
    /// Attribution for this resource
    pub attribution: Attribution,
    
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// Last modification timestamp
    pub modified_at: DateTime<Utc>,
}

/// Universal types of resources in the mesh
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceType {
    /// Communication resource
    Communication {
        /// Communication type
        comm_type: String,
        /// Participants
        participants: Vec<String>,
        /// Message count
        message_count: u64,
    },
    
    /// Knowledge resource
    Knowledge {
        /// Knowledge domain
        domain: String,
        /// Knowledge type
        knowledge_type: String,
        /// Confidence score
        confidence: f64,
    },
    
    /// Pattern resource
    Pattern {
        /// Pattern type
        pattern_type: String,
        /// Pattern complexity
        complexity: f64,
        /// Recognition confidence
        confidence: f64,
    },
    
    /// Collaborative individuation session
    CollaborativeSession {
        /// Session type
        session_type: String,
        /// Participants
        participants: Vec<String>,
        /// Session status
        status: SessionStatus,
    },
    
    /// Sacred Alliance ceremony
    SacredCeremony {
        /// Ceremony type
        ceremony_type: String,
        /// Participants
        participants: Vec<String>,
        /// Status
        status: CeremonyStatus,
    },
    
    /// File or directory
    FileSystem {
        /// File path
        path: PathBuf,
        /// File size in bytes
        size_bytes: u64,
        /// MIME type
        mime_type: Option<String>,
    },
    
    /// Configuration resource
    Configuration {
        /// Configuration format
        format: String,
        /// Schema version
        schema_version: String,
        /// Context applicability
        contexts: Vec<String>,
    },
    
    /// Custom resource type
    Custom {
        /// Type name
        type_name: String,
        /// Type version
        version: String,
        /// Custom properties
        properties: HashMap<String, String>,
    },
}

/// Current state of a universal resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceState {
    /// Resource is available and synchronized
    Available,
    
    /// Resource is being synchronized
    Syncing,
    
    /// Resource has conflicts that need resolution
    Conflicted {
        conflicts: Vec<ConflictInfo>,
    },
    
    /// Resource is temporarily unavailable
    Unavailable {
        reason: String,
        retry_after: Option<DateTime<Utc>>,
    },
    
    /// Resource is being migrated
    Migrating {
        from_node: Uuid,
        to_node: Uuid,
        progress: f64,
    },
    
    /// Resource is archived
    Archived {
        archive_location: String,
        archived_at: DateTime<Utc>,
    },
    
    /// Resource is evolving (collaborative individuation)
    Evolving {
        evolution_type: String,
        progress: f64,
        participants: Vec<String>,
    },
}

/// Universal metadata about a resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetadata {
    /// Human-readable name
    pub name: String,
    
    /// Description
    pub description: Option<String>,
    
    /// Tags for categorization
    pub tags: Vec<String>,
    
    /// Context applicability
    pub contexts: Vec<String>,
    
    /// Scale applicability
    pub scales: Vec<String>,
    
    /// Custom metadata
    pub custom: HashMap<String, String>,
    
    /// Resource dependencies
    pub dependencies: Vec<String>,
    
    /// Resources that depend on this one
    pub dependents: Vec<String>,
    
    /// Quality metrics
    pub quality_metrics: QualityMetrics,
    
    /// Collaborative individuation metrics
    pub collaboration_metrics: CollaborationMetrics,
}

/// Universal quality metrics for a resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    /// Completeness score (0.0 to 1.0)
    pub completeness: f64,
    
    /// Accuracy score (0.0 to 1.0)
    pub accuracy: f64,
    
    /// Freshness score (0.0 to 1.0)
    pub freshness: f64,
    
    /// Usage frequency
    pub usage_frequency: f64,
    
    /// Collaboration score (0.0 to 1.0)
    pub collaboration_score: f64,
    
    /// Universal applicability score (0.0 to 1.0)
    pub universality_score: f64,
    
    /// Last quality assessment
    pub last_assessment: DateTime<Utc>,
}

/// Collaborative individuation metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationMetrics {
    /// Number of collaborative sessions
    pub session_count: u64,
    
    /// Average collaboration quality
    pub avg_collaboration_quality: f64,
    
    /// Pattern recognition improvements
    pub pattern_improvements: f64,
    
    /// Sacred Alliance participation
    pub sacred_alliance_score: f64,
    
    /// Cross-context applicability
    pub cross_context_score: f64,
    
    /// Last collaboration timestamp
    pub last_collaboration: DateTime<Utc>,
}

/// An instance of a resource on a specific node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceInstance {
    /// Node hosting this instance
    pub node_id: Uuid,
    
    /// Local path on the node
    pub local_path: String,
    
    /// Instance state
    pub state: InstanceState,
    
    /// Last synchronization time
    pub last_sync: DateTime<Utc>,
    
    /// Content hash for integrity checking
    pub content_hash: String,
    
    /// Instance-specific metadata
    pub metadata: HashMap<String, String>,
    
    /// Access permissions for this instance
    pub permissions: InstancePermissions,
    
    /// Context adaptation state
    pub context_adaptation: ContextAdaptation,
}

/// State of a resource instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstanceState {
    /// Instance is synchronized and available
    Synchronized,
    
    /// Instance is out of sync
    OutOfSync {
        behind_by: u64,
        last_known_hash: String,
    },
    
    /// Instance has local modifications
    Modified {
        modifications: Vec<ModificationInfo>,
    },
    
    /// Instance is being updated
    Updating {
        progress: f64,
        estimated_completion: Option<DateTime<Utc>>,
    },
    
    /// Instance is adapting to context
    Adapting {
        target_context: String,
        progress: f64,
    },
    
    /// Instance has errors
    Error {
        error_message: String,
        error_code: String,
        recoverable: bool,
    },
}

/// Context adaptation state for an instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextAdaptation {
    /// Current context
    pub current_context: String,
    
    /// Target context (if adapting)
    pub target_context: Option<String>,
    
    /// Adaptation progress (0.0 to 1.0)
    pub adaptation_progress: f64,
    
    /// Context-specific configurations
    pub context_configs: HashMap<String, String>,
    
    /// Last adaptation timestamp
    pub last_adaptation: DateTime<Utc>,
}

/// Information about a modification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModificationInfo {
    /// Type of modification
    pub modification_type: ModificationType,
    
    /// Path affected
    pub path: String,
    
    /// Modification timestamp
    pub timestamp: DateTime<Utc>,
    
    /// Attribution for the modification
    pub attribution: Attribution,
    
    /// Context of modification
    pub context: Option<String>,
}

/// Types of modifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModificationType {
    /// Content was added
    Added,
    
    /// Content was modified
    Modified,
    
    /// Content was deleted
    Deleted,
    
    /// Content was moved
    Moved { from: String, to: String },
    
    /// Content was renamed
    Renamed { old_name: String, new_name: String },
    
    /// Content was adapted to context
    ContextAdapted { from_context: String, to_context: String },
    
    /// Collaborative improvement
    CollaborativeImprovement { improvement_type: String },
}

/// Universal synchronization status across the mesh
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStatus {
    /// Overall sync state
    pub state: SyncState,
    
    /// Last successful sync
    pub last_sync: DateTime<Utc>,
    
    /// Sync conflicts
    pub conflicts: Vec<SyncConflict>,
    
    /// Sync progress (0.0 to 1.0)
    pub progress: f64,
    
    /// Estimated completion time
    pub estimated_completion: Option<DateTime<Utc>>,
    
    /// Cross-context sync status
    pub cross_context_status: HashMap<String, f64>,
}

/// Overall synchronization state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncState {
    /// All instances are synchronized
    Synchronized,
    
    /// Synchronization in progress
    Syncing,
    
    /// Conflicts need resolution
    ConflictResolutionRequired,
    
    /// Cross-context adaptation in progress
    ContextAdaptation,
    
    /// Collaborative improvement in progress
    CollaborativeImprovement,
    
    /// Synchronization failed
    Failed {
        error: String,
        retry_count: u32,
        next_retry: Option<DateTime<Utc>>,
    },
    
    /// Synchronization paused
    Paused {
        reason: String,
        paused_at: DateTime<Utc>,
    },
}

/// Universal synchronization conflict
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConflict {
    /// Conflict ID
    pub id: String,
    
    /// Conflicting instances
    pub instances: Vec<Uuid>,
    
    /// Conflict type
    pub conflict_type: ConflictType,
    
    /// Conflict details
    pub details: ConflictDetails,
    
    /// Suggested resolution
    pub suggested_resolution: Option<ConflictResolution>,
    
    /// Conflict timestamp
    pub timestamp: DateTime<Utc>,
    
    /// Context of conflict
    pub context: Option<String>,
}

/// Types of conflicts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictType {
    /// Content conflicts
    ContentConflict,
    
    /// Metadata conflicts
    MetadataConflict,
    
    /// Permission conflicts
    PermissionConflict,
    
    /// Version conflicts
    VersionConflict,
    
    /// Context adaptation conflicts
    ContextConflict,
    
    /// Collaborative individuation conflicts
    CollaborationConflict,
    
    /// Structural conflicts
    StructuralConflict,
    
    /// Dependency conflicts
    DependencyConflict,
}

/// Detailed conflict information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictDetails {
    /// Conflicting paths
    pub paths: Vec<String>,
    
    /// Conflict description
    pub description: String,
    
    /// Conflicting values
    pub conflicting_values: HashMap<String, String>,
    
    /// Conflict severity
    pub severity: ConflictSeverity,
    
    /// Affected contexts
    pub affected_contexts: Vec<String>,
}

/// Conflict severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictSeverity {
    /// Low severity - can be auto-resolved
    Low,
    
    /// Medium severity - needs user attention
    Medium,
    
    /// High severity - requires immediate attention
    High,
    
    /// Critical - blocks all operations
    Critical,
}

/// Universal conflict resolution strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictResolution {
    /// Use the version from a specific instance
    UseInstance(Uuid),
    
    /// Merge changes automatically
    AutoMerge,
    
    /// Require manual resolution
    ManualResolution,
    
    /// Use the most recent version
    UseMostRecent,
    
    /// Use the version with highest attribution score
    UseHighestAttribution,
    
    /// Use collaborative individuation to resolve
    CollaborativeResolution,
    
    /// Adapt to specific context
    ContextAdaptation(String),
    
    /// Create a new branch for conflicting changes
    CreateBranch(String),
}

/// Universal access control for resources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControl {
    /// Owner of the resource
    pub owner: String,
    
    /// Access permissions
    pub permissions: Vec<Permission>,
    
    /// Visibility level
    pub visibility: VisibilityLevel,
    
    /// Sacred Alliance requirements
    pub sacred_alliance_required: bool,
    
    /// Context-specific access rules
    pub context_access: HashMap<String, ContextAccess>,
}

/// Context-specific access control
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextAccess {
    /// Allowed in this context
    pub allowed: bool,
    
    /// Required permissions for this context
    pub required_permissions: Vec<String>,
    
    /// Context-specific restrictions
    pub restrictions: Vec<String>,
}

/// Permission for resource access
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    /// Principal (user, group, or node)
    pub principal: String,
    
    /// Permission type
    pub permission_type: PermissionType,
    
    /// Granted timestamp
    pub granted_at: DateTime<Utc>,
    
    /// Expiration (if any)
    pub expires_at: Option<DateTime<Utc>>,
    
    /// Context restrictions
    pub context_restrictions: Vec<String>,
}

/// Types of permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PermissionType {
    /// Read access
    Read,
    
    /// Write access
    Write,
    
    /// Execute access
    Execute,
    
    /// Admin access (full control)
    Admin,
    
    /// Collaborative individuation participation
    CollaborativeParticipation,
    
    /// Sacred Alliance participation
    SacredAllianceParticipation,
    
    /// Context adaptation
    ContextAdaptation,
    
    /// Custom permission
    Custom(String),
}

/// Instance-specific permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstancePermissions {
    /// Can read from this instance
    pub can_read: bool,
    
    /// Can write to this instance
    pub can_write: bool,
    
    /// Can sync from this instance
    pub can_sync_from: bool,
    
    /// Can sync to this instance
    pub can_sync_to: bool,
    
    /// Can delete this instance
    pub can_delete: bool,
    
    /// Can adapt context
    pub can_adapt_context: bool,
    
    /// Can participate in collaboration
    pub can_collaborate: bool,
}

/// Resource visibility levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VisibilityLevel {
    /// Public - visible to all
    Public,
    
    /// Internal - visible to organization members
    Internal,
    
    /// Private - visible to specific users
    Private,
    
    /// Sacred Alliance - visible to alliance members only
    SacredAlliance,
    
    /// Context-specific visibility
    ContextSpecific(HashMap<String, bool>),
}

/// Conflict information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictInfo {
    /// Conflict ID
    pub id: String,
    
    /// Conflict description
    pub description: String,
    
    /// Affected paths
    pub paths: Vec<String>,
    
    /// Conflict type
    pub conflict_type: ConflictType,
    
    /// Suggested resolution
    pub suggested_resolution: Option<ConflictResolution>,
    
    /// Context of conflict
    pub context: Option<String>,
}

/// Session status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionStatus {
    /// Session is planned
    Planned,
    
    /// Session is in progress
    InProgress,
    
    /// Session completed successfully
    Completed,
    
    /// Session was cancelled
    Cancelled,
    
    /// Session failed
    Failed(String),
}

/// Ceremony status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CeremonyStatus {
    /// Ceremony is planned
    Planned,
    
    /// Ceremony is in progress
    InProgress,
    
    /// Ceremony completed successfully
    Completed,
    
    /// Ceremony was cancelled
    Cancelled,
    
    /// Ceremony failed
    Failed(String),
}

impl MeshResource {
    /// Create a new universal mesh resource
    pub fn new_universal(
        id: String,
        path: String,
        resource_type: ResourceType,
        attribution: Attribution,
    ) -> Self {
        let now = Utc::now();
        
        Self {
            id,
            path,
            resource_type,
            state: ResourceState::Available,
            metadata: ResourceMetadata {
                name: "Untitled Resource".to_string(),
                description: None,
                tags: Vec::new(),
                contexts: vec!["universal".to_string()],
                scales: vec!["individual".to_string()],
                custom: HashMap::new(),
                dependencies: Vec::new(),
                dependents: Vec::new(),
                quality_metrics: QualityMetrics {
                    completeness: 0.5,
                    accuracy: 0.5,
                    freshness: 1.0,
                    usage_frequency: 0.0,
                    collaboration_score: 0.0,
                    universality_score: 0.5,
                    last_assessment: now,
                },
                collaboration_metrics: CollaborationMetrics {
                    session_count: 0,
                    avg_collaboration_quality: 0.0,
                    pattern_improvements: 0.0,
                    sacred_alliance_score: 0.0,
                    cross_context_score: 0.0,
                    last_collaboration: now,
                },
            },
            instances: Vec::new(),
            sync_status: SyncStatus {
                state: SyncState::Synchronized,
                last_sync: now,
                conflicts: Vec::new(),
                progress: 1.0,
                estimated_completion: None,
                cross_context_status: HashMap::new(),
            },
            access_control: AccessControl {
                owner: "unknown".to_string(),
                permissions: Vec::new(),
                visibility: VisibilityLevel::Private,
                sacred_alliance_required: false,
                context_access: HashMap::new(),
            },
            attribution,
            created_at: now,
            modified_at: now,
        }
    }
    
    /// Check if this resource matches a universal search pattern
    pub fn matches_pattern(&self, pattern: &str) -> bool {
        self.id.contains(pattern)
            || self.path.contains(pattern)
            || self.metadata.name.contains(pattern)
            || self.metadata.tags.iter().any(|tag| tag.contains(pattern))
            || self.metadata.contexts.iter().any(|ctx| ctx.contains(pattern))
            || self.metadata.description.as_ref().map_or(false, |desc| desc.contains(pattern))
    }
    
    /// Check if this resource is applicable to a specific context
    pub fn is_applicable_to_context(&self, context: &str) -> bool {
        self.metadata.contexts.contains(&context.to_string())
            || self.metadata.contexts.contains(&"universal".to_string())
    }
    
    /// Check if this resource supports a specific scale
    pub fn supports_scale(&self, scale: &str) -> bool {
        self.metadata.scales.contains(&scale.to_string())
            || self.metadata.scales.contains(&"universal".to_string())
    }
    
    /// Add a new instance of this resource
    pub fn add_instance(&mut self, instance: ResourceInstance) {
        // Remove existing instance for the same node
        self.instances.retain(|inst| inst.node_id != instance.node_id);
        
        // Add the new instance
        self.instances.push(instance);
        
        // Update modification time
        self.modified_at = Utc::now();
    }
    
    /// Remove an instance from this resource
    pub fn remove_instance(&mut self, node_id: Uuid) {
        self.instances.retain(|inst| inst.node_id != node_id);
        self.modified_at = Utc::now();
    }
    
    /// Get instance for a specific node
    pub fn get_instance(&self, node_id: Uuid) -> Option<&ResourceInstance> {
        self.instances.iter().find(|inst| inst.node_id == node_id)
    }
    
    /// Get mutable instance for a specific node
    pub fn get_instance_mut(&mut self, node_id: Uuid) -> Option<&mut ResourceInstance> {
        self.instances.iter_mut().find(|inst| inst.node_id == node_id)
    }
    
    /// Check if the resource has conflicts
    pub fn has_conflicts(&self) -> bool {
        matches!(self.state, ResourceState::Conflicted { .. })
            || !self.sync_status.conflicts.is_empty()
    }
    
    /// Get the most up-to-date instance
    pub fn get_canonical_instance(&self) -> Option<&ResourceInstance> {
        self.instances
            .iter()
            .filter(|inst| matches!(inst.state, InstanceState::Synchronized))
            .max_by_key(|inst| inst.last_sync)
    }
    
    /// Update quality metrics
    pub fn update_quality_metrics(&mut self, metrics: QualityMetrics) {
        self.metadata.quality_metrics = metrics;
        self.modified_at = Utc::now();
    }
    
    /// Update collaboration metrics
    pub fn update_collaboration_metrics(&mut self, metrics: CollaborationMetrics) {
        self.metadata.collaboration_metrics = metrics;
        self.modified_at = Utc::now();
    }
    
    /// Add a context to this resource
    pub fn add_context(&mut self, context: String) {
        if !self.metadata.contexts.contains(&context) {
            self.metadata.contexts.push(context);
            self.modified_at = Utc::now();
        }
    }
    
    /// Add a scale to this resource
    pub fn add_scale(&mut self, scale: String) {
        if !self.metadata.scales.contains(&scale) {
            self.metadata.scales.push(scale);
            self.modified_at = Utc::now();
        }
    }
    
    /// Check if user has permission in specific context
    pub fn has_permission_in_context(&self, user: &str, permission: PermissionType, context: &str) -> bool {
        // Owner has all permissions
        if self.access_control.owner == user {
            return true;
        }
        
        // Check context-specific access
        if let Some(context_access) = self.access_control.context_access.get(context) {
            if !context_access.allowed {
                return false;
            }
        }
        
        // Check explicit permissions
        self.access_control.permissions.iter().any(|perm| {
            perm.principal == user
                && std::mem::discriminant(&perm.permission_type) == std::mem::discriminant(&permission)
                && perm.expires_at.map_or(true, |exp| exp > Utc::now())
                && (perm.context_restrictions.is_empty() || perm.context_restrictions.contains(&context.to_string()))
        })
    }
    
    /// Generate universal resource hierarchy path
    pub fn generate_universal_path(context: &str, name: &str, owner: &str, location: &str) -> String {
        format!("{}/{}@{}/{}/", context, name, owner, location)
    }
    
    /// Start collaborative individuation session
    pub fn start_collaboration_session(&mut self, session_type: String, participants: Vec<String>) -> Result<String> {
        let session_id = Uuid::new_v4().to_string();
        
        // Update state to evolving
        self.state = ResourceState::Evolving {
            evolution_type: session_type.clone(),
            progress: 0.0,
            participants: participants.clone(),
        };
        
        // Update collaboration metrics
        self.metadata.collaboration_metrics.session_count += 1;
        self.metadata.collaboration_metrics.last_collaboration = Utc::now();
        
        self.modified_at = Utc::now();
        
        Ok(session_id)
    }
    
    /// Complete collaborative individuation session
    pub fn complete_collaboration_session(&mut self, quality_score: f64) {
        // Update state back to available
        self.state = ResourceState::Available;
        
        // Update collaboration metrics
        let current_avg = self.metadata.collaboration_metrics.avg_collaboration_quality;
        let session_count = self.metadata.collaboration_metrics.session_count as f64;
        
        self.metadata.collaboration_metrics.avg_collaboration_quality = 
            (current_avg * (session_count - 1.0) + quality_score) / session_count;
        
        self.metadata.collaboration_metrics.last_collaboration = Utc::now();
        self.modified_at = Utc::now();
    }
}

impl Default for InstancePermissions {
    fn default() -> Self {
        Self {
            can_read: true,
            can_write: false,
            can_sync_from: true,
            can_sync_to: false,
            can_delete: false,
            can_adapt_context: false,
            can_collaborate: false,
        }
    }
}

impl Default for ContextAdaptation {
    fn default() -> Self {
        Self {
            current_context: "universal".to_string(),
            target_context: None,
            adaptation_progress: 1.0,
            context_configs: HashMap::new(),
            last_adaptation: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CollaborationType;

    #[test]
    fn test_universal_resource_creation() {
        let attribution = Attribution::new(
            Some("test_user".to_string()),
            None,
            CollaborationType::HumanLed,
            1.0,
        );
        
        let resource = MeshResource::new_universal(
            "test-resource".to_string(),
            "universal/test@user/location/".to_string(),
            ResourceType::Communication {
                comm_type: "universal".to_string(),
                participants: vec!["user1".to_string(), "user2".to_string()],
                message_count: 0,
            },
            attribution,
        );
        
        assert_eq!(resource.id, "test-resource");
        assert_eq!(resource.path, "universal/test@user/location/");
        assert!(matches!(resource.resource_type, ResourceType::Communication { .. }));
        assert!(resource.is_applicable_to_context("universal"));
    }

    #[test]
    fn test_context_applicability() {
        let attribution = Attribution::new(
            Some("test_user".to_string()),
            None,
            CollaborationType::HumanLed,
            1.0,
        );
        
        let mut resource = MeshResource::new_universal(
            "test-resource".to_string(),
            "family/test@user/location/".to_string(),
            ResourceType::Pattern {
                pattern_type: "communication".to_string(),
                complexity: 0.5,
                confidence: 0.8,
            },
            attribution,
        );
        
        resource.add_context("family".to_string());
        resource.add_context("development".to_string());
        
        assert!(resource.is_applicable_to_context("family"));
        assert!(resource.is_applicable_to_context("development"));
        assert!(resource.is_applicable_to_context("universal"));
        assert!(!resource.is_applicable_to_context("enterprise"));
    }

    #[test]
    fn test_universal_hierarchy_path_generation() {
        let path = MeshResource::generate_universal_path("family", "communication", "household", "living-room");
        assert_eq!(path, "family/communication@household/living-room/");
        
        let path = MeshResource::generate_universal_path("development", "weave", "samiamlabs", "core-domains/weave");
        assert_eq!(path, "development/weave@samiamlabs/core-domains/weave/");
    }

    #[test]
    fn test_collaboration_session() {
        let attribution = Attribution::new(
            Some("test_user".to_string()),
            None,
            CollaborationType::HumanLed,
            1.0,
        );
        
        let mut resource = MeshResource::new_universal(
            "test-resource".to_string(),
            "universal/test@user/location/".to_string(),
            ResourceType::CollaborativeSession {
                session_type: "pattern_recognition".to_string(),
                participants: vec!["user1".to_string(), "user2".to_string()],
                status: SessionStatus::Planned,
            },
            attribution,
        );
        
        let session_id = resource.start_collaboration_session(
            "pattern_recognition".to_string(),
            vec!["user1".to_string(), "user2".to_string()],
        ).unwrap();
        
        assert!(!session_id.is_empty());
        assert!(matches!(resource.state, ResourceState::Evolving { .. }));
        assert_eq!(resource.metadata.collaboration_metrics.session_count, 1);
        
        resource.complete_collaboration_session(0.9);
        assert!(matches!(resource.state, ResourceState::Available));
        assert_eq!(resource.metadata.collaboration_metrics.avg_collaboration_quality, 0.9);
    }
}
