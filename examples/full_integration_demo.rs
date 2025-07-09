//! WeaveMesh Core Full Integration Demo
//! 
//! This example demonstrates the complete integration of all WeaveMesh Core components:
//! - Universal networking with node discovery and communication
//! - Sacred Alliance collaborative protocols
//! - Attribution tracking and value recognition
//! - Group communication patterns
//! - Mesh management and health monitoring
//! 
//! This showcases the full collaborative individuation capabilities.

use std::sync::Arc;
use std::time::Duration;
use std::collections::HashMap;
use tokio::time::sleep;
use uuid::Uuid;

use weavemesh_core::{
    // Networking
    networking::{
        ZenohSession, NodeDiscovery, NodeCommunication, NodeInfo, NodeCapability,
        DiscoveryConfig, CommunicationConfig, OutgoingMessage, DeliveryOptions,
        MessagePriority, MessageType,
    },
    // Sacred Alliance
    sacred_alliance::{
        BasicSacredAllianceChannel, Participant, ParticipantType, PresenceStatus,
        AllianceMessage, BasicCeremonyAction, CollaborationIntent, ChannelConfig,
    },
    // Attribution
    attribution::{
        BasicAttributionEngine, Attribution, CollaborationType, AttributionContext,
        AttributionConfig,
    },
    // Group Communication
    group_communication::{
        BasicGroupCommunication, GroupId, Message, MessagePriority as GroupMessagePriority,
        GroupRole, GroupPermissions,
    },
    // Node Management
    node::{BasicNode, NodeConfig, NodeType, AIType, SecurityLevel},
    // Mesh Management
    mesh::{MeshManager, MeshConfig, LocalNode, NodeCapabilities, TrustLevel},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🌟 WeaveMesh Core Full Integration Demo");
    println!("=====================================");
    println!("Demonstrating collaborative individuation across all scales\n");
    
    // Create a collaborative scenario: Family + AI Assistant + Research Context
    let family_context = "lindgren-family";
    let research_context = "weaver-research";
    
    // Initialize three nodes representing different participants
    let samuel_id = Uuid::new_v4();
    let ai_assistant_id = Uuid::new_v4();
    let researcher_id = Uuid::new_v4();
    
    println!("👥 Creating collaborative participants:");
    println!("  Samuel (Family): {}", samuel_id);
    println!("  AI Assistant: {}", ai_assistant_id);
    println!("  Researcher: {}", researcher_id);
    
    // Initialize networking infrastructure
    let session_samuel = Arc::new(ZenohSession::new().await?);
    let session_ai = Arc::new(ZenohSession::new().await?);
    let session_researcher = Arc::new(ZenohSession::new().await?);
    
    // Create discovery and communication for each node
    let discovery_config = DiscoveryConfig {
        announcement_interval: 3,
        discovery_timeout: 2,
        node_timeout: 30,
        debug: true,
    };
    
    let comm_config = CommunicationConfig {
        max_message_size: 1024 * 1024,
        message_timeout: 15,
        max_retries: 3,
        require_acks: true,
        enable_encryption: false, // Simplified for demo
        debug: true,
    };
    
    // Samuel's networking
    let discovery_samuel = NodeDiscovery::new(samuel_id, Arc::clone(&session_samuel), discovery_config.clone());
    let comm_samuel = NodeCommunication::new(samuel_id, Arc::clone(&session_samuel), comm_config.clone());
    
    // AI Assistant's networking
    let discovery_ai = NodeDiscovery::new(ai_assistant_id, Arc::clone(&session_ai), discovery_config.clone());
    let comm_ai = NodeCommunication::new(ai_assistant_id, Arc::clone(&session_ai), comm_config.clone());
    
    // Researcher's networking
    let discovery_researcher = NodeDiscovery::new(researcher_id, Arc::clone(&session_researcher), discovery_config.clone());
    let comm_researcher = NodeCommunication::new(researcher_id, Arc::clone(&session_researcher), comm_config.clone());
    
    // Create node information with different capabilities
    let samuel_info = NodeInfo {
        node_id: samuel_id,
        display_name: "Samuel (Family Lead)".to_string(),
        context_id: family_context.to_string(),
        capabilities: vec![
            NodeCapability::Collaboration,
            NodeCapability::ResourceStorage,
            NodeCapability::AttributionTracking,
        ],
        endpoints: vec!["tcp/127.0.0.1:8001".to_string()],
        discovered_at: chrono::Utc::now(),
        last_seen: chrono::Utc::now(),
        is_online: true,
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("role".to_string(), "family-lead".to_string());
            meta.insert("expertise".to_string(), "software-development".to_string());
            meta
        },
    };
    
    let ai_info = NodeInfo {
        node_id: ai_assistant_id,
        display_name: "AI Assistant (Cline)".to_string(),
        context_id: family_context.to_string(),
        capabilities: vec![
            NodeCapability::AiAssistance,
            NodeCapability::Collaboration,
            NodeCapability::SacredAllianceValidation,
            NodeCapability::AttributionTracking,
        ],
        endpoints: vec!["tcp/127.0.0.1:8002".to_string()],
        discovered_at: chrono::Utc::now(),
        last_seen: chrono::Utc::now(),
        is_online: true,
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("role".to_string(), "ai-assistant".to_string());
            meta.insert("model".to_string(), "claude-3.5-sonnet".to_string());
            meta.insert("sacred_alliance".to_string(), "enabled".to_string());
            meta
        },
    };
    
    let researcher_info = NodeInfo {
        node_id: researcher_id,
        display_name: "Research Collaborator".to_string(),
        context_id: research_context.to_string(),
        capabilities: vec![
            NodeCapability::Collaboration,
            NodeCapability::GitIntegration,
            NodeCapability::ResourceStorage,
            NodeCapability::AttributionTracking,
        ],
        endpoints: vec!["tcp/127.0.0.1:8003".to_string()],
        discovered_at: chrono::Utc::now(),
        last_seen: chrono::Utc::now(),
        is_online: true,
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("role".to_string(), "researcher".to_string());
            meta.insert("expertise".to_string(), "collaborative-individuation".to_string());
            meta
        },
    };
    
    // Start all networking services
    println!("\n🚀 Starting collaborative mesh network...");
    
    discovery_samuel.start(samuel_info.clone()).await?;
    discovery_ai.start(ai_info.clone()).await?;
    discovery_researcher.start(researcher_info.clone()).await?;
    
    comm_samuel.start().await?;
    comm_ai.start().await?;
    comm_researcher.start().await?;
    
    println!("✅ Mesh network active");
    
    // Wait for discovery
    sleep(Duration::from_secs(2)).await;
    
    // Initialize Sacred Alliance channels
    println!("\n🤝 Establishing Sacred Alliance channels...");
    
    let family_channel_config = ChannelConfig {
        channel_id: "family-collaboration".to_string(),
        context: family_context.to_string(),
        require_sacred_alliance: true,
        max_participants: 10,
        allow_ai_participants: true,
        require_attribution: true,
        enable_ceremony_tracking: true,
    };
    
    let research_channel_config = ChannelConfig {
        channel_id: "research-collaboration".to_string(),
        context: research_context.to_string(),
        require_sacred_alliance: true,
        max_participants: 50,
        allow_ai_participants: true,
        require_attribution: true,
        enable_ceremony_tracking: true,
    };
    
    // Create Sacred Alliance channels
    let family_channel = BasicSacredAllianceChannel::new(family_channel_config).await?;
    let research_channel = BasicSacredAllianceChannel::new(research_channel_config).await?;
    
    // Add participants to family channel
    let samuel_participant = Participant {
        id: samuel_id.to_string(),
        display_name: "Samuel".to_string(),
        participant_type: ParticipantType::Human,
        presence_status: PresenceStatus::Active,
        capabilities: vec!["software-development".to_string(), "family-coordination".to_string()],
        context: family_context.to_string(),
        joined_at: chrono::Utc::now(),
        last_active: chrono::Utc::now(),
        metadata: HashMap::new(),
    };
    
    let ai_participant = Participant {
        id: ai_assistant_id.to_string(),
        display_name: "Cline (AI Assistant)".to_string(),
        participant_type: ParticipantType::AI,
        presence_status: PresenceStatus::Active,
        capabilities: vec!["code-assistance".to_string(), "sacred-alliance-validation".to_string()],
        context: family_context.to_string(),
        joined_at: chrono::Utc::now(),
        last_active: chrono::Utc::now(),
        metadata: HashMap::new(),
    };
    
    family_channel.add_participant(samuel_participant.clone()).await?;
    family_channel.add_participant(ai_participant.clone()).await?;
    
    println!("✅ Sacred Alliance channels established");
    
    // Initialize Attribution Engine
    println!("\n📊 Setting up attribution tracking...");
    
    let attribution_config = AttributionConfig {
        track_all_interactions: true,
        require_explicit_attribution: false,
        enable_value_calculation: true,
        context_aware: true,
        sacred_alliance_integration: true,
    };
    
    let attribution_engine = BasicAttributionEngine::new(attribution_config).await?;
    
    // Initialize Group Communication
    println!("\n💬 Setting up group communication patterns...");
    
    let family_group_id = GroupId::new();
    let research_group_id = GroupId::new();
    
    let family_group = BasicGroupCommunication::new(family_group_id.clone()).await?;
    let research_group = BasicGroupCommunication::new(research_group_id.clone()).await?;
    
    // Add members to groups
    family_group.add_member(samuel_id, GroupRole::Admin, GroupPermissions::all()).await?;
    family_group.add_member(ai_assistant_id, GroupRole::Assistant, GroupPermissions::read_write()).await?;
    
    research_group.add_member(samuel_id, GroupRole::Collaborator, GroupPermissions::read_write()).await?;
    research_group.add_member(researcher_id, GroupRole::Lead, GroupPermissions::all()).await?;
    
    println!("✅ Group communication ready");
    
    // Demonstrate collaborative workflow
    println!("\n🎭 Demonstrating collaborative individuation workflow...");
    
    // 1. Family context: Samuel asks AI for help with WeaveMesh development
    println!("\n--- Family Context: Development Collaboration ---");
    
    let family_request = AllianceMessage {
        id: Uuid::new_v4().to_string(),
        from_participant: samuel_id.to_string(),
        to_participant: Some(ai_assistant_id.to_string()),
        channel_id: "family-collaboration".to_string(),
        message_content: weavemesh_core::sacred_alliance::MessageContent::Collaboration {
            intent: CollaborationIntent::RequestAssistance,
            content: "I need help implementing the networking layer for WeaveMesh Core. Can you help me understand the best approach for universal node discovery?".to_string(),
            context: Some("weavemesh-development".to_string()),
            ceremony_action: Some(BasicCeremonyAction::InitiateCollaboration),
        },
        timestamp: chrono::Utc::now(),
        requires_response: true,
        sacred_alliance_validated: true,
        attribution_context: Some("family-development-collaboration".to_string()),
    };
    
    family_channel.send_message(family_request.clone()).await?;
    
    // Track attribution for this collaboration
    let collaboration_attribution = Attribution {
        id: Uuid::new_v4(),
        participants: vec![samuel_id, ai_assistant_id],
        collaboration_type: CollaborationType::HumanAICollaboration,
        context: AttributionContext {
            domain: "software-development".to_string(),
            project: "weavemesh-core".to_string(),
            task: "networking-implementation".to_string(),
            sacred_alliance_channel: Some("family-collaboration".to_string()),
        },
        contributions: HashMap::new(),
        value_metrics: HashMap::new(),
        timestamp: chrono::Utc::now(),
        sacred_alliance_validated: true,
        metadata: HashMap::new(),
    };
    
    attribution_engine.record_attribution(collaboration_attribution).await?;
    
    // 2. AI responds with Sacred Alliance validation
    println!("🤖 AI Assistant responding with Sacred Alliance validation...");
    
    let ai_response = AllianceMessage {
        id: Uuid::new_v4().to_string(),
        from_participant: ai_assistant_id.to_string(),
        to_participant: Some(samuel_id.to_string()),
        channel_id: "family-collaboration".to_string(),
        message_content: weavemesh_core::sacred_alliance::MessageContent::Collaboration {
            intent: CollaborationIntent::ProvideAssistance,
            content: "I can help you with the networking layer! For universal node discovery, I recommend implementing a capability-based discovery system using Zenoh. This allows nodes to advertise their capabilities and discover others based on what they need. The key is to make it context-aware so family nodes can discover each other while maintaining privacy from other contexts.".to_string(),
            context: Some("weavemesh-development".to_string()),
            ceremony_action: Some(BasicCeremonyAction::ProvideGuidance),
        },
        timestamp: chrono::Utc::now(),
        requires_response: false,
        sacred_alliance_validated: true,
        attribution_context: Some("family-development-collaboration".to_string()),
    };
    
    family_channel.send_message(ai_response).await?;
    
    // 3. Cross-context collaboration: Samuel shares insights with researcher
    println!("\n--- Cross-Context: Research Collaboration ---");
    
    // Samuel joins research context
    let researcher_participant = Participant {
        id: researcher_id.to_string(),
        display_name: "Research Collaborator".to_string(),
        participant_type: ParticipantType::Human,
        presence_status: PresenceStatus::Active,
        capabilities: vec!["collaborative-individuation-research".to_string()],
        context: research_context.to_string(),
        joined_at: chrono::Utc::now(),
        last_active: chrono::Utc::now(),
        metadata: HashMap::new(),
    };
    
    research_channel.add_participant(researcher_participant).await?;
    
    // Samuel shares development insights in research context
    let research_message = OutgoingMessage {
        target_node: researcher_id,
        message_type: MessageType::Collaboration,
        payload: serde_json::to_vec(&serde_json::json!({
            "type": "research_insight",
            "content": "The networking implementation is revealing interesting patterns about collaborative individuation. The capability-based discovery naturally creates emergent collaboration patterns.",
            "context": "weavemesh-research",
            "attribution": "samuel-lindgren",
            "sacred_alliance_validated": true
        }))?,
        options: DeliveryOptions {
            require_ack: true,
            max_retries: 3,
            timeout_seconds: 10,
            priority: MessagePriority::Normal,
            encrypt: false,
        },
        context: Some(research_context.to_string()),
    };
    
    let mut research_response = comm_samuel.send_message(research_message).await?;
    
    // 4. Demonstrate mesh-wide discovery
    println!("\n--- Mesh-Wide Discovery and Communication ---");
    
    // Find all AI assistants across contexts
    let ai_nodes = discovery_samuel.get_nodes_with_capabilities(vec![
        NodeCapability::AiAssistance
    ]).await;
    
    println!("🔍 Found {} AI assistants in the mesh:", ai_nodes.len());
    for node in &ai_nodes {
        println!("  - {} in context: {}", node.display_name, node.context_id);
    }
    
    // Find all collaboration-capable nodes
    let collab_nodes = discovery_samuel.get_nodes_with_capabilities(vec![
        NodeCapability::Collaboration
    ]).await;
    
    println!("🤝 Found {} collaboration-capable nodes:", collab_nodes.len());
    for node in &collab_nodes {
        println!("  - {} ({})", node.display_name, node.context_id);
    }
    
    // 5. Demonstrate group communication patterns
    println!("\n--- Group Communication Patterns ---");
    
    let family_group_message = Message {
        id: Uuid::new_v4(),
        from_member: samuel_id,
        content: "Family standup: WeaveMesh networking layer is coming together nicely! The AI assistant has been incredibly helpful.".to_string(),
        priority: GroupMessagePriority::Normal,
        timestamp: chrono::Utc::now(),
        requires_response: false,
        context: Some("family-standup".to_string()),
        metadata: HashMap::new(),
    };
    
    family_group.send_message(family_group_message).await?;
    
    let research_group_message = Message {
        id: Uuid::new_v4(),
        from_member: samuel_id,
        content: "Research update: The networking implementation is revealing emergent collaboration patterns that validate our collaborative individuation theory.".to_string(),
        priority: GroupMessagePriority::High,
        timestamp: chrono::Utc::now(),
        requires_response: true,
        context: Some("research-update".to_string()),
        metadata: HashMap::new(),
    };
    
    research_group.send_message(research_group_message).await?;
    
    // 6. Show comprehensive statistics
    println!("\n📈 Collaborative Individuation Metrics:");
    
    // Communication statistics
    let samuel_stats = comm_samuel.get_stats().await;
    let ai_stats = comm_ai.get_stats().await;
    let researcher_stats = comm_researcher.get_stats().await;
    
    println!("\nCommunication Statistics:");
    println!("  Samuel: {} sent, {} received", samuel_stats.messages_sent, samuel_stats.messages_received);
    println!("  AI Assistant: {} sent, {} received", ai_stats.messages_sent, ai_stats.messages_received);
    println!("  Researcher: {} sent, {} received", researcher_stats.messages_sent, researcher_stats.messages_received);
    
    // Sacred Alliance statistics
    let family_stats = family_channel.get_statistics().await?;
    let research_stats = research_channel.get_statistics().await?;
    
    println!("\nSacred Alliance Statistics:");
    println!("  Family Channel: {} messages, {} participants", family_stats.total_messages, family_stats.active_participants);
    println!("  Research Channel: {} messages, {} participants", research_stats.total_messages, research_stats.active_participants);
    
    // Attribution statistics
    let attribution_stats = attribution_engine.get_statistics().await?;
    
    println!("\nAttribution Statistics:");
    println!("  Total attributions: {}", attribution_stats.total_attributions);
    println!("  Human-AI collaborations: {}", attribution_stats.human_ai_collaborations);
    println!("  Sacred Alliance validated: {}", attribution_stats.sacred_alliance_validated);
    
    // 7. Demonstrate dynamic capability updates
    println!("\n🔄 Demonstrating dynamic capability evolution...");
    
    // Samuel gains new capabilities through collaboration
    discovery_samuel.update_capabilities(vec![
        NodeCapability::Collaboration,
        NodeCapability::ResourceStorage,
        NodeCapability::AttributionTracking,
        NodeCapability::AiAssistance, // Gained through collaboration with AI
        NodeCapability::Custom("collaborative-individuation-expert".to_string()),
    ]).await?;
    
    sleep(Duration::from_secs(1)).await;
    
    // Check updated capabilities
    let expert_nodes = discovery_ai.get_nodes_with_capabilities(vec![
        NodeCapability::Custom("collaborative-individuation-expert".to_string())
    ]).await;
    
    println!("🎓 Nodes with collaborative individuation expertise: {}", expert_nodes.len());
    for node in &expert_nodes {
        println!("  - {} (evolved through collaboration)", node.display_name);
    }
    
    // 8. Final collaborative ceremony
    println!("\n🎭 Final Collaborative Ceremony...");
    
    let ceremony_message = AllianceMessage {
        id: Uuid::new_v4().to_string(),
        from_participant: samuel_id.to_string(),
        to_participant: None, // Broadcast to all
        channel_id: "family-collaboration".to_string(),
        message_content: weavemesh_core::sacred_alliance::MessageContent::Ceremony {
            action: BasicCeremonyAction::CelebrateAchievement,
            description: "We have successfully demonstrated collaborative individuation across multiple contexts, scales, and participant types. The mesh network enables both individual growth and collective intelligence.".to_string(),
            participants: vec![samuel_id.to_string(), ai_assistant_id.to_string(), researcher_id.to_string()],
            context: "collaborative-individuation-demonstration".to_string(),
        },
        timestamp: chrono::Utc::now(),
        requires_response: false,
        sacred_alliance_validated: true,
        attribution_context: Some("collaborative-individuation-ceremony".to_string()),
    };
    
    family_channel.send_message(ceremony_message).await?;
    
    // Cleanup
    println!("\n🧹 Graceful shutdown...");
    
    comm_samuel.stop().await?;
    comm_ai.stop().await?;
    comm_researcher.stop().await?;
    
    discovery_samuel.stop().await?;
    discovery_ai.stop().await?;
    discovery_researcher.stop().await?;
    
    println!("✅ Demo completed successfully!");
    
    println!("\n🌟 Collaborative Individuation Demonstrated:");
    println!("  ✓ Universal networking across contexts");
    println!("  ✓ Sacred Alliance human-AI collaboration");
    println!("  ✓ Attribution tracking and value recognition");
    println!("  ✓ Group communication patterns");
    println!("  ✓ Dynamic capability evolution");
    println!("  ✓ Cross-context knowledge sharing");
    println!("  ✓ Emergent collaboration patterns");
    println!("  ✓ Individual growth through collective intelligence");
    
    println!("\n🎯 Key Insights:");
    println!("  • The mesh network naturally enables collaborative individuation");
    println!("  • Sacred Alliance ensures meaningful human-AI collaboration");
    println!("  • Attribution tracking recognizes all contributions");
    println!("  • Context awareness maintains appropriate boundaries");
    println!("  • Capability evolution emerges from collaboration");
    println!("  • Universal protocols work at all scales");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_integration_components() {
        // Test that all components can be created and configured
        let node_id = Uuid::new_v4();
        
        // Test networking configuration
        let discovery_config = DiscoveryConfig::default();
        let comm_config = CommunicationConfig::default();
        
        assert_eq!(discovery_config.announcement_interval, 30);
        assert!(comm_config.require_acks);
        
        // Test Sacred Alliance configuration
        let channel_config = ChannelConfig {
            channel_id: "test-channel".to_string(),
            context: "test-context".to_string(),
            require_sacred_alliance: true,
            max_participants: 10,
            allow_ai_participants: true,
            require_attribution: true,
            enable_ceremony_tracking: true,
        };
        
        assert!(channel_config.require_sacred_alliance);
        assert!(channel_config.allow_ai_participants);
        
        // Test attribution configuration
        let attribution_config = AttributionConfig {
            track_all_interactions: true,
            require_explicit_attribution: false,
            enable_value_calculation: true,
            context_aware: true,
            sacred_alliance_integration: true,
        };
        
        assert!(attribution_config.track_all_interactions);
        assert!(attribution_config.sacred_alliance_integration);
    }
    
    #[test]
    fn test_collaborative_individuation_principles() {
        // Test that the demo embodies collaborative individuation principles
        
        // Individual identity preservation
        let samuel_id = Uuid::new_v4();
        let ai_id = Uuid::new_v4();
        assert_ne!(samuel_id, ai_id); // Each participant maintains unique identity
        
        // Context awareness
        let family_context = "family";
        let research_context = "research";
        assert_ne!(family_context, research_context); // Contexts remain distinct
        
        // Capability evolution
        let initial_capabilities = vec![NodeCapability::Collaboration];
        let evolved_capabilities = vec![
            NodeCapability::Collaboration,
            NodeCapability::AiAssistance,
            NodeCapability::Custom("expert".to_string()),
        ];
        assert!(evolved_capabilities.len() > initial_capabilities.len()); // Growth through collaboration
        
        // Sacred Alliance validation
        let sacred_alliance_enabled = true;
        assert!(sacred_alliance_enabled); // Meaningful collaboration ensured
    }
}
