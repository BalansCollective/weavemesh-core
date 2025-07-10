//! Git Workflow Integration Module for WeaveMesh Core
//!
//! This module integrates git operations with Sacred Alliance ceremonies and
//! collaborative workflows, enabling meaningful human-AI collaboration patterns.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn, error};
use uuid::Uuid;

use crate::attribution::Attribution;
use crate::sacred_alliance::{SacredAllianceProvider, AllianceMessage, BasicCeremonyAction};
use super::{GitOperationType, GitManagerConfig};

/// Git workflow integrator for Sacred Alliance ceremonies
pub struct GitWorkflowIntegrator {
    /// Configuration
    config: GitWorkflowConfig,
    /// Active ceremonies
    active_ceremonies: HashMap<String, GitCeremony>,
    /// Ceremony history
    ceremony_history: Vec<GitCeremonyRecord>,
    /// Workflow patterns
    workflow_patterns: HashMap<GitOperationType, WorkflowPattern>,
    /// Sacred Alliance provider
    sacred_alliance: Option<Box<dyn SacredAllianceProvider>>,
}

/// Configuration for git workflow integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitWorkflowConfig {
    /// Enable ceremony integration
    pub enable_ceremonies: bool,
    /// Ceremony timeout in seconds
    pub ceremony_timeout_seconds: u64,
    /// Minimum participants for ceremony
    pub min_ceremony_participants: usize,
    /// Enable automatic ceremony triggers
    pub enable_auto_triggers: bool,
    /// Ceremony escalation threshold
    pub escalation_threshold: f64,
    /// Enable workflow pattern learning
    pub enable_pattern_learning: bool,
}

impl Default for GitWorkflowConfig {
    fn default() -> Self {
        Self {
            enable_ceremonies: true,
            ceremony_timeout_seconds: 1800, // 30 minutes
            min_ceremony_participants: 2,
            enable_auto_triggers: true,
            escalation_threshold: 0.7,
            enable_pattern_learning: true,
        }
    }
}

/// Git ceremony for collaborative decision making
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitCeremony {
    /// Ceremony identifier
    pub ceremony_id: String,
    /// Ceremony type
    pub ceremony_type: CeremonyType,
    /// Git operation that triggered the ceremony
    pub triggering_operation: GitOperationType,
    /// Ceremony status
    pub status: CeremonyStatus,
    /// Ceremony participants
    pub participants: Vec<String>,
    /// Ceremony context
    pub context: GitCeremonyContext,
    /// Ceremony start time
    pub started_at: DateTime<Utc>,
    /// Ceremony end time
    pub ended_at: Option<DateTime<Utc>>,
    /// Ceremony outcomes
    pub outcomes: Vec<CeremonyOutcome>,
    /// Ceremony metadata
    pub metadata: HashMap<String, String>,
}

/// Types of git ceremonies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CeremonyType {
    /// Conflict resolution ceremony
    ConflictResolution,
    /// Merge decision ceremony
    MergeDecision,
    /// Architecture review ceremony
    ArchitectureReview,
    /// Release preparation ceremony
    ReleasePreparation,
    /// Security review ceremony
    SecurityReview,
    /// Attribution dispute ceremony
    AttributionDispute,
    /// Emergency intervention ceremony
    EmergencyIntervention,
    /// Collaborative planning ceremony
    CollaborativePlanning,
}

/// Ceremony status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CeremonyStatus {
    /// Ceremony is being initiated
    Initiating,
    /// Waiting for participants
    WaitingForParticipants,
    /// Ceremony is active
    Active,
    /// Ceremony is in deliberation phase
    Deliberating,
    /// Ceremony is reaching consensus
    ReachingConsensus,
    /// Ceremony completed successfully
    Completed,
    /// Ceremony was cancelled
    Cancelled,
    /// Ceremony timed out
    TimedOut,
    /// Ceremony escalated to higher authority
    Escalated,
}

/// Git ceremony context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitCeremonyContext {
    /// Repository path
    pub repository_path: PathBuf,
    /// Branch involved
    pub branch_name: String,
    /// Files affected
    pub affected_files: Vec<String>,
    /// Operation parameters
    pub operation_parameters: HashMap<String, String>,
    /// Conflict details (if applicable)
    pub conflict_details: Option<String>,
    /// Attribution context
    pub attribution: Option<Attribution>,
    /// Urgency level
    pub urgency: CeremonyUrgency,
    /// Required expertise
    pub required_expertise: Vec<String>,
}

