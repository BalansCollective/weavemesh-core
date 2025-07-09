//! Universal Group Communication Interface
//! 
//! This module provides the universal TALK/LISTEN/JOIN_GROUP/SYNC_STATE
//! primitives that enable group-aware communication. Context-specific
//! behaviors are implemented through plugins.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Unique identifier for a group
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GroupId(String);

impl GroupId {
    /// Create a new group ID from a string
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }
    
    /// Get the string representation
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for GroupId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for GroupId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

/// Unique identifier for a message
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MessageId(Uuid);

impl MessageId {
    /// Create a new random message ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    /// Get the string representation
    pub fn as_string(&self) -> String {
        self.0.to_string()
    }
}

impl Default for MessageId {
    fn default() -> Self {
        Self::new()
    }
}

/// Group pattern for subscription-based listening
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupPattern {
    /// Pattern string (e.g., "group/family/**", "group/engineering/urgent")
    pub pattern: String,
    /// Whether to include historical messages
    pub include_history: bool,
    /// Maximum number of historical messages to include
    pub history_limit: Option<usize>,
}

impl GroupPattern {
    /// Create a new group pattern
    pub fn new(pattern: &str) -> Self {
        Self {
            pattern: pattern.to_string(),
            include_history: false,
            history_limit: None,
        }
    }
    
    /// Create a pattern that includes historical messages
    pub fn with_history(pattern: &str, limit: Option<usize>) -> Self {
        Self {
            pattern: pattern.to_string(),
            include_history: true,
            history_limit: limit,
        }
    }
    
    /// Check if a group ID matches this pattern
    pub fn matches(&self, group_id: &GroupId) -> bool {
        // Simple pattern matching - can be enhanced by plugins
        if self.pattern.ends_with("/**") {
            let prefix = &self.pattern[..self.pattern.len() - 3];
            group_id.as_str().starts_with(prefix)
        } else if self.pattern.contains('*') {
            // More complex pattern matching can be implemented by plugins
            false
        } else {
            group_id.as_str() == self.pattern
        }
    }
}

/// Basic message content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Unique message identifier
    pub id: MessageId,
    /// Message content
    pub content: String,
    /// Sender identifier
    pub sender: String,
    /// When the message was created
    pub timestamp: DateTime<Utc>,
    /// Message metadata (context-specific data goes here)
    pub metadata: HashMap<String, String>,
    /// Message priority
    pub priority: MessagePriority,
    /// Whether this message requires acknowledgment
    pub requires_ack: bool,
}

/// Message priority levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessagePriority {
    Low,
    Normal,
    High,
    Urgent,
}

impl Default for MessagePriority {
    fn default() -> Self {
        Self::Normal
    }
}

/// Response to a message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageResponse {
    /// Response to message ID
    pub message_id: MessageId,
    /// Response content
    pub content: String,
    /// Response sender
    pub sender: String,
    /// Response timestamp
    pub timestamp: DateTime<Utc>,
    /// Response type
    pub response_type: ResponseType,
}

/// Types of message responses
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ResponseType {
    Acknowledgment,
    Reply,
    Reaction(String), // e.g., "👍", "❤️", "🤔"
    Question,
    Suggestion,
}

/// Stream of messages for listening
pub type MessageStream = mpsc::Receiver<Message>;

/// Group membership information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupMembership {
    /// Group identifier
    pub group_id: GroupId,
    /// Member's role in the group
    pub role: GroupRole,
    /// Member's permissions
    pub permissions: GroupPermissions,
    /// When the member joined
    pub joined_at: DateTime<Utc>,
    /// Whether membership is active
    pub is_active: bool,
    /// Context-specific metadata
    pub metadata: HashMap<String, String>,
}

/// Roles within a group
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GroupRole {
    Member,
    Moderator,
    Administrator,
    Observer,
    Custom(String),
}

/// Permissions within a group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupPermissions {
    pub can_send_messages: bool,
    pub can_read_messages: bool,
    pub can_invite_members: bool,
    pub can_remove_members: bool,
    pub can_modify_group: bool,
    pub can_access_history: bool,
}

impl Default for GroupPermissions {
    fn default() -> Self {
        Self {
            can_send_messages: true,
            can_read_messages: true,
            can_invite_members: false,
            can_remove_members: false,
            can_modify_group: false,
            can_access_history: true,
        }
    }
}

/// Invitation to join a group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupInvitation {
    /// Invitation ID
    pub id: Uuid,
    /// Group to join
    pub group_id: GroupId,
    /// Who sent the invitation
    pub inviter: String,
    /// Who is being invited
    pub invitee: String,
    /// Proposed role
    pub role: GroupRole,
    /// Proposed permissions
    pub permissions: GroupPermissions,
    /// Invitation message
    pub message: Option<String>,
    /// When invitation was created
    pub created_at: DateTime<Utc>,
    /// When invitation expires
    pub expires_at: Option<DateTime<Utc>>,
    /// Whether invitation has been accepted
    pub accepted: Option<bool>,
}

