//! Situation Provider Interface for WeaveMesh Core
//!
//! This module provides the plugin architecture foundation that enables
//! situation-specific adaptations while maintaining universal communication primitives.

use std::collections::HashMap;
use std::sync::Arc;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use async_trait::async_trait;

/// Situation provider trait for implementing situation-specific behavior
#[async_trait]
pub trait SituationProvider: Send + Sync {
    /// Get the unique identifier for this situation provider
    fn get_situation_id(&self) -> &str;
    
    /// Get the human-readable name for this situation
    fn get_situation_name(&self) -> &str;
    
    /// Get the version of this situation provider
    fn get_version(&self) -> &str;
    
    /// Get the description of what this situation provides
    fn get_description(&self) -> &str;
    
    /// Detect if this situation applies to the current conditions
    async fn detect_situation(&self, detection_data: &SituationDetectionData) -> Result<SituationMatch>;
    
    /// Adapt behavior for this situation
    async fn adapt_behavior(&self, adaptation_request: &BehaviorAdaptationRequest) -> Result<BehaviorAdaptation>;
    
    /// Get situation-specific configuration
    fn get_situation_config(&self) -> SituationConfig;
    
    /// Validate that this situation provider is compatible with the core
    fn validate_compatibility(&self, core_version: &str) -> Result<()>;
    
    /// Initialize the situation provider
    async fn initialize(&mut self, init_data: &SituationInitData) -> Result<()>;
    
    /// Shutdown the situation provider
    async fn shutdown(&mut self) -> Result<()>;
}

/// Data used for situation detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SituationDetectionData {
    /// Current environment information
    pub environment: EnvironmentInfo,
    /// Active participants
    pub participants: Vec<ParticipantInfo>,
    /// Current communication patterns
    pub communication_patterns: Vec<CommunicationPattern>,
    /// System capabilities
    pub system_capabilities: Vec<String>,
    /// User preferences
    pub user_preferences: HashMap<String, serde_json::Value>,
    /// Current time and location situation
    pub temporal_situation: TemporalSituation,
}

/// Information about the current environment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentInfo {
    /// Environment type (family, business, educational, etc.)
    pub environment_type: String,
    /// Security level required
    pub security_level: String,
    /// Available resources
    pub available_resources: Vec<String>,
    /// Network topology
    pub network_topology: NetworkTopology,
    /// Device capabilities
    pub device_capabilities: Vec<String>,
}

/// Information about a participant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantInfo {
    /// Participant ID
    pub id: String,
    /// Participant type (human, ai, system)
    pub participant_type: String,
    /// Participant role
    pub role: String,
    /// Capabilities
    pub capabilities: Vec<String>,
    /// Preferences
    pub preferences: HashMap<String, serde_json::Value>,
    /// Current status
    pub status: String,
}

/// Communication pattern information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationPattern {
    /// Pattern type
    pub pattern_type: String,
    /// Frequency
    pub frequency: f64,
    /// Participants involved
    pub participants: Vec<String>,
    /// Success rate
    pub success_rate: f64,
    /// Situation tags
    pub tags: Vec<String>,
}

/// Temporal situation information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalSituation {
    /// Current timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Time zone
    pub timezone: String,
    /// Day of week
    pub day_of_week: String,
    /// Time of day category (morning, afternoon, evening, night)
    pub time_of_day: String,
    /// Is it a weekend/holiday
    pub is_leisure_time: bool,
}

/// Network topology information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkTopology {
    /// Topology type (mesh, star, hierarchical)
    pub topology_type: String,
    /// Number of nodes
    pub node_count: usize,
    /// Connection quality
    pub connection_quality: f64,
    /// Bandwidth availability
    pub bandwidth: String,
    /// Latency characteristics
    pub latency: String,
}

/// Result of situation detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SituationMatch {
    /// Whether this situation matches
    pub matches: bool,
    /// Confidence level (0.0 to 1.0)
    pub confidence: f64,
    /// Reasons for the match/no-match
    pub reasons: Vec<String>,
    /// Suggested adaptations
    pub suggested_adaptations: Vec<String>,
    /// Priority level if multiple situations match
    pub priority: u32,
}

/// Request for behavior adaptation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorAdaptationRequest {
    /// Type of adaptation requested
    pub adaptation_type: AdaptationType,
    /// Current behavior configuration
    pub current_behavior: HashMap<String, serde_json::Value>,
    /// Situation-specific parameters
    pub situation_parameters: HashMap<String, serde_json::Value>,
    /// Participants affected
    pub affected_participants: Vec<String>,
    /// Urgency level
    pub urgency: UrgencyLevel,
}