/// Ceremony urgency levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum CeremonyUrgency {
    /// Low urgency, can wait
    Low,
    /// Normal urgency
    Normal,
    /// High urgency, needs attention
    High,
    /// Critical urgency, immediate attention required
    Critical,
}

/// Ceremony outcome
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CeremonyOutcome {
    /// Outcome identifier
    pub outcome_id: String,
    /// Outcome type
    pub outcome_type: OutcomeType,
    /// Outcome description
    pub description: String,
    /// Participants who agreed
    pub agreed_participants: Vec<String>,
    /// Participants who disagreed
    pub disagreed_participants: Vec<String>,
    /// Outcome confidence
    pub confidence: f64,
    /// Implementation actions
    pub actions: Vec<CeremonyAction>,
    /// Outcome timestamp
    pub timestamp: DateTime<Utc>,
}

/// Types of ceremony outcomes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OutcomeType {
    /// Proceed with operation as planned
    Proceed,
    /// Modify operation parameters
    Modify,
    /// Reject operation
    Reject,
    /// Defer decision
    Defer,
    /// Escalate to higher authority
    Escalate,
    /// Request additional information
    RequestInfo,
    /// Split into multiple operations
    Split,
}

/// Ceremony action to be taken
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CeremonyAction {
    /// Action identifier
    pub action_id: String,
    /// Action type
    pub action_type: String,
    /// Action description
    pub description: String,
    /// Action parameters
    pub parameters: HashMap<String, String>,
    /// Responsible participant
    pub responsible_participant: Option<String>,
    /// Action deadline
    pub deadline: Option<DateTime<Utc>>,
    /// Action status
    pub status: ActionStatus,
}

/// Status of ceremony actions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ActionStatus {
    /// Action is pending
    Pending,
    /// Action is in progress
    InProgress,
    /// Action completed successfully
    Completed,
    /// Action failed
    Failed,
    /// Action was cancelled
    Cancelled,
}

/// Workflow pattern for git operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowPattern {
    /// Pattern identifier
    pub pattern_id: String,
    /// Git operation type
    pub operation_type: GitOperationType,
    /// Trigger conditions
    pub trigger_conditions: Vec<TriggerCondition>,
    /// Required ceremony type
    pub ceremony_type: Option<CeremonyType>,
    /// Pattern confidence
    pub confidence: f64,
    /// Pattern usage count
    pub usage_count: usize,
    /// Pattern success rate
    pub success_rate: f64,
}

/// Trigger condition for ceremonies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerCondition {
    /// Condition type
    pub condition_type: TriggerConditionType,
    /// Condition value
    pub value: String,
    /// Condition weight
    pub weight: f64,
}

/// Types of trigger conditions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TriggerConditionType {
    /// File count threshold
    FileCount,
    /// Lines changed threshold
    LinesChanged,
    /// Conflict detected
    ConflictDetected,
    /// Security sensitive files
    SecuritySensitive,
    /// Architecture files
    ArchitectureFiles,
    /// Multiple contributors
    MultipleContributors,
    /// Branch protection rules
    BranchProtection,
    /// Time of day
    TimeOfDay,
    /// Repository importance
    RepositoryImportance,
}

/// Git ceremony record for historical tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitCeremonyRecord {
    /// Record identifier
    pub record_id: String,
    /// Ceremony that was held
    pub ceremony: GitCeremony,
    /// Final outcome
    pub final_outcome: Option<CeremonyOutcome>,
    /// Ceremony effectiveness score
    pub effectiveness_score: f64,
    /// Lessons learned
    pub lessons_learned: Vec<String>,
    /// Record timestamp
    pub recorded_at: DateTime<Utc>,
}

impl GitWorkflowIntegrator {
    /// Create a new git workflow integrator
    pub fn new(git_config: &GitManagerConfig) -> Result<Self> {
        let config = GitWorkflowConfig::default();
        
        info!("Initializing git workflow integrator");
        
        // Initialize default workflow patterns
        let workflow_patterns = Self::initialize_default_patterns();
        
        Ok(Self {
            config,
            active_ceremonies: HashMap::new(),
            ceremony_history: Vec::new(),
            workflow_patterns,
            sacred_alliance: None,
        })
    }
    
