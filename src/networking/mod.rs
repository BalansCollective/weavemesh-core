//! Universal Networking Infrastructure
//!
//! Core networking components that provide the foundation for distributed
//! mesh communication. This module contains universal primitives that can be
//! extended by context-specific plugins.

pub mod zenoh_integration;
pub mod node_discovery;
pub mod node_communication;

// Re-export key types for convenience
pub use zenoh_integration::{
    ZenohSession, ZenohConfig, ZenohMode, WeaveMeshMessage, MessageType,
    WeaveMeshTopics, ZenohError
};
pub use node_discovery::{
    NodeDiscovery, DiscoveryConfig, NodeInfo, NodeCapability, NodeAnnouncement,
    AnnouncementType, DiscoveryQuery, NodeFilter, DiscoveryError
};
pub use node_communication::{
    NodeCommunication, CommunicationConfig, IncomingMessage, OutgoingMessage,
    DeliveryOptions, MessagePriority, MessageResult, CommunicationStats,
    CommunicationError, MessageHandler
};

use anyhow::Result;
use std::sync::Arc;
use uuid::Uuid;

/// Universal networking interface for different contexts
#[async_trait::async_trait]
pub trait NetworkingProvider: Send + Sync {
    /// Provider name
    fn name(&self) -> &str;
    
    /// Initialize the networking provider
    async fn initialize(&mut self, config: &serde_json::Value) -> Result<()>;
    
    /// Handle network events
    async fn handle_network_event(&self, event: &NetworkEvent) -> Result<()>;
    
    /// Get network statistics
    async fn get_network_stats(&self) -> Result<NetworkStats>;
    
    /// Cleanup provider resources
    async fn cleanup(&mut self) -> Result<()>;
}

/// Network events that can be handled by providers
#[derive(Debug, Clone)]
pub enum NetworkEvent {
    /// Node joined the network
    NodeJoined {
        node_id: String,
        node_info: NodeInfo,
    },
    
    /// Node left the network
    NodeLeft {
        node_id: String,
    },
    
    /// Message received
    MessageReceived {
        message: WeaveMeshMessage,
    },
    
    /// Message sent
    MessageSent {
        message_id: String,
        target_node: String,
    },
    
    /// Network error occurred
    NetworkError {
        error: String,
    },
    
    /// Connection status changed
    ConnectionStatusChanged {
        is_connected: bool,
    },
}

/// Network statistics
#[derive(Debug, Clone, Default)]
pub struct NetworkStats {
    /// Total nodes discovered
    pub nodes_discovered: u64,
    
    /// Currently active nodes
    pub active_nodes: u64,
    
    /// Total messages sent
    pub messages_sent: u64,
    
    /// Total messages received
    pub messages_received: u64,
    
    /// Total bytes sent
    pub bytes_sent: u64,
    
    /// Total bytes received
    pub bytes_received: u64,
    
    /// Average message latency in milliseconds
    pub avg_latency_ms: f64,
    
    /// Network uptime in seconds
    pub uptime_seconds: u64,
}

/// Universal networking manager
pub struct NetworkingManager {
    /// Zenoh session for mesh communication
    zenoh_session: Option<Arc<ZenohSession>>,
    
    /// Node discovery manager
    node_discovery: Option<Arc<NodeDiscovery>>,
    
    /// Node communication manager
    node_communication: Option<Arc<NodeCommunication>>,
    
    /// Registered networking providers
    providers: Vec<Box<dyn NetworkingProvider>>,
    
    /// Whether networking is active
    is_active: bool,
}

impl NetworkingManager {
    /// Create a new networking manager
    pub fn new() -> Self {
        Self {
            zenoh_session: None,
            node_discovery: None,
            node_communication: None,
            providers: Vec::new(),
            is_active: false,
        }
    }
    
    /// Initialize networking with Zenoh session
    pub async fn initialize(
        &mut self,
        node_id: Uuid,
        zenoh_config: ZenohConfig,
        discovery_config: DiscoveryConfig,
        communication_config: CommunicationConfig,
    ) -> Result<(), NetworkingError> {
        // Create Zenoh session
        let zenoh_session = Arc::new(
            ZenohSession::new(node_id, zenoh_config)
                .await
                .map_err(|e| NetworkingError::InitializationFailed(e.to_string()))?
        );
        
        // Create node discovery
        let node_discovery = Arc::new(NodeDiscovery::new(
            node_id,
            Arc::clone(&zenoh_session),
            discovery_config,
        ));
        
        // Create node communication
        let node_communication = Arc::new(NodeCommunication::new(
            node_id,
            Arc::clone(&zenoh_session),
            communication_config,
        ));
        
        self.zenoh_session = Some(zenoh_session);
        self.node_discovery = Some(node_discovery);
        self.node_communication = Some(node_communication);
        
        Ok(())
    }
    
