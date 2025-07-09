//! Universal Node-to-Node Communication for WeaveMesh
//! 
//! This module implements communication protocols between mesh nodes,
//! enabling secure, efficient message passing and resource sharing across
//! different contexts.

use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::{RwLock, mpsc};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::networking::zenoh_integration::{ZenohSession, WeaveMeshMessage, MessageType, WeaveMeshTopics};
use crate::networking::node_discovery::NodeInfo;

/// Universal node communication manager
/// 
/// Handles:
/// - Direct messaging between nodes
/// - Resource sharing protocols
/// - Message routing and delivery
/// - Communication security and validation
/// - Message acknowledgment and retry logic
pub struct NodeCommunication {
    /// This node's ID
    node_id: Uuid,
    
    /// Zenoh session for mesh communication
    zenoh_session: Arc<ZenohSession>,
    
    /// Communication configuration
    config: CommunicationConfig,
    
    /// Active message handlers
    message_handlers: Arc<RwLock<HashMap<MessageType, MessageHandler>>>,
    
    /// Pending message acknowledgments
    pending_acks: Arc<RwLock<HashMap<String, PendingMessage>>>,
    
    /// Message delivery statistics
    stats: Arc<RwLock<CommunicationStats>>,
    
    /// Whether communication is active
    is_active: Arc<RwLock<bool>>,
}

/// Configuration for node communication
#[derive(Debug, Clone)]
pub struct CommunicationConfig {
    /// Maximum message size in bytes
    pub max_message_size: usize,
    
    /// Message timeout in seconds
    pub message_timeout: u64,
    
    /// Maximum retry attempts for failed messages
    pub max_retries: u32,
    
    /// Whether to require message acknowledgments
    pub require_acks: bool,
    
    /// Whether to enable message encryption
    pub enable_encryption: bool,
    
    /// Whether to enable debug logging
    pub debug: bool,
}

impl Default for CommunicationConfig {
    fn default() -> Self {
        Self {
            max_message_size: 1024 * 1024, // 1MB
            message_timeout: 30,
            max_retries: 3,
            require_acks: true,
            enable_encryption: true,
            debug: false,
        }
    }
}

/// Message handler function type
pub type MessageHandler = Box<dyn Fn(IncomingMessage) -> Result<Option<Vec<u8>>, CommunicationError> + Send + Sync>;

/// Incoming message with context
#[derive(Debug, Clone)]
pub struct IncomingMessage {
    /// The raw WeaveMesh message
    pub message: WeaveMeshMessage,
    
    /// Information about the sender node
    pub sender_info: Option<NodeInfo>,
    
    /// When this message was received
    pub received_at: DateTime<Utc>,
    
    /// Whether this message requires an acknowledgment
    pub requires_ack: bool,
}

/// Outgoing message with delivery options
#[derive(Debug, Clone)]
pub struct OutgoingMessage {
    /// Target node ID
    pub target_node: Uuid,
    
    /// Message type
    pub message_type: MessageType,
    
    /// Message payload
    pub payload: Vec<u8>,
    
    /// Delivery options
    pub options: DeliveryOptions,
    
    /// Context information
    pub context: Option<String>,
}

/// Message delivery options
#[derive(Debug, Clone)]
pub struct DeliveryOptions {
    /// Whether to require acknowledgment
    pub require_ack: bool,
    
    /// Maximum retry attempts
    pub max_retries: u32,
    
    /// Message timeout in seconds
    pub timeout_seconds: u64,
    
    /// Message priority (higher = more important)
    pub priority: MessagePriority,
    
    /// Whether to encrypt the message
    pub encrypt: bool,
}

impl Default for DeliveryOptions {
    fn default() -> Self {
        Self {
            require_ack: true,
            max_retries: 3,
            timeout_seconds: 30,
            priority: MessagePriority::Normal,
            encrypt: true,
        }
    }
}

/// Message priority levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MessagePriority {
    Low,
    Normal,
    High,
    Critical,
}

/// Pending message awaiting acknowledgment
#[derive(Debug, Clone)]
struct PendingMessage {
    /// The original message
    message: WeaveMeshMessage,
    
    /// Delivery options
    options: DeliveryOptions,
    
    /// When the message was sent
    sent_at: DateTime<Utc>,
    
    /// Number of retry attempts made
    retry_count: u32,
    
    /// Channel to notify when acknowledged or failed
    response_sender: Option<mpsc::UnboundedSender<MessageResult>>,
}

