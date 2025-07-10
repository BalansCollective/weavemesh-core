//! # Complete Core IDE Integration Demo
//!
//! This example demonstrates the full integration of all core IDE components
//! for collaborative individuation, showing how they work together to support
//! human-AI partnership in development workflows.

use anyhow::Result;
use chrono::Utc;
use std::path::PathBuf;
use std::time::Duration;
use tokio::time::sleep;
use uuid::Uuid;

use weavemesh_core::{
    attribution::{Attribution, CollaborationType},
    group_communication::{GroupId, Message, MessageType, UniversalNode},
    ide::{
        ceremony::{CoreCeremonyManager, CoreCeremonyType, CoreCeremonyOutcome},
        collaboration::{CoreCollaborationManager, CoreCollaborationSession, CoreSessionType},
        editor::{CoreEditorManager, CoreEditorSession, CoreEditOperation, CoreEditType},
        project::{CoreProjectManager, CoreProject, CoreProjectConfig, CoreCeremonyOutcome as ProjectCeremonyOutcome},
        security::{CoreSecurityManager, CoreClassification, CoreClearanceLevel},
        CoreIdeManager, SessionType,
    },
    sacred_alliance::{
        SacredAllianceManager, Participant, ParticipantType, PresenceStatus,
        CeremonyType, SacredAllianceLevel,
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    println!("🌟 Starting Complete Core IDE Integration Demo");
    println!("===============================================");
    
    // Initialize all core IDE components
    let mut ide_manager = CoreIdeManager::new().await?;
    let mut project_manager = CoreProjectManager::new();
    let mut collaboration_manager = CoreCollaborationManager::new().await?;
    let mut editor_manager = CoreEditorManager::new().await?;
    let mut ceremony_manager = CoreCeremonyManager::new().await?;
    let mut security_manager = CoreSecurityManager::new().await?;
    let mut sacred_alliance = SacredAllianceManager::new().await?;
    
    println!("✅ All core IDE components initialized");
    
    // Demo 1: Create a collaborative project with Sacred Alliance integration
    println!("\n🎯 Demo 1: Creating Collaborative Project");
    println!("==========================================");
    
    let project = create_collaborative_project(&mut project_manager).await?;
    println!("✅ Created project: {}", project.name);
    println!("   - Sacred Alliance enabled: {}", project.config.sacred_alliance.enabled);
    println!("   - Collaboration score: {:.2}", project.collaboration_metrics.collaboration_score);
    
    // Demo 2: Start IDE session with human-AI participants
    println!("\n🤝 Demo 2: Starting Human-AI Collaboration Session");
    println!("==================================================");
    
    let participants = create_participants().await?;
    let session_id = ide_manager.start_session(
        SessionType::PairProgramming,
        participants.clone(),
    ).await?;
    
    println!("✅ Started IDE session: {}", session_id);
    println!("   - Participants: {}", participants.len());
    println!("   - Sacred Alliance channel: {:?}", 
        ide_manager.get_session(&session_id).unwrap().alliance_channel);
    
    // Demo 3: Security clearance and classification
    println!("\n🔒 Demo 3: Security Framework Integration");
    println!("========================================");
    
    let security_context = demonstrate_security_framework(&mut security_manager).await?;
    println!("✅ Security context established");
    println!("   - Classification: {:?}", security_context.classification);
    println!("   - Required clearance: {:?}", security_context.required_clearance);
    
    // Demo 4: Collaborative editing with attribution
    println!("\n✏️  Demo 4: Collaborative Editing with Attribution");
    println!("=================================================");
    
    let edit_session = demonstrate_collaborative_editing(
        &mut editor_manager,
        &project,
        &participants,
    ).await?;
    
    println!("✅ Collaborative editing session completed");
    println!("   - Edit operations: {}", edit_session.operations.len());
    println!("   - Attribution tracking: enabled");
    
    // Demo 5: Sacred Alliance ceremony integration
    println!("\n🎭 Demo 5: Sacred Alliance Ceremony Integration");
    println!("==============================================");
    
    let ceremony_result = demonstrate_sacred_alliance_ceremony(
        &mut ceremony_manager,
        &mut project_manager,
        &project.id,
        &participants,
    ).await?;
    
    println!("✅ Sacred Alliance ceremony completed");
    println!("   - Ceremony type: {:?}", ceremony_result.ceremony_type);
    println!("   - Outcome: {:?}", ceremony_result.outcome);
    println!("   - Collaboration impact: {:.2}", ceremony_result.collaboration_impact);
    
    // Demo 6: Real-time collaboration with group communication
    println!("\n📡 Demo 6: Real-time Group Communication");
    println!("========================================");
    
    let collaboration_session = demonstrate_group_communication(
        &mut collaboration_manager,
        &participants,
    ).await?;
    
    println!("✅ Group communication session completed");
    println!("   - Messages exchanged: {}", collaboration_session.message_count);
    println!("   - Collaboration patterns detected: {}", collaboration_session.patterns.len());
    
    // Demo 7: Project collaboration metrics and analytics
    println!("\n📊 Demo 7: Collaboration Analytics");
    println!("==================================");
    
    let analytics = demonstrate_collaboration_analytics(
        &project_manager,
        &project.id,
    ).await?;
    
    println!("✅ Collaboration analytics generated");
    println!("   - Overall collaboration score: {:.2}", analytics.collaboration_score);
    println!("   - Sacred Alliance level: {:.2}", analytics.sacred_alliance_level);
    println!("   - Recent ceremonies: {}", analytics.recent_ceremony_count);
    
    // Demo 8: Complete workflow integration
    println!("\n🔄 Demo 8: Complete Workflow Integration");
    println!("=======================================");
    
    demonstrate_complete_workflow(
        &mut ide_manager,
        &mut project_manager,
        &mut collaboration_manager,
        &mut editor_manager,
        &mut ceremony_manager,
        &project.id,
        &session_id,
        &participants,
    ).await?;
    
    println!("✅ Complete workflow integration demonstrated");
    
    // Cleanup
    println!("\n🧹 Cleaning up demo resources");
    println!("=============================");
    
    ide_manager.end_session(session_id).await?;
    println!("✅ IDE session ended");
    
    println!("\n🎉 Complete Core IDE Integration Demo Completed Successfully!");
    println!("============================================================");
    println!("All core IDE components demonstrated collaborative individuation");
    println!("patterns with Sacred Alliance integration and human-AI partnership.");
    
    Ok(())
}

/// Create a collaborative project with Sacred Alliance integration
async fn create_collaborative_project(
    project_manager: &mut CoreProjectManager,
) -> Result<CoreProject> {
    let temp_dir = std::env::temp_dir().join("collaborative_demo_project");
    std::fs::create_dir_all(&temp_dir)?;
    
    // Create project configuration with collaborative individuation settings
    let mut config = CoreProjectConfig::default();
    config.languages = vec!["Rust".to_string(), "TypeScript".to_string()];
    config.sacred_alliance.enabled = true;
    config.sacred_alliance.individuation_tracking = true;
    config.collaboration.human_ai_partnership.partnership_required = true;
    config.collaboration.human_ai_partnership.minimum_collaboration_score = 0.7;
    
    let project = project_manager.create_project(
        "Collaborative Individuation Demo".to_string(),
        "A demonstration project for human-AI collaborative individuation".to_string(),
        temp_dir,
        Some(config),
    )?;
    
    Ok(project)
}

/// Create participants for the collaboration session
async fn create_participants() -> Result<Vec<Participant>> {
    let participants = vec![
        Participant {
            id: "human-developer".to_string(),
            participant_type: ParticipantType::Human,
            presence: PresenceStatus::Active,
            capabilities: vec![
                "creative_thinking".to_string(),
                "domain_expertise".to_string(),
                "intuitive_problem_solving".to_string(),
            ],
            joined_at: Utc::now(),
        },
        Participant {
            id: "ai-assistant".to_string(),
            participant_type: ParticipantType::AI,
            presence: PresenceStatus::Active,
            capabilities: vec![
                "code_generation".to_string(),
                "pattern_recognition".to_string(),
                "systematic_analysis".to_string(),
                "documentation_generation".to_string(),
            ],
            joined_at: Utc::now(),
        },
    ];
    
    Ok(participants)
}

/// Demonstrate security framework integration
async fn demonstrate_security_framework(
    security_manager: &mut CoreSecurityManager,
) -> Result<SecurityContext> {
    // Create security context for the collaboration
    let context = SecurityContext {
        classification: CoreClassification::Internal,
        required_clearance: CoreClearanceLevel::TeamMember,
        content_filtering_enabled: true,
        auto_classification_enabled: true,
    };
    
    // Validate security requirements
    security_manager.validate_access(
        "human-developer",
        CoreClearanceLevel::TeamMember,
        CoreClassification::Internal,
    ).await?;
    
    security_manager.validate_access(
        "ai-assistant",
        CoreClearanceLevel::TeamMember,
        CoreClassification::Internal,
    ).await?;
    
    Ok(context)
}

/// Demonstrate collaborative editing with attribution
async fn demonstrate_collaborative_editing(
    editor_manager: &mut CoreEditorManager,
    project: &CoreProject,
    participants: &[Participant],
) -> Result<EditSessionResult> {
    // Start editor session
    let session_id = editor_manager.start_session(
        project.root_path.join("src/main.rs"),
        participants.clone(),
    ).await?;
    
    // Simulate collaborative editing operations
    let operations = vec![
        CoreEditOperation {
            id: Uuid::new_v4(),
            edit_type: CoreEditType::Insert,
            position: 0,
            content: "// Collaborative individuation demo\n".to_string(),
            attribution: Attribution::new(
                Some("human-developer".to_string()),
                None,
                CollaborationType::HumanLed,
                0.9,
            ),
            timestamp: Utc::now(),
        },
        CoreEditOperation {
            id: Uuid::new_v4(),
            edit_type: CoreEditType::Insert,
            position: 35,
            content: "use std::collections::HashMap;\n\n".to_string(),
            attribution: Attribution::new(
                Some("human-developer".to_string()),
                Some("ai-assistant".to_string()),
                CollaborationType::CoCreated,
                0.8,
            ),
            timestamp: Utc::now(),
        },
        CoreEditOperation {
            id: Uuid::new_v4(),
            edit_type: CoreEditType::Insert,
            position: 70,
            content: "fn main() {\n    println!(\"Hello, collaborative world!\");\n}\n".to_string(),
            attribution: Attribution::new(
                None,
                Some("ai-assistant".to_string()),
                CollaborationType::AiLed,
                0.7,
            ),
            timestamp: Utc::now(),
        },
    ];
    
    // Apply operations
    for operation in &operations {
        editor_manager.apply_operation(session_id, operation.clone()).await?;
    }
    
    // End session
    editor_manager.end_session(session_id).await?;
    
    Ok(EditSessionResult {
        operations: operations.clone(),
        attribution_tracking: true,
    })
}

/// Demonstrate Sacred Alliance ceremony integration
async fn demonstrate_sacred_alliance_ceremony(
    ceremony_manager: &mut CoreCeremonyManager,
    project_manager: &mut CoreProjectManager,
    project_id: &Uuid,
    participants: &[Participant],
) -> Result<CeremonyResult> {
    // Initiate a commit ceremony
    let ceremony_id = ceremony_manager.initiate_ceremony(
        CoreCeremonyType::Commit,
        participants.iter().map(|p| p.id.clone()).collect(),
        Some("Collaborative editing session completed".to_string()),
    ).await?;
    
    // Simulate ceremony progression
    sleep(Duration::from_millis(100)).await;
    
    // Complete ceremony
    let outcome = ceremony_manager.complete_ceremony(
        ceremony_id,
        CoreCeremonyOutcome::Successful,
        0.8, // collaboration impact
    ).await?;
    
    // Record ceremony in project
    let project_ceremony_id = project_manager.record_ceremony(
        project_id,
        CeremonyType::Commit,
        participants.iter().map(|p| p.id.clone()).collect(),
        ProjectCeremonyOutcome::Successful,
        0.8,
    )?;
    
    Ok(CeremonyResult {
        ceremony_type: CoreCeremonyType::Commit,
        outcome: CoreCeremonyOutcome::Successful,
        collaboration_impact: 0.8,
        project_ceremony_id,
    })
}

/// Demonstrate group communication
async fn demonstrate_group_communication(
    collaboration_manager: &mut CoreCollaborationManager,
    participants: &[Participant],
) -> Result<GroupCommunicationResult> {
    // Start collaboration session
    let session_id = collaboration_manager.start_session(
        CoreSessionType::PairProgramming,
        participants.clone(),
    ).await?;
    
    // Simulate message exchange
    let messages = vec![
        Message {
            id: Uuid::new_v4(),
            sender: "human-developer".to_string(),
            content: "Let's implement the collaborative editing feature".to_string(),
            message_type: MessageType::Text,
            timestamp: Utc::now(),
            metadata: std::collections::HashMap::new(),
        },
        Message {
            id: Uuid::new_v4(),
            sender: "ai-assistant".to_string(),
            content: "I can help with the attribution tracking implementation".to_string(),
            message_type: MessageType::Text,
            timestamp: Utc::now(),
            metadata: std::collections::HashMap::new(),
        },
        Message {
            id: Uuid::new_v4(),
            sender: "human-developer".to_string(),
            content: "Great! Let's start with the core data structures".to_string(),
            message_type: MessageType::Text,
            timestamp: Utc::now(),
            metadata: std::collections::HashMap::new(),
        },
    ];
    
    // Send messages
    for message in &messages {
        collaboration_manager.send_message(session_id, message.clone()).await?;
    }
    
    // Detect collaboration patterns
    let patterns = collaboration_manager.detect_patterns(session_id).await?;
    
    // End session
    collaboration_manager.end_session(session_id).await?;
    
    Ok(GroupCommunicationResult {
        message_count: messages.len(),
        patterns,
    })
}

/// Demonstrate collaboration analytics
async fn demonstrate_collaboration_analytics(
    project_manager: &CoreProjectManager,
    project_id: &Uuid,
) -> Result<CollaborationAnalytics> {
    let status = project_manager.get_collaboration_status(project_id)
        .ok_or_else(|| anyhow::anyhow!("Project not found"))?;
    
    Ok(CollaborationAnalytics {
        collaboration_score: status.collaboration_score,
        partnership_balance: status.partnership_balance,
        attribution_transparency: status.attribution_transparency,
        sacred_alliance_level: status.sacred_alliance_level,
        active_contributors: status.active_contributors,
        recent_ceremony_count: status.recent_ceremony_count,
        last_activity: status.last_activity,
        active_goals: status.active_goals,
    })
}

/// Demonstrate complete workflow integration
async fn demonstrate_complete_workflow(
    ide_manager: &mut CoreIdeManager,
    project_manager: &mut CoreProjectManager,
    collaboration_manager: &mut CoreCollaborationManager,
    editor_manager: &mut CoreEditorManager,
    ceremony_manager: &mut CoreCeremonyManager,
    project_id: &Uuid,
    session_id: &Uuid,
    participants: &[Participant],
) -> Result<()> {
    println!("   🔄 Integrating all components in unified workflow...");
    
    // 1. Update project collaboration metrics based on session activity
    let attribution = Attribution::new(
        Some("human-developer".to_string()),
        Some("ai-assistant".to_string()),
        CollaborationType::CoCreated,
        0.85,
    );
    
    project_manager.update_collaboration_metrics(project_id, &attribution)?;
    
    // 2. Trigger automatic ceremony based on collaboration milestone
    let milestone_ceremony = ceremony_manager.initiate_ceremony(
        CoreCeremonyType::Milestone,
        participants.iter().map(|p| p.id.clone()).collect(),
        Some("Workflow integration milestone reached".to_string()),
    ).await?;
    
    // 3. Complete ceremony with positive outcome
    ceremony_manager.complete_ceremony(
        milestone_ceremony,
        CoreCeremonyOutcome::Successful,
        0.9,
    ).await?;
    
    // 4. Record ceremony impact on project
    project_manager.record_ceremony(
        project_id,
        CeremonyType::Milestone,
        participants.iter().map(|p| p.id.clone()).collect(),
        ProjectCeremonyOutcome::Successful,
        0.9,
    )?;
    
    println!("   ✅ Workflow integration completed successfully");
    
    Ok(())
}

// Supporting types for demo results

#[derive(Debug)]
struct SecurityContext {
    classification: CoreClassification,
    required_clearance: CoreClearanceLevel,
    content_filtering_enabled: bool,
    auto_classification_enabled: bool,
}

#[derive(Debug)]
struct EditSessionResult {
    operations: Vec<CoreEditOperation>,
    attribution_tracking: bool,
}

#[derive(Debug)]
struct CeremonyResult {
    ceremony_type: CoreCeremonyType,
    outcome: CoreCeremonyOutcome,
    collaboration_impact: f64,
    project_ceremony_id: Uuid,
}

#[derive(Debug)]
struct GroupCommunicationResult {
    message_count: usize,
    patterns: Vec<String>,
}

#[derive(Debug)]
struct CollaborationAnalytics {
    collaboration_score: f64,
    partnership_balance: f64,
    attribution_transparency: f64,
    sacred_alliance_level: f64,
    active_contributors: usize,
    recent_ceremony_count: usize,
    last_activity: chrono::DateTime<Utc>,
    active_goals: usize,
}
