//! Core Collaboration Framework for WeaveMesh Core
//!
//! This module provides foundational collaborative individuation patterns
//! that can be extended by context-specific plugins.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::group_communication::{GroupCommunication, GroupId, Message, MessageId, GroupMembership};
use crate::sacred_alliance::{SacredAllianceProvider, Participant, ParticipantType, PresenceStatus};

/// Core IDE session types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CoreSessionType {
    /// Individual development session
    Individual,
    /// Pair programming session
    PairProgramming,
    /// Team collaboration session
    TeamCollaboration,
    /// Sacred Alliance ceremony session
    SacredAllianceCeremony,
}

/// Core participant types in IDE sessions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CoreParticipantType {
    /// Human developer
    Human,
    /// AI assistant
    AIAssistant,
    /// System agent
    SystemAgent,
}

/// Core collaboration session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCollaborationSession {
    /// Session identifier
    pub id: Uuid,
    /// Session type
    pub session_type: CoreSessionType,
    /// Session participants
    pub participants: Vec<CoreParticipant>,
    /// Session start time
    pub started_at: DateTime<Utc>,
    /// Session end time (if completed)
    pub ended_at: Option<DateTime<Utc>>,
    /// Session status
    pub status: CoreSessionStatus,
    /// Core collaborative individuation metrics
    pub metrics: CoreCollaborativeIndividuationMetrics,
    /// Session events log
    pub events: Vec<CoreCollaborationEvent>,
    /// Session configuration
    pub config: CoreSessionConfig,
}

/// Core participant in collaboration session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreParticipant {
    /// Participant identifier
    pub id: String,
    /// Participant type
    pub participant_type: CoreParticipantType,
    /// Current presence status
    pub presence: CorePresenceStatus,
    /// Join timestamp
    pub joined_at: DateTime<Utc>,
    /// Last activity timestamp
    pub last_activity: DateTime<Utc>,
    /// Participant capabilities
    pub capabilities: Vec<String>,
}

/// Core presence status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CorePresenceStatus {
    /// Actively participating
    Active,
    /// Present but idle
    Idle,
    /// Away temporarily
    Away,
    /// Offline
    Offline,
}

/// Core session status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CoreSessionStatus {
    /// Session is active
    Active,
    /// Session is paused
    Paused,
    /// Session has ended
    Ended,
}

/// Core collaborative individuation metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCollaborativeIndividuationMetrics {
    /// Individual contribution score (0.0 to 1.0)
    pub individual_contribution: f64,
    /// Collective synergy score (0.0 to 1.0)
    pub collective_synergy: f64,
    /// Innovation emergence score (0.0 to 1.0)
    pub innovation_emergence: f64,
    /// Conflict resolution effectiveness (0.0 to 1.0)
    pub conflict_resolution_effectiveness: f64,
    /// Sacred Alliance integration level (0.0 to 1.0)
    pub sacred_alliance_integration: f64,
    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,
}

impl Default for CoreCollaborativeIndividuationMetrics {
    fn default() -> Self {
        Self {
            individual_contribution: 0.5,
            collective_synergy: 0.5,
            innovation_emergence: 0.5,
            conflict_resolution_effectiveness: 0.7,
            sacred_alliance_integration: 0.6,
            last_updated: Utc::now(),
        }
    }
}

/// Core collaboration events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCollaborationEvent {
    /// Event identifier
    pub id: Uuid,
    /// Event type
    pub event_type: CoreCollaborationEventType,
    /// Participant who triggered the event
    pub participant_id: String,
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
    /// Event details
    pub details: HashMap<String, String>,
}

/// Core collaboration event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoreCollaborationEventType {
    /// Participant joined session
    ParticipantJoined,
    /// Participant left session
    ParticipantLeft,
    /// Conflict detected
    ConflictDetected,
    /// Conflict resolved
    ConflictResolved,
    /// Innovation breakthrough
    InnovationBreakthrough,
    /// Sacred Alliance milestone
    SacredAllianceMilestone,
    /// Session paused
    SessionPaused,
    /// Session resumed
    SessionResumed,
}