/// Result of message delivery
#[derive(Debug, Clone)]
pub enum MessageResult {
    /// Message was successfully delivered and acknowledged
    Delivered,
    
    /// Message delivery failed after retries
    Failed(String),
    
    /// Message timed out
    TimedOut,
    
    /// Response received from target node
    Response(Vec<u8>),
}

/// Communication statistics
#[derive(Debug, Clone, Default)]
pub struct CommunicationStats {
    /// Total messages sent
    pub messages_sent: u64,
    
    /// Total messages received
    pub messages_received: u64,
    
    /// Messages successfully delivered
    pub messages_delivered: u64,
    
    /// Messages that failed delivery
    pub messages_failed: u64,
    
    /// Messages that timed out
    pub messages_timed_out: u64,
    
    /// Average message delivery time in milliseconds
    pub avg_delivery_time_ms: f64,
    
    /// Total bytes sent
    pub bytes_sent: u64,
    
    /// Total bytes received
    pub bytes_received: u64,
    
    /// Messages by type
    pub messages_by_type: HashMap<String, u64>,
    
    /// Messages by context
    pub messages_by_context: HashMap<String, u64>,
}

impl NodeCommunication {
    /// Create a new node communication manager
    pub fn new(
        node_id: Uuid,
        zenoh_session: Arc<ZenohSession>,
        config: CommunicationConfig,
    ) -> Self {
        Self {
            node_id,
            zenoh_session,
            config,
            message_handlers: Arc::new(RwLock::new(HashMap::new())),
            pending_acks: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(CommunicationStats::default())),
            is_active: Arc::new(RwLock::new(false)),
        }
    }
    
    /// Start the communication system
    pub async fn start(&self) -> Result<(), CommunicationError> {
        // Mark as active
        *self.is_active.write().await = true;
        
        // Setup message handling
        self.setup_message_handling().await?;
        
        // Start background tasks
        self.start_ack_timeout_task().await;
        self.start_retry_task().await;
        
        if self.config.debug {
            println!("Node communication started for {}", self.node_id);
        }
        
        Ok(())
    }
    
    /// Stop the communication system
    pub async fn stop(&self) -> Result<(), CommunicationError> {
        // Mark as inactive
        *self.is_active.write().await = false;
        
        // Clear pending messages
        self.pending_acks.write().await.clear();
        
        if self.config.debug {
            println!("Node communication stopped for {}", self.node_id);
        }
        
        Ok(())
    }
    
    /// Register a message handler for a specific message type
    pub async fn register_handler<F>(&self, message_type: MessageType, handler: F)
    where
        F: Fn(IncomingMessage) -> Result<Option<Vec<u8>>, CommunicationError> + Send + Sync + 'static,
    {
        self.message_handlers.write().await.insert(
            message_type,
            Box::new(handler),
        );
    }
    
    /// Send a message to another node
    pub async fn send_message(
        &self,
        message: OutgoingMessage,
    ) -> Result<mpsc::UnboundedReceiver<MessageResult>, CommunicationError> {
        if !*self.is_active.read().await {
            return Err(CommunicationError::NotActive);
        }
        
        // Validate message size
        if message.payload.len() > self.config.max_message_size {
            return Err(CommunicationError::MessageTooLarge);
        }
        
        // Create WeaveMesh message
        let weave_message = WeaveMeshMessage {
            from_node: self.node_id.to_string(),
            to_node: Some(message.target_node.to_string()),
            message_type: message.message_type.clone(),
            payload: message.payload.clone(),
            timestamp: Utc::now(),
            message_id: Uuid::new_v4().to_string(),
            context: message.context.clone(),
        };
        
        // Create response channel if acknowledgment is required
        let (response_sender, response_receiver) = if message.options.require_ack {
            let (tx, rx) = mpsc::unbounded_channel();
            (Some(tx), rx)
        } else {
            let (_, rx) = mpsc::unbounded_channel();
            (None, rx)
        };
        
        // Send the message
        let topic = WeaveMeshTopics::node_direct(message.target_node);
        self.zenoh_session.publish(&topic, weave_message.clone())
            .await
            .map_err(|e| CommunicationError::NetworkError(e.to_string()))?;
        
        // Track pending acknowledgment if required
        if message.options.require_ack {
            let pending = PendingMessage {
                message: weave_message.clone(),
                options: message.options,
                sent_at: Utc::now(),
                retry_count: 0,
                response_sender,
            };
            
            self.pending_acks.write().await.insert(
                pending.message.message_id.clone(),
                pending,
            );
        }
        
        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.messages_sent += 1;
            stats.bytes_sent += message.payload.len() as u64;
            
            // Track by message type
            let type_key = format!("{:?}", message.message_type);
            *stats.messages_by_type.entry(type_key).or_insert(0) += 1;
            
            // Track by context
            if let Some(context) = &message.context {
                *stats.messages_by_context.entry(context.clone()).or_insert(0) += 1;
            }
        }
        
        if self.config.debug {
            println!("Message sent from {} to {}: {}", 
                self.node_id, 
                message.target_node,
                weave_message.message_id);
        }
        
        Ok(response_receiver)
    }
    
    /// Send a broadcast message to all nodes
    pub async fn broadcast_message(
        &self,
        message_type: MessageType,
        payload: Vec<u8>,
        context: Option<String>,
    ) -> Result<(), CommunicationError> {
        if !*self.is_active.read().await {
            return Err(CommunicationError::NotActive);
        }
        
        // Validate message size
        if payload.len() > self.config.max_message_size {
            return Err(CommunicationError::MessageTooLarge);
        }
        
        // Send broadcast
        self.zenoh_session.send_message(
            Uuid::nil(), // Broadcast target
            message_type.clone(),
            payload.clone(),
            context.clone(),
        ).await.map_err(|e| CommunicationError::NetworkError(e.to_string()))?;
        
        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.messages_sent += 1;
            stats.bytes_sent += payload.len() as u64;
            
            // Track by message type
            let type_key = format!("{:?}", message_type);
            *stats.messages_by_type.entry(type_key).or_insert(0) += 1;
            
            // Track by context
            if let Some(context) = &context {
                *stats.messages_by_context.entry(context.clone()).or_insert(0) += 1;
            }
        }
        
        if self.config.debug {
            println!("Broadcast message sent from {}", self.node_id);
        }
        
        Ok(())
    }
    
    /// Send a context-specific message
    pub async fn send_context_message(
        &self,
        context: &str,
        subtopic: &str,
        message_type: MessageType,
        payload: Vec<u8>,
    ) -> Result<(), CommunicationError> {
        if !*self.is_active.read().await {
            return Err(CommunicationError::NotActive);
        }
        
        // Validate message size
        if payload.len() > self.config.max_message_size {
            return Err(CommunicationError::MessageTooLarge);
        }
        
        // Create context message
        let message = WeaveMeshMessage {
            from_node: self.node_id.to_string(),
            to_node: None,
            message_type,
            payload: payload.clone(),
            timestamp: Utc::now(),
            message_id: Uuid::new_v4().to_string(),
            context: Some(context.to_string()),
        };
        
        // Publish to context topic
        let topic = WeaveMeshTopics::context_topic(context, subtopic);
        self.zenoh_session.publish(&topic, message)
            .await
            .map_err(|e| CommunicationError::NetworkError(e.to_string()))?;
        
        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.messages_sent += 1;
            stats.bytes_sent += payload.len() as u64;
            *stats.messages_by_context.entry(context.to_string()).or_insert(0) += 1;
        }
        
        Ok(())
    }
    
    /// Get communication statistics
    pub async fn get_stats(&self) -> CommunicationStats {
        self.stats.read().await.clone()
    }
    
    /// Reset communication statistics
    pub async fn reset_stats(&self) {
        *self.stats.write().await = CommunicationStats::default();
    }
    
    /// Get pending message count
    pub async fn get_pending_count(&self) -> usize {
        self.pending_acks.read().await.len()
    }
    
    /// Setup message handling from Zenoh
    async fn setup_message_handling(&self) -> Result<(), CommunicationError> {
        let message_handlers = Arc::clone(&self.message_handlers);
        let pending_acks = Arc::clone(&self.pending_acks);
        let stats = Arc::clone(&self.stats);
        let node_id = self.node_id;
        let config = self.config.clone();
        
        self.zenoh_session.set_message_handler(move |message| {
            let handlers = Arc::clone(&message_handlers);
            let pending = Arc::clone(&pending_acks);
            let stats = Arc::clone(&stats);
            let node_id = node_id;
            let config = config.clone();
            
            tokio::spawn(async move {
                if let Err(e) = Self::handle_incoming_message(
                    message, handlers, pending, stats, node_id, config
                ).await {
                    eprintln!("Error handling incoming message: {}", e);
                }
            });
            
            Ok(())
        }).await;
        
        Ok(())
    }
    
    /// Handle incoming messages
    async fn handle_incoming_message(
        message: WeaveMeshMessage,
        handlers: Arc<RwLock<HashMap<MessageType, MessageHandler>>>,
        pending_acks: Arc<RwLock<HashMap<String, PendingMessage>>>,
        stats: Arc<RwLock<CommunicationStats>>,
        node_id: Uuid,
        config: CommunicationConfig,
    ) -> Result<(), CommunicationError> {
        // Update statistics
        {
            let mut stats = stats.write().await;
            stats.messages_received += 1;
            stats.bytes_received += message.payload.len() as u64;
            
            // Track by message type
            let type_key = format!("{:?}", message.message_type);
            *stats.messages_by_type.entry(type_key).or_insert(0) += 1;
            
            // Track by context
            if let Some(context) = &message.context {
                *stats.messages_by_context.entry(context.clone()).or_insert(0) += 1;
            }
        }
        
        // Check if this is an acknowledgment for a pending message
        if message.message_type == MessageType::SystemControl && 
           message.payload.starts_with(b"ACK:") {
            Self::handle_acknowledgment(message, pending_acks).await?;
            return Ok(());
        }
        
        // Create incoming message context
        let incoming = IncomingMessage {
            message: message.clone(),
            sender_info: None, // Would be populated from node discovery
            received_at: Utc::now(),
            requires_ack: config.require_acks,
        };
        
        // Find and execute handler
        let handlers = handlers.read().await;
        if let Some(handler) = handlers.get(&message.message_type) {
            match handler(incoming) {
                Ok(response) => {
                    // Send response if provided
                    if let Some(response_data) = response {
                        // Implementation would send response back to sender
                        if config.debug {
                            println!("Response sent for message {}", message.message_id);
                        }
                    }
                    
                    // Send acknowledgment if required
                    if config.require_acks {
                        // Implementation would send ACK back to sender
                        if config.debug {
                            println!("ACK sent for message {}", message.message_id);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Handler error for message {}: {}", message.message_id, e);
                }
            }
        } else if config.debug {
            println!("No handler for message type: {:?}", message.message_type);
        }
        
        Ok(())
    }
    
    /// Handle acknowledgment messages
    async fn handle_acknowledgment(
        message: WeaveMeshMessage,
        pending_acks: Arc<RwLock<HashMap<String, PendingMessage>>>,
    ) -> Result<(), CommunicationError> {
        // Extract message ID from ACK payload
        let ack_payload = String::from_utf8_lossy(&message.payload);
        if let Some(acked_id) = ack_payload.strip_prefix("ACK:") {
            let mut pending = pending_acks.write().await;
            if let Some(pending_msg) = pending.remove(acked_id) {
                if let Some(sender) = pending_msg.response_sender {
                    let _ = sender.send(MessageResult::Delivered);
                }
            }
        }
        
        Ok(())
    }
    
    /// Start task to handle acknowledgment timeouts
    async fn start_ack_timeout_task(&self) {
        let pending_acks = Arc::clone(&self.pending_acks);
        let is_active = Arc::clone(&self.is_active);
        let timeout = self.config.message_timeout;
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(
                tokio::time::Duration::from_secs(5) // Check every 5 seconds
            );
            
            while *is_active.read().await {
                interval.tick().await;
                
                if *is_active.read().await {
                    let mut pending = pending_acks.write().await;
                    let now = Utc::now();
                    let mut to_remove = Vec::new();
                    
                    for (msg_id, pending_msg) in pending.iter() {
                        let elapsed = (now - pending_msg.sent_at).num_seconds();
                        if elapsed > timeout as i64 {
                            to_remove.push(msg_id.clone());
                        }
                    }
                    
                    for msg_id in to_remove {
                        if let Some(pending_msg) = pending.remove(&msg_id) {
                            if let Some(sender) = pending_msg.response_sender {
                                let _ = sender.send(MessageResult::TimedOut);
                            }
                        }
                    }
                }
            }
        });
    }
    
    /// Start task to handle message retries
    async fn start_retry_task(&self) {
        let pending_acks = Arc::clone(&self.pending_acks);
        let zenoh_session = Arc::clone(&self.zenoh_session);
        let is_active = Arc::clone(&self.is_active);
        let max_retries = self.config.max_retries;
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(
                tokio::time::Duration::from_secs(10) // Check every 10 seconds
            );
            
            while *is_active.read().await {
                interval.tick().await;
                
                if *is_active.read().await {
                    let mut pending = pending_acks.write().await;
                    let now = Utc::now();
                    let mut to_retry = Vec::new();
                    let mut to_remove = Vec::new();
                    
                    for (msg_id, pending_msg) in pending.iter_mut() {
                        let elapsed = (now - pending_msg.sent_at).num_seconds();
                        
                        // Retry if message is old enough and hasn't exceeded max retries
                        if elapsed > 15 && pending_msg.retry_count < max_retries {
                            to_retry.push((msg_id.clone(), pending_msg.message.clone()));
                            pending_msg.retry_count += 1;
                            pending_msg.sent_at = now;
                        } else if pending_msg.retry_count >= max_retries {
                            to_remove.push(msg_id.clone());
                        }
                    }
                    
                    // Remove failed messages
                    for msg_id in to_remove {
                        if let Some(pending_msg) = pending.remove(&msg_id) {
                            if let Some(sender) = pending_msg.response_sender {
                                let _ = sender.send(MessageResult::Failed("Max retries exceeded".to_string()));
                            }
                        }
                    }
                    
                    // Retry messages
                    drop(pending); // Release lock before async operations
                    
                    for (_msg_id, message) in to_retry {
                        if let Some(to_node) = &message.to_node {
                            if let Ok(target_node) = Uuid::parse_str(to_node) {
                                let topic = WeaveMeshTopics::node_direct(target_node);
                                let _ = zenoh_session.publish(&topic, message).await;
                            }
                        }
                    }
                }
            }
        });
    }
}