/// Group synchronization state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupSyncState {
    /// Group identifier
    pub group_id: GroupId,
    /// Vector clock for ordering
    pub vector_clock: HashMap<String, u64>,
    /// Last known message ID
    pub last_message_id: Option<MessageId>,
    /// Checksum of group state
    pub state_checksum: String,
    /// Timestamp of last sync
    pub last_sync: DateTime<Utc>,
}

/// Universal group communication trait
#[async_trait::async_trait]
pub trait GroupCommunication {
    /// Send a message to a group
    async fn talk(&self, group_id: GroupId, message: Message) -> Result<(), GroupCommunicationError>;
    
    /// Listen for messages matching a pattern
    async fn listen(&self, pattern: GroupPattern) -> Result<MessageStream, GroupCommunicationError>;
    
    /// Join a group with an invitation
    async fn join_group(&mut self, group_id: GroupId, invitation: GroupInvitation) -> Result<(), GroupCommunicationError>;
    
    /// Leave a group
    async fn leave_group(&mut self, group_id: GroupId) -> Result<(), GroupCommunicationError>;
    
    /// Synchronize state with a group
    async fn sync_state(&self, group_id: GroupId) -> Result<GroupSyncState, GroupCommunicationError>;
    
    /// Get current group memberships
    async fn get_memberships(&self) -> Result<Vec<GroupMembership>, GroupCommunicationError>;
    
    /// Send a response to a message
    async fn respond(&self, response: MessageResponse) -> Result<(), GroupCommunicationError>;
}

/// Errors that can occur in group communication
#[derive(Debug, thiserror::Error)]
pub enum GroupCommunicationError {
    #[error("Group not found: {0}")]
    GroupNotFound(String),
    
    #[error("Not a member of group: {0}")]
    NotAMember(String),
    
    #[error("Insufficient permissions for operation")]
    InsufficientPermissions,
    
    #[error("Invalid invitation: {0}")]
    InvalidInvitation(String),
    
    #[error("Message delivery failed: {0}")]
    DeliveryFailed(String),
    
    #[error("Synchronization failed: {0}")]
    SyncFailed(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Context mismatch: expected {expected}, got {actual}")]
    ContextMismatch { expected: String, actual: String },
    
    #[error("Group communication not initialized")]
    NotInitialized,
}

/// Basic group communication implementation using WeaveMesh protocol
pub struct BasicGroupCommunication {
    /// Node identifier
    node_id: String,
    /// Current group memberships
    memberships: HashMap<GroupId, GroupMembership>,
    /// Message history
    message_history: HashMap<GroupId, Vec<Message>>,
}

impl BasicGroupCommunication {
    /// Create a new basic group communication instance
    pub fn new(node_id: String) -> Self {
        Self {
            node_id,
            memberships: HashMap::new(),
            message_history: HashMap::new(),
        }
    }
    
    /// Add a group membership
    pub fn add_membership(&mut self, membership: GroupMembership) {
        self.memberships.insert(membership.group_id.clone(), membership);
    }
    
    /// Remove a group membership
    pub fn remove_membership(&mut self, group_id: &GroupId) {
        self.memberships.remove(group_id);
    }
    
    /// Get message history for a group
    pub fn get_message_history(&self, group_id: &GroupId) -> Option<&Vec<Message>> {
        self.message_history.get(group_id)
    }
    
    /// Add a message to history
    pub fn add_message_to_history(&mut self, group_id: GroupId, message: Message) {
        self.message_history.entry(group_id).or_insert_with(Vec::new).push(message);
    }
}

#[async_trait::async_trait]
impl GroupCommunication for BasicGroupCommunication {
    async fn talk(&self, group_id: GroupId, _message: Message) -> Result<(), GroupCommunicationError> {
        // Check if we're a member of the group
        let membership = self.memberships.get(&group_id)
            .ok_or_else(|| GroupCommunicationError::NotAMember(group_id.as_str().to_string()))?;
        
        // Check permissions
        if !membership.permissions.can_send_messages {
            return Err(GroupCommunicationError::InsufficientPermissions);
        }
        
        // In a real implementation, this would send the message through the mesh
        // For now, we'll just validate the operation
        Ok(())
    }
    
    async fn listen(&self, _pattern: GroupPattern) -> Result<MessageStream, GroupCommunicationError> {
        // Create a channel for message streaming
        let (tx, rx) = mpsc::channel(100);
        
        // In a real implementation, this would set up subscription to the mesh
        // For now, we'll just return the receiver
        drop(tx); // Close the sender to indicate no messages
        Ok(rx)
    }
    
