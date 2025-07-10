//! Git Integration Module for WeaveMesh Core
//!
//! This module provides git-specific functionality for WeaveMesh Core,
//! integrating git operations with the unified attribution system and
//! Sacred Alliance patterns for collaborative development.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn, error};
use uuid::Uuid;

use crate::attribution::{Attribution, AttributionContext, BasicAttributionEngine, CollaborationType};

pub mod operations;
pub mod repository;
pub mod attribution_integration;
pub mod workflow_integration;
pub mod conflict_detection;
pub mod hooks;
pub mod state_tracking;

// Re-export key types for easier access
pub use operations::{GitOperationsHandler, GitOperationsConfig, GitOperationResult, GitOperationMetrics};
pub use repository::{RepositoryTracker, TrackedRepository, RepositoryState, RepositoryHealth};
pub use attribution_integration::{GitAttributionEngine, GitAttributionContext};
pub use workflow_integration::{GitWorkflowIntegrator, GitCeremony, CeremonyType, CeremonyStatus};
pub use conflict_detection::{GitConflictDetector, GitConflict, ConflictSeverity, ConflictType};
pub use hooks::{GitHooksManager, GitHook, GitHookType, HookExecutionRecord};
pub use state_tracking::{GitStateTracker, StateChangeEvent, StateChangeType};

/// Git integration manager for WeaveMesh Core
pub struct GitManager {
    /// Repository tracker
    repository_tracker: RepositoryTracker,
    /// Git operations handler
    operations_handler: GitOperationsHandler,
    /// Attribution engine for git operations
    attribution_engine: GitAttributionEngine,
    /// Workflow integrator for Sacred Alliance ceremonies
    workflow_integrator: GitWorkflowIntegrator,
    /// Conflict detection system
    conflict_detector: GitConflictDetector,
    /// State tracking system
    state_tracker: GitStateTracker,
    /// Git manager configuration
    config: GitManagerConfig,
    /// Active repository sessions
    active_sessions: HashMap<String, GitSession>,
}

/// Configuration for git manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitManagerConfig {
    /// Maximum number of concurrent git operations
    pub max_concurrent_operations: usize,
    /// Conflict detection sensitivity
    pub conflict_detection_sensitivity: f64,
    /// State sync interval in seconds
    pub state_sync_interval_seconds: u64,
    /// Repository health check interval in seconds
    pub health_check_interval_seconds: u64,
    /// Maximum repository cache size
    pub max_repository_cache_size: usize,
    /// Enable automatic conflict resolution
    pub enable_auto_conflict_resolution: bool,
    /// Enable Sacred Alliance ceremony integration
    pub enable_ceremony_integration: bool,
    /// Enable attribution tracking
    pub enable_attribution_tracking: bool,
}

impl Default for GitManagerConfig {
    fn default() -> Self {
        Self {
            max_concurrent_operations: 10,
            conflict_detection_sensitivity: 0.8,
            state_sync_interval_seconds: 30,
            health_check_interval_seconds: 60,
            max_repository_cache_size: 100,
            enable_auto_conflict_resolution: true,
            enable_ceremony_integration: true,
            enable_attribution_tracking: true,
        }
    }
}

/// Active git session for a repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitSession {
    /// Session identifier
    pub session_id: String,
    /// Repository identifier
    pub repository_id: String,
    /// Repository path
    pub repository_path: PathBuf,
    /// Current branch
    pub current_branch: String,
    /// Session owner
    pub owner_id: String,
    /// Session start time
    pub started_at: DateTime<Utc>,
    /// Last activity time
    pub last_activity: DateTime<Utc>,
    /// Session state
    pub state: GitSessionState,
    /// Active operations
    pub active_operations: Vec<GitOperation>,
    /// Session metadata
    pub metadata: HashMap<String, String>,
}

/// Git session state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GitSessionState {
    /// Session is active and ready for operations
    Active,
    /// Session is performing an operation
    Operating,
    /// Session has conflicts that need resolution
    ConflictResolution,
    /// Session is in ceremony mode
    Ceremony,
    /// Session is paused
    Paused,
    /// Session is being terminated
    Terminating,
    /// Session has ended
    Ended,
}