/// Core session configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSessionConfig {
    /// Maximum participants
    pub max_participants: usize,
    /// Auto-save interval in seconds
    pub auto_save_interval: u32,
    /// Enable conflict detection
    pub conflict_detection_enabled: bool,
    /// Enable Sacred Alliance integration
    pub sacred_alliance_enabled: bool,
    /// Session timeout in minutes
    pub session_timeout_minutes: u32,
}

impl Default for CoreSessionConfig {
    fn default() -> Self {
        Self {
            max_participants: 10,
            auto_save_interval: 30,
            conflict_detection_enabled: true,
            sacred_alliance_enabled: true,
            session_timeout_minutes: 480, // 8 hours
        }
    }
}

/// Core collaboration statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCollaborationStatistics {
    /// Total sessions created
    pub total_sessions: u64,
    /// Active sessions count
    pub active_sessions: usize,
    /// Total participants across all sessions
    pub total_participants: usize,
    /// Average session duration in minutes
    pub average_session_duration: f64,
    /// Total conflicts detected
    pub total_conflicts_detected: u64,
    /// Total conflicts resolved
    pub total_conflicts_resolved: u64,
    /// Innovation breakthroughs
    pub innovation_breakthroughs: u64,
    /// Sacred Alliance milestones
    pub sacred_alliance_milestones: u64,
}

impl Default for CoreCollaborationStatistics {
    fn default() -> Self {
        Self {
            total_sessions: 0,
            active_sessions: 0,
            total_participants: 0,
            average_session_duration: 0.0,
            total_conflicts_detected: 0,
            total_conflicts_resolved: 0,
            innovation_breakthroughs: 0,
            sacred_alliance_milestones: 0,
        }
    }
}

/// Core collaboration manager
pub struct CoreCollaborationManager {
    /// Active collaboration sessions
    pub active_sessions: HashMap<Uuid, CoreCollaborationSession>,
    /// Manager configuration
    pub config: CoreCollaborationConfig,
    /// Collaboration statistics
    pub statistics: CoreCollaborationStatistics,
    /// Group communication interface
    pub group_communication: Option<Box<dyn GroupCommunication + Send + Sync>>,
    /// Sacred Alliance provider
    pub sacred_alliance: Option<Box<dyn SacredAllianceProvider + Send + Sync>>,
}

/// Core collaboration manager configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCollaborationConfig {
    /// Enable automatic conflict detection
    pub auto_conflict_detection: bool,
    /// Enable Sacred Alliance integration
    pub sacred_alliance_integration: bool,
    /// Default session timeout in minutes
    pub default_session_timeout: u32,
    /// Maximum concurrent sessions
    pub max_concurrent_sessions: usize,
    /// Enable session persistence
    pub session_persistence_enabled: bool,
}

impl Default for CoreCollaborationConfig {
    fn default() -> Self {
        Self {
            auto_conflict_detection: true,
            sacred_alliance_integration: true,
            default_session_timeout: 480, // 8 hours
            max_concurrent_sessions: 50,
            session_persistence_enabled: true,
        }
    }
}

impl CoreCollaborationManager {
    /// Create a new core collaboration manager
    pub fn new() -> Self {
        Self {
            active_sessions: HashMap::new(),
            config: CoreCollaborationConfig::default(),
            statistics: CoreCollaborationStatistics::default(),
            group_communication: None,
            sacred_alliance: None,
        }
    }
    
    /// Set group communication provider
    pub fn set_group_communication(&mut self, provider: Box<dyn GroupCommunication + Send + Sync>) {
        self.group_communication = Some(provider);
    }
    
    /// Set Sacred Alliance provider
    pub fn set_sacred_alliance(&mut self, provider: Box<dyn SacredAllianceProvider + Send + Sync>) {
        self.sacred_alliance = Some(provider);
    }
    
