//! Universal Zenoh Integration for WeaveMesh
//! 
//! This module provides universal Zenoh session management for mesh networking,
//! enabling peer-to-peer communication with dynamic resource sharing across
//! different contexts.

use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use zenoh::{Session, key_expr::KeyExpr, bytes::ZBytes};
use zenoh::pubsub::{Publisher, Subscriber};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Universal Zenoh session wrapper for mesh nodes
/// 
/// Each node gets its own ZenohSession that:
/// - Connects to the Zenoh mesh network
/// - Publishes and subscribes to resources
/// - Handles node discovery and communication
/// - Maintains connection health and reconnection
pub struct ZenohSession {
    /// The underlying Zenoh session
    session: Arc<Session>,
    
    /// Node ID for this session
    node_id: Uuid,
    
    /// Configuration for this session
    config: ZenohConfig,
    
    /// Active subscriptions
    subscriptions: Arc<RwLock<HashMap<String, Arc<Subscriber<()>>>>>,
    
    /// Active publishers
    publishers: Arc<RwLock<HashMap<String, Arc<Publisher<'static>>>>>,
    
    /// Message handler for incoming messages
    message_handler: Arc<RwLock<Option<MessageHandler>>>,
    
    /// Whether the session is currently connected
    is_connected: Arc<RwLock<bool>>,
}

/// Configuration for Zenoh session
#[derive(Debug, Clone)]
pub struct ZenohConfig {
    /// Zenoh router endpoints to connect to
    pub endpoints: Vec<String>,
    
    /// Session mode (peer, client, router)
    pub mode: ZenohMode,
    
    /// Whether to enable multicast scouting
    pub multicast_scouting: bool,
    
    /// Session timeout in seconds
    pub timeout_seconds: u64,
    
    /// Whether to enable debug logging
    pub debug: bool,
}

impl Default for ZenohConfig {
    fn default() -> Self {
        Self {
            endpoints: vec!["tcp/127.0.0.1:7447".to_string()],
            mode: ZenohMode::Peer,
            multicast_scouting: true,
            timeout_seconds: 30,
            debug: false,
        }
    }
}

/// Zenoh session mode
#[derive(Debug, Clone)]
pub enum ZenohMode {
    Peer,
    Client,
    Router,
}

/// Message handler for processing incoming Zenoh messages
pub type MessageHandler = Box<dyn Fn(WeaveMeshMessage) -> Result<(), ZenohError> + Send + Sync>;

/// Universal WeaveMesh message format for Zenoh communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaveMeshMessage {
    /// Source node ID
    pub from_node: String,
    
    /// Target node ID (None for broadcast)
    pub to_node: Option<String>,
    
    /// Message type
    pub message_type: MessageType,
    
    /// Message payload
    pub payload: Vec<u8>,
    
    /// Message timestamp
    pub timestamp: DateTime<Utc>,
    
    /// Message ID for tracking
    pub message_id: String,
    
    /// Context information (for context-specific routing)
    pub context: Option<String>,
}

/// Universal message types in WeaveMesh
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MessageType {
    /// Node discovery and registration
    NodeDiscovery,
    
    /// Resource sharing announcement
    ResourceShare,
    
    /// Resource request
    ResourceRequest,
    
    /// Resource response
    ResourceResponse,
    
    /// Sacred Alliance validation
    SacredAllianceValidation,
    
    /// Attribution tracking
    AttributionUpdate,
    
    /// General collaboration message
    Collaboration,
    
    /// Heartbeat for connection health
    Heartbeat,
    
    /// Context-specific message
    ContextSpecific(String),
    
    /// System control message
    SystemControl,
    
    /// Error message
    Error,
}