/// Types of behavior adaptation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AdaptationType {
    /// Communication style adaptation
    CommunicationStyle,
    /// Security level adjustment
    SecurityLevel,
    /// User interface adaptation
    UserInterface,
    /// Performance optimization
    Performance,
    /// Content filtering
    ContentFiltering,
    /// Workflow adaptation
    Workflow,
    /// Resource allocation
    ResourceAllocation,
    /// Custom adaptation
    Custom(String),
}

/// Urgency levels for adaptations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UrgencyLevel {
    /// Low priority, can be delayed
    Low,
    /// Normal priority
    Normal,
    /// High priority, should be handled quickly
    High,
    /// Critical, must be handled immediately
    Critical,
}

/// Result of behavior adaptation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorAdaptation {
    /// Whether adaptation was successful
    pub success: bool,
    /// New behavior configuration
    pub new_behavior: HashMap<String, serde_json::Value>,
    /// Changes made
    pub changes: Vec<BehaviorChange>,
    /// Warnings or notes
    pub warnings: Vec<String>,
    /// Duration this adaptation should remain active
    pub duration: Option<chrono::Duration>,
}

/// Description of a behavior change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorChange {
    /// Component that was changed
    pub component: String,
    /// Type of change
    pub change_type: String,
    /// Old value
    pub old_value: Option<serde_json::Value>,
    /// New value
    pub new_value: serde_json::Value,
    /// Reason for the change
    pub reason: String,
}

/// Configuration for a situation provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SituationConfig {
    /// Situation identifier
    pub situation_id: String,
    /// Priority level (higher numbers = higher priority)
    pub priority: u32,
    /// Whether this situation can override others
    pub can_override: bool,
    /// Maximum adaptation frequency
    pub max_adaptation_frequency: chrono::Duration,
    /// Required capabilities
    pub required_capabilities: Vec<String>,
    /// Optional capabilities
    pub optional_capabilities: Vec<String>,
    /// Configuration parameters
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Initialization data for situation providers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SituationInitData {
    /// Core system version
    pub core_version: String,
    /// Available system capabilities
    pub system_capabilities: Vec<String>,
    /// Initial configuration
    pub initial_config: HashMap<String, serde_json::Value>,
    /// Network information
    pub network_info: NetworkTopology,
    /// Security situation
    pub security_situation: SecuritySituation,
}

/// Security situation information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySituation {
    /// Current security level
    pub security_level: String,
    /// Available authentication methods
    pub auth_methods: Vec<String>,
    /// Encryption requirements
    pub encryption_required: bool,
    /// Access control policies
    pub access_policies: Vec<String>,
}

/// Registry for managing situation providers
pub struct SituationProviderRegistry {
    /// Registered providers
    providers: HashMap<String, Arc<dyn SituationProvider>>,
    /// Active situations
    active_situations: HashMap<String, SituationState>,
    /// Registry configuration
    config: RegistryConfig,
}

impl std::fmt::Debug for SituationProviderRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SituationProviderRegistry")
            .field("providers", &format!("{} providers", self.providers.len()))
            .field("active_situations", &self.active_situations)
            .field("config", &self.config)
            .finish()
    }
}

/// State of an active situation
#[derive(Debug, Clone)]
pub struct SituationState {
    /// Situation ID
    pub situation_id: String,
    /// When it was activated
    pub activated_at: chrono::DateTime<chrono::Utc>,
    /// Current confidence level
    pub confidence: f64,
    /// Active adaptations
    pub active_adaptations: Vec<String>,
    /// Last update time
    pub last_update: chrono::DateTime<chrono::Utc>,
}

/// Configuration for the situation provider registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryConfig {
    /// Maximum number of active situations
    pub max_active_situations: usize,
    /// Situation detection interval
    pub detection_interval: chrono::Duration,
    /// Minimum confidence for activation
    pub min_activation_confidence: f64,
    /// Enable automatic situation switching
    pub auto_situation_switching: bool,
    /// Conflict resolution strategy
    pub conflict_resolution: ConflictResolution,
}

/// Strategies for resolving situation conflicts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConflictResolution {
    /// Use highest priority situation
    HighestPriority,
    /// Use highest confidence situation
    HighestConfidence,
    /// Merge compatible situations
    Merge,
    /// Ask user to choose
    UserChoice,
    /// Use most recent situation
    MostRecent,
}

impl Default for RegistryConfig {
    fn default() -> Self {
        Self {
            max_active_situations: 3,
            detection_interval: chrono::Duration::seconds(30),
            min_activation_confidence: 0.7,
            auto_situation_switching: true,
            conflict_resolution: ConflictResolution::HighestPriority,
        }
    }
}