    /// Start a new collaboration session
    pub async fn start_session(
        &mut self,
        session_type: CoreSessionType,
        participants: Vec<CoreParticipant>,
    ) -> Result<Uuid> {
        // Check session limits
        if self.active_sessions.len() >= self.config.max_concurrent_sessions {
            return Err(anyhow::anyhow!("Maximum concurrent sessions reached"));
        }
        
        let session_id = Uuid::new_v4();
        let session = CoreCollaborationSession {
            id: session_id,
            session_type,
            participants,
            started_at: Utc::now(),
            ended_at: None,
            status: CoreSessionStatus::Active,
            metrics: CoreCollaborativeIndividuationMetrics::default(),
            events: Vec::new(),
            config: CoreSessionConfig::default(),
        };
        
        // Create Sacred Alliance channel if enabled
        if self.config.sacred_alliance_integration {
            if let Some(sacred_alliance) = &self.sacred_alliance {
                let channel_id = format!("collaboration-session-{}", session_id);
                let config = crate::sacred_alliance::ChannelConfig::default();
                let _ = sacred_alliance.create_channel(channel_id, config);
            }
        }
        
        self.active_sessions.insert(session_id, session);
        
        // Update statistics
        self.statistics.total_sessions += 1;
        self.statistics.active_sessions = self.active_sessions.len();
        self.update_participant_count();
        
        Ok(session_id)
    }
    
    /// End a collaboration session
    pub async fn end_session(&mut self, session_id: Uuid) -> Result<()> {
        if let Some(mut session) = self.active_sessions.remove(&session_id) {
            session.status = CoreSessionStatus::Ended;
            session.ended_at = Some(Utc::now());
            
            // Calculate session duration and update statistics
            let duration = session.ended_at.unwrap()
                .signed_duration_since(session.started_at)
                .num_minutes() as f64;
            
            self.update_average_session_duration(duration);
            self.statistics.active_sessions = self.active_sessions.len();
            self.update_participant_count();
        }
        
        Ok(())
    }
    
    /// Add participant to session
    pub async fn add_participant(
        &mut self,
        session_id: Uuid,
        participant: CoreParticipant,
    ) -> Result<()> {
        if let Some(session) = self.active_sessions.get_mut(&session_id) {
            // Check if participant already exists
            if session.participants.iter().any(|p| p.id == participant.id) {
                return Err(anyhow::anyhow!("Participant already in session"));
            }
            
            // Check session capacity
            if session.participants.len() >= session.config.max_participants {
                return Err(anyhow::anyhow!("Session at maximum capacity"));
            }
            
            session.participants.push(participant.clone());
            
            // Log event
            let participant_id = participant.id.clone();
            self.log_event(
                session_id,
                CoreCollaborationEventType::ParticipantJoined,
                participant_id,
                HashMap::new(),
            ).await?;
            
            // Add to Sacred Alliance channel if enabled
            if self.config.sacred_alliance_integration {
                if let Some(sacred_alliance) = &self.sacred_alliance {
                    let channel_id = format!("collaboration-session-{}", session_id);
                    let sa_participant = Participant {
                        id: participant.id.clone(),
                        participant_type: match participant.participant_type {
                            CoreParticipantType::Human => ParticipantType::Human,
                            CoreParticipantType::AIAssistant => ParticipantType::Ai,
                            CoreParticipantType::SystemAgent => ParticipantType::Ai,
                        },
                        presence: match participant.presence {
                            CorePresenceStatus::Active => PresenceStatus::Active,
                            CorePresenceStatus::Idle => PresenceStatus::Present,
                            CorePresenceStatus::Away => PresenceStatus::Away,
                            CorePresenceStatus::Offline => PresenceStatus::Offline,
                        },
                        capabilities: participant.capabilities,
                        joined_at: participant.joined_at,
                    };
                    let _ = sacred_alliance.add_participant(&channel_id, sa_participant);
                }
            }
            
            self.update_participant_count();
        }
        
        Ok(())
    }
    
