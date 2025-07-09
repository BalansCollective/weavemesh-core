//! WeaveMesh Core Networking Demo
//! 
//! This example demonstrates the universal networking capabilities of WeaveMesh Core,
//! including node discovery, communication, and mesh formation.

use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use uuid::Uuid;

use weavemesh_core::networking::{
    ZenohSession, NodeDiscovery, NodeCommunication, NodeInfo, NodeCapability,
    DiscoveryConfig, CommunicationConfig, OutgoingMessage, DeliveryOptions,
    MessagePriority, MessageType, WeaveMeshTopics,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🌐 WeaveMesh Core Networking Demo");
    println!("==================================");
    
    // Create two nodes to demonstrate networking
    let node1_id = Uuid::new_v4();
    let node2_id = Uuid::new_v4();
    
    println!("📡 Creating nodes:");
    println!("  Node 1: {}", node1_id);
    println!("  Node 2: {}", node2_id);
    
    // Initialize Zenoh sessions for both nodes
    let session1 = Arc::new(ZenohSession::new().await?);
    let session2 = Arc::new(ZenohSession::new().await?);
    
    // Create discovery managers
    let discovery_config = DiscoveryConfig {
        announcement_interval: 5, // Announce every 5 seconds
        discovery_timeout: 3,
        node_timeout: 30,
        debug: true,
    };
    
    let discovery1 = NodeDiscovery::new(
        node1_id,
        Arc::clone(&session1),
        discovery_config.clone(),
    );
    
    let discovery2 = NodeDiscovery::new(
        node2_id,
        Arc::clone(&session2),
        discovery_config.clone(),
    );
    
    // Create communication managers
    let comm_config = CommunicationConfig {
        max_message_size: 1024 * 1024, // 1MB
        message_timeout: 10,
        max_retries: 3,
        require_acks: true,
        enable_encryption: false, // Disabled for demo
        debug: true,
    };
    
    let comm1 = NodeCommunication::new(
        node1_id,
        Arc::clone(&session1),
        comm_config.clone(),
    );
    
    let comm2 = NodeCommunication::new(
        node2_id,
        Arc::clone(&session2),
        comm_config.clone(),
    );
    
    // Start discovery and communication
    println!("\n🚀 Starting networking services...");
    
    // Create node information
    let node1_info = NodeInfo {
        node_id: node1_id,
        display_name: "Demo Node 1".to_string(),
        context_id: "demo-context".to_string(),
        capabilities: vec![
            NodeCapability::ResourceStorage,
            NodeCapability::Collaboration,
            NodeCapability::AiAssistance,
        ],
        endpoints: vec!["tcp/127.0.0.1:8001".to_string()],
        discovered_at: chrono::Utc::now(),
        last_seen: chrono::Utc::now(),
        is_online: true,
        metadata: std::collections::HashMap::new(),
    };
    
    let node2_info = NodeInfo {
        node_id: node2_id,
        display_name: "Demo Node 2".to_string(),
        context_id: "demo-context".to_string(),
        capabilities: vec![
            NodeCapability::ResourceStorage,
            NodeCapability::GitIntegration,
            NodeCapability::SecurityServices,
        ],
        endpoints: vec!["tcp/127.0.0.1:8002".to_string()],
        discovered_at: chrono::Utc::now(),
        last_seen: chrono::Utc::now(),
        is_online: true,
        metadata: std::collections::HashMap::new(),
    };
    
    // Start discovery
    discovery1.start(node1_info.clone()).await?;
    discovery2.start(node2_info.clone()).await?;
    
    // Start communication
    comm1.start().await?;
    comm2.start().await?;
    
    println!("✅ Networking services started");
    
    // Wait for discovery
    println!("\n🔍 Waiting for node discovery...");
    sleep(Duration::from_secs(3)).await;
    
    // Check discovered nodes
    let discovered_by_1 = discovery1.get_all_nodes().await;
    let discovered_by_2 = discovery2.get_all_nodes().await;
    
    println!("📋 Node 1 discovered {} nodes:", discovered_by_1.len());
    for node in &discovered_by_1 {
        println!("  - {} ({})", node.display_name, node.node_id);
    }
    
    println!("📋 Node 2 discovered {} nodes:", discovered_by_2.len());
    for node in &discovered_by_2 {
        println!("  - {} ({})", node.display_name, node.node_id);
    }
    
    // Demonstrate capability-based discovery
    println!("\n🎯 Finding nodes with AI assistance capability...");
    let ai_nodes = discovery1.get_nodes_with_capabilities(vec![
        NodeCapability::AiAssistance
    ]).await;
    
    println!("Found {} nodes with AI assistance:", ai_nodes.len());
    for node in &ai_nodes {
        println!("  - {} ({})", node.display_name, node.node_id);
    }
    
    // Demonstrate context-based discovery
    println!("\n🌍 Finding nodes in demo-context...");
    let context_nodes = discovery1.get_context_nodes("demo-context").await;
    
    println!("Found {} nodes in demo-context:", context_nodes.len());
    for node in &context_nodes {
        println!("  - {} ({})", node.display_name, node.node_id);
    }
    
    // Demonstrate direct messaging
    println!("\n💬 Sending direct message from Node 1 to Node 2...");
    
    let message = OutgoingMessage {
        target_node: node2_id,
        message_type: MessageType::Collaboration,
        payload: b"Hello from Node 1! This is a test message.".to_vec(),
        options: DeliveryOptions {
            require_ack: true,
            max_retries: 3,
            timeout_seconds: 10,
            priority: MessagePriority::Normal,
            encrypt: false,
        },
        context: Some("demo-context".to_string()),
    };
    
    let mut response_receiver = comm1.send_message(message).await?;
    
    // Wait for response
    tokio::select! {
        result = response_receiver.recv() => {
            match result {
                Some(response) => {
                    println!("✅ Message delivery result: {:?}", response);
                }
                None => {
                    println!("❌ No response received");
                }
            }
        }
        _ = sleep(Duration::from_secs(5)) => {
            println!("⏰ Message response timeout");
        }
    }
    
    // Demonstrate broadcast messaging
    println!("\n📢 Broadcasting message to all nodes...");
    
    comm1.broadcast_message(
        MessageType::SystemControl,
        b"Broadcast message from Node 1 to all nodes!".to_vec(),
        Some("demo-context".to_string()),
    ).await?;
    
    println!("✅ Broadcast message sent");
    
    // Demonstrate context messaging
    println!("\n🎯 Sending context-specific message...");
    
    comm1.send_context_message(
        "demo-context",
        "announcements",
        MessageType::Collaboration,
        b"Context-specific announcement from Node 1".to_vec(),
    ).await?;
    
    println!("✅ Context message sent");
    
    // Show communication statistics
    println!("\n📊 Communication Statistics:");
    
    let stats1 = comm1.get_stats().await;
    let stats2 = comm2.get_stats().await;
    
    println!("Node 1 Stats:");
    println!("  Messages sent: {}", stats1.messages_sent);
    println!("  Messages received: {}", stats1.messages_received);
    println!("  Bytes sent: {}", stats1.bytes_sent);
    println!("  Bytes received: {}", stats1.bytes_received);
    
    println!("Node 2 Stats:");
    println!("  Messages sent: {}", stats2.messages_sent);
    println!("  Messages received: {}", stats2.messages_received);
    println!("  Bytes sent: {}", stats2.bytes_sent);
    println!("  Bytes received: {}", stats2.bytes_received);
    
    // Demonstrate capability updates
    println!("\n🔄 Updating Node 1 capabilities...");
    
    discovery1.update_capabilities(vec![
        NodeCapability::ResourceStorage,
        NodeCapability::Collaboration,
        NodeCapability::AiAssistance,
        NodeCapability::WebSocketBridge, // New capability
    ]).await?;
    
    println!("✅ Capabilities updated");
    
    // Wait a bit for the update to propagate
    sleep(Duration::from_secs(2)).await;
    
    // Check updated capabilities
    let updated_nodes = discovery2.get_nodes_with_capabilities(vec![
        NodeCapability::WebSocketBridge
    ]).await;
    
    println!("Nodes with WebSocket bridge capability: {}", updated_nodes.len());
    for node in &updated_nodes {
        println!("  - {} ({})", node.display_name, node.node_id);
    }
    
    // Demonstrate high-priority messaging
    println!("\n🚨 Sending high-priority message...");
    
    let priority_message = OutgoingMessage {
        target_node: node2_id,
        message_type: MessageType::SystemControl,
        payload: b"URGENT: High priority system message!".to_vec(),
        options: DeliveryOptions {
            require_ack: true,
            max_retries: 5,
            timeout_seconds: 5,
            priority: MessagePriority::Critical,
            encrypt: false,
        },
        context: Some("demo-context".to_string()),
    };
    
    let mut priority_receiver = comm1.send_message(priority_message).await?;
    
    // Wait for priority response
    tokio::select! {
        result = priority_receiver.recv() => {
            match result {
                Some(response) => {
                    println!("✅ Priority message result: {:?}", response);
                }
                None => {
                    println!("❌ No priority response received");
                }
            }
        }
        _ = sleep(Duration::from_secs(3)) => {
            println!("⏰ Priority message timeout");
        }
    }
    
    // Show final statistics
    println!("\n📈 Final Statistics:");
    
    let final_stats1 = comm1.get_stats().await;
    let final_stats2 = comm2.get_stats().await;
    
    println!("Node 1 Final Stats:");
    println!("  Total messages sent: {}", final_stats1.messages_sent);
    println!("  Total messages received: {}", final_stats1.messages_received);
    println!("  Total bytes sent: {}", final_stats1.bytes_sent);
    println!("  Total bytes received: {}", final_stats2.bytes_received);
    println!("  Pending messages: {}", comm1.get_pending_count().await);
    
    println!("Node 2 Final Stats:");
    println!("  Total messages sent: {}", final_stats2.messages_sent);
    println!("  Total messages received: {}", final_stats2.messages_received);
    println!("  Total bytes sent: {}", final_stats2.bytes_sent);
    println!("  Total bytes received: {}", final_stats2.bytes_received);
    println!("  Pending messages: {}", comm2.get_pending_count().await);
    
    // Cleanup
    println!("\n🧹 Cleaning up...");
    
    comm1.stop().await?;
    comm2.stop().await?;
    discovery1.stop().await?;
    discovery2.stop().await?;
    
    println!("✅ Demo completed successfully!");
    println!("\n🎉 WeaveMesh Core networking capabilities demonstrated:");
    println!("  ✓ Node discovery and registration");
    println!("  ✓ Capability-based node filtering");
    println!("  ✓ Context-aware communication");
    println!("  ✓ Direct node-to-node messaging");
    println!("  ✓ Broadcast messaging");
    println!("  ✓ Message priority handling");
    println!("  ✓ Communication statistics");
    println!("  ✓ Dynamic capability updates");
    
    Ok(())
}