    /// Start networking
    pub async fn start(&mut self, node_info: NodeInfo) -> Result<(), NetworkingError> {
        if self.is_active {
            return Ok(());
        }
        
        // Start node communication
        if let Some(ref comm) = self.node_communication {
            comm.start()
                .await
                .map_err(|e| NetworkingError::StartFailed(e.to_string()))?;
        }
        
        // Start node discovery
        if let Some(ref discovery) = self.node_discovery {
            discovery.start(node_info)
                .await
                .map_err(|e| NetworkingError::StartFailed(e.to_string()))?;
        }
        
        // Initialize all providers
        for provider in &mut self.providers {
            provider.initialize(&serde_json::Value::Null)
                .await
                .map_err(|e| NetworkingError::ProviderError(e.to_string()))?;
        }
        
        self.is_active = true;
        Ok(())
    }
    
    /// Stop networking
    pub async fn stop(&mut self) -> Result<(), NetworkingError> {
        if !self.is_active {
            return Ok(());
        }
        
        // Cleanup all providers
        for provider in &mut self.providers {
            provider.cleanup()
                .await
                .map_err(|e| NetworkingError::ProviderError(e.to_string()))?;
        }
        
        // Stop node discovery
        if let Some(ref discovery) = self.node_discovery {
            discovery.stop()
                .await
                .map_err(|e| NetworkingError::StopFailed(e.to_string()))?;
        }
        
        // Stop node communication
        if let Some(ref comm) = self.node_communication {
            comm.stop()
                .await
                .map_err(|e| NetworkingError::StopFailed(e.to_string()))?;
        }
        
        self.is_active = false;
        Ok(())
    }
    
    /// Register a networking provider
    pub fn register_provider(&mut self, provider: Box<dyn NetworkingProvider>) {
        self.providers.push(provider);
    }
    
    /// Get Zenoh session
    pub fn zenoh_session(&self) -> Option<Arc<ZenohSession>> {
        self.zenoh_session.clone()
    }
    
    /// Get node discovery
    pub fn node_discovery(&self) -> Option<Arc<NodeDiscovery>> {
        self.node_discovery.clone()
    }
    
    /// Get node communication
    pub fn node_communication(&self) -> Option<Arc<NodeCommunication>> {
        self.node_communication.clone()
    }
    
    /// Get combined network statistics
    pub async fn get_network_stats(&self) -> Result<NetworkStats, NetworkingError> {
        let mut stats = NetworkStats::default();
        
        // Get communication stats
        if let Some(ref comm) = self.node_communication {
            let comm_stats = comm.get_stats().await;
            stats.messages_sent = comm_stats.messages_sent;
            stats.messages_received = comm_stats.messages_received;
            stats.bytes_sent = comm_stats.bytes_sent;
            stats.bytes_received = comm_stats.bytes_received;
            stats.avg_latency_ms = comm_stats.avg_delivery_time_ms;
        }
        
        // Get discovery stats
        if let Some(ref discovery) = self.node_discovery {
            let nodes = discovery.get_all_nodes().await;
            stats.nodes_discovered = nodes.len() as u64;
            stats.active_nodes = nodes.iter().filter(|n| n.is_online).count() as u64;
        }
        
        // Get provider stats
        for provider in &self.providers {
            if let Ok(provider_stats) = provider.get_network_stats().await {
                // Combine provider stats (implementation specific)
                stats.uptime_seconds = provider_stats.uptime_seconds.max(stats.uptime_seconds);
            }
        }
        
        Ok(stats)
    }
    
    /// Broadcast network event to all providers
    pub async fn broadcast_event(&self, event: NetworkEvent) -> Result<(), NetworkingError> {
        for provider in &self.providers {
            provider.handle_network_event(&event)
                .await
                .map_err(|e| NetworkingError::ProviderError(e.to_string()))?;
        }
        Ok(())
    }
    
    /// Check if networking is active
    pub fn is_active(&self) -> bool {
        self.is_active
    }
}

impl Default for NetworkingManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Networking errors
#[derive(Debug, thiserror::Error)]
pub enum NetworkingError {
    #[error("Initialization failed: {0}")]
    InitializationFailed(String),
    
    #[error("Start failed: {0}")]
    StartFailed(String),
    