    /// Remove participant from session
    pub async fn remove_participant(
        &mut self,
        session_id: Uuid,
        participant_id: String,
    ) -> Result<()> {
        if let Some(session) = self.active_sessions.get_mut(&session_id) {
            session.participants.retain(|p| p.id != participant_id);
            
            // Log event
            self.log_event(
                session_id,
                CoreCollaborationEventType::ParticipantLeft,
                participant_id,
                HashMap::new(),
            ).await?;
            
            self.update_participant_count();
        }
        
        Ok(())
    }
    
    /// Detect and log conflicts
    pub async fn detect_conflict(
        &mut self,
        session_id: Uuid,
        conflict_description: String,
    ) -> Result<()> {
        if !self.config.auto_conflict_detection {
            return Ok(());
        }
        
        // First, log the event
        let mut details = HashMap::new();
        details.insert("description".to_string(), conflict_description);
        
        self.log_event(
            session_id,
            CoreCollaborationEventType::ConflictDetected,
            "system".to_string(),
            details,
        ).await?;
        
        // Then update statistics and metrics
        self.statistics.total_conflicts_detected += 1;
        
        if let Some(session) = self.active_sessions.get_mut(&session_id) {
            session.metrics.conflict_resolution_effectiveness *= 0.9; // Slight decrease
            session.metrics.last_updated = Utc::now();
        }
        
        Ok(())
    }
    
    /// Mark conflict as resolved
    pub async fn resolve_conflict(
        &mut self,
        session_id: Uuid,
        resolution_description: String,
    ) -> Result<()> {
        // First, log the event
        let mut details = HashMap::new();
        details.insert("resolution".to_string(), resolution_description);
        
        self.log_event(
            session_id,
            CoreCollaborationEventType::ConflictResolved,
            "system".to_string(),
            details,
        ).await?;
        
        // Then update statistics and metrics
        self.statistics.total_conflicts_resolved += 1;
        
        if let Some(session) = self.active_sessions.get_mut(&session_id) {
            session.metrics.conflict_resolution_effectiveness = 
                (session.metrics.conflict_resolution_effectiveness + 0.1).min(1.0);
            session.metrics.collective_synergy = 
                (session.metrics.collective_synergy + 0.05).min(1.0);
            session.metrics.last_updated = Utc::now();
        }
        
        Ok(())
    }
    
    /// Record innovation breakthrough
    pub async fn record_innovation_breakthrough(
        &mut self,
        session_id: Uuid,
        innovation_description: String,
    ) -> Result<()> {
        // First, log the event
        let mut details = HashMap::new();
        details.insert("innovation".to_string(), innovation_description);
        
        self.log_event(
            session_id,
            CoreCollaborationEventType::InnovationBreakthrough,
            "system".to_string(),
            details,
        ).await?;
        
        // Then update statistics and metrics
        self.statistics.innovation_breakthroughs += 1;
        
        if let Some(session) = self.active_sessions.get_mut(&session_id) {
            session.metrics.innovation_emergence = 
                (session.metrics.innovation_emergence + 0.1).min(1.0);
            session.metrics.collective_synergy = 
                (session.metrics.collective_synergy + 0.05).min(1.0);
            session.metrics.last_updated = Utc::now();
        }
        
        Ok(())
    }
    
    /// Get session by ID
    pub fn get_session(&self, session_id: &Uuid) -> Option<&CoreCollaborationSession> {
        self.active_sessions.get(session_id)
    }
    
    /// List all active sessions
    pub fn list_active_sessions(&self) -> Vec<&CoreCollaborationSession> {
        self.active_sessions.values().collect()
    }
    
    /// Get collaboration statistics
    pub fn get_statistics(&self) -> &CoreCollaborationStatistics {
        &self.statistics
    }
    
    /// Log collaboration event
    async fn log_event(
        &mut self,
        session_id: Uuid,
        event_type: CoreCollaborationEventType,
        participant_id: String,
        details: HashMap<String, String>,
    ) -> Result<()> {
        if let Some(session) = self.active_sessions.get_mut(&session_id) {
            let event = CoreCollaborationEvent {
                id: Uuid::new_v4(),
                event_type,
                participant_id,
                timestamp: Utc::now(),
                details,
            };
            
            session.events.push(event);
            
            // Keep event log manageable
            if session.events.len() > 1000 {
                session.events.remove(0);
            }
        }
        
        Ok(())
    }
    