/// Git operation being performed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitOperation {
    /// Operation identifier
    pub operation_id: String,
    /// Operation type
    pub operation_type: GitOperationType,
    /// Operation status
    pub status: GitOperationStatus,
    /// Operation parameters
    pub parameters: HashMap<String, String>,
    /// Started timestamp
    pub started_at: DateTime<Utc>,
    /// Completed timestamp
    pub completed_at: Option<DateTime<Utc>>,
    /// Operation result
    pub result: Option<GitOperationResult>,
    /// Attribution for this operation
    pub attribution: Option<Attribution>,
    /// Ceremony associated with operation
    pub ceremony_id: Option<String>,
}

/// Types of git operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GitOperationType {
    /// Clone repository
    Clone,
    /// Pull changes
    Pull,
    /// Push changes
    Push,
    /// Commit changes
    Commit,
    /// Create branch
    CreateBranch,
    /// Switch branch
    SwitchBranch,
    /// Merge branches
    Merge,
    /// Rebase operation
    Rebase,
    /// Cherry-pick commits
    CherryPick,
    /// Stash changes
    Stash,
    /// Apply stash
    StashApply,
    /// Tag creation
    Tag,
    /// Conflict resolution
    ConflictResolution,
    /// Repository initialization
    Init,
    /// Remote management
    RemoteManagement,
}

/// Git operation status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GitOperationStatus {
    /// Operation is queued
    Queued,
    /// Operation is running
    Running,
    /// Operation completed successfully
    Completed,
    /// Operation failed
    Failed,
    /// Operation was cancelled
    Cancelled,
    /// Operation requires user intervention
    RequiresIntervention,
    /// Operation is waiting for ceremony
    WaitingForCeremony,
}

impl GitManager {
    /// Create a new git manager
    pub fn new(config: GitManagerConfig) -> Result<Self> {
        info!("Initializing git manager with WeaveMesh Core integration");
        
        let repository_tracker = RepositoryTracker::new(&config)?;
        let operations_handler = GitOperationsHandler::new(&config)?;
        let attribution_engine = GitAttributionEngine::new(&config)?;
        let workflow_integrator = GitWorkflowIntegrator::new(&config)?;
        let conflict_detector = GitConflictDetector::new(&config)?;
        let state_tracker = GitStateTracker::new(&config)?;
        
        Ok(Self {
            repository_tracker,
            operations_handler,
            attribution_engine,
            workflow_integrator,
            conflict_detector,
            state_tracker,
            config,
            active_sessions: HashMap::new(),
        })
    }
    
    /// Start a new git session for a repository
    pub async fn start_session(&mut self, repository_path: &Path, owner_id: &str) -> Result<GitSession> {
        info!("Starting git session for repository: {:?}", repository_path);
        
        let session_id = Uuid::new_v4().to_string();
        let repository_id = self.repository_tracker.get_or_create_repository_id(repository_path).await?;
        
        // Get current branch
        let current_branch = self.operations_handler.get_current_branch(repository_path).await?;
        
        let session = GitSession {
            session_id: session_id.clone(),
            repository_id,
            repository_path: repository_path.to_path_buf(),
            current_branch,
            owner_id: owner_id.to_string(),
            started_at: Utc::now(),
            last_activity: Utc::now(),
            state: GitSessionState::Active,
            active_operations: Vec::new(),
            metadata: HashMap::new(),
        };
        
        self.active_sessions.insert(session_id.clone(), session.clone());
        
        info!("Git session started: {}", session_id);
        Ok(session)
    }
    