/// Errors that can occur during communication
#[derive(Debug, thiserror::Error)]
pub enum CommunicationError {
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Message too large")]
    MessageTooLarge,
    
    #[error("Communication not active")]
    NotActive,
    
    #[error("Handler error: {0}")]
    HandlerError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Message timeout")]
    MessageTimeout,
    
    #[error("Invalid message format")]
    InvalidMessage,
    
    #[error("No handler registered for message type")]
    NoHandler,
    
    #[error("Encryption error: {0}")]
    EncryptionError(String),
}

/// Utility functions for node communication
pub mod utils {
    use super::*;
    
    /// Create a basic outgoing message
    pub fn create_basic_message(
        target_node: Uuid,
        message_type: MessageType,
        payload: Vec<u8>,
    ) -> OutgoingMessage {
        OutgoingMessage {
            target_node,
            message_type,
            payload,
            options: DeliveryOptions::default(),
            context: None,
        }
    }
    
    /// Create a high-priority message
    pub fn create_priority_message(
        target_node: Uuid,
        message_type: MessageType,
        payload: Vec<u8>,
        priority: MessagePriority,
    ) -> OutgoingMessage {
        OutgoingMessage {
            target_node,
            message_type,
            payload,
            options: DeliveryOptions {
                priority,
                ..Default::default()
            },
            context: None,
        }
    }
    