    /// Update participant count across all sessions
    fn update_participant_count(&mut self) {
        self.statistics.total_participants = self.active_sessions
            .values()
            .map(|s| s.participants.len())
            .sum();
    }
    
    /// Update average session duration
    fn update_average_session_duration(&mut self, new_duration: f64) {
        let total_completed = self.statistics.total_sessions - self.statistics.active_sessions as u64;
        if total_completed > 0 {
            self.statistics.average_session_duration = 
                (self.statistics.average_session_duration * (total_completed - 1) as f64 + new_duration) 
                / total_completed as f64;
        } else {
            self.statistics.average_session_duration = new_duration;
        }
    }
}

impl Default for CoreCollaborationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_core_collaboration_session_creation() {
        let mut manager = CoreCollaborationManager::new();
        
        let participants = vec![
            CoreParticipant {
                id: "human1".to_string(),
                participant_type: CoreParticipantType::Human,
                presence: CorePresenceStatus::Active,
                joined_at: Utc::now(),
                last_activity: Utc::now(),
                capabilities: vec!["coding".to_string()],
            }
        ];
        
        let session_id = manager.start_session(
            CoreSessionType::Individual,
            participants,
        ).await.unwrap();
        
        assert!(manager.get_session(&session_id).is_some());
        assert_eq!(manager.statistics.total_sessions, 1);
        assert_eq!(manager.statistics.active_sessions, 1);
    }
    
    #[tokio::test]
    async fn test_participant_management() {
        let mut manager = CoreCollaborationManager::new();
        
        let session_id = manager.start_session(
            CoreSessionType::PairProgramming,
            vec![],
        ).await.unwrap();
        
        let participant = CoreParticipant {
            id: "human1".to_string(),
            participant_type: CoreParticipantType::Human,
            presence: CorePresenceStatus::Active,
            joined_at: Utc::now(),
            last_activity: Utc::now(),
            capabilities: vec!["coding".to_string()],
        };
        
        assert!(manager.add_participant(session_id, participant).await.is_ok());
        
        let session = manager.get_session(&session_id).unwrap();
        assert_eq!(session.participants.len(), 1);
        assert_eq!(session.events.len(), 1); // Join event
        
        assert!(manager.remove_participant(session_id, "human1".to_string()).await.is_ok());
        
        let session = manager.get_session(&session_id).unwrap();
        assert_eq!(session.participants.len(), 0);
        assert_eq!(session.events.len(), 2); // Join + leave events
    }
    
    #[tokio::test]
    async fn test_conflict_detection_and_resolution() {
        let mut manager = CoreCollaborationManager::new();
        
        let session_id = manager.start_session(
            CoreSessionType::TeamCollaboration,
            vec![],
        ).await.unwrap();
        
        assert!(manager.detect_conflict(
            session_id,
            "Merge conflict detected".to_string()
        ).await.is_ok());
        
        assert_eq!(manager.statistics.total_conflicts_detected, 1);
        
        assert!(manager.resolve_conflict(
            session_id,
            "Conflict resolved through discussion".to_string()
        ).await.is_ok());
        
        assert_eq!(manager.statistics.total_conflicts_resolved, 1);
        
        let session = manager.get_session(&session_id).unwrap();
        assert_eq!(session.events.len(), 2); // Conflict detected + resolved
    }
    
    #[tokio::test]
    async fn test_innovation_breakthrough_recording() {
        let mut manager = CoreCollaborationManager::new();
        
        let session_id = manager.start_session(
            CoreSessionType::TeamCollaboration,
            vec![],
        ).await.unwrap();
        
        assert!(manager.record_innovation_breakthrough(
            session_id,
            "New algorithm discovered".to_string()
        ).await.is_ok());
        
        assert_eq!(manager.statistics.innovation_breakthroughs, 1);
        
        let session = manager.get_session(&session_id).unwrap();
        assert!(session.metrics.innovation_emergence > 0.5);
    }
}