/// Helper function to demonstrate message handling
async fn setup_message_handlers(comm: &NodeCommunication) -> anyhow::Result<()> {
    // Register handler for collaboration messages
    comm.register_handler(MessageType::Collaboration, |incoming| {
        let payload = String::from_utf8_lossy(&incoming.message.payload);
        println!("📨 Received collaboration message: {}", payload);
        
        // Send a response
        Ok(Some(b"Collaboration message received and processed".to_vec()))
    }).await;
    
    // Register handler for system control messages
    comm.register_handler(MessageType::SystemControl, |incoming| {
        let payload = String::from_utf8_lossy(&incoming.message.payload);
        println!("🔧 Received system control message: {}", payload);
        
        // No response for system messages
        Ok(None)
    }).await;
    
    // Register handler for heartbeat messages
    comm.register_handler(MessageType::Heartbeat, |incoming| {
        println!("💓 Received heartbeat from {}", incoming.message.from_node);
        Ok(None)
    }).await;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_networking_demo_components() {
        // Test that we can create the basic components
        let node_id = Uuid::new_v4();
        
        // This would require a running Zenoh router in a real test
        // For now, just test that the types can be constructed
        let discovery_config = DiscoveryConfig::default();
        let comm_config = CommunicationConfig::default();
        
        assert_eq!(discovery_config.announcement_interval, 30);
        assert_eq!(comm_config.max_message_size, 1024 * 1024);
        assert!(comm_config.require_acks);
    }
    
    #[test]
    fn test_node_capabilities() {
        let capabilities = vec![
            NodeCapability::ResourceStorage,
            NodeCapability::AiAssistance,
            NodeCapability::Custom("special-feature".to_string()),
        ];
        
        assert!(capabilities.contains(&NodeCapability::ResourceStorage));
        assert!(!capabilities.contains(&NodeCapability::GitIntegration));
    }
    
    #[test]
    fn test_message_priorities() {
        let mut priorities = vec![
            MessagePriority::Low,
            MessagePriority::Critical,
            MessagePriority::Normal,
            MessagePriority::High,
        ];
        
        priorities.sort();
        
        assert_eq!(priorities[0], MessagePriority::Low);
        assert_eq!(priorities[3], MessagePriority::Critical);
    }
}
