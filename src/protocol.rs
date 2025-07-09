//! Core WeaveMesh protocol implementation on Zenoh
//!
//! This module provides the foundational communication layer for
//! universal mesh networking with basic Sacred Alliance interface.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};
use uuid::Uuid;
use zenoh::Config;

/// Core WeaveMesh protocol client
pub struct WeaveProtocol {
    /// Zenoh session for communication
    session: Arc<zenoh::Session>,
    /// Node identifier in the mesh
    node_id: Uuid,
    /// Active subscriptions
    subscriptions: Arc<RwLock<HashMap<String, String>>>,
    /// Protocol configuration
    config: WeaveConfig,
}

/// Configuration for WeaveMesh protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaveConfig {
    /// Zenoh router endpoints
    pub connect_endpoints: Vec<String>,
    /// Listen endpoints for this node
    pub listen_endpoints: Vec<String>,
    /// Node identifier (auto-generated if None)
    pub node_id: Option<Uuid>,
    /// Enable multicast scouting
    pub multicast_scouting: bool,
    /// Default timeout for operations (seconds)
    pub default_timeout: u64,
    /// Maximum message size (bytes)
    pub max_message_size: usize,
}

impl Default for WeaveConfig {
    fn default() -> Self {
        Self {
            connect_endpoints: vec!["tcp/127.0.0.1:7447".to_string()],
            listen_endpoints: vec![],
            node_id: None,
            multicast_scouting: true,
            default_timeout: 30,
            max_message_size: 1024 * 1024, // 1MB
        }
    }
}

/// WeaveMesh resource types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WeaveResource {
    /// Basic message content
    Message(MessageContent),
    /// Node heartbeat for discovery
    Heartbeat(NodeHeartbeat),
    /// Basic ceremonial event
    Ceremony(BasicCeremonyEvent),
    /// Attribution information
    Attribution(BasicAttribution),
    /// Collaboration pattern
    Pattern(CollaborationPattern),
}

/// Basic message content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageContent {
    /// Message identifier
    pub id: Uuid,
    /// Sender identifier
    pub sender: String,
    /// Message text
    pub text: String,
    /// Message timestamp
    pub timestamp: DateTime<Utc>,
    /// Message metadata
    pub metadata: HashMap<String, String>,
}

/// Node heartbeat for mesh discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeHeartbeat {
    /// Node identifier
    pub node_id: Uuid,
    /// Node capabilities
    pub capabilities: Vec<String>,
    /// Current load (0.0 to 1.0)
    pub load: f32,
    /// Heartbeat timestamp
    pub timestamp: DateTime<Utc>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Basic ceremonial event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicCeremonyEvent {
    /// Event identifier
    pub id: Uuid,
    /// Type of ceremony
    pub ceremony_type: String,
    /// Participants
    pub participants: Vec<String>,
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
    /// Ceremony data
    pub data: HashMap<String, String>,
}

/// Basic attribution information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicAttribution {
    /// Attribution identifier
    pub id: Uuid,
    /// Human contributor
    pub human_contributor: Option<String>,
    /// AI contributor
    pub ai_contributor: Option<String>,
    /// Collaboration type
    pub collaboration_type: String,
    /// Confidence score
    pub confidence: f32,
    /// Attribution timestamp
    pub timestamp: DateTime<Utc>,
}

/// Collaboration pattern detected in the mesh
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationPattern {
    /// Pattern identifier
    pub id: Uuid,
    /// Pattern type
    pub pattern_type: String,
    /// Frequency of occurrence
    pub frequency: f32,
    /// Effectiveness score
    pub effectiveness: f32,
    /// Pattern description
    pub description: String,
    /// Detection timestamp
    pub detected_at: DateTime<Utc>,
}

/// WeaveMesh key patterns for Zenoh
pub struct WeaveKeys;

impl WeaveKeys {
    /// Messages: weave/messages/{channel}
    pub fn message(channel: &str) -> String {
        format!("weave/messages/{}", channel)
    }
    
    /// Attribution: weave/attribution/{resource_id}
    pub fn attribution(resource_id: &str) -> String {
        format!("weave/attribution/{}", resource_id)
    }
    
    /// Ceremonies: weave/ceremonies/{ceremony_id}
    pub fn ceremony(ceremony_id: &Uuid) -> String {
        format!("weave/ceremonies/{}", ceremony_id)
    }
    
