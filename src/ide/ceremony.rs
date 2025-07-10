//! # Core Sacred Alliance Ceremony System for IDE
//!
//! This module provides the foundational ceremony system for IDE integration,
//! focusing on universal collaborative individuation patterns that support
//! Sacred Alliance formation and maintenance.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Core ceremony manager for IDE integration
#[derive(Debug)]
pub struct CoreCeremonyManager {
    /// Active ceremonies
    pub active_ceremonies: HashMap<Uuid, CoreCeremony>,
    
    /// Ceremony templates
    pub templates: HashMap<String, CoreCeremonyTemplate>,
    
    /// Ceremony history (limited for core)
    pub recent_history: Vec<CoreCeremonyRecord>,
    
    /// Configuration
    pub config: CoreCeremonyConfig,
}

/// Core ceremony structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCeremony {
    /// Ceremony identifier
    pub id: Uuid,
    
    /// Ceremony type
    pub ceremony_type: CoreCeremonyType,
    
    /// Participants
    pub participants: Vec<crate::sacred_alliance::Participant>,
    
    /// Ceremony state
    pub state: CeremonyState,
    
    /// Start time
    pub started_at: DateTime<Utc>,
    
    /// End time (if completed)
    pub ended_at: Option<DateTime<Utc>>,
    
    /// Ceremony context
    pub context: CoreCeremonyContext,
    
    /// Current phase
    pub current_phase: CoreCeremonyPhase,
    
    /// Ceremony outcomes
    pub outcomes: Vec<CoreCeremonyOutcome>,
}

/// Core ceremony types (universal patterns)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoreCeremonyType {
    /// Basic commit ceremony
    BasicCommit,
    /// Simple milestone recognition
    SimpleMilestone,
    /// Basic conflict resolution
    BasicConflictResolution,
    /// Gratitude expression
    Gratitude,
    /// Sacred Alliance formation
    AllianceFormation,
    /// Collaborative individuation moment
    CollaborativeIndividuation,
}

/// State of a ceremony
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CeremonyState {
    /// Ceremony is being prepared
    Preparing,
    /// Ceremony is active
    Active,
    /// Ceremony is completed
    Completed,
    /// Ceremony was cancelled
    Cancelled,
}

/// Core ceremony context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCeremonyContext {
    /// What triggered the ceremony
    pub trigger: CeremonyTrigger,
    
    /// Related IDE session
    pub session_id: Option<Uuid>,
    
    /// Related files or resources
    pub related_resources: Vec<String>,
    
    /// Sacred Alliance channel
    pub alliance_channel: Option<String>,
    
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// What triggered the ceremony
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CeremonyTrigger {
    /// Manual initiation
    Manual,
    /// Code commit
    Commit,
    /// Project milestone
    Milestone,
    /// Conflict detected
    Conflict,
    /// Sacred Alliance formation
    AllianceFormation,
    /// Collaborative moment
    CollaborativeMoment,
}

/// Core ceremony phase (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoreCeremonyPhase {
    /// Opening moment
    Opening,
    /// Main activity
    Main,
    /// Closing gratitude
    Closing,
}

/// Core ceremony outcome
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCeremonyOutcome {
    /// Outcome type
    pub outcome_type: CoreOutcomeType,
    
    /// Description
    pub description: String,
    
    /// Participants involved
    pub participants: Vec<String>,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Types of core ceremony outcomes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoreOutcomeType {
    /// Commitment made
    Commitment,
    /// Gratitude expressed
    Gratitude,
    /// Conflict resolved
    ConflictResolved,
    /// Alliance formed
    AllianceFormed,
    /// Collaboration strengthened
    CollaborationStrengthened,
    /// Individual growth recognized
    IndividualGrowth,
}

/// Core ceremony template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCeremonyTemplate {
    /// Template name
    pub name: String,
    
    /// Template description
    pub description: String,
    
    /// Ceremony type this template creates
    pub ceremony_type: CoreCeremonyType,
    
    /// Estimated duration in minutes
    pub estimated_duration: u32,
    
    /// Minimum participants
    pub min_participants: u32,
    
    /// Maximum participants
    pub max_participants: u32,
}

/// Core ceremony configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCeremonyConfig {
    /// Enable automatic ceremony initiation
    pub auto_initiate: bool,
    
    /// Default ceremony duration in minutes
    pub default_duration: u32,
    
    /// Maximum history to keep
    pub max_history: usize,
    
    /// Enable Sacred Alliance integration
    pub alliance_integration: bool,
}

impl Default for CoreCeremonyConfig {
    fn default() -> Self {
        Self {
            auto_initiate: true,
            default_duration: 5,
            max_history: 50,
            alliance_integration: true,
        }
    }
}