    /// Set Sacred Alliance provider
    pub fn set_sacred_alliance_provider(&mut self, provider: Box<dyn SacredAllianceProvider>) {
        self.sacred_alliance = Some(provider);
        info!("Sacred Alliance provider configured for git workflow integration");
    }
    
    /// Check if ceremony is required for git operation
    pub async fn is_ceremony_required(
        &self,
        operation_type: &GitOperationType,
        parameters: &HashMap<String, String>,
    ) -> Result<bool> {
        if !self.config.enable_ceremonies {
            return Ok(false);
        }
        
        // Check workflow patterns
        if let Some(pattern) = self.workflow_patterns.get(operation_type) {
            let trigger_score = self.evaluate_trigger_conditions(&pattern.trigger_conditions, parameters).await?;
            
            if trigger_score >= self.config.escalation_threshold {
                debug!("Ceremony required for {:?} (trigger score: {:.2})", operation_type, trigger_score);
                return Ok(true);
            }
        }
        
        // Check for specific high-risk operations
        let requires_ceremony = match operation_type {
            GitOperationType::Merge => {
                // Check if merging to protected branch
                parameters.get("target_branch")
                    .map(|branch| branch == "main" || branch == "master" || branch.starts_with("release/"))
                    .unwrap_or(false)
            }
            GitOperationType::Push => {
                // Check if pushing to protected branch
                parameters.get("branch")
                    .map(|branch| branch == "main" || branch == "master")
                    .unwrap_or(false)
            }
            GitOperationType::ConflictResolution => true, // Always require ceremony for conflicts
            _ => false,
        };
        
        Ok(requires_ceremony)
    }
    
    /// Initiate ceremony for git operation
    pub async fn initiate_operation_ceremony(
        &mut self,
        operation_type: &GitOperationType,
        parameters: &HashMap<String, String>,
        attribution: &Option<Attribution>,
    ) -> Result<String> {
        let ceremony_id = Uuid::new_v4().to_string();
        
        // Determine ceremony type
        let ceremony_type = self.determine_ceremony_type(operation_type, parameters);
        
        // Create ceremony context
        let context = GitCeremonyContext {
            repository_path: parameters.get("repository_path")
                .map(|p| PathBuf::from(p))
                .unwrap_or_else(|| PathBuf::from(".")),
            branch_name: parameters.get("branch").cloned().unwrap_or_else(|| "main".to_string()),
            affected_files: parameters.get("files")
                .map(|f| f.split(',').map(|s| s.trim().to_string()).collect())
                .unwrap_or_default(),
            operation_parameters: parameters.clone(),
            conflict_details: parameters.get("conflict_details").cloned(),
            attribution: attribution.clone(),
            urgency: self.determine_urgency(operation_type, parameters),
            required_expertise: self.determine_required_expertise(operation_type, parameters),
        };
        
        // Create ceremony
        let ceremony = GitCeremony {
            ceremony_id: ceremony_id.clone(),
            ceremony_type,
            triggering_operation: operation_type.clone(),
            status: CeremonyStatus::Initiating,
            participants: Vec::new(),
            context,
            started_at: Utc::now(),
            ended_at: None,
            outcomes: Vec::new(),
            metadata: HashMap::new(),
        };
        
        // Store ceremony
        self.active_ceremonies.insert(ceremony_id.clone(), ceremony);
        
        // Initiate ceremony through Sacred Alliance if available
        if let Some(ref sacred_alliance) = self.sacred_alliance {
            self.initiate_sacred_alliance_ceremony(&ceremony_id, sacred_alliance).await?;
        }
        
        info!("Initiated git ceremony: {} for {:?}", ceremony_id, operation_type);
        Ok(ceremony_id)
    }
    