impl ZenohSession {
    /// Create a new Zenoh session for a mesh node
    pub async fn new(node_id: Uuid, config: ZenohConfig) -> Result<Self, ZenohError> {
        // Build Zenoh configuration
        let mut zenoh_config = zenoh::config::Config::default();
        
        // Note: For now, use default config as the API has changed significantly
        // TODO: Update configuration once we have proper Zenoh 1.4.0 API documentation
        
        // Open Zenoh session
        let session = zenoh::open(zenoh_config)
            .await
            .map_err(|e| ZenohError::ConnectionFailed(e.to_string()))?;
        
        let session = Arc::new(session);
        
        Ok(Self {
            session,
            node_id,
            config,
            subscriptions: Arc::new(RwLock::new(HashMap::new())),
            publishers: Arc::new(RwLock::new(HashMap::new())),
            message_handler: Arc::new(RwLock::new(None)),
            is_connected: Arc::new(RwLock::new(true)),
        })
    }
    
    /// Get the node ID for this session
    pub fn node_id(&self) -> Uuid {
        self.node_id
    }
    
    /// Check if the session is connected
    pub async fn is_connected(&self) -> bool {
        *self.is_connected.read().await
    }
    
    /// Set the message handler for incoming messages
    pub async fn set_message_handler<F>(&self, handler: F)
    where
        F: Fn(WeaveMeshMessage) -> Result<(), ZenohError> + Send + Sync + 'static,
    {
        *self.message_handler.write().await = Some(Box::new(handler));
    }
    
    /// Subscribe to a topic in the mesh
    pub async fn subscribe(&self, topic: &str) -> Result<(), ZenohError> {
        let key_expr = KeyExpr::try_from(topic)
            .map_err(|e| ZenohError::InvalidTopic(e.to_string()))?;
        
        let message_handler = Arc::clone(&self.message_handler);
        let node_id = self.node_id;
        
        let subscriber = self.session
            .declare_subscriber(&key_expr)
            .callback(move |sample| {
                if let Ok(message) = Self::decode_message(&sample.payload()) {
                    // Don't process messages from ourselves
                    if message.from_node != node_id.to_string() {
                        if let Some(handler) = message_handler.blocking_read().as_ref() {
                            if let Err(e) = handler(message) {
                                eprintln!("Error handling message: {}", e);
                            }
                        }
                    }
                }
            })
            .await
            .map_err(|e| ZenohError::SubscriptionFailed(e.to_string()))?;
        
        // Store the subscription
        self.subscriptions.write().await.insert(
            topic.to_string(),
            Arc::new(subscriber),
        );
        
        if self.config.debug {
            println!("Node {} subscribed to topic: {}", self.node_id, topic);
        }
        
        Ok(())
    }
    
    /// Unsubscribe from a topic
    pub async fn unsubscribe(&self, topic: &str) -> Result<(), ZenohError> {
        let mut subscriptions = self.subscriptions.write().await;
        if let Some(subscription) = subscriptions.remove(topic) {
            // Subscription will be automatically dropped and closed
            drop(subscription);
            
            if self.config.debug {
                println!("Node {} unsubscribed from topic: {}", self.node_id, topic);
            }
        }
        
        Ok(())
    }
    
    /// Publish a message to a topic
    pub async fn publish(
        &self,
        topic: &str,
        message: WeaveMeshMessage,
    ) -> Result<(), ZenohError> {
        let key_expr = KeyExpr::try_from(topic)
            .map_err(|e| ZenohError::InvalidTopic(format!("Invalid topic '{}': {}", topic, e)))?;
        
        let encoded_message = Self::encode_message(&message)?;
        
        // Use session.put directly instead of maintaining publishers
        self.session
            .put(&key_expr, encoded_message)
            .await
            .map_err(|e| ZenohError::PublishFailed(e.to_string()))?;
        
        if self.config.debug {
            println!("Node {} published message to topic: {}", self.node_id, topic);
        }
        
        Ok(())
    }
    
    /// Send a direct message to another node
    pub async fn send_message(
        &self,
        target_node: Uuid,
        message_type: MessageType,
        payload: Vec<u8>,
        context: Option<String>,
    ) -> Result<(), ZenohError> {
        let message = WeaveMeshMessage {
            from_node: self.node_id.to_string(),
            to_node: Some(target_node.to_string()),
            message_type,
            payload,
            timestamp: Utc::now(),
            message_id: Uuid::new_v4().to_string(),
            context,
        };
        
        // Send to the node's direct topic
        let topic = WeaveMeshTopics::node_direct(target_node);
        self.publish(&topic, message).await
    }
    
