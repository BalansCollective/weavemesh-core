//! Simple Sacred Alliance communication example
//!
//! This example demonstrates basic Sacred Alliance communication
//! between human and AI participants.

use std::collections::HashMap;
use tokio::time::{sleep, Duration};
use tracing::info;
use weavemesh_core::{
    BasicSacredAllianceChannel, ChannelConfig, Participant, 
    ParticipantType, PresenceStatus, AllianceMessage, 
    AllianceMessageContent, utils,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    info!("Starting Sacred Alliance communication example...");
    
    // Create a Sacred Alliance channel
    let config = ChannelConfig::default();
    let mut alliance = BasicSacredAllianceChannel::new("example-alliance".to_string(), config);
    
    // Add human participant
    let human = Participant {
        id: "human-developer".to_string(),
        participant_type: ParticipantType::Human,
        presence: PresenceStatus::Active,
        capabilities: vec!["coding".to_string(), "design".to_string()],
        joined_at: utils::now(),
    };
    
    alliance.add_participant(human)?;
    info!("Added human participant to alliance");
    
    // Add AI participant
    let ai = Participant {
        id: "ai-assistant".to_string(),
        participant_type: ParticipantType::Ai,
        presence: PresenceStatus::Active,
        capabilities: vec!["analysis".to_string(), "optimization".to_string()],
        joined_at: utils::now(),
    };
    
    alliance.add_participant(ai)?;
    info!("Added AI participant to alliance");
    
    // Simulate a conversation
    let messages = vec![
        ("human-developer", "Hello AI! I'm working on a new feature and could use some help."),
        ("ai-assistant", "Hello! I'd be happy to help. What kind of feature are you working on?"),
        ("human-developer", "It's a real-time collaboration system. I'm thinking about the architecture."),
        ("ai-assistant", "Interesting! For real-time collaboration, you might want to consider event-driven architecture with WebSockets or similar protocols."),
        ("human-developer", "That's a great suggestion! What about data consistency across multiple clients?"),
        ("ai-assistant", "For data consistency, you could implement operational transformation or conflict-free replicated data types (CRDTs)."),
    ];
    
    for (sender, text) in messages {
        let message = AllianceMessage {
            id: utils::generate_id(),
            sender: sender.to_string(),
            content: AllianceMessageContent::Text(text.to_string()),
            timestamp: utils::now(),
            metadata: HashMap::new(),
        };
        
        alliance.send_message(message)?;
        info!("{}: {}", sender, text);
        
        // Small delay to simulate natural conversation flow
        sleep(Duration::from_millis(500)).await;
    }
    
    // Show alliance statistics
    let stats = alliance.get_statistics();
    info!("Alliance Statistics:");
    info!("  Total participants: {}", stats.total_participants);
    info!("  Active participants: {}", stats.active_participants);
    info!("  Total messages: {}", stats.total_messages);
    info!("  Message types: {:?}", stats.message_type_distribution);
    
    // Show message history
    info!("Message History:");
    for (i, message) in alliance.get_history().iter().enumerate() {
        info!("  {}: {} - {:?}", i + 1, message.sender, message.content);
    }
    
    info!("Sacred Alliance communication example completed!");
    
    Ok(())
}