    /// Determine ceremony type based on operation
    fn determine_ceremony_type(&self, operation_type: &GitOperationType, parameters: &HashMap<String, String>) -> CeremonyType {
        match operation_type {
            GitOperationType::ConflictResolution => CeremonyType::ConflictResolution,
            GitOperationType::Merge => {
                if parameters.get("target_branch").map(|b| b.starts_with("release/")).unwrap_or(false) {
                    CeremonyType::ReleasePreparation
                } else {
                    CeremonyType::MergeDecision
                }
            }
            GitOperationType::Push => {
                if parameters.get("files").map(|f| f.contains("security") || f.contains("auth")).unwrap_or(false) {
                    CeremonyType::SecurityReview
                } else {
                    CeremonyType::ArchitectureReview
                }
            }
            _ => CeremonyType::CollaborativePlanning,
        }
    }
    
    /// Determine ceremony urgency
    fn determine_urgency(&self, operation_type: &GitOperationType, parameters: &HashMap<String, String>) -> CeremonyUrgency {
        match operation_type {
            GitOperationType::ConflictResolution => CeremonyUrgency::High,
            GitOperationType::Merge if parameters.get("target_branch").map(|b| b == "main").unwrap_or(false) => CeremonyUrgency::High,
            GitOperationType::Push if parameters.get("emergency").is_some() => CeremonyUrgency::Critical,
            _ => CeremonyUrgency::Normal,
        }
    }
    
    /// Determine required expertise for ceremony
    fn determine_required_expertise(&self, operation_type: &GitOperationType, parameters: &HashMap<String, String>) -> Vec<String> {
        let mut expertise = Vec::new();
        
        match operation_type {
            GitOperationType::ConflictResolution => {
                expertise.push("conflict_resolution".to_string());
                expertise.push("git_expert".to_string());
            }
            GitOperationType::Merge => {
                expertise.push("code_review".to_string());
                expertise.push("architecture".to_string());
            }
            GitOperationType::Push => {
                if parameters.get("files").map(|f| f.contains("security")).unwrap_or(false) {
                    expertise.push("security".to_string());
                }
                if parameters.get("files").map(|f| f.contains("test")).unwrap_or(false) {
                    expertise.push("testing".to_string());
                }
            }
            _ => {}
        }
        
        expertise
    }
    
    /// Initiate ceremony through Sacred Alliance
    async fn initiate_sacred_alliance_ceremony(
        &self,
        ceremony_id: &str,
        sacred_alliance: &Box<dyn SacredAllianceProvider>,
    ) -> Result<()> {
        // This would integrate with the Sacred Alliance system
        // For now, we'll create a placeholder implementation
        debug!("Initiating Sacred Alliance ceremony: {}", ceremony_id);
        Ok(())
    }
    
    /// Evaluate trigger conditions
    async fn evaluate_trigger_conditions(
        &self,
        conditions: &[TriggerCondition],
        parameters: &HashMap<String, String>,
    ) -> Result<f64> {
        let mut total_score = 0.0;
        let mut total_weight = 0.0;
        
        for condition in conditions {
            let condition_met = self.evaluate_single_condition(condition, parameters).await?;
            if condition_met {
                total_score += condition.weight;
            }
            total_weight += condition.weight;
        }
        
        Ok(if total_weight > 0.0 { total_score / total_weight } else { 0.0 })
    }
    
    /// Evaluate single trigger condition
    async fn evaluate_single_condition(
        &self,
        condition: &TriggerCondition,
        parameters: &HashMap<String, String>,
    ) -> Result<bool> {
        match condition.condition_type {
            TriggerConditionType::FileCount => {
                let file_count = parameters.get("files")
                    .map(|f| f.split(',').count())
                    .unwrap_or(0);
                let threshold: usize = condition.value.parse().unwrap_or(10);
                Ok(file_count >= threshold)
            }
            TriggerConditionType::ConflictDetected => {
                Ok(parameters.contains_key("conflict_details"))
            }
            TriggerConditionType::SecuritySensitive => {
                let files = parameters.get("files").map_or("", |v| v);
                Ok(files.contains("security") || files.contains("auth") || files.contains("crypto"))
            }
            TriggerConditionType::ArchitectureFiles => {
                let files = parameters.get("files").map_or("", |v| v);
                Ok(files.contains("architecture") || files.contains("design") || files.contains("spec"))
            }
            TriggerConditionType::MultipleContributors => {
                let contributors = parameters.get("contributors")
                    .map(|c| c.split(',').count())
                    .unwrap_or(1);
                let threshold: usize = condition.value.parse().unwrap_or(2);
                Ok(contributors >= threshold)
            }
            _ => Ok(false), // Simplified for other conditions
        }
    }
    