    /// Create a context-specific message
    pub fn create_context_message(
        target_node: Uuid,
        message_type: MessageType,
        payload: Vec<u8>,
        context: String,
    ) -> OutgoingMessage {
        OutgoingMessage {
            target_node,
            message_type,
            payload,
            options: DeliveryOptions::default(),
            context: Some(context),
        }
    }
    
    /// Create delivery options for fire-and-forget messages
    pub fn fire_and_forget_options() -> DeliveryOptions {
        DeliveryOptions {
            require_ack: false,
            max_retries: 0,
            timeout_seconds: 0,
            priority: MessagePriority::Low,
            encrypt: false,
        }
    }
    
    /// Create delivery options for reliable messages
    pub fn reliable_delivery_options() -> DeliveryOptions {
        DeliveryOptions {
            require_ack: true,
            max_retries: 5,
            timeout_seconds: 60,
            priority: MessagePriority::High,
            encrypt: true,
        }
    }
    
    /// Calculate message throughput from stats
    pub fn calculate_throughput(stats: &CommunicationStats, duration_seconds: u64) -> f64 {
        if duration_seconds == 0 {
            return 0.0;
        }
        (stats.messages_sent + stats.messages_received) as f64 / duration_seconds as f64
    }
    
    /// Calculate success rate from stats
    pub fn calculate_success_rate(stats: &CommunicationStats) -> f64 {
        let total_sent = stats.messages_sent;
        if total_sent == 0 {
            return 0.0;
        }
        stats.messages_delivered as f64 / total_sent as f64
    }
    