    /// Patterns: weave/patterns/{pattern_type}
    pub fn pattern(pattern_type: &str) -> String {
        format!("weave/patterns/{}", pattern_type)
    }
    
    /// Heartbeats: weave/heartbeat/{node_id}
    pub fn heartbeat(node_id: &Uuid) -> String {
        format!("weave/heartbeat/{}", node_id)
    }
    
    /// Basic Sacred Alliance channel: weave/sacred-alliance/{channel}
    pub fn sacred_alliance(channel: &str) -> String {
        format!("weave/sacred-alliance/{}", channel)
    }
}

impl WeaveProtocol {
    /// Create a new WeaveMesh protocol instance
    pub async fn new(config: WeaveConfig) -> Result<Self> {
        info!("Initializing WeaveMesh protocol with config: {:?}", config);
        
        // Create Zenoh configuration
        let zenoh_config = Config::default();
        
        // Open Zenoh session
        let session = zenoh::open(zenoh_config)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to open Zenoh session: {}", e))?;
        
        let node_id = config.node_id.unwrap_or_else(Uuid::new_v4);
        
        info!("WeaveMesh protocol initialized with node ID: {}", node_id);
        
        Ok(Self {
            session: Arc::new(session),
            node_id,
            subscriptions: Arc::new(RwLock::new(HashMap::new())),
            config,
        })
    }
    
    /// Get the node ID
    pub fn node_id(&self) -> Uuid {
        self.node_id
    }
    
    /// Publish a resource to the mesh
    pub async fn publish_resource(
        &self,
        key: &str,
        resource: WeaveResource,
    ) -> Result<()> {
        debug!("Publishing resource to key: {}", key);
        
        // Serialize the resource
        let payload = serde_json::to_vec(&resource)?;
        
        // Check message size
        if payload.len() > self.config.max_message_size {
            return Err(anyhow::anyhow!(
                "Message size {} exceeds maximum {}",
                payload.len(),
                self.config.max_message_size
            ));
        }
        
        // Publish to Zenoh
        self.session
            .put(key, payload)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to publish: {}", e))?;
        
        debug!("Successfully published resource to key: {}", key);
        Ok(())
    }
    
    /// Get a resource from the mesh
    pub async fn get_resource(&self, key: &str) -> Result<Option<WeaveResource>> {
        debug!("Getting resource from key: {}", key);
        
        let replies = self.session
            .get(key)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to get: {}", e))?;
        
        // Get the first reply
        while let Ok(reply) = replies.recv_async().await {
            match reply.result() {
                Ok(sample) => {
                    let resource: WeaveResource = serde_json::from_slice(&sample.payload().to_bytes())?;
                    
                    debug!("Successfully retrieved resource from key: {}", key);
                    return Ok(Some(resource));
                }
                Err(e) => {
                    warn!("Error in sample: {}", e);
                }
            }
        }
        
        debug!("No resource found at key: {}", key);
        Ok(None)
    }
    
    /// Subscribe to resources matching a key expression
    pub async fn subscribe<F>(&self, key_expr: &str, callback: F) -> Result<()>
    where
        F: Fn(WeaveResource) + Send + Sync + 'static,
    {
        info!("Subscribing to key expression: {}", key_expr);
        
        let subscriber = self.session
            .declare_subscriber(key_expr)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to subscribe: {}", e))?;
        
        // Store subscription for cleanup
        let mut subscriptions = self.subscriptions.write().await;
        subscriptions.insert(key_expr.to_string(), "subscriber".to_string());
        
        // Handle incoming samples in a separate task
        let callback = Arc::new(callback);
        tokio::spawn(async move {
            while let Ok(sample) = subscriber.recv_async().await {
                match serde_json::from_slice::<WeaveResource>(&sample.payload().to_bytes()) {
                    Ok(resource) => {
                        callback(resource);
                    }
                    Err(e) => {
                        error!("Failed to deserialize resource: {}", e);
                    }
                }
            }
        });
        
        info!("Successfully subscribed to key expression: {}", key_expr);
        Ok(())
    }
    