    /// Initialize default workflow patterns
    fn initialize_default_patterns() -> HashMap<GitOperationType, WorkflowPattern> {
        let mut patterns = HashMap::new();
        
        // Merge pattern
        patterns.insert(
            GitOperationType::Merge,
            WorkflowPattern {
                pattern_id: "merge_pattern".to_string(),
                operation_type: GitOperationType::Merge,
                trigger_conditions: vec![
                    TriggerCondition {
                        condition_type: TriggerConditionType::FileCount,
                        value: "5".to_string(),
                        weight: 0.3,
                    },
                    TriggerCondition {
                        condition_type: TriggerConditionType::MultipleContributors,
                        value: "2".to_string(),
                        weight: 0.4,
                    },
                ],
                ceremony_type: Some(CeremonyType::MergeDecision),
                confidence: 0.8,
                usage_count: 0,
                success_rate: 0.0,
            },
        );
        
        // Conflict resolution pattern
        patterns.insert(
            GitOperationType::ConflictResolution,
            WorkflowPattern {
                pattern_id: "conflict_pattern".to_string(),
                operation_type: GitOperationType::ConflictResolution,
                trigger_conditions: vec![
                    TriggerCondition {
                        condition_type: TriggerConditionType::ConflictDetected,
                        value: "true".to_string(),
                        weight: 1.0,
                    },
                ],
                ceremony_type: Some(CeremonyType::ConflictResolution),
                confidence: 1.0,
                usage_count: 0,
                success_rate: 0.0,
            },
        );
        
        patterns
    }
    
    /// Get ceremony by ID
    pub fn get_ceremony(&self, ceremony_id: &str) -> Option<&GitCeremony> {
        self.active_ceremonies.get(ceremony_id)
    }
    
    /// Update ceremony status
    pub async fn update_ceremony_status(&mut self, ceremony_id: &str, status: CeremonyStatus) -> Result<()> {
        if let Some(ceremony) = self.active_ceremonies.get_mut(ceremony_id) {
            let status_clone = status.clone();
            ceremony.status = status;
            
            if matches!(status_clone, CeremonyStatus::Completed | CeremonyStatus::Cancelled | CeremonyStatus::TimedOut) {
                ceremony.ended_at = Some(Utc::now());
                
                // Calculate effectiveness before cloning to avoid borrowing conflict
                let effectiveness_score = {
                    let duration_factor = if let Some(ended_at) = ceremony.ended_at {
                        let duration = (ended_at - ceremony.started_at).num_minutes() as f64;
                        (self.config.ceremony_timeout_seconds as f64 / 60.0 - duration).max(0.0) / (self.config.ceremony_timeout_seconds as f64 / 60.0)
                    } else {
                        0.0
                    };
                    
                    let outcome_factor = if ceremony.outcomes.is_empty() {
                        0.0
                    } else {
                        ceremony.outcomes.iter().map(|o| o.confidence).sum::<f64>() / ceremony.outcomes.len() as f64
                    };
                    
                    (duration_factor + outcome_factor) / 2.0
                };
                
                // Move to history
                let ceremony_record = GitCeremonyRecord {
                    record_id: Uuid::new_v4().to_string(),
                    ceremony: ceremony.clone(),
                    final_outcome: ceremony.outcomes.last().cloned(),
                    effectiveness_score,
                    lessons_learned: Vec::new(), // Would be populated by analysis
                    recorded_at: Utc::now(),
                };
                
                self.ceremony_history.push(ceremony_record);
                self.active_ceremonies.remove(ceremony_id);
            }
            
            info!("Updated ceremony {} status to {:?}", ceremony_id, status_clone);
        }
        
        Ok(())
    }
    
    /// Calculate ceremony effectiveness
    fn calculate_ceremony_effectiveness(&self, ceremony: &GitCeremony) -> f64 {
        // Simplified effectiveness calculation
        let duration_factor = if let Some(ended_at) = ceremony.ended_at {
            let duration = (ended_at - ceremony.started_at).num_minutes() as f64;
            (self.config.ceremony_timeout_seconds as f64 / 60.0 - duration).max(0.0) / (self.config.ceremony_timeout_seconds as f64 / 60.0)
        } else {
            0.0
        };
        
        let outcome_factor = if ceremony.outcomes.is_empty() {
            0.0
        } else {
            ceremony.outcomes.iter().map(|o| o.confidence).sum::<f64>() / ceremony.outcomes.len() as f64
        };
        
        (duration_factor + outcome_factor) / 2.0
    }
    