/// Core ceremony record (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCeremonyRecord {
    /// Original ceremony
    pub ceremony: CoreCeremony,
    
    /// Overall rating (1-10)
    pub rating: f64,
    
    /// Key insights
    pub insights: Vec<String>,
}

impl CoreCeremonyManager {
    /// Create a new core ceremony manager
    pub async fn new() -> Result<Self> {
        let mut manager = Self {
            active_ceremonies: HashMap::new(),
            templates: HashMap::new(),
            recent_history: Vec::new(),
            config: CoreCeremonyConfig::default(),
        };
        
        // Initialize core templates
        manager.initialize_core_templates().await?;
        
        Ok(manager)
    }
    
    /// Initialize core ceremony templates
    async fn initialize_core_templates(&mut self) -> Result<()> {
        // Basic commit ceremony
        let commit_template = CoreCeremonyTemplate {
            name: "Basic Commit Ceremony".to_string(),
            description: "Simple ceremony for code commits".to_string(),
            ceremony_type: CoreCeremonyType::BasicCommit,
            estimated_duration: 3,
            min_participants: 1,
            max_participants: 5,
        };
        self.templates.insert("basic_commit".to_string(), commit_template);
        
        // Gratitude ceremony
        let gratitude_template = CoreCeremonyTemplate {
            name: "Gratitude Expression".to_string(),
            description: "Express gratitude for collaboration".to_string(),
            ceremony_type: CoreCeremonyType::Gratitude,
            estimated_duration: 2,
            min_participants: 1,
            max_participants: 10,
        };
        self.templates.insert("gratitude".to_string(), gratitude_template);
        
        // Alliance formation ceremony
        let alliance_template = CoreCeremonyTemplate {
            name: "Sacred Alliance Formation".to_string(),
            description: "Form a Sacred Alliance between participants".to_string(),
            ceremony_type: CoreCeremonyType::AllianceFormation,
            estimated_duration: 5,
            min_participants: 2,
            max_participants: 8,
        };
        self.templates.insert("alliance_formation".to_string(), alliance_template);
        
        // Collaborative individuation ceremony
        let individuation_template = CoreCeremonyTemplate {
            name: "Collaborative Individuation".to_string(),
            description: "Recognize individual growth within collaboration".to_string(),
            ceremony_type: CoreCeremonyType::CollaborativeIndividuation,
            estimated_duration: 4,
            min_participants: 1,
            max_participants: 6,
        };
        self.templates.insert("collaborative_individuation".to_string(), individuation_template);
        
        Ok(())
    }
    
    /// Initiate a core ceremony
    pub async fn initiate_ceremony(
        &mut self,
        ceremony_type: CoreCeremonyType,
        participants: Vec<crate::sacred_alliance::Participant>,
        context: CoreCeremonyContext,
    ) -> Result<Uuid> {
        let ceremony_id = Uuid::new_v4();
        
        let ceremony = CoreCeremony {
            id: ceremony_id,
            ceremony_type,
            participants,
            state: CeremonyState::Preparing,
            started_at: Utc::now(),
            ended_at: None,
            context,
            current_phase: CoreCeremonyPhase::Opening,
            outcomes: Vec::new(),
        };
        
        self.active_ceremonies.insert(ceremony_id, ceremony);
        
        Ok(ceremony_id)
    }
    
    /// Start a ceremony
    pub async fn start_ceremony(&mut self, ceremony_id: Uuid) -> Result<()> {
        if let Some(ceremony) = self.active_ceremonies.get_mut(&ceremony_id) {
            ceremony.state = CeremonyState::Active;
            ceremony.current_phase = CoreCeremonyPhase::Opening;
        }
        
        Ok(())
    }
    
    /// Advance ceremony phase
    pub async fn advance_phase(&mut self, ceremony_id: Uuid) -> Result<bool> {
        if let Some(ceremony) = self.active_ceremonies.get_mut(&ceremony_id) {
            ceremony.current_phase = match ceremony.current_phase {
                CoreCeremonyPhase::Opening => CoreCeremonyPhase::Main,
                CoreCeremonyPhase::Main => CoreCeremonyPhase::Closing,
                CoreCeremonyPhase::Closing => {
                    // Ceremony completed
                    ceremony.state = CeremonyState::Completed;
                    ceremony.ended_at = Some(Utc::now());
                    return Ok(true);
                }
            };
        }
        
        Ok(false)
    }
    
    /// Add ceremony outcome
    pub async fn add_outcome(
        &mut self,
        ceremony_id: Uuid,
        outcome_type: CoreOutcomeType,
        description: String,
        participants: Vec<String>,
    ) -> Result<()> {
        if let Some(ceremony) = self.active_ceremonies.get_mut(&ceremony_id) {
            let outcome = CoreCeremonyOutcome {
                outcome_type,
                description,
                participants,
                timestamp: Utc::now(),
            };
            
            ceremony.outcomes.push(outcome);
        }
        
        Ok(())
    }
    