impl SituationProviderRegistry {
    /// Create a new situation provider registry
    pub fn new(config: RegistryConfig) -> Self {
        Self {
            providers: HashMap::new(),
            active_situations: HashMap::new(),
            config,
        }
    }
    
    /// Register a situation provider
    pub fn register_provider(&mut self, provider: Arc<dyn SituationProvider>) -> Result<()> {
        let situation_id = provider.get_situation_id().to_string();
        
        // Validate the provider
        provider.validate_compatibility(env!("CARGO_PKG_VERSION"))?;
        
        // Check for conflicts
        if self.providers.contains_key(&situation_id) {
            return Err(anyhow::anyhow!("Situation provider already registered: {}", situation_id));
        }
        
        self.providers.insert(situation_id, provider);
        Ok(())
    }
    
    /// Unregister a situation provider
    pub fn unregister_provider(&mut self, situation_id: &str) -> Result<()> {
        // Deactivate if active
        self.deactivate_situation(situation_id)?;
        
        // Remove from registry
        self.providers.remove(situation_id)
            .ok_or_else(|| anyhow::anyhow!("Situation provider not found: {}", situation_id))?;
        
        Ok(())
    }
    
    /// Detect and activate appropriate situations
    pub async fn detect_and_activate_situations(&mut self, detection_data: &SituationDetectionData) -> Result<Vec<String>> {
        let mut matches = Vec::new();
        
        // Check all registered providers
        for (situation_id, provider) in &self.providers {
            match provider.detect_situation(detection_data).await {
                Ok(situation_match) => {
                    if situation_match.matches && situation_match.confidence >= self.config.min_activation_confidence {
                        matches.push((situation_id.clone(), situation_match));
                    }
                }
                Err(e) => {
                    eprintln!("Error detecting situation {}: {}", situation_id, e);
                }
            }
        }
        
        // Sort by priority and confidence
        matches.sort_by(|a, b| {
            let a_priority = self.providers[&a.0].get_situation_config().priority;
            let b_priority = self.providers[&b.0].get_situation_config().priority;
            
            b_priority.cmp(&a_priority)
                .then_with(|| b.1.confidence.partial_cmp(&a.1.confidence).unwrap_or(std::cmp::Ordering::Equal))
        });
        
        // Activate situations based on configuration
        let mut activated = Vec::new();
        for (situation_id, situation_match) in matches.into_iter().take(self.config.max_active_situations) {
            if let Err(e) = self.activate_situation(&situation_id, situation_match.confidence).await {
                eprintln!("Error activating situation {}: {}", situation_id, e);
            } else {
                activated.push(situation_id);
            }
        }
        
        Ok(activated)
    }
    
    /// Activate a specific situation
    pub async fn activate_situation(&mut self, situation_id: &str, confidence: f64) -> Result<()> {
        if !self.providers.contains_key(situation_id) {
            return Err(anyhow::anyhow!("Situation provider not found: {}", situation_id));
        }
        
        let state = SituationState {
            situation_id: situation_id.to_string(),
            activated_at: chrono::Utc::now(),
            confidence,
            active_adaptations: Vec::new(),
            last_update: chrono::Utc::now(),
        };
        
        self.active_situations.insert(situation_id.to_string(), state);
        Ok(())
    }
    
    /// Deactivate a situation
    pub fn deactivate_situation(&mut self, situation_id: &str) -> Result<()> {
        self.active_situations.remove(situation_id)
            .ok_or_else(|| anyhow::anyhow!("Situation not active: {}", situation_id))?;
        Ok(())
    }
    
    /// Request behavior adaptation from active situations
    pub async fn request_adaptation(&mut self, request: &BehaviorAdaptationRequest) -> Result<Vec<BehaviorAdaptation>> {
        let mut adaptations = Vec::new();
        
        for situation_id in self.active_situations.keys() {
            if let Some(provider) = self.providers.get(situation_id) {
                match provider.adapt_behavior(request).await {
                    Ok(adaptation) => {
                        if adaptation.success {
                            adaptations.push(adaptation);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error adapting behavior for situation {}: {}", situation_id, e);
                    }
                }
            }
        }
        
        Ok(adaptations)
    }
    
    /// Get list of active situations
    pub fn get_active_situations(&self) -> Vec<&SituationState> {
        self.active_situations.values().collect()
    }
    
    /// Get list of registered providers
    pub fn get_registered_providers(&self) -> Vec<&str> {
        self.providers.keys().map(|s| s.as_str()).collect()
    }
}

/// Basic situation provider implementation for testing
pub struct BasicSituationProvider {
    situation_id: String,
    situation_name: String,
    version: String,
    description: String,
    config: SituationConfig,
}

impl BasicSituationProvider {
    pub fn new(situation_id: String, situation_name: String) -> Self {
        let config = SituationConfig {
            situation_id: situation_id.clone(),
            priority: 1,
            can_override: false,
            max_adaptation_frequency: chrono::Duration::minutes(5),
            required_capabilities: vec!["basic-communication".to_string()],
            optional_capabilities: vec![],
            parameters: HashMap::new(),
        };
        
        Self {
            situation_id,
            situation_name,
            version: "1.0.0".to_string(),
            description: "Basic situation provider for testing".to_string(),
            config,
        }
    }
}

#[async_trait]
impl SituationProvider for BasicSituationProvider {
    fn get_situation_id(&self) -> &str {
        &self.situation_id
    }
    
