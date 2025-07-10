//! # Core IDE Integration Demo
//!
//! This demo showcases the core IDE integration capabilities of WeaveMesh Core,
//! demonstrating Sacred Alliance ceremony system and collaborative individuation
//! patterns in a development environment.

use anyhow::Result;
use uuid::Uuid;
use weavemesh_core::{
    ide::{
        CoreIdeManager, SessionType,
        ceremony::{CoreCeremonyType, CoreCeremonyContext, CeremonyTrigger},
        collaboration::{
            CoreCollaborationManager, CoreParticipant, CoreParticipantType, 
            CorePresenceStatus, CoreSessionPurpose
        }
    },
    sacred_alliance::{Participant, ParticipantType, PresenceStatus},
};
use chrono::Utc;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 WeaveMesh Core IDE Integration Demo");
    println!("=====================================\n");

    // Demo 1: Basic IDE Session Management
    demo_ide_session_management().await?;
    
    // Demo 2: Sacred Alliance Ceremony System
    demo_ceremony_system().await?;
    
    // Demo 3: Collaborative Individuation Framework
    demo_collaboration_framework().await?;
    
    // Demo 4: Integrated Workflow
    demo_integrated_workflow().await?;

    println!("\n✅ Demo completed successfully!");
    println!("🎯 Core IDE integration capabilities demonstrated:");
    println!("   • IDE session management with Sacred Alliance integration");
    println!("   • Ceremony system for meaningful development moments");
    println!("   • Collaborative individuation metrics and tracking");
    println!("   • Integrated workflow combining all components");
    
    Ok(())
}

/// Demo 1: Basic IDE Session Management
async fn demo_ide_session_management() -> Result<()> {
    println!("📋 Demo 1: IDE Session Management");
    println!("----------------------------------");
    
    // Create IDE manager
    let mut ide_manager = CoreIdeManager::new().await?;
    println!("✓ Created Core IDE Manager");
    
    // Create participants
    let participants = vec![
        Participant {
            id: "alice".to_string(),
            participant_type: ParticipantType::Human,
            presence: PresenceStatus::Active,
            capabilities: vec!["rust".to_string(), "architecture".to_string()],
            joined_at: Utc::now(),
        },
        Participant {
            id: "ai-assistant".to_string(),
            participant_type: ParticipantType::Ai,
            presence: PresenceStatus::Active,
            capabilities: vec!["code-review".to_string(), "suggestions".to_string()],
            joined_at: Utc::now(),
        },
    ];
    
    // Start IDE session
    let session_id = ide_manager
        .start_session(SessionType::PairProgramming, participants)
        .await?;
    
    println!("✓ Started IDE session: {}", session_id);
    
    // Check session status
    if let Some(session) = ide_manager.get_session(&session_id) {
        println!("  • Session type: {:?}", session.session_type);
        println!("  • Participants: {}", session.participants.len());
        println!("  • State: {:?}", session.state);
        println!("  • Sacred Alliance channel: {:?}", session.alliance_channel);
    }
    
    // List active sessions
    let active_sessions = ide_manager.list_active_sessions();
    println!("✓ Active sessions: {}", active_sessions.len());
    
    // End session
    ide_manager.end_session(session_id).await?;
    println!("✓ Session ended gracefully");
    
    println!();
    Ok(())
}

