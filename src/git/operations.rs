//! Git Operations Module for WeaveMesh Core
//!
//! This module provides core git operations integrated with WeaveMesh's
//! attribution system and Sacred Alliance patterns.

use anyhow::Result;
use chrono::{DateTime, Utc};
use git2::{Repository, Signature, Oid, BranchType, StatusOptions};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn, error};

use super::{GitOperationType, GitManagerConfig};

/// Git operations handler for WeaveMesh Core
pub struct GitOperationsHandler {
    /// Configuration
    config: GitOperationsConfig,
    /// Operation metrics
    metrics: GitOperationMetrics,
}

/// Configuration for git operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitOperationsConfig {
    /// Default commit author name
    pub default_author_name: String,
    /// Default commit author email
    pub default_author_email: String,
    /// Maximum operation timeout in seconds
    pub operation_timeout_seconds: u64,
    /// Enable operation metrics collection
    pub enable_metrics: bool,
    /// Git operation retry count
    pub retry_count: u32,
    /// Enable automatic garbage collection
    pub enable_auto_gc: bool,
}

impl Default for GitOperationsConfig {
    fn default() -> Self {
        Self {
            default_author_name: "WeaveMesh".to_string(),
            default_author_email: "weavemesh@example.com".to_string(),
            operation_timeout_seconds: 300, // 5 minutes
            enable_metrics: true,
            retry_count: 3,
            enable_auto_gc: false,
        }
    }
}

/// Result of a git operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitOperationResult {
    /// Whether the operation succeeded
    pub success: bool,
    /// Operation result message
    pub message: String,
    /// Files that were changed
    pub changed_files: Vec<String>,
    /// Commit hash if applicable
    pub commit_hash: Option<String>,
    /// Any conflicts detected
    pub conflicts: Vec<GitConflict>,
    /// Operation metrics
    pub metrics: GitOperationMetrics,
    /// Ceremony outcomes if applicable
    pub ceremony_outcomes: Vec<String>,
}

// GitConflict types moved to conflict_detection module for unified pattern recognition
use crate::git::conflict_detection::{GitConflict, ConflictType, ConflictSeverity};

/// Metrics for git operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitOperationMetrics {
    /// Operation duration in milliseconds
    pub duration_ms: u64,
    /// Number of files processed
    pub files_processed: usize,
    /// Bytes transferred (for network operations)
    pub bytes_transferred: u64,
    /// Network latency in milliseconds
    pub network_latency_ms: Option<u64>,
    /// CPU usage percentage during operation
    pub cpu_usage_percent: Option<f64>,
    /// Memory usage in bytes
    pub memory_usage_bytes: Option<u64>,
    /// Disk I/O operations count
    pub disk_io_operations: Option<u64>,
}

impl Default for GitOperationMetrics {
    fn default() -> Self {
        Self {
            duration_ms: 0,
            files_processed: 0,
            bytes_transferred: 0,
            network_latency_ms: None,
            cpu_usage_percent: None,
            memory_usage_bytes: None,
            disk_io_operations: None,
        }
    }
}

impl GitOperationsHandler {
    /// Create a new git operations handler
    pub fn new(git_config: &GitManagerConfig) -> Result<Self> {
        let config = GitOperationsConfig::default();
        
        info!("Initializing git operations handler");
        
        Ok(Self {
            config,
            metrics: GitOperationMetrics::default(),
        })
    }
    
    /// Execute a git operation
    pub async fn execute_operation(
        &mut self,
        repository_path: &Path,
        operation_type: &GitOperationType,
        parameters: &HashMap<String, String>,
    ) -> Result<GitOperationResult> {
        let start_time = std::time::Instant::now();
        
        debug!("Executing git operation: {:?} at {:?}", operation_type, repository_path);
        
        let result = match operation_type {
            GitOperationType::Clone => self.clone_repository(repository_path, parameters).await,
            GitOperationType::Pull => self.pull_changes(repository_path, parameters).await,
            GitOperationType::Push => self.push_changes(repository_path, parameters).await,
            GitOperationType::Commit => self.commit_changes(repository_path, parameters).await,
            GitOperationType::CreateBranch => self.create_branch(repository_path, parameters).await,
            GitOperationType::SwitchBranch => self.switch_branch(repository_path, parameters).await,
            GitOperationType::Merge => self.merge_branches(repository_path, parameters).await,
            GitOperationType::Stash => self.stash_changes(repository_path, parameters).await,
            GitOperationType::StashApply => self.apply_stash(repository_path, parameters).await,
            GitOperationType::Tag => self.create_tag(repository_path, parameters).await,
            GitOperationType::Init => self.init_repository(repository_path, parameters).await,
            _ => {
                warn!("Unsupported git operation: {:?}", operation_type);
                Ok(GitOperationResult {
                    success: false,
                    message: format!("Unsupported operation: {:?}", operation_type),
                    changed_files: Vec::new(),
                    commit_hash: None,
                    conflicts: Vec::new(),
                    metrics: GitOperationMetrics {
                        duration_ms: start_time.elapsed().as_millis() as u64,
                        ..Default::default()
                    },
                    ceremony_outcomes: Vec::new(),
                })
            }
        };
        
        let duration = start_time.elapsed();
        
        match result {
            Ok(mut op_result) => {
                op_result.metrics.duration_ms = duration.as_millis() as u64;
                info!("Git operation {:?} completed in {:?}", operation_type, duration);
                Ok(op_result)
            }
            Err(e) => {
                error!("Git operation {:?} failed: {}", operation_type, e);
                Ok(GitOperationResult {
                    success: false,
                    message: e.to_string(),
                    changed_files: Vec::new(),
                    commit_hash: None,
                    conflicts: Vec::new(),
                    metrics: GitOperationMetrics {
                        duration_ms: duration.as_millis() as u64,
                        ..Default::default()
                    },
                    ceremony_outcomes: Vec::new(),
                })
            }
        }
    }
    