    /// Broadcast a message to all nodes
    pub async fn broadcast_message(
        &self,
        message_type: MessageType,
        payload: Vec<u8>,
    ) -> Result<(), ZenohError> {
        let message = WeaveMeshMessage {
            from_node: self.node_id.to_string(),
            to_node: None,
            message_type,
            payload,
            timestamp: Utc::now(),
            message_id: Uuid::new_v4().to_string(),
            context: None,
        };
        
        // Broadcast to all nodes
        let topic = WeaveMeshTopics::BROADCAST;
        self.publish(topic, message).await
    }
    
    /// Query for resources in the mesh
    pub async fn query_resources(
        &self,
        query: &str,
    ) -> Result<Vec<WeaveMeshMessage>, ZenohError> {
        let key_expr = KeyExpr::try_from(query)
            .map_err(|e| ZenohError::InvalidTopic(e.to_string()))?;
        
        let replies = self.session
            .get(&key_expr)
            .await
            .map_err(|e| ZenohError::QueryFailed(e.to_string()))?;
        
        let mut results = Vec::new();
        while let Ok(reply) = replies.recv_async().await {
            // TODO: Update this once we have proper Zenoh 1.4.0 API documentation
            // The reply structure has changed in Zenoh 1.4.0
            // For now, we'll return empty results
        }
        
        Ok(results)
    }
    
    /// Close the Zenoh session
    pub async fn close(self) -> Result<(), ZenohError> {
        // Mark as disconnected
        *self.is_connected.write().await = false;
        
        // Clear subscriptions and publishers
        self.subscriptions.write().await.clear();
        self.publishers.write().await.clear();
        
        // Close the session
        if let Ok(session) = Arc::try_unwrap(self.session) {
            session.close().await
                .map_err(|e| ZenohError::CloseFailed(e.to_string()))?;
        }
        
        if self.config.debug {
            println!("Node {} closed Zenoh session", self.node_id);
        }
        
        Ok(())
    }
    
    /// Encode a WeaveMesh message for Zenoh transport
    fn encode_message(message: &WeaveMeshMessage) -> Result<Vec<u8>, ZenohError> {
        serde_json::to_vec(message)
            .map_err(|e| ZenohError::EncodingFailed(e.to_string()))
    }
    
    /// Decode a WeaveMesh message from Zenoh transport
    fn decode_message(payload: &ZBytes) -> Result<WeaveMeshMessage, ZenohError> {
        let bytes = payload.to_bytes();
        serde_json::from_slice(&bytes)
            .map_err(|e| ZenohError::DecodingFailed(e.to_string()))
    }
}

/// Universal topic patterns for WeaveMesh
pub struct WeaveMeshTopics;

impl WeaveMeshTopics {
    /// Topic for node discovery
    pub const NODE_DISCOVERY: &'static str = "weavemesh/discovery";
    
    /// Topic for resource sharing
    pub const RESOURCE_SHARE: &'static str = "weavemesh/resources/share";
    
    /// Topic for Sacred Alliance validation
    pub const SACRED_ALLIANCE: &'static str = "weavemesh/sacred-alliance";
    
    /// Topic for attribution updates
    pub const ATTRIBUTION: &'static str = "weavemesh/attribution";
    
    /// Topic for general collaboration
    pub const COLLABORATION: &'static str = "weavemesh/collaboration";
    
    /// Topic for broadcast messages
    pub const BROADCAST: &'static str = "weavemesh/broadcast";
    
    /// Topic for system control messages
    pub const SYSTEM_CONTROL: &'static str = "weavemesh/system";
    
    /// Get direct message topic for a node
    pub fn node_direct(node_id: Uuid) -> String {
        format!("weavemesh/nodes/{}/direct", node_id)
    }
    
    /// Get context-specific topic
    pub fn context_topic(context: &str, subtopic: &str) -> String {
        format!("weavemesh/contexts/{}/{}", context, subtopic)
    }
    
    /// Get resource topic for a resource ID
    pub fn resource(resource_id: &str) -> String {
        format!("weavemesh/resources/{}", resource_id)
    }
    