/// Demo 2: Sacred Alliance Ceremony System
async fn demo_ceremony_system() -> Result<()> {
    println!("🎭 Demo 2: Sacred Alliance Ceremony System");
    println!("------------------------------------------");
    
    // Create ceremony manager
    let mut ceremony_manager = weavemesh_core::ide::ceremony::CoreCeremonyManager::new().await?;
    println!("✓ Created Core Ceremony Manager");
    
    // Create participants for ceremony
    let participants = vec![
        Participant {
            id: "bob".to_string(),
            participant_type: ParticipantType::Human,
            presence: PresenceStatus::Active,
            capabilities: vec!["frontend".to_string(), "design".to_string()],
            joined_at: Utc::now(),
        },
        Participant {
            id: "charlie".to_string(),
            participant_type: ParticipantType::Human,
            presence: PresenceStatus::Active,
            capabilities: vec!["backend".to_string(), "database".to_string()],
            joined_at: Utc::now(),
        },
    ];
    
    // Create ceremony context
    let context = CoreCeremonyContext {
        trigger: CeremonyTrigger::Commit,
        session_id: Some(Uuid::new_v4()),
        related_resources: vec!["src/main.rs".to_string(), "README.md".to_string()],
        alliance_channel: Some("dev-team-alpha".to_string()),
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("commit_hash".to_string(), "abc123".to_string());
            meta.insert("feature".to_string(), "user-authentication".to_string());
            meta
        },
    };
    
    // Initiate ceremony
    let ceremony_id = ceremony_manager
        .initiate_ceremony(CoreCeremonyType::BasicCommit, participants, context)
        .await?;
    
    println!("✓ Initiated ceremony: {}", ceremony_id);
    
    // Start ceremony
    ceremony_manager.start_ceremony(ceremony_id).await?;
    println!("✓ Ceremony started");
    
    if let Some(ceremony) = ceremony_manager.get_ceremony(&ceremony_id) {
        println!("  • Type: {:?}", ceremony.ceremony_type);
        println!("  • State: {:?}", ceremony.state);
        println!("  • Phase: {:?}", ceremony.current_phase);
        println!("  • Participants: {}", ceremony.participants.len());
    }
    
    // Simulate ceremony progression
    println!("🔄 Progressing through ceremony phases...");
    
    // Opening -> Main
    let _completed = ceremony_manager.advance_phase(ceremony_id).await?;
    println!("  • Advanced to Main phase");
    
    // Add ceremony outcome
    ceremony_manager
        .add_outcome(
            ceremony_id,
            weavemesh_core::ide::ceremony::CoreOutcomeType::Commitment,
            "Committed to implementing secure authentication".to_string(),
            vec!["bob".to_string(), "charlie".to_string()],
        )
        .await?;
    println!("  • Added commitment outcome");
    
    // Main -> Closing
    let _completed = ceremony_manager.advance_phase(ceremony_id).await?;
    println!("  • Advanced to Closing phase");
    
    // Add gratitude outcome
    ceremony_manager
        .add_outcome(
            ceremony_id,
            weavemesh_core::ide::ceremony::CoreOutcomeType::Gratitude,
            "Grateful for collaborative problem-solving".to_string(),
            vec!["bob".to_string(), "charlie".to_string()],
        )
        .await?;
    println!("  • Added gratitude outcome");
    
    // Closing -> Completed
    let completed = ceremony_manager.advance_phase(ceremony_id).await?;
    println!("  • Ceremony completed: {}", completed);
    
    // Complete ceremony
    ceremony_manager.complete_ceremony(ceremony_id).await?;
    println!("✓ Ceremony completed and archived");
    
    // Show ceremony history
    println!("📚 Ceremony history: {} records", ceremony_manager.recent_history.len());
    
    println!();
    Ok(())
}