    /// Perform a git operation with attribution tracking
    pub async fn perform_operation(
        &mut self,
        session_id: &str,
        operation_type: GitOperationType,
        parameters: HashMap<String, String>,
        attribution: Option<Attribution>,
    ) -> Result<GitOperation> {
        debug!("Performing git operation: {:?} for session: {}", operation_type, session_id);
        
        // Clone session data to avoid borrowing conflicts
        let (repository_path, session_attribution) = {
            let session = self.active_sessions.get(session_id)
                .ok_or_else(|| anyhow::anyhow!("Session not found: {}", session_id))?;
            (session.repository_path.clone(), attribution.clone())
        };
        
        // Create attribution context for this operation
        let attribution_context = GitAttributionContext::from_git_operation(
            &operation_type,
            &parameters,
            &repository_path,
        );
        
        // Analyze attribution if enabled
        let analyzed_attribution = if self.config.enable_attribution_tracking {
            match self.attribution_engine.analyze_git_operation(&attribution_context).await {
                Ok(analysis) => Some(analysis.attribution),
                Err(e) => {
                    warn!("Attribution analysis failed: {}", e);
                    session_attribution
                }
            }
        } else {
            session_attribution
        };
        
        // Check if ceremony is required
        let ceremony_required = if self.config.enable_ceremony_integration {
            self.workflow_integrator.is_ceremony_required(&operation_type, &parameters).await?
        } else {
            false
        };
        
        let operation_id = Uuid::new_v4().to_string();
        let mut operation = GitOperation {
            operation_id: operation_id.clone(),
            operation_type: operation_type.clone(),
            status: if ceremony_required {
                GitOperationStatus::WaitingForCeremony
            } else {
                GitOperationStatus::Queued
            },
            parameters: parameters.clone(),
            started_at: Utc::now(),
            completed_at: None,
            result: None,
            attribution: analyzed_attribution.clone(),
            ceremony_id: None,
        };
        
        // If ceremony is required, initiate it
        if ceremony_required {
            let ceremony_id = self.workflow_integrator.initiate_operation_ceremony(
                &operation_type,
                &parameters,
                &analyzed_attribution,
            ).await?;
            operation.ceremony_id = Some(ceremony_id);
        } else {
            // Perform the operation immediately
            operation = self.execute_git_operation(&repository_path, operation).await?;
        }
        
        // Update session with the operation
        if let Some(session) = self.active_sessions.get_mut(session_id) {
            session.active_operations.push(operation.clone());
            session.last_activity = Utc::now();
        }
        
        Ok(operation)
    }
    
    /// Execute a git operation
    async fn execute_git_operation(&mut self, repository_path: &Path, mut operation: GitOperation) -> Result<GitOperation> {
        operation.status = GitOperationStatus::Running;
        
        let start_time = std::time::Instant::now();
        
        // Perform conflict detection before operation
        let pre_conflicts = self.conflict_detector.detect_conflicts(repository_path).await?;
        if !pre_conflicts.is_empty() && !self.can_proceed_with_conflicts(&operation.operation_type, &pre_conflicts) {
            operation.status = GitOperationStatus::RequiresIntervention;
            operation.result = Some(GitOperationResult {
                success: false,
                message: "Conflicts detected, intervention required".to_string(),
                changed_files: Vec::new(),
                commit_hash: None,
                conflicts: pre_conflicts,
                metrics: GitOperationMetrics {
                    duration_ms: start_time.elapsed().as_millis() as u64,
                    files_processed: 0,
                    bytes_transferred: 0,
                    network_latency_ms: None,
                    cpu_usage_percent: None,
                    memory_usage_bytes: None,
                    disk_io_operations: None,
                },
                ceremony_outcomes: Vec::new(),
            });
            return Ok(operation);
        }
        
        // Execute the actual git operation
        let result = self.operations_handler.execute_operation(
            repository_path,
            &operation.operation_type,
            &operation.parameters,
        ).await;
        
        let duration = start_time.elapsed();
        
        match result {
            Ok(operation_result) => {
                operation.status = GitOperationStatus::Completed;
                operation.completed_at = Some(Utc::now());
                operation.result = Some(operation_result);
                
                // Update repository state
                self.state_tracker.update_repository_state(repository_path).await?;
                
                // Check for post-operation conflicts
                let post_conflicts = self.conflict_detector.detect_conflicts(repository_path).await?;
                if let Some(ref mut result) = operation.result {
                    result.conflicts = post_conflicts;
                }
                
                info!("Git operation completed successfully: {} in {:?}", operation.operation_id, duration);
            }
            Err(e) => {
                operation.status = GitOperationStatus::Failed;
                operation.completed_at = Some(Utc::now());
                operation.result = Some(GitOperationResult {
                    success: false,
                    message: e.to_string(),
                    changed_files: Vec::new(),
                    commit_hash: None,
                    conflicts: Vec::new(),
                    metrics: GitOperationMetrics {
                        duration_ms: duration.as_millis() as u64,
                        files_processed: 0,
                        bytes_transferred: 0,
                        network_latency_ms: None,
                        cpu_usage_percent: None,
                        memory_usage_bytes: None,
                        disk_io_operations: None,
                    },
                    ceremony_outcomes: Vec::new(),
                });
                
                error!("Git operation failed: {} - {}", operation.operation_id, e);
            }
        }
        
        Ok(operation)
    }
    
