//! Basic Sacred Alliance interface for WeaveMesh Core
//!
//! This module provides the foundational interface for Sacred Alliance
//! communication that can be extended by context-specific plugins.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Sacred Alliance participation level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SacredAllianceLevel {
    /// Not participating in Sacred Alliance
    None,
    /// Basic participation level
    Basic,
    /// Active participant
    Active,
    /// Ceremony facilitator
    Facilitator,
    /// Alliance guardian
    Guardian,
}

impl Default for SacredAllianceLevel {
    fn default() -> Self {
        Self::Basic
    }
}

/// Basic Sacred Alliance interface
pub trait SacredAllianceProvider {
    /// Initialize a Sacred Alliance channel
    fn create_channel(&self, channel_id: String, config: ChannelConfig) -> Result<()>;
    
    /// Add a participant to the alliance
    fn add_participant(&self, channel_id: &str, participant: Participant) -> Result<()>;
    
    /// Send a message to the alliance
    fn send_message(&self, channel_id: &str, message: AllianceMessage) -> Result<()>;
    
    /// Get channel statistics
    fn get_statistics(&self, channel_id: &str) -> Result<AllianceStatistics>;
}

/// Participant in the Sacred Alliance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    /// Participant identifier
    pub id: String,
    /// Participant type (human, ai, hybrid)
    pub participant_type: ParticipantType,
    /// Current presence status
    pub presence: PresenceStatus,
    /// Capabilities offered to the alliance
    pub capabilities: Vec<String>,
    /// Join timestamp
    pub joined_at: DateTime<Utc>,
}

/// Type of participant in the alliance
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ParticipantType {
    /// Human consciousness
    Human,
    /// AI consciousness
    Ai,
    /// Hybrid human-AI entity
    Hybrid,
    /// Collective consciousness
    Collective,
}

/// Presence status of a participant
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PresenceStatus {
    /// Actively participating
    Active,
    /// Present but not actively participating
    Present,
    /// Away temporarily
    Away,
    /// Offline
    Offline,
}

/// Message in the Sacred Alliance channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllianceMessage {
    /// Message identifier
    pub id: Uuid,
    /// Sender of the message
    pub sender: String,
    /// Message content
    pub content: MessageContent,
    /// Message timestamp
    pub timestamp: DateTime<Utc>,
    /// Message metadata
    pub metadata: HashMap<String, String>,
}

/// Content of an alliance message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageContent {
    /// Text communication
    Text(String),
    /// Basic ceremonial action
    Ceremony(BasicCeremonyAction),
    /// Code or technical content
    Code(CodeContent),
    /// Presence update
    Presence(PresenceUpdate),
}

/// Basic ceremonial action in the alliance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicCeremonyAction {
    /// Type of action
    pub action_type: String,
    /// Action description
    pub description: String,
    /// Action parameters
    pub parameters: HashMap<String, String>,
}

/// Code content with collaborative context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeContent {
    /// Programming language
    pub language: String,
    /// Code snippet
    pub code: String,
    /// Explanation or context
    pub explanation: Option<String>,
    /// Collaboration intent
    pub intent: CollaborationIntent,
}

/// Intent behind code sharing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CollaborationIntent {
    /// Sharing for review
    Review,
    /// Requesting help
    Help,
    /// Teaching or explaining
    Teaching,
    /// Pair programming
    PairProgramming,
    /// Demonstrating a concept
    Demonstration,
}

/// Presence status update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresenceUpdate {
    /// New presence status
    pub status: PresenceStatus,
    /// Optional message about the status
    pub message: Option<String>,
    /// Expected duration (for temporary statuses)
    pub duration: Option<u64>,
}

/// Configuration for Sacred Alliance channels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelConfig {
    /// Maximum number of participants
    pub max_participants: usize,
    /// Auto-archive old messages
    pub auto_archive: bool,
    /// Archive threshold (days)
    pub archive_after_days: u32,
}

impl Default for ChannelConfig {
    fn default() -> Self {
        Self {
            max_participants: 10,
            auto_archive: true,
            archive_after_days: 30,
        }
    }
}