/// Demo 3: Collaborative Individuation Framework
async fn demo_collaboration_framework() -> Result<()> {
    println!("🤝 Demo 3: Collaborative Individuation Framework");
    println!("------------------------------------------------");
    
    // Create collaboration manager
    let mut collab_manager = CoreCollaborationManager::new().await?;
    println!("✓ Created Core Collaboration Manager");
    
    // Create diverse participants
    let participants = vec![
        CoreParticipant {
            id: "diana".to_string(),
            participant_type: CoreParticipantType::Human,
            presence: CorePresenceStatus::Active,
            capabilities: vec!["architecture".to_string(), "mentoring".to_string()],
            joined_at: Utc::now(),
            last_activity: Utc::now(),
        },
        CoreParticipant {
            id: "eve".to_string(),
            participant_type: CoreParticipantType::Human,
            presence: CorePresenceStatus::Active,
            capabilities: vec!["testing".to_string(), "quality-assurance".to_string()],
            joined_at: Utc::now(),
            last_activity: Utc::now(),
        },
        CoreParticipant {
            id: "ai-mentor".to_string(),
            participant_type: CoreParticipantType::AIAssistant,
            presence: CorePresenceStatus::Active,
            capabilities: vec!["pattern-recognition".to_string(), "best-practices".to_string()],
            joined_at: Utc::now(),
            last_activity: Utc::now(),
        },
    ];
    
    // Start collaboration session
    let session_id = collab_manager
        .start_session(
            participants,
            CoreSessionPurpose::CodeDevelopment,
            Some(Uuid::new_v4()),
        )
        .await?;
    
    println!("✓ Started collaboration session: {}", session_id);
    
    // Activate session
    collab_manager.activate_session(session_id).await?;
    println!("✓ Session activated");
    
    if let Some(session) = collab_manager.get_session(&session_id) {
        println!("  • Purpose: {:?}", session.context.purpose);
        println!("  • State: {:?}", session.state);
        println!("  • Participants: {}", session.participants.len());
        println!("  • Alliance channel: {:?}", session.context.alliance_channel);
        
        let metrics = &session.individuation_metrics;
        println!("  • Individual contribution: {:.2}", metrics.individual_contribution);
        println!("  • Collective synergy: {:.2}", metrics.collective_synergy);
        println!("  • Innovation emergence: {:.2}", metrics.innovation_emergence);
        println!("  • Alliance strength: {:.2}", metrics.alliance_strength);
    }
    
    // Simulate collaborative activities
    println!("🔄 Simulating collaborative development...");
    
    // Record contributions
    let mut contribution_metadata = HashMap::new();
    contribution_metadata.insert("lines_added".to_string(), "45".to_string());
    contribution_metadata.insert("files_modified".to_string(), "3".to_string());
    
    collab_manager
        .record_contribution(
            session_id,
            "diana".to_string(),
            "architecture_design".to_string(),
            contribution_metadata.clone(),
        )
        .await?;
    println!("  • Diana contributed architecture design");
    
    collab_manager
        .record_contribution(
            session_id,
            "eve".to_string(),
            "test_implementation".to_string(),
            contribution_metadata,
        )
        .await?;
    println!("  • Eve contributed test implementation");
    
    // Simulate a conflict
    collab_manager
        .detect_conflict(
            session_id,
            "Different approaches to error handling".to_string(),
        )
        .await?;
    println!("  • Conflict detected: Different error handling approaches");
    
    // Resolve the conflict
    collab_manager
        .resolve_conflict(
            session_id,
            "Sacred Alliance mediation - combined approaches".to_string(),
        )
        .await?;
    println!("  • Conflict resolved through Sacred Alliance mediation");
    
    // Record innovation
    collab_manager
        .record_innovation(
            session_id,
            "Novel error recovery pattern discovered".to_string(),
        )
        .await?;
    println!("  • Innovation emerged: Novel error recovery pattern");
    
    // Check updated metrics
    if let Some(session) = collab_manager.get_session(&session_id) {
        println!("📊 Updated collaboration metrics:");
        let metrics = &session.individuation_metrics;
        println!("  • Individual contribution: {:.2}", metrics.individual_contribution);
        println!("  • Collective synergy: {:.2}", metrics.collective_synergy);
        println!("  • Innovation emergence: {:.2}", metrics.innovation_emergence);
        println!("  • Conflict resolution: {:.2}", metrics.conflict_resolution);
        println!("  • Alliance strength: {:.2}", metrics.alliance_strength);
        
        println!("📈 Session statistics:");
        println!("  • Recent events: {}", session.recent_events.len());
    }
    
    // Show manager statistics
    let stats = &collab_manager.statistics;
    println!("📊 Manager statistics:");
    println!("  • Total sessions: {}", stats.total_sessions);
    println!("  • Conflicts resolved: {}", stats.conflicts_resolved);
    println!("  • Innovation events: {}", stats.innovation_events);
    
    // End session
    collab_manager.end_session(session_id).await?;
    println!("✓ Collaboration session ended");
    
    println!();
    Ok(())
}