    /// Publish a message to a channel
    pub async fn publish_message(
        &self,
        channel: &str,
        sender: String,
        text: String,
        metadata: HashMap<String, String>,
    ) -> Result<()> {
        let message = MessageContent {
            id: Uuid::new_v4(),
            sender,
            text,
            timestamp: Utc::now(),
            metadata,
        };
        
        let key = WeaveKeys::message(channel);
        self.publish_resource(&key, WeaveResource::Message(message)).await
    }
    
    /// Publish a basic ceremony event
    pub async fn publish_ceremony(&self, ceremony: BasicCeremonyEvent) -> Result<()> {
        let key = WeaveKeys::ceremony(&ceremony.id);
        self.publish_resource(&key, WeaveResource::Ceremony(ceremony)).await
    }
    
    /// Start heartbeat for node discovery
    pub async fn start_heartbeat(&self, capabilities: Vec<String>) -> Result<()> {
        let node_id = self.node_id;
        let session = self.session.clone();
        let key = WeaveKeys::heartbeat(&node_id);
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30));
            
            loop {
                interval.tick().await;
                
                let heartbeat = NodeHeartbeat {
                    node_id,
                    capabilities: capabilities.clone(),
                    load: 0.5, // TODO: Implement actual load calculation
                    timestamp: Utc::now(),
                    metadata: HashMap::new(),
                };
                
                let payload = match serde_json::to_vec(&WeaveResource::Heartbeat(heartbeat)) {
                    Ok(payload) => payload,
                    Err(e) => {
                        error!("Failed to serialize heartbeat: {}", e);
                        continue;
                    }
                };
                
                if let Err(e) = session.put(&key, payload).await {
                    error!("Failed to publish heartbeat: {}", e);
                }
            }
        });
        
        info!("Started heartbeat for node: {}", node_id);
        Ok(())
    }
    
    /// Subscribe to Sacred Alliance communication channel (basic interface)
    pub async fn subscribe_sacred_alliance<F>(&self, channel: &str, callback: F) -> Result<()>
    where
        F: Fn(WeaveResource) + Send + Sync + 'static,
    {
        let key = WeaveKeys::sacred_alliance(channel);
        self.subscribe(&key, callback).await
    }
    
    /// Publish to Sacred Alliance communication channel (basic interface)
    pub async fn publish_sacred_alliance(
        &self,
        channel: &str,
        resource: WeaveResource,
    ) -> Result<()> {
        let key = WeaveKeys::sacred_alliance(channel);
        self.publish_resource(&key, resource).await
    }
    
    /// Close the protocol and cleanup resources
    pub async fn close(self) -> Result<()> {
        info!("Closing WeaveMesh protocol for node: {}", self.node_id);
        
        // Close all subscriptions
        let subscriptions = self.subscriptions.read().await;
        for (key, _) in subscriptions.iter() {
            debug!("Closing subscription for key: {}", key);
        }
        
        // Close Zenoh session
        if let Ok(session) = Arc::try_unwrap(self.session) {
            session.close().await
                .map_err(|e| anyhow::anyhow!("Failed to close session: {}", e))?;
        }
        
        info!("WeaveMesh protocol closed successfully");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_weave_keys() {
        assert_eq!(
            WeaveKeys::message("general"),
            "weave/messages/general"
        );
        
        let ceremony_id = Uuid::new_v4();
        assert_eq!(
            WeaveKeys::ceremony(&ceremony_id),
            format!("weave/ceremonies/{}", ceremony_id)
        );
    }
    
    #[tokio::test]
    async fn test_weave_config_default() {
        let config = WeaveConfig::default();
        assert_eq!(config.connect_endpoints, vec!["tcp/127.0.0.1:7447"]);
        assert!(config.multicast_scouting);
        assert_eq!(config.default_timeout, 30);
    }
    
    #[tokio::test]
    async fn test_resource_serialization() {
        let message = MessageContent {
            id: Uuid::new_v4(),
            sender: "test".to_string(),
            text: "Hello".to_string(),
            timestamp: Utc::now(),
            metadata: HashMap::new(),
        };
        
        let resource = WeaveResource::Message(message);
        let serialized = serde_json::to_vec(&resource).unwrap();
        let deserialized: WeaveResource = serde_json::from_slice(&serialized).unwrap();
        
        match deserialized {
            WeaveResource::Message(msg) => {
                assert_eq!(msg.text, "Hello");
            }
            _ => panic!("Wrong resource type"),
        }
    }
}