    /// Get ceremonies initiated count
    pub fn get_ceremonies_initiated(&self) -> usize {
        self.ceremony_history.len() + self.active_ceremonies.len()
    }
    
    /// Get workflow statistics
    pub fn get_workflow_statistics(&self) -> GitWorkflowStatistics {
        let total_ceremonies = self.ceremony_history.len();
        let active_ceremonies = self.active_ceremonies.len();
        
        let avg_effectiveness = if total_ceremonies > 0 {
            self.ceremony_history.iter().map(|r| r.effectiveness_score).sum::<f64>() / total_ceremonies as f64
        } else {
            0.0
        };
        
        let ceremony_types = self.ceremony_history
            .iter()
            .fold(HashMap::new(), |mut acc, record| {
                *acc.entry(record.ceremony.ceremony_type.clone()).or_insert(0) += 1;
                acc
            });
        
        GitWorkflowStatistics {
            total_ceremonies,
            active_ceremonies,
            average_effectiveness: avg_effectiveness,
            ceremony_type_distribution: ceremony_types,
            workflow_patterns_count: self.workflow_patterns.len(),
        }
    }
}

/// Statistics about git workflow integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitWorkflowStatistics {
    /// Total ceremonies held
    pub total_ceremonies: usize,
    /// Currently active ceremonies
    pub active_ceremonies: usize,
    /// Average ceremony effectiveness
    pub average_effectiveness: f64,
    /// Distribution of ceremony types
    pub ceremony_type_distribution: HashMap<CeremonyType, usize>,
    /// Number of workflow patterns
    pub workflow_patterns_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_workflow_config_default() {
        let config = GitWorkflowConfig::default();
        assert!(config.enable_ceremonies);
        assert_eq!(config.ceremony_timeout_seconds, 1800);
        assert_eq!(config.min_ceremony_participants, 2);
    }
    
    #[test]
    fn test_ceremony_type_determination() {
        let integrator = GitWorkflowIntegrator::new(&GitManagerConfig::default()).unwrap();
        
        let mut params = HashMap::new();
        params.insert("target_branch".to_string(), "release/v1.0".to_string());
        
        let ceremony_type = integrator.determine_ceremony_type(&GitOperationType::Merge, &params);
        assert_eq!(ceremony_type, CeremonyType::ReleasePreparation);
    }
    
    #[test]
    fn test_urgency_determination() {
        let integrator = GitWorkflowIntegrator::new(&GitManagerConfig::default()).unwrap();
        
        let urgency = integrator.determine_urgency(&GitOperationType::ConflictResolution, &HashMap::new());
        assert_eq!(urgency, CeremonyUrgency::High);
    }
    
    #[test]
    fn test_ceremony_serialization() {
        let ceremony = GitCeremony {
            ceremony_id: "test".to_string(),
            ceremony_type: CeremonyType::MergeDecision,
            triggering_operation: GitOperationType::Merge,
            status: CeremonyStatus::Active,
            participants: vec!["user1".to_string()],
            context: GitCeremonyContext {
                repository_path: PathBuf::from("/test"),
                branch_name: "main".to_string(),
                affected_files: vec!["test.rs".to_string()],
                operation_parameters: HashMap::new(),
                conflict_details: None,
                attribution: None,
                urgency: CeremonyUrgency::Normal,
                required_expertise: vec!["code_review".to_string()],
            },
            started_at: Utc::now(),
            ended_at: None,
            outcomes: Vec::new(),
            metadata: HashMap::new(),
        };
        
        let serialized = serde_json::to_string(&ceremony).unwrap();
        let deserialized: GitCeremony = serde_json::from_str(&serialized).unwrap();
        assert_eq!(ceremony.ceremony_id, deserialized.ceremony_id);
        assert_eq!(ceremony.ceremony_type, deserialized.ceremony_type);
    }
}