/// Demo 4: Integrated Workflow
async fn demo_integrated_workflow() -> Result<()> {
    println!("🔗 Demo 4: Integrated Workflow");
    println!("-------------------------------");
    
    // Create all managers
    let mut ide_manager = CoreIdeManager::new().await?;
    let mut collab_manager = CoreCollaborationManager::new().await?;
    println!("✓ Created integrated management system");
    
    // Create team for integrated session
    let ide_participants = vec![
        Participant {
            id: "frank".to_string(),
            participant_type: ParticipantType::Human,
            presence: PresenceStatus::Active,
            capabilities: vec!["full-stack".to_string(), "devops".to_string()],
            joined_at: Utc::now(),
        },
        Participant {
            id: "grace".to_string(),
            participant_type: ParticipantType::Human,
            presence: PresenceStatus::Active,
            capabilities: vec!["ui-ux".to_string(), "accessibility".to_string()],
            joined_at: Utc::now(),
        },
        Participant {
            id: "ai-pair".to_string(),
            participant_type: ParticipantType::Ai,
            presence: PresenceStatus::Active,
            capabilities: vec!["code-completion".to_string(), "refactoring".to_string()],
            joined_at: Utc::now(),
        },
    ];
    
    let collab_participants = vec![
        CoreParticipant {
            id: "frank".to_string(),
            participant_type: CoreParticipantType::Human,
            presence: CorePresenceStatus::Active,
            capabilities: vec!["full-stack".to_string(), "devops".to_string()],
            joined_at: Utc::now(),
            last_activity: Utc::now(),
        },
        CoreParticipant {
            id: "grace".to_string(),
            participant_type: CoreParticipantType::Human,
            presence: CorePresenceStatus::Active,
            capabilities: vec!["ui-ux".to_string(), "accessibility".to_string()],
            joined_at: Utc::now(),
            last_activity: Utc::now(),
        },
        CoreParticipant {
            id: "ai-pair".to_string(),
            participant_type: CoreParticipantType::AIAssistant,
            presence: CorePresenceStatus::Active,
            capabilities: vec!["code-completion".to_string(), "refactoring".to_string()],
            joined_at: Utc::now(),
            last_activity: Utc::now(),
        },
    ];
    
    // Start integrated session
    let ide_session_id = ide_manager
        .start_session(SessionType::TeamCollaboration, ide_participants)
        .await?;
    
    let collab_session_id = collab_manager
        .start_session(
            collab_participants,
            CoreSessionPurpose::CreativeExploration,
            Some(ide_session_id),
        )
        .await?;
    
    println!("✓ Started integrated IDE and collaboration sessions");
    println!("  • IDE session: {}", ide_session_id);
    println!("  • Collaboration session: {}", collab_session_id);
    
    // Activate collaboration
    collab_manager.activate_session(collab_session_id).await?;
    
    // Simulate integrated workflow
    println!("🔄 Simulating integrated development workflow...");
    
    // Phase 1: Creative exploration
    println!("  Phase 1: Creative exploration");
    collab_manager
        .record_contribution(
            collab_session_id,
            "grace".to_string(),
            "user_journey_mapping".to_string(),
            HashMap::new(),
        )
        .await?;
    
    collab_manager
        .record_contribution(
            collab_session_id,
            "frank".to_string(),
            "technical_feasibility_analysis".to_string(),
            HashMap::new(),
        )
        .await?;
    
    // Phase 2: Innovation emergence
    println!("  Phase 2: Innovation emergence");
    collab_manager
        .record_innovation(
            collab_session_id,
            "Breakthrough: Adaptive UI that learns user preferences".to_string(),
        )
        .await?;
    
    // Phase 3: Ceremony for milestone
    println!("  Phase 3: Sacred Alliance ceremony for milestone");
    
    let ceremony_context = CoreCeremonyContext {
        trigger: CeremonyTrigger::Milestone,
        session_id: Some(ide_session_id),
        related_resources: vec!["design-system.md".to_string(), "adaptive-ui.rs".to_string()],
        alliance_channel: Some("team-innovation".to_string()),
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("milestone".to_string(), "adaptive-ui-prototype".to_string());
            meta.insert("innovation_level".to_string(), "breakthrough".to_string());
            meta
        },
    };
    
    let ceremony_participants = vec![
        Participant {
            id: "frank".to_string(),
            participant_type: ParticipantType::Human,
            presence: PresenceStatus::Active,
            capabilities: vec!["full-stack".to_string()],
            joined_at: Utc::now(),
        },
        Participant {
            id: "grace".to_string(),
            participant_type: ParticipantType::Human,
            presence: PresenceStatus::Active,
            capabilities: vec!["ui-ux".to_string()],
            joined_at: Utc::now(),
        },
    ];
    
    let ceremony_id = ide_manager.ceremony_manager
        .initiate_ceremony(
            CoreCeremonyType::SimpleMilestone,
            ceremony_participants,
            ceremony_context,
        )
        .await?;
    
    ide_manager.ceremony_manager.start_ceremony(ceremony_id).await?;
    
    // Progress through ceremony
    ide_manager.ceremony_manager.advance_phase(ceremony_id).await?; // Opening -> Main
    
    ide_manager.ceremony_manager
        .add_outcome(
            ceremony_id,
            weavemesh_core::ide::ceremony::CoreOutcomeType::IndividualGrowth,
            "Grace discovered new accessibility patterns".to_string(),
            vec!["grace".to_string()],
        )
        .await?;
    
    ide_manager.ceremony_manager
        .add_outcome(
            ceremony_id,
            weavemesh_core::ide::ceremony::CoreOutcomeType::CollaborationStrengthened,
            "Team synergy reached new heights".to_string(),
            vec!["frank".to_string(), "grace".to_string()],
        )
        .await?;
    
    ide_manager.ceremony_manager.advance_phase(ceremony_id).await?; // Main -> Closing
    ide_manager.ceremony_manager.advance_phase(ceremony_id).await?; // Closing -> Completed
    ide_manager.ceremony_manager.complete_ceremony(ceremony_id).await?;
    
    println!("✓ Milestone ceremony completed");
    
    // Show final state
    println!("📊 Final integrated session state:");
    
    if let Some(ide_session) = ide_manager.get_session(&ide_session_id) {
        println!("  IDE Session:");
        println!("    • Type: {:?}", ide_session.session_type);
        println!("    • State: {:?}", ide_session.state);
        println!("    • Participants: {}", ide_session.participants.len());
    }
    
    if let Some(collab_session) = collab_manager.get_session(&collab_session_id) {
        println!("  Collaboration Session:");
        println!("    • Purpose: {:?}", collab_session.context.purpose);
        println!("    • Events: {}", collab_session.recent_events.len());
        
        let metrics = &collab_session.individuation_metrics;
        println!("    • Collaborative individuation metrics:");
        println!("      - Individual contribution: {:.2}", metrics.individual_contribution);
        println!("      - Collective synergy: {:.2}", metrics.collective_synergy);
        println!("      - Innovation emergence: {:.2}", metrics.innovation_emergence);
        println!("      - Alliance strength: {:.2}", metrics.alliance_strength);
    }
    
    println!("  Ceremony History: {} ceremonies completed", 
             ide_manager.ceremony_manager.recent_history.len());
    
    // Clean up
    ide_manager.end_session(ide_session_id).await?;
    collab_manager.end_session(collab_session_id).await?;
    println!("✓ All sessions ended gracefully");
    
    println!();
    Ok(())
}