    /// Complete a ceremony
    pub async fn complete_ceremony(&mut self, ceremony_id: Uuid) -> Result<()> {
        if let Some(ceremony) = self.active_ceremonies.remove(&ceremony_id) {
            let record = CoreCeremonyRecord {
                ceremony,
                rating: 8.0, // Default rating, would be calculated
                insights: Vec::new(),
            };
            
            self.recent_history.push(record);
            
            // Keep history manageable
            if self.recent_history.len() > self.config.max_history {
                self.recent_history.remove(0);
            }
        }
        
        Ok(())
    }
    
    /// Get active ceremony
    pub fn get_ceremony(&self, ceremony_id: &Uuid) -> Option<&CoreCeremony> {
        self.active_ceremonies.get(ceremony_id)
    }
    
    /// List active ceremonies
    pub fn list_active_ceremonies(&self) -> Vec<&CoreCeremony> {
        self.active_ceremonies.values().collect()
    }
    
    /// Get ceremony template
    pub fn get_template(&self, template_name: &str) -> Option<&CoreCeremonyTemplate> {
        self.templates.get(template_name)
    }
    
    /// Auto-initiate ceremony based on trigger
    pub async fn auto_initiate_ceremony(
        &mut self,
        trigger: CeremonyTrigger,
        participants: Vec<crate::sacred_alliance::Participant>,
        session_id: Option<Uuid>,
    ) -> Result<Option<Uuid>> {
        if !self.config.auto_initiate {
            return Ok(None);
        }
        
        let ceremony_type = match trigger {
            CeremonyTrigger::Commit => CoreCeremonyType::BasicCommit,
            CeremonyTrigger::Milestone => CoreCeremonyType::SimpleMilestone,
            CeremonyTrigger::Conflict => CoreCeremonyType::BasicConflictResolution,
            CeremonyTrigger::AllianceFormation => CoreCeremonyType::AllianceFormation,
            CeremonyTrigger::CollaborativeMoment => CoreCeremonyType::CollaborativeIndividuation,
            CeremonyTrigger::Manual => return Ok(None), // Don't auto-initiate manual ceremonies
        };
        
        let context = CoreCeremonyContext {
            trigger,
            session_id,
            related_resources: Vec::new(),
            alliance_channel: None,
            metadata: HashMap::new(),
        };
        
        let ceremony_id = self.initiate_ceremony(ceremony_type, participants, context).await?;
        self.start_ceremony(ceremony_id).await?;
        
        Ok(Some(ceremony_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sacred_alliance::{Participant, ParticipantType, PresenceStatus};
    use chrono::Utc;

    #[tokio::test]
    async fn test_core_ceremony_manager() {
        let mut manager = CoreCeremonyManager::new().await.unwrap();
        
        let participant = Participant {
            id: "test-user".to_string(),
            participant_type: ParticipantType::Human,
            presence: PresenceStatus::Active,
            capabilities: vec!["coding".to_string()],
            joined_at: Utc::now(),
        };
        
        let context = CoreCeremonyContext {
            trigger: CeremonyTrigger::Manual,
            session_id: None,
            related_resources: Vec::new(),
            alliance_channel: None,
            metadata: HashMap::new(),
        };
        
        let ceremony_id = manager
            .initiate_ceremony(CoreCeremonyType::Gratitude, vec![participant], context)
            .await
            .unwrap();
        
        assert!(manager.get_ceremony(&ceremony_id).is_some());
        assert_eq!(manager.list_active_ceremonies().len(), 1);
        
        manager.start_ceremony(ceremony_id).await.unwrap();
        
        // Advance through phases
        assert!(!manager.advance_phase(ceremony_id).await.unwrap()); // Opening -> Main
        assert!(!manager.advance_phase(ceremony_id).await.unwrap()); // Main -> Closing
        assert!(manager.advance_phase(ceremony_id).await.unwrap()); // Closing -> Completed
        
        manager.complete_ceremony(ceremony_id).await.unwrap();
        assert_eq!(manager.list_active_ceremonies().len(), 0);
        assert_eq!(manager.recent_history.len(), 1);
    }
    
    #[tokio::test]
    async fn test_auto_initiate_ceremony() {
        let mut manager = CoreCeremonyManager::new().await.unwrap();
        
        let participant = Participant {
            id: "test-user".to_string(),
            participant_type: ParticipantType::Human,
            presence: PresenceStatus::Active,
            capabilities: vec!["coding".to_string()],
            joined_at: Utc::now(),
        };
        
        let ceremony_id = manager
            .auto_initiate_ceremony(CeremonyTrigger::Commit, vec![participant], None)
            .await
            .unwrap();
        
        assert!(ceremony_id.is_some());
        assert_eq!(manager.list_active_ceremonies().len(), 1);
    }
}