    /// Check if operation can proceed with existing conflicts
    fn can_proceed_with_conflicts(&self, operation_type: &GitOperationType, conflicts: &[GitConflict]) -> bool {
        match operation_type {
            GitOperationType::ConflictResolution => true,
            GitOperationType::Stash | GitOperationType::StashApply => true,
            _ => conflicts.iter().all(|c| c.severity <= ConflictSeverity::Minor),
        }
    }
    
    /// Get session by ID
    pub fn get_session(&self, session_id: &str) -> Option<&GitSession> {
        self.active_sessions.get(session_id)
    }
    
    /// Get all active sessions
    pub fn get_active_sessions(&self) -> Vec<&GitSession> {
        self.active_sessions.values().collect()
    }
    
    /// End a git session
    pub async fn end_session(&mut self, session_id: &str) -> Result<()> {
        if let Some(mut session) = self.active_sessions.remove(session_id) {
            session.state = GitSessionState::Ended;
            info!("Git session ended: {}", session_id);
        }
        
        Ok(())
    }
    
    /// Get git manager statistics
    pub fn get_statistics(&self) -> GitManagerStatistics {
        GitManagerStatistics {
            active_sessions: self.active_sessions.len(),
            total_operations: self.active_sessions.values()
                .map(|s| s.active_operations.len())
                .sum(),
            repositories_tracked: self.repository_tracker.get_repository_count(),
            conflicts_detected: self.conflict_detector.get_total_conflicts_detected(),
            ceremonies_initiated: self.workflow_integrator.get_ceremonies_initiated(),
        }
    }
}

/// Statistics about git manager performance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitManagerStatistics {
    /// Number of active sessions
    pub active_sessions: usize,
    /// Total operations performed
    pub total_operations: usize,
    /// Number of repositories being tracked
    pub repositories_tracked: usize,
    /// Total conflicts detected
    pub conflicts_detected: usize,
    /// Total ceremonies initiated
    pub ceremonies_initiated: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_manager_config_default() {
        let config = GitManagerConfig::default();
        assert_eq!(config.max_concurrent_operations, 10);
        assert_eq!(config.conflict_detection_sensitivity, 0.8);
        assert!(config.enable_ceremony_integration);
        assert!(config.enable_attribution_tracking);
    }
    
    #[test]
    fn test_git_operation_serialization() {
        let operation_type = GitOperationType::Commit;
        let serialized = serde_json::to_string(&operation_type).unwrap();
        let deserialized: GitOperationType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(operation_type, deserialized);
    }
    
    #[test]
    fn test_git_session_state_transitions() {
        let mut session = GitSession {
            session_id: "test".to_string(),
            repository_id: "repo".to_string(),
            repository_path: PathBuf::from("/test"),
            current_branch: "main".to_string(),
            owner_id: "user".to_string(),
            started_at: Utc::now(),
            last_activity: Utc::now(),
            state: GitSessionState::Active,
            active_operations: Vec::new(),
            metadata: HashMap::new(),
        };
        
        assert_eq!(session.state, GitSessionState::Active);
        
        session.state = GitSessionState::Operating;
        assert_eq!(session.state, GitSessionState::Operating);
        
        session.state = GitSessionState::Ended;
        assert_eq!(session.state, GitSessionState::Ended);
    }
}