    #[error("Stop failed: {0}")]
    StopFailed(String),
    
    #[error("Provider error: {0}")]
    ProviderError(String),
    
    #[error("Not initialized")]
    NotInitialized,
    
    #[error("Already active")]
    AlreadyActive,
    
    #[error("Not active")]
    NotActive,
}

/// Utility functions for networking
pub mod utils {
    use super::*;
    
    /// Generate a unique message ID
    pub fn generate_message_id() -> String {
        Uuid::new_v4().to_string()
    }
    
    /// Create default node capabilities
    pub fn default_node_capabilities() -> Vec<NodeCapability> {
        vec![
            NodeCapability::ResourceStorage,
            NodeCapability::Collaboration,
        ]
    }
    
    /// Check if two nodes are compatible for communication
    pub fn nodes_compatible(node1: &NodeInfo, node2: &NodeInfo) -> bool {
        // Basic compatibility check - both must be online
        node1.is_online && node2.is_online
    }
    
    /// Calculate network health score based on stats
    pub fn calculate_network_health(stats: &NetworkStats) -> f64 {
        if stats.nodes_discovered == 0 {
            return 0.0;
        }
        
        let connectivity_score = stats.active_nodes as f64 / stats.nodes_discovered as f64;
        let latency_score = if stats.avg_latency_ms > 0.0 {
            (1000.0 / stats.avg_latency_ms).min(1.0)
        } else {
            1.0
        };
        
        (connectivity_score + latency_score) / 2.0
    }
    
    /// Format network statistics for display
    pub fn format_network_stats(stats: &NetworkStats) -> String {
        format!(
            "Nodes: {}/{}, Messages: {}/{}, Bytes: {}/{}, Latency: {:.1}ms, Health: {:.1}%",
            stats.active_nodes,
            stats.nodes_discovered,
            stats.messages_sent,
            stats.messages_received,
            stats.bytes_sent,
            stats.bytes_received,
            stats.avg_latency_ms,
            calculate_network_health(stats) * 100.0
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::utils::*;

    #[test]
    fn test_networking_manager_creation() {
        let manager = NetworkingManager::new();
        assert!(!manager.is_active());
        assert!(manager.zenoh_session().is_none());
        assert!(manager.node_discovery().is_none());
        assert!(manager.node_communication().is_none());
    }

    #[test]
    fn test_network_health_calculation() {
        let stats = NetworkStats {
            nodes_discovered: 10,
            active_nodes: 8,
            avg_latency_ms: 50.0,
            ..Default::default()
        };
        
        let health = calculate_network_health(&stats);
        assert!(health > 0.0 && health <= 1.0);
    }

    #[test]
    fn test_node_compatibility() {
        let node1 = NodeInfo {
            node_id: Uuid::new_v4(),
            display_name: "Node 1".to_string(),
            context_id: "context1".to_string(),
            capabilities: default_node_capabilities(),
            endpoints: vec!["tcp/127.0.0.1:8080".to_string()],
            discovered_at: chrono::Utc::now(),
            last_seen: chrono::Utc::now(),
            is_online: true,
            metadata: std::collections::HashMap::new(),
        };
        
        let node2 = NodeInfo {
            node_id: Uuid::new_v4(),
            display_name: "Node 2".to_string(),
            context_id: "context2".to_string(),
            capabilities: default_node_capabilities(),
            endpoints: vec!["tcp/127.0.0.1:8081".to_string()],
            discovered_at: chrono::Utc::now(),
            last_seen: chrono::Utc::now(),
            is_online: true,
            metadata: std::collections::HashMap::new(),
        };
        
        assert!(nodes_compatible(&node1, &node2));
        
        let mut node3 = node2.clone();
        node3.is_online = false;
        assert!(!nodes_compatible(&node1, &node3));
    }

    #[test]
    fn test_message_id_generation() {
        let id1 = generate_message_id();
        let id2 = generate_message_id();
        
        assert_ne!(id1, id2);
        assert!(!id1.is_empty());
        assert!(!id2.is_empty());
    }

    #[test]
    fn test_network_stats_formatting() {
        let stats = NetworkStats {
            nodes_discovered: 5,
            active_nodes: 4,
            messages_sent: 100,
            messages_received: 95,
            bytes_sent: 1024,
            bytes_received: 980,
            avg_latency_ms: 25.5,
            uptime_seconds: 3600,
        };
        
        let formatted = format_network_stats(&stats);
        assert!(formatted.contains("4/5"));
        assert!(formatted.contains("100/95"));
        assert!(formatted.contains("25.5ms"));
    }
}