    fn get_situation_name(&self) -> &str {
        &self.situation_name
    }
    
    fn get_version(&self) -> &str {
        &self.version
    }
    
    fn get_description(&self) -> &str {
        &self.description
    }
    
    async fn detect_situation(&self, _detection_data: &SituationDetectionData) -> Result<SituationMatch> {
        // Basic implementation always matches with low confidence
        Ok(SituationMatch {
            matches: true,
            confidence: 0.5,
            reasons: vec!["Basic situation provider always matches".to_string()],
            suggested_adaptations: vec!["basic-adaptation".to_string()],
            priority: 1,
        })
    }
    
    async fn adapt_behavior(&self, request: &BehaviorAdaptationRequest) -> Result<BehaviorAdaptation> {
        // Basic implementation just returns success without changes
        Ok(BehaviorAdaptation {
            success: true,
            new_behavior: request.current_behavior.clone(),
            changes: vec![],
            warnings: vec!["Basic situation provider made no changes".to_string()],
            duration: Some(chrono::Duration::minutes(30)),
        })
    }
    
    fn get_situation_config(&self) -> SituationConfig {
        self.config.clone()
    }
    
    fn validate_compatibility(&self, _core_version: &str) -> Result<()> {
        // Basic implementation is always compatible
        Ok(())
    }
    
    async fn initialize(&mut self, _init_data: &SituationInitData) -> Result<()> {
        // Basic implementation needs no initialization
        Ok(())
    }
    
    async fn shutdown(&mut self) -> Result<()> {
        // Basic implementation needs no cleanup
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_basic_situation_provider() {
        let provider = BasicSituationProvider::new(
            "test-situation".to_string(),
            "Test Situation".to_string(),
        );
        
        assert_eq!(provider.get_situation_id(), "test-situation");
        assert_eq!(provider.get_situation_name(), "Test Situation");
        assert!(provider.validate_compatibility("1.0.0").is_ok());
        
        let detection_data = SituationDetectionData {
            environment: EnvironmentInfo {
                environment_type: "test".to_string(),
                security_level: "basic".to_string(),
                available_resources: vec![],
                network_topology: NetworkTopology {
                    topology_type: "mesh".to_string(),
                    node_count: 1,
                    connection_quality: 1.0,
                    bandwidth: "high".to_string(),
                    latency: "low".to_string(),
                },
                device_capabilities: vec![],
            },
            participants: vec![],
            communication_patterns: vec![],
            system_capabilities: vec![],
            user_preferences: HashMap::new(),
            temporal_situation: TemporalSituation {
                timestamp: chrono::Utc::now(),
                timezone: "UTC".to_string(),
                day_of_week: "Monday".to_string(),
                time_of_day: "morning".to_string(),
                is_leisure_time: false,
            },
        };
        
        let situation_match = provider.detect_situation(&detection_data).await.unwrap();
        assert!(situation_match.matches);
        assert!(situation_match.confidence > 0.0);
    }
    
    #[tokio::test]
    async fn test_situation_provider_registry() {
        let mut registry = SituationProviderRegistry::new(RegistryConfig::default());
        
        let provider = Arc::new(BasicSituationProvider::new(
            "test-situation".to_string(),
            "Test Situation".to_string(),
        ));
        
        // Register provider
        registry.register_provider(provider).unwrap();
        assert_eq!(registry.get_registered_providers().len(), 1);
        
        // Activate situation
        registry.activate_situation("test-situation", 0.8).await.unwrap();
        assert_eq!(registry.get_active_situations().len(), 1);
        
        // Deactivate situation
        registry.deactivate_situation("test-situation").unwrap();
        assert_eq!(registry.get_active_situations().len(), 0);
    }
}
