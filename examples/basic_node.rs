//! Basic WeaveMesh node example
//!
//! This example demonstrates how to create a basic WeaveMesh node
//! that can communicate with other nodes in the mesh.

use std::collections::HashMap;
use tokio::time::{sleep, Duration};
use tracing::info;
use weavemesh_core::{WeaveMeshBuilder, WeaveResource};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    info!("Starting basic WeaveMesh node...");
    
    // Build the WeaveMesh protocol
    let protocol = WeaveMeshBuilder::new()
        .add_capability("basic-communication".to_string())
        .add_capability("message-relay".to_string())
        .build()
        .await?;
    
    info!("WeaveMesh node started with ID: {}", protocol.node_id());
    
    // Subscribe to general messages
    protocol.subscribe("weave/messages/general", |resource| {
        match resource {
            WeaveResource::Message(msg) => {
                info!("Received message from {}: {}", msg.sender, msg.text);
            }
            _ => {
                info!("Received non-message resource");
            }
        }
    }).await?;
    
    // Send some test messages
    for i in 1..=5 {
        let message = format!("Hello from basic node #{}", i);
        protocol.publish_message(
            "general",
            format!("basic-node-{}", protocol.node_id()),
            message,
            HashMap::new(),
        ).await?;
        
        info!("Sent message #{}", i);
        sleep(Duration::from_secs(2)).await;
    }
    
    // Keep the node running for a bit to receive messages
    info!("Listening for messages for 30 seconds...");
    sleep(Duration::from_secs(30)).await;
    
    // Clean shutdown
    info!("Shutting down basic node...");
    protocol.close().await?;
    
    Ok(())
}