    async fn join_group(&mut self, group_id: GroupId, invitation: GroupInvitation) -> Result<(), GroupCommunicationError> {
        // Validate invitation
        if invitation.group_id != group_id {
            return Err(GroupCommunicationError::InvalidInvitation("Group ID mismatch".to_string()));
        }
        
        if invitation.invitee != self.node_id {
            return Err(GroupCommunicationError::InvalidInvitation("Invitation not for this node".to_string()));
        }
        
        // Create membership
        let membership = GroupMembership {
            group_id: group_id.clone(),
            role: invitation.role,
            permissions: invitation.permissions,
            joined_at: chrono::Utc::now(),
            is_active: true,
            metadata: HashMap::new(),
        };
        
        self.add_membership(membership);
        Ok(())
    }
    
    async fn leave_group(&mut self, group_id: GroupId) -> Result<(), GroupCommunicationError> {
        self.remove_membership(&group_id);
        Ok(())
    }
    
    async fn sync_state(&self, group_id: GroupId) -> Result<GroupSyncState, GroupCommunicationError> {
        // Check if we're a member
        if !self.memberships.contains_key(&group_id) {
            return Err(GroupCommunicationError::NotAMember(group_id.as_str().to_string()));
        }
        
        // Create basic sync state
        let sync_state = GroupSyncState {
            group_id,
            vector_clock: HashMap::new(),
            last_message_id: None,
            state_checksum: "basic".to_string(),
            last_sync: chrono::Utc::now(),
        };
        
        Ok(sync_state)
    }
    
    async fn get_memberships(&self) -> Result<Vec<GroupMembership>, GroupCommunicationError> {
        Ok(self.memberships.values().cloned().collect())
    }
    
    async fn respond(&self, response: MessageResponse) -> Result<(), GroupCommunicationError> {
        // In a real implementation, this would send the response through the mesh
        // For now, we'll just validate the operation
        let _ = response; // Use the response to avoid unused variable warning
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_group_id_creation() {
        let group_id = GroupId::new("group/family");
        assert_eq!(group_id.as_str(), "group/family");
        
        let group_id2: GroupId = "group/engineering".into();
        assert_eq!(group_id2.as_str(), "group/engineering");
    }
    
    #[test]
    fn test_group_pattern_matching() {
        let pattern = GroupPattern::new("group/family/**");
        
        assert!(pattern.matches(&GroupId::new("group/family/general")));
        assert!(pattern.matches(&GroupId::new("group/family/urgent")));
        assert!(!pattern.matches(&GroupId::new("group/engineering/general")));
        
        let exact_pattern = GroupPattern::new("group/family");
        assert!(exact_pattern.matches(&GroupId::new("group/family")));
        assert!(!exact_pattern.matches(&GroupId::new("group/family/general")));
    }
    
    #[test]
    fn test_message_creation() {
        let message = Message {
            id: MessageId::new(),
            content: "Hello group!".to_string(),
            sender: "test_node".to_string(),
            timestamp: chrono::Utc::now(),
            metadata: HashMap::new(),
            priority: MessagePriority::Normal,
            requires_ack: false,
        };
        
        assert_eq!(message.content, "Hello group!");
        assert_eq!(message.priority, MessagePriority::Normal);
    }
    
    #[test]
    fn test_group_permissions() {
        let default_permissions = GroupPermissions::default();
        assert!(default_permissions.can_send_messages);
        assert!(default_permissions.can_read_messages);
        assert!(!default_permissions.can_invite_members);
        
        let admin_permissions = GroupPermissions {
            can_send_messages: true,
            can_read_messages: true,
            can_invite_members: true,
            can_remove_members: true,
            can_modify_group: true,
            can_access_history: true,
        };
        assert!(admin_permissions.can_modify_group);
    }
    
    #[tokio::test]
    async fn test_basic_group_communication() {
        let mut comm = BasicGroupCommunication::new("test_node".to_string());
        
        // Test joining a group
        let group_id = GroupId::new("test_group");
        let invitation = GroupInvitation {
            id: Uuid::new_v4(),
            group_id: group_id.clone(),
            inviter: "inviter".to_string(),
            invitee: "test_node".to_string(),
            role: GroupRole::Member,
            permissions: GroupPermissions::default(),
            message: None,
            created_at: chrono::Utc::now(),
            expires_at: None,
            accepted: None,
        };
        
        assert!(comm.join_group(group_id.clone(), invitation).await.is_ok());
        
        // Test getting memberships
        let memberships = comm.get_memberships().await.unwrap();
        assert_eq!(memberships.len(), 1);
        assert_eq!(memberships[0].group_id, group_id);
        
        // Test leaving group
        assert!(comm.leave_group(group_id.clone()).await.is_ok());
        
        let memberships = comm.get_memberships().await.unwrap();
        assert_eq!(memberships.len(), 0);
    }
}