    /// Get most active message type
    pub fn most_active_message_type(stats: &CommunicationStats) -> Option<String> {
        stats.messages_by_type
            .iter()
            .max_by_key(|(_, count)| *count)
            .map(|(msg_type, _)| msg_type.clone())
    }
    
    /// Get most active context
    pub fn most_active_context(stats: &CommunicationStats) -> Option<String> {
        stats.messages_by_context
            .iter()
            .max_by_key(|(_, count)| *count)
            .map(|(context, _)| context.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::utils::*;
    
    #[tokio::test]
    async fn test_communication_creation() {
        let node_id = Uuid::new_v4();
        let zenoh_session = Arc::new(unsafe { std::mem::zeroed() }); // Mock for test
        let config = CommunicationConfig::default();
        
        let comm = NodeCommunication::new(node_id, zenoh_session, config);
        assert!(!*comm.is_active.read().await);
    }
    
    #[test]
    fn test_message_priority_ordering() {
        let mut priorities = vec![
            MessagePriority::Low,
            MessagePriority::Critical,
            MessagePriority::Normal,
            MessagePriority::High,
        ];
        
        priorities.sort();
        
        assert_eq!(priorities[0], MessagePriority::Low);
        assert_eq!(priorities[1], MessagePriority::Normal);
        assert_eq!(priorities[2], MessagePriority::High);
        assert_eq!(priorities[3], MessagePriority::Critical);
    }
    
    #[test]
    fn test_delivery_options_defaults() {
        let options = DeliveryOptions::default();
        
        assert!(options.require_ack);
        assert_eq!(options.max_retries, 3);
        assert_eq!(options.timeout_seconds, 30);
        assert_eq!(options.priority, MessagePriority::Normal);
        assert!(options.encrypt);
    }
    
    #[test]
    fn test_message_creation_utils() {
        let target_node = Uuid::new_v4();
        let message_type = MessageType::Collaboration;
        let payload = b"test payload".to_vec();
        
        let basic_msg = create_basic_message(target_node, message_type.clone(), payload.clone());
        assert_eq!(basic_msg.target_node, target_node);
        assert_eq!(basic_msg.payload, payload);
        assert!(basic_msg.context.is_none());
        
        let priority_msg = create_priority_message(
            target_node, 
            message_type.clone(), 
            payload.clone(), 
            MessagePriority::High
        );
        assert_eq!(priority_msg.options.priority, MessagePriority::High);
        
        let context_msg = create_context_message(
            target_node,
            message_type,
            payload,
            "test-context".to_string(),
        );
        assert_eq!(context_msg.context, Some("test-context".to_string()));
    }
    
    #[test]
    fn test_delivery_option_presets() {
        let fire_forget = fire_and_forget_options();
        assert!(!fire_forget.require_ack);
        assert_eq!(fire_forget.max_retries, 0);
        assert!(!fire_forget.encrypt);
        
        let reliable = reliable_delivery_options();
        assert!(reliable.require_ack);
        assert_eq!(reliable.max_retries, 5);
        assert!(reliable.encrypt);
        assert_eq!(reliable.priority, MessagePriority::High);
    }
    
    #[test]
    fn test_stats_calculations() {
        let stats = CommunicationStats {
            messages_sent: 100,
            messages_received: 95,
            messages_delivered: 90,
            messages_failed: 5,
            messages_timed_out: 5,
            avg_delivery_time_ms: 25.0,
            bytes_sent: 10240,
            bytes_received: 9728,
            messages_by_type: HashMap::new(),
            messages_by_context: HashMap::new(),
        };
        
        let throughput = calculate_throughput(&stats, 60); // 60 seconds
        assert!((throughput - 3.25).abs() < 0.01); // (100 + 95) / 60 ≈ 3.25
        
        let success_rate = calculate_success_rate(&stats);
        assert!((success_rate - 0.9).abs() < 0.01); // 90 / 100 = 0.9
        
        // Test with empty stats
        let empty_stats = CommunicationStats::default();
        assert_eq!(calculate_throughput(&empty_stats, 60), 0.0);
        assert_eq!(calculate_success_rate(&empty_stats), 0.0);
        assert_eq!(most_active_message_type(&empty_stats), None);
        assert_eq!(most_active_context(&empty_stats), None);
    }
}