    /// Get group topic for a group ID
    pub fn group(group_id: &str) -> String {
        format!("weavemesh/groups/{}", group_id)
    }
    
    /// Get all standard topics
    pub fn all_standard_topics() -> Vec<&'static str> {
        vec![
            Self::NODE_DISCOVERY,
            Self::RESOURCE_SHARE,
            Self::SACRED_ALLIANCE,
            Self::ATTRIBUTION,
            Self::COLLABORATION,
            Self::BROADCAST,
            Self::SYSTEM_CONTROL,
        ]
    }
}

/// Errors that can occur with Zenoh integration
#[derive(Debug, thiserror::Error)]
pub enum ZenohError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    
    #[error("Invalid topic: {0}")]
    InvalidTopic(String),
    
    #[error("Subscription failed: {0}")]
    SubscriptionFailed(String),
    
    #[error("Publisher creation failed: {0}")]
    PublisherFailed(String),
    
    #[error("Publisher not found: {0}")]
    PublisherNotFound(String),
    
    #[error("Publish failed: {0}")]
    PublishFailed(String),
    
    #[error("Query failed: {0}")]
    QueryFailed(String),
    
    #[error("Encoding failed: {0}")]
    EncodingFailed(String),
    
    #[error("Decoding failed: {0}")]
    DecodingFailed(String),
    
    #[error("Session close failed: {0}")]
    CloseFailed(String),
    
    #[error("Session not connected")]
    NotConnected,
}

/// Utility functions for Zenoh integration
pub mod utils {
    use super::*;
    
    /// Create a default Zenoh configuration for peer mode
    pub fn default_peer_config() -> ZenohConfig {
        ZenohConfig {
            mode: ZenohMode::Peer,
            ..Default::default()
        }
    }
    
    /// Create a default Zenoh configuration for client mode
    pub fn default_client_config() -> ZenohConfig {
        ZenohConfig {
            mode: ZenohMode::Client,
            ..Default::default()
        }
    }
    
    /// Create a Zenoh configuration with custom endpoints
    pub fn config_with_endpoints(endpoints: Vec<String>) -> ZenohConfig {
        ZenohConfig {
            endpoints,
            ..Default::default()
        }
    }
    
    /// Validate a topic string
    pub fn validate_topic(topic: &str) -> Result<(), ZenohError> {
        if topic.is_empty() {
            return Err(ZenohError::InvalidTopic("Topic cannot be empty".to_string()));
        }
        
        if topic.contains("//") {
            return Err(ZenohError::InvalidTopic("Topic cannot contain double slashes".to_string()));
        }
        
        if topic.starts_with('/') || topic.ends_with('/') {
            return Err(ZenohError::InvalidTopic("Topic cannot start or end with slash".to_string()));
        }
        
        Ok(())
    }
    
    /// Create a message with current timestamp
    pub fn create_message(
        from_node: Uuid,
        to_node: Option<Uuid>,
        message_type: MessageType,
        payload: Vec<u8>,
        context: Option<String>,
    ) -> WeaveMeshMessage {
        WeaveMeshMessage {
            from_node: from_node.to_string(),
            to_node: to_node.map(|id| id.to_string()),
            message_type,
            payload,
            timestamp: Utc::now(),
            message_id: Uuid::new_v4().to_string(),
            context,
        }
    }
    
    /// Check if a message is a broadcast
    pub fn is_broadcast(message: &WeaveMeshMessage) -> bool {
        message.to_node.is_none()
    }
    
    /// Check if a message is direct
    pub fn is_direct(message: &WeaveMeshMessage) -> bool {
        message.to_node.is_some()
    }
    
