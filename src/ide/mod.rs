//! # IDE Integration Module for WeaveMesh Core
//!
//! This module provides the core IDE integration capabilities for WeaveMesh,
//! focusing on universal collaborative individuation patterns that can be
//! extended by context-specific plugins.

pub mod ceremony;
pub mod collaboration;
pub mod editor;
pub mod project;
pub mod security;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Core IDE integration manager
#[derive(Debug)]
pub struct CoreIdeManager {
    /// Active IDE sessions
    pub sessions: HashMap<Uuid, IdeSession>,
    
    /// Ceremony manager
    pub ceremony_manager: ceremony::CoreCeremonyManager,
    
    /// Configuration
    pub config: CoreIdeConfig,
}

/// Core IDE session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdeSession {
    /// Session identifier
    pub id: Uuid,
    
    /// Session type
    pub session_type: SessionType,
    
    /// Participants in the session
    pub participants: Vec<crate::sacred_alliance::Participant>,
    
    /// Current Sacred Alliance channel
    pub alliance_channel: Option<String>,
    
    /// Session state
    pub state: SessionState,
    
    /// Session metadata
    pub metadata: HashMap<String, String>,
}

/// Types of IDE sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionType {
    /// Individual development session
    Individual,
    /// Pair programming session
    PairProgramming,
    /// Team collaboration session
    TeamCollaboration,
    /// Code review session
    CodeReview,
    /// Sacred Alliance ceremony session
    Ceremony,
}

/// State of an IDE session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionState {
    /// Session is initializing
    Initializing,
    /// Session is active
    Active,
    /// Session is paused
    Paused,
    /// Session is completed
    Completed,
    /// Session was terminated
    Terminated,
}

/// Core IDE configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreIdeConfig {
    /// Enable automatic ceremony initiation
    pub auto_ceremony: bool,
    
    /// Default session timeout in minutes
    pub session_timeout: u32,
    
    /// Enable Sacred Alliance integration
    pub sacred_alliance_enabled: bool,
    
    /// Maximum concurrent sessions
    pub max_sessions: usize,
}

impl Default for CoreIdeConfig {
    fn default() -> Self {
        Self {
            auto_ceremony: true,
            session_timeout: 60,
            sacred_alliance_enabled: true,
            max_sessions: 10,
        }
    }
}

impl CoreIdeManager {
    /// Create a new core IDE manager
    pub async fn new() -> Result<Self> {
        let ceremony_manager = ceremony::CoreCeremonyManager::new().await?;
        
        Ok(Self {
            sessions: HashMap::new(),
            ceremony_manager,
            config: CoreIdeConfig::default(),
        })
    }
    
    /// Start a new IDE session
    pub async fn start_session(
        &mut self,
        session_type: SessionType,
        participants: Vec<crate::sacred_alliance::Participant>,
    ) -> Result<Uuid> {
        if self.sessions.len() >= self.config.max_sessions {
            return Err(anyhow::anyhow!("Maximum sessions reached"));
        }
        
        let session_id = Uuid::new_v4();
        
        let session = IdeSession {
            id: session_id,
            session_type,
            participants,
            alliance_channel: None,
            state: SessionState::Initializing,
            metadata: HashMap::new(),
        };
        
        self.sessions.insert(session_id, session);
        
        // Initialize Sacred Alliance channel if enabled
        if self.config.sacred_alliance_enabled {
            self.initialize_alliance_channel(session_id).await?;
        }
        
        // Set session to active
        if let Some(session) = self.sessions.get_mut(&session_id) {
            session.state = SessionState::Active;
        }
        
        Ok(session_id)
    }
    
    /// Initialize Sacred Alliance channel for session
    async fn initialize_alliance_channel(&mut self, session_id: Uuid) -> Result<()> {
        let channel_id = format!("ide-session-{}", session_id);
        
        // Create Sacred Alliance channel
        let config = crate::sacred_alliance::ChannelConfig::default();
        // Note: This would integrate with the actual Sacred Alliance provider
        
        if let Some(session) = self.sessions.get_mut(&session_id) {
            session.alliance_channel = Some(channel_id);
        }
        
        Ok(())
    }
    
    /// Get session by ID
    pub fn get_session(&self, session_id: &Uuid) -> Option<&IdeSession> {
        self.sessions.get(session_id)
    }
    
    /// End a session
    pub async fn end_session(&mut self, session_id: Uuid) -> Result<()> {
        if let Some(session) = self.sessions.get_mut(&session_id) {
            session.state = SessionState::Completed;
            
            // Clean up Sacred Alliance channel
            if let Some(_channel_id) = &session.alliance_channel {
                // Note: This would clean up the actual Sacred Alliance channel
            }
        }
        
        Ok(())
    }
    
    /// List active sessions
    pub fn list_active_sessions(&self) -> Vec<&IdeSession> {
        self.sessions
            .values()
            .filter(|s| matches!(s.state, SessionState::Active))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sacred_alliance::{Participant, ParticipantType, PresenceStatus};
    use chrono::Utc;

    #[tokio::test]
    async fn test_core_ide_manager() {
        let mut manager = CoreIdeManager::new().await.unwrap();
        
        let participant = Participant {
            id: "test-user".to_string(),
            participant_type: ParticipantType::Human,
            presence: PresenceStatus::Active,
            capabilities: vec!["coding".to_string()],
            joined_at: Utc::now(),
        };
        
        let session_id = manager
            .start_session(SessionType::Individual, vec![participant])
            .await
            .unwrap();
        
        assert!(manager.get_session(&session_id).is_some());
        assert_eq!(manager.list_active_sessions().len(), 1);
        
        manager.end_session(session_id).await.unwrap();
        assert_eq!(manager.list_active_sessions().len(), 0);
    }
}