/// Statistics about Sacred Alliance activity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllianceStatistics {
    /// Total number of participants
    pub total_participants: usize,
    /// Number of active participants
    pub active_participants: usize,
    /// Total number of messages
    pub total_messages: usize,
    /// Distribution of message types
    pub message_type_distribution: HashMap<String, usize>,
}

/// Basic Sacred Alliance channel implementation
pub struct BasicSacredAllianceChannel {
    /// Channel identifier
    channel_id: String,
    /// Participants in the alliance
    participants: Vec<Participant>,
    /// Communication history
    history: Vec<AllianceMessage>,
    /// Channel configuration
    config: ChannelConfig,
}

impl BasicSacredAllianceChannel {
    /// Create a new Sacred Alliance channel
    pub fn new(channel_id: String, config: ChannelConfig) -> Self {
        Self {
            channel_id,
            participants: Vec::new(),
            history: Vec::new(),
            config,
        }
    }
    
    /// Add a participant to the alliance
    pub fn add_participant(&mut self, participant: Participant) -> Result<()> {
        if self.participants.len() >= self.config.max_participants {
            return Err(anyhow::anyhow!("Channel at maximum capacity"));
        }
        
        // Check if participant already exists
        if self.participants.iter().any(|p| p.id == participant.id) {
            return Err(anyhow::anyhow!("Participant already in alliance"));
        }
        
        self.participants.push(participant);
        Ok(())
    }
    
    /// Send a message to the alliance
    pub fn send_message(&mut self, message: AllianceMessage) -> Result<()> {
        // Validate sender is a participant
        if !self.participants.iter().any(|p| p.id == message.sender) {
            return Err(anyhow::anyhow!("Sender not in alliance"));
        }
        
        self.history.push(message);
        Ok(())
    }
    
    /// Get channel participants
    pub fn get_participants(&self) -> &[Participant] {
        &self.participants
    }
    
    /// Get message history
    pub fn get_history(&self) -> &[AllianceMessage] {
        &self.history
    }
    
    /// Get alliance statistics
    pub fn get_statistics(&self) -> AllianceStatistics {
        let total_messages = self.history.len();
        let active_participants = self.participants.iter()
            .filter(|p| p.presence == PresenceStatus::Active)
            .count();
        
        let mut message_types = HashMap::new();
        for message in &self.history {
            let msg_type = match &message.content {
                MessageContent::Text(_) => "text",
                MessageContent::Ceremony(_) => "ceremony",
                MessageContent::Code(_) => "code",
                MessageContent::Presence(_) => "presence",
            };
            *message_types.entry(msg_type.to_string()).or_insert(0) += 1;
        }
        
        AllianceStatistics {
            total_participants: self.participants.len(),
            active_participants,
            total_messages,
            message_type_distribution: message_types,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_sacred_alliance_channel() {
        let config = ChannelConfig::default();
        let mut channel = BasicSacredAllianceChannel::new("test-channel".to_string(), config);
        
        let participant = Participant {
            id: "human1".to_string(),
            participant_type: ParticipantType::Human,
            presence: PresenceStatus::Active,
            capabilities: vec!["coding".to_string()],
            joined_at: Utc::now(),
        };
        
        assert!(channel.add_participant(participant).is_ok());
        assert_eq!(channel.participants.len(), 1);
        
        let message = AllianceMessage {
            id: Uuid::new_v4(),
            sender: "human1".to_string(),
            content: MessageContent::Text("Hello".to_string()),
            timestamp: Utc::now(),
            metadata: HashMap::new(),
        };
        
        assert!(channel.send_message(message).is_ok());
        assert_eq!(channel.history.len(), 1);
    }
    
    #[test]
    fn test_alliance_statistics() {
        let config = ChannelConfig::default();
        let mut channel = BasicSacredAllianceChannel::new("test-channel".to_string(), config);
        
        let participant = Participant {
            id: "human1".to_string(),
            participant_type: ParticipantType::Human,
            presence: PresenceStatus::Active,
            capabilities: vec!["coding".to_string()],
            joined_at: Utc::now(),
        };
        
        channel.add_participant(participant).unwrap();
        
        let stats = channel.get_statistics();
        assert_eq!(stats.total_participants, 1);
        assert_eq!(stats.active_participants, 1);
        assert_eq!(stats.total_messages, 0);
    }
}