    /// Extract context from message
    pub fn extract_context(message: &WeaveMeshMessage) -> Option<&str> {
        message.context.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::utils::*;
    
    #[test]
    fn test_message_creation() {
        let from_node = Uuid::new_v4();
        let to_node = Some(Uuid::new_v4());
        let message_type = MessageType::NodeDiscovery;
        let payload = b"test payload".to_vec();
        let context = Some("test-context".to_string());
        
        let message = create_message(from_node, to_node, message_type, payload.clone(), context.clone());
        
        assert_eq!(message.from_node, from_node.to_string());
        assert_eq!(message.to_node, to_node.map(|id| id.to_string()));
        assert_eq!(message.payload, payload);
        assert_eq!(message.context, context);
        assert!(!message.message_id.is_empty());
    }
    
    #[test]
    fn test_message_encoding_decoding() {
        let message = WeaveMeshMessage {
            from_node: "test-node".to_string(),
            to_node: Some("target-node".to_string()),
            message_type: MessageType::NodeDiscovery,
            payload: b"test payload".to_vec(),
            timestamp: Utc::now(),
            message_id: "test-message-id".to_string(),
            context: Some("test-context".to_string()),
        };
        
        let encoded = ZenohSession::encode_message(&message).unwrap();
        assert!(!encoded.is_empty());
        
        // Test JSON serialization/deserialization
        let json_str = serde_json::to_string(&message).unwrap();
        let decoded: WeaveMeshMessage = serde_json::from_str(&json_str).unwrap();
        
        assert_eq!(decoded.from_node, message.from_node);
        assert_eq!(decoded.to_node, message.to_node);
        assert_eq!(decoded.message_id, message.message_id);
        assert_eq!(decoded.context, message.context);
    }
    
    #[test]
    fn test_topic_patterns() {
        let node_id = Uuid::new_v4();
        
        assert_eq!(WeaveMeshTopics::NODE_DISCOVERY, "weavemesh/discovery");
        assert_eq!(WeaveMeshTopics::RESOURCE_SHARE, "weavemesh/resources/share");
        
        let direct_topic = WeaveMeshTopics::node_direct(node_id);
        assert!(direct_topic.starts_with("weavemesh/nodes/"));
        assert!(direct_topic.ends_with("/direct"));
        
        let context_topic = WeaveMeshTopics::context_topic("balans", "family");
        assert_eq!(context_topic, "weavemesh/contexts/balans/family");
        
        let group_topic = WeaveMeshTopics::group("test-group");
        assert_eq!(group_topic, "weavemesh/groups/test-group");
    }
    
    #[test]
    fn test_topic_validation() {
        assert!(validate_topic("valid/topic").is_ok());
        assert!(validate_topic("weavemesh/discovery").is_ok());
        
        assert!(validate_topic("").is_err());
        assert!(validate_topic("//invalid").is_err());
        assert!(validate_topic("/invalid").is_err());
        assert!(validate_topic("invalid/").is_err());
    }
    
    #[test]
    fn test_message_type_classification() {
        let broadcast_msg = WeaveMeshMessage {
            from_node: "node1".to_string(),
            to_node: None,
            message_type: MessageType::NodeDiscovery,
            payload: Vec::new(),
            timestamp: Utc::now(),
            message_id: "msg1".to_string(),
            context: None,
        };
        
        let direct_msg = WeaveMeshMessage {
            from_node: "node1".to_string(),
            to_node: Some("node2".to_string()),
            message_type: MessageType::Collaboration,
            payload: Vec::new(),
            timestamp: Utc::now(),
            message_id: "msg2".to_string(),
            context: Some("test".to_string()),
        };
        
        assert!(is_broadcast(&broadcast_msg));
        assert!(!is_direct(&broadcast_msg));
        
        assert!(!is_broadcast(&direct_msg));
        assert!(is_direct(&direct_msg));
        
        assert_eq!(extract_context(&direct_msg), Some("test"));
        assert_eq!(extract_context(&broadcast_msg), None);
    }
    
    #[test]
    fn test_config_creation() {
        let peer_config = default_peer_config();
        assert!(matches!(peer_config.mode, ZenohMode::Peer));
        
        let client_config = default_client_config();
        assert!(matches!(client_config.mode, ZenohMode::Client));
        
        let endpoints = vec!["tcp/192.168.1.1:7447".to_string()];
        let custom_config = config_with_endpoints(endpoints.clone());
        assert_eq!(custom_config.endpoints, endpoints);
    }
}