    /// Get current branch name
    pub async fn get_current_branch(&self, repository_path: &Path) -> Result<String> {
        let repo = Repository::open(repository_path)?;
        let head = repo.head()?;
        
        if let Some(branch_name) = head.shorthand() {
            Ok(branch_name.to_string())
        } else {
            Ok("HEAD".to_string())
        }
    }
    
    /// Initialize a new git repository
    async fn init_repository(&self, repository_path: &Path, _parameters: &HashMap<String, String>) -> Result<GitOperationResult> {
        let repo = Repository::init(repository_path)?;
        
        // Create initial commit if requested
        let signature = Signature::now(&self.config.default_author_name, &self.config.default_author_email)?;
        let tree_id = {
            let mut index = repo.index()?;
            index.write_tree()?
        };
        let tree = repo.find_tree(tree_id)?;
        
        let _commit_id = repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            "Initial commit",
            &tree,
            &[],
        )?;
        
        Ok(GitOperationResult {
            success: true,
            message: "Repository initialized successfully".to_string(),
            changed_files: Vec::new(),
            commit_hash: None,
            conflicts: Vec::new(),
            metrics: GitOperationMetrics::default(),
            ceremony_outcomes: Vec::new(),
        })
    }
    
    /// Clone a repository
    async fn clone_repository(&self, repository_path: &Path, parameters: &HashMap<String, String>) -> Result<GitOperationResult> {
        let url = parameters.get("url")
            .ok_or_else(|| anyhow::anyhow!("Clone URL not provided"))?;
        
        let _repo = Repository::clone(url, repository_path)?;
        
        Ok(GitOperationResult {
            success: true,
            message: format!("Repository cloned from {}", url),
            changed_files: Vec::new(),
            commit_hash: None,
            conflicts: Vec::new(),
            metrics: GitOperationMetrics::default(),
            ceremony_outcomes: Vec::new(),
        })
    }
    
    /// Pull changes from remote
    async fn pull_changes(&self, repository_path: &Path, _parameters: &HashMap<String, String>) -> Result<GitOperationResult> {
        let repo = Repository::open(repository_path)?;
        
        // This is a simplified pull - in practice you'd want to handle remotes, authentication, etc.
        let mut remote = repo.find_remote("origin")?;
        remote.fetch(&[] as &[&str], None, None)?;
        
        // For now, just return success - full merge logic would be more complex
        Ok(GitOperationResult {
            success: true,
            message: "Changes pulled successfully".to_string(),
            changed_files: Vec::new(),
            commit_hash: None,
            conflicts: Vec::new(),
            metrics: GitOperationMetrics::default(),
            ceremony_outcomes: Vec::new(),
        })
    }
    
    /// Push changes to remote
    async fn push_changes(&self, repository_path: &Path, parameters: &HashMap<String, String>) -> Result<GitOperationResult> {
        let repo = Repository::open(repository_path)?;
        let branch = parameters.get("branch").map(|s| s.as_str()).unwrap_or("main");
        
        let mut remote = repo.find_remote("origin")?;
        let refspec = format!("refs/heads/{}:refs/heads/{}", branch, branch);
        remote.push(&[&refspec], None)?;
        
        Ok(GitOperationResult {
            success: true,
            message: format!("Changes pushed to {}", branch),
            changed_files: Vec::new(),
            commit_hash: None,
            conflicts: Vec::new(),
            metrics: GitOperationMetrics::default(),
            ceremony_outcomes: Vec::new(),
        })
    }
    
    /// Commit changes
    async fn commit_changes(&self, repository_path: &Path, parameters: &HashMap<String, String>) -> Result<GitOperationResult> {
        let repo = Repository::open(repository_path)?;
        let message = parameters.get("message")
            .ok_or_else(|| anyhow::anyhow!("Commit message not provided"))?;
        
        let signature = Signature::now(&self.config.default_author_name, &self.config.default_author_email)?;
        
        // Add all changes to index
        let mut index = repo.index()?;
        index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
        index.write()?;
        
        let tree_id = index.write_tree()?;
        let tree = repo.find_tree(tree_id)?;
        
        let parent_commit = repo.head()?.peel_to_commit()?;
        let commit_id = repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &[&parent_commit],
        )?;
        
        // Get list of changed files
        let changed_files = self.get_changed_files(&repo)?;
        
        Ok(GitOperationResult {
            success: true,
            message: format!("Changes committed: {}", message),
            changed_files,
            commit_hash: Some(commit_id.to_string()),
            conflicts: Vec::new(),
            metrics: GitOperationMetrics::default(),
            ceremony_outcomes: Vec::new(),
        })
    }
    
    /// Create a new branch
    async fn create_branch(&self, repository_path: &Path, parameters: &HashMap<String, String>) -> Result<GitOperationResult> {
        let repo = Repository::open(repository_path)?;
        let branch_name = parameters.get("name")
            .ok_or_else(|| anyhow::anyhow!("Branch name not provided"))?;
        
        let head_commit = repo.head()?.peel_to_commit()?;
        repo.branch(branch_name, &head_commit, false)?;
        
        Ok(GitOperationResult {
            success: true,
            message: format!("Branch '{}' created successfully", branch_name),
            changed_files: Vec::new(),
            commit_hash: None,
            conflicts: Vec::new(),
            metrics: GitOperationMetrics::default(),
            ceremony_outcomes: Vec::new(),
        })
    }
    
    /// Switch to a different branch
    async fn switch_branch(&self, repository_path: &Path, parameters: &HashMap<String, String>) -> Result<GitOperationResult> {
        let repo = Repository::open(repository_path)?;
        let branch_name = parameters.get("name")
            .ok_or_else(|| anyhow::anyhow!("Branch name not provided"))?;
        
        let branch = repo.find_branch(branch_name, BranchType::Local)?;
        let branch_ref = branch.get();
        repo.set_head(branch_ref.name().unwrap())?;
        repo.checkout_head(None)?;
        
        Ok(GitOperationResult {
            success: true,
            message: format!("Switched to branch '{}'", branch_name),
            changed_files: Vec::new(),
            commit_hash: None,
            conflicts: Vec::new(),
            metrics: GitOperationMetrics::default(),
            ceremony_outcomes: Vec::new(),
        })
    }
    
    /// Merge branches
    async fn merge_branches(&self, repository_path: &Path, parameters: &HashMap<String, String>) -> Result<GitOperationResult> {
        let repo = Repository::open(repository_path)?;
        let source_branch = parameters.get("source")
            .ok_or_else(|| anyhow::anyhow!("Source branch not provided"))?;
        
        let source_branch_ref = repo.find_branch(source_branch, BranchType::Local)?;
        let source_commit = source_branch_ref.get().peel_to_commit()?;
        let head_commit = repo.head()?.peel_to_commit()?;
        
        // Perform merge analysis
        let annotated_commit = repo.find_annotated_commit(source_commit.id())?;
        let analysis = repo.merge_analysis(&[&annotated_commit])?;
        
        if analysis.0.is_up_to_date() {
            return Ok(GitOperationResult {
                success: true,
                message: "Already up to date".to_string(),
                changed_files: Vec::new(),
                commit_hash: None,
                conflicts: Vec::new(),
                metrics: GitOperationMetrics::default(),
                ceremony_outcomes: Vec::new(),
            });
        }
        
        if analysis.0.is_fast_forward() {
            // Fast-forward merge
            let target_oid = source_commit.id();
            let mut reference = repo.head()?;
            reference.set_target(target_oid, "Fast-forward merge")?;
            repo.set_head(reference.name().unwrap())?;
            repo.checkout_head(None)?;
            
            Ok(GitOperationResult {
                success: true,
                message: format!("Fast-forward merged '{}'", source_branch),
                changed_files: Vec::new(),
                commit_hash: Some(target_oid.to_string()),
                conflicts: Vec::new(),
                metrics: GitOperationMetrics::default(),
                ceremony_outcomes: Vec::new(),
            })
        } else {
            // This would require more complex merge logic
            Ok(GitOperationResult {
                success: false,
                message: "Complex merge not yet implemented".to_string(),
                changed_files: Vec::new(),
                commit_hash: None,
                conflicts: Vec::new(),
                metrics: GitOperationMetrics::default(),
                ceremony_outcomes: Vec::new(),
            })
        }
    }
    
    /// Stash changes
    async fn stash_changes(&self, repository_path: &Path, parameters: &HashMap<String, String>) -> Result<GitOperationResult> {
        let mut repo = Repository::open(repository_path)?;
        let message = parameters.get("message").map(|s| s.as_str()).unwrap_or("WeaveMesh stash");
        
        let signature = Signature::now(&self.config.default_author_name, &self.config.default_author_email)?;
        let _stash_id = repo.stash_save(&signature, message, None)?;
        
        Ok(GitOperationResult {
            success: true,
            message: "Changes stashed successfully".to_string(),
            changed_files: Vec::new(),
            commit_hash: None,
            conflicts: Vec::new(),
            metrics: GitOperationMetrics::default(),
            ceremony_outcomes: Vec::new(),
        })
    }
    
    /// Apply stash
    async fn apply_stash(&self, repository_path: &Path, parameters: &HashMap<String, String>) -> Result<GitOperationResult> {
        let mut repo = Repository::open(repository_path)?;
        let stash_index = parameters.get("index")
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(0);
        
        repo.stash_apply(stash_index, None)?;
        
        Ok(GitOperationResult {
            success: true,
            message: "Stash applied successfully".to_string(),
            changed_files: Vec::new(),
            commit_hash: None,
            conflicts: Vec::new(),
            metrics: GitOperationMetrics::default(),
            ceremony_outcomes: Vec::new(),
        })
    }
    
    /// Create a tag
    async fn create_tag(&self, repository_path: &Path, parameters: &HashMap<String, String>) -> Result<GitOperationResult> {
        let repo = Repository::open(repository_path)?;
        let tag_name = parameters.get("name")
            .ok_or_else(|| anyhow::anyhow!("Tag name not provided"))?;
        let message = parameters.get("message").unwrap_or(tag_name);
        
        let head_commit = repo.head()?.peel_to_commit()?;
        let signature = Signature::now(&self.config.default_author_name, &self.config.default_author_email)?;
        
        repo.tag(tag_name, head_commit.as_object(), &signature, message, false)?;
        
        Ok(GitOperationResult {
            success: true,
            message: format!("Tag '{}' created successfully", tag_name),
            changed_files: Vec::new(),
            commit_hash: Some(head_commit.id().to_string()),
            conflicts: Vec::new(),
            metrics: GitOperationMetrics::default(),
            ceremony_outcomes: Vec::new(),
        })
    }
    
    /// Get list of changed files
    fn get_changed_files(&self, repo: &Repository) -> Result<Vec<String>> {
        let mut opts = StatusOptions::new();
        opts.include_untracked(true);
        
        let statuses = repo.statuses(Some(&mut opts))?;
        let mut changed_files = Vec::new();
        
        for entry in statuses.iter() {
            if let Some(path) = entry.path() {
                changed_files.push(path.to_string());
            }
        }
        
        Ok(changed_files)
    }
    
    /// Get operation metrics
    pub fn get_metrics(&self) -> &GitOperationMetrics {
        &self.metrics
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_git_operations_config_default() {
        let config = GitOperationsConfig::default();
        assert_eq!(config.default_author_name, "WeaveMesh");
        assert_eq!(config.operation_timeout_seconds, 300);
        assert!(config.enable_metrics);
    }
    
    #[test]
    fn test_conflict_severity_ordering() {
        assert!(ConflictSeverity::Critical > ConflictSeverity::Major);
        assert!(ConflictSeverity::Major > ConflictSeverity::Moderate);
        assert!(ConflictSeverity::Moderate > ConflictSeverity::Minor);
    }
    
    #[test]
    fn test_git_operation_result_serialization() {
        let result = GitOperationResult {
            success: true,
            message: "Test operation".to_string(),
            changed_files: vec!["test.txt".to_string()],
            commit_hash: Some("abc123".to_string()),
            conflicts: Vec::new(),
            metrics: GitOperationMetrics::default(),
            ceremony_outcomes: Vec::new(),
        };
        
        let serialized = serde_json::to_string(&result).unwrap();
        let deserialized: GitOperationResult = serde_json::from_str(&serialized).unwrap();
        assert_eq!(result.success, deserialized.success);
        assert_eq!(result.message, deserialized.message);
    }
    
    #[tokio::test]
    async fn test_git_operations_handler_creation() {
        let git_config = GitManagerConfig::default();
        let handler = GitOperationsHandler::new(&git_config);
        assert!(handler.is_ok());
    }
}
