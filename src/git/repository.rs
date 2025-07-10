//! Repository Tracking Module for WeaveMesh Core
//!
//! This module handles git repository scanning, state tracking, and metadata
//! management for the WeaveMesh Core git integration system.

use anyhow::Result;
use chrono::{DateTime, Utc};
use git2::{Repository, StatusOptions};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use tracing::{debug, info, warn, error};
use uuid::Uuid;

use super::GitManagerConfig;

/// Repository tracker for managing git repository state
pub struct RepositoryTracker {
    /// Configuration
    config: RepositoryTrackerConfig,
    /// Tracked repositories
    repositories: HashMap<String, TrackedRepository>,
    /// Repository path to ID mapping
    path_to_id: HashMap<PathBuf, String>,
    /// Repository health status
    health_status: HashMap<String, RepositoryHealth>,
}

/// Configuration for repository tracker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryTrackerConfig {
    /// Maximum number of repositories to track
    pub max_repositories: usize,
    /// Repository scan interval in seconds
    pub scan_interval_seconds: u64,
    /// Health check timeout in seconds
    pub health_check_timeout_seconds: u64,
    /// Enable automatic repository discovery
    pub enable_auto_discovery: bool,
    /// Repository metadata cache size
    pub metadata_cache_size: usize,
}

impl Default for RepositoryTrackerConfig {
    fn default() -> Self {
        Self {
            max_repositories: 100,
            scan_interval_seconds: 300, // 5 minutes
            health_check_timeout_seconds: 30,
            enable_auto_discovery: true,
            metadata_cache_size: 50,
        }
    }
}

/// Tracked repository information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackedRepository {
    /// Repository identifier
    pub repository_id: String,
    /// Repository path
    pub path: PathBuf,
    /// Repository name
    pub name: String,
    /// Repository URL (if remote)
    pub remote_url: Option<String>,
    /// Current branch
    pub current_branch: String,
    /// All branches
    pub branches: Vec<String>,
    /// Repository state
    pub state: RepositoryState,
    /// Repository metadata
    pub metadata: RepositoryMetadata,
    /// Last scan timestamp
    pub last_scanned: DateTime<Utc>,
    /// Repository statistics
    pub statistics: RepositoryStatistics,
}

/// Repository state information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryState {
    /// Working directory status
    pub working_directory_clean: bool,
    /// Staged changes count
    pub staged_changes: usize,
    /// Unstaged changes count
    pub unstaged_changes: usize,
    /// Untracked files count
    pub untracked_files: usize,
    /// Ahead commits count
    pub ahead_commits: usize,
    /// Behind commits count
    pub behind_commits: usize,
    /// Stash count
    pub stash_count: usize,
    /// Last commit hash
    pub last_commit_hash: Option<String>,
    /// Last commit timestamp
    pub last_commit_timestamp: Option<DateTime<Utc>>,
    /// Repository size in bytes
    pub repository_size_bytes: u64,
}

/// Repository metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryMetadata {
    /// Repository description
    pub description: Option<String>,
    /// Repository tags
    pub tags: Vec<String>,
    /// Repository contributors
    pub contributors: Vec<String>,
    /// Repository languages
    pub languages: HashMap<String, f64>,
    /// Repository license
    pub license: Option<String>,
    /// Repository creation date
    pub created_at: Option<DateTime<Utc>>,
    /// Last activity date
    pub last_activity: Option<DateTime<Utc>>,
    /// Custom metadata
    pub custom: HashMap<String, String>,
}

/// Repository statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryStatistics {
    /// Total commits
    pub total_commits: usize,
    /// Total files
    pub total_files: usize,
    /// Total lines of code
    pub total_lines_of_code: usize,
    /// Commit frequency (commits per day)
    pub commit_frequency: f64,
    /// Active contributors count
    pub active_contributors: usize,
    /// Average commit size
    pub average_commit_size: f64,
    /// Repository activity score
    pub activity_score: f64,
}

/// Repository health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryHealth {
    /// Overall health status
    pub status: HealthStatus,
    /// Health score (0.0 to 1.0)
    pub score: f64,
    /// Health checks
    pub checks: Vec<HealthCheck>,
    /// Last health check timestamp
    pub last_checked: DateTime<Utc>,
    /// Health issues
    pub issues: Vec<HealthIssue>,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Health status levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    /// Repository is healthy
    Healthy,
    /// Repository has minor issues
    Warning,
    /// Repository has significant issues
    Critical,
    /// Repository is inaccessible or corrupted
    Failed,
    /// Health status unknown
    Unknown,
}

/// Individual health check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    /// Check name
    pub name: String,
    /// Check status
    pub status: HealthCheckStatus,
    /// Check message
    pub message: String,
    /// Check duration in milliseconds
    pub duration_ms: u64,
    /// Check timestamp
    pub timestamp: DateTime<Utc>,
}

/// Health check status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthCheckStatus {
    /// Check passed
    Passed,
    /// Check failed
    Failed,
    /// Check skipped
    Skipped,
    /// Check timed out
    Timeout,
}

/// Health issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthIssue {
    /// Issue identifier
    pub issue_id: String,
    /// Issue type
    pub issue_type: HealthIssueType,
    /// Issue severity
    pub severity: IssueSeverity,
    /// Issue description
    pub description: String,
    /// Issue detected timestamp
    pub detected_at: DateTime<Utc>,
    /// Suggested fix
    pub suggested_fix: Option<String>,
    /// Issue resolution status
    pub resolution_status: IssueResolutionStatus,
}

/// Types of health issues
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthIssueType {
    /// Repository corruption
    Corruption,
    /// Missing files
    MissingFiles,
    /// Permission issues
    PermissionDenied,
    /// Network connectivity issues
    NetworkIssues,
    /// Disk space issues
    DiskSpace,
    /// Configuration issues
    Configuration,
    /// Performance issues
    Performance,
    /// Security issues
    Security,
}

/// Issue severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum IssueSeverity {
    /// Low severity issue
    Low,
    /// Medium severity issue
    Medium,
    /// High severity issue
    High,
    /// Critical severity issue
    Critical,
}

/// Issue resolution status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IssueResolutionStatus {
    /// Issue is open
    Open,
    /// Issue is being investigated
    InProgress,
    /// Issue has been resolved
    Resolved,
    /// Issue has been ignored
    Ignored,
    /// Issue resolution failed
    Failed,
}

impl RepositoryTracker {
    /// Create a new repository tracker
    pub fn new(git_config: &GitManagerConfig) -> Result<Self> {
        let config = RepositoryTrackerConfig {
            max_repositories: git_config.max_repository_cache_size,
            ..Default::default()
        };
        
        info!("Initializing repository tracker");
        
        Ok(Self {
            config,
            repositories: HashMap::new(),
            path_to_id: HashMap::new(),
            health_status: HashMap::new(),
        })
    }
    
    /// Get or create repository ID for a path
    pub async fn get_or_create_repository_id(&mut self, path: &Path) -> Result<String> {
        // Check if we already have this repository
        if let Some(repo_id) = self.path_to_id.get(path) {
            return Ok(repo_id.clone());
        }
        
        // Verify this is a git repository
        if !self.is_git_repository(path).await? {
            return Err(anyhow::anyhow!("Path is not a git repository: {:?}", path));
        }
        
        // Create new repository entry
        let repo_id = Uuid::new_v4().to_string();
        let tracked_repo = self.scan_repository(path, &repo_id).await?;
        
        self.repositories.insert(repo_id.clone(), tracked_repo);
        self.path_to_id.insert(path.to_path_buf(), repo_id.clone());
        
        info!("Registered new repository: {} at {:?}", repo_id, path);
        Ok(repo_id)
    }
    
    /// Check if path is a git repository
    async fn is_git_repository(&self, path: &Path) -> Result<bool> {
        match Repository::open(path) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
    
    /// Scan repository and collect information
    async fn scan_repository(&self, path: &Path, repo_id: &str) -> Result<TrackedRepository> {
        debug!("Scanning repository at {:?}", path);
        
        let repo = Repository::open(path)?;
        
        let name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();
        
        let current_branch = self.get_current_branch(&repo)?;
        let branches = self.get_all_branches(&repo)?;
        let remote_url = self.get_remote_url(&repo).ok();
        let state = self.get_repository_state(&repo)?;
        let metadata = self.get_repository_metadata(&repo)?;
        let statistics = self.calculate_repository_statistics(&repo)?;
        
        Ok(TrackedRepository {
            repository_id: repo_id.to_string(),
            path: path.to_path_buf(),
            name,
            remote_url,
            current_branch,
            branches,
            state,
            metadata,
            last_scanned: Utc::now(),
            statistics,
        })
    }
    
    /// Get current branch
    fn get_current_branch(&self, repo: &Repository) -> Result<String> {
        let head = repo.head()?;
        
        if let Some(branch_name) = head.shorthand() {
            Ok(branch_name.to_string())
        } else {
            Ok("HEAD".to_string())
        }
    }
    
    /// Get all branches
    fn get_all_branches(&self, repo: &Repository) -> Result<Vec<String>> {
        let branches = repo.branches(None)?;
        let mut branch_names = Vec::new();
        
        for branch in branches {
            let (branch, _) = branch?;
            if let Some(name) = branch.name()? {
                branch_names.push(name.to_string());
            }
        }
        
        Ok(branch_names)
    }
    
    /// Get remote URL
    fn get_remote_url(&self, repo: &Repository) -> Result<String> {
        let remote = repo.find_remote("origin")?;
        if let Some(url) = remote.url() {
            Ok(url.to_string())
        } else {
            Err(anyhow::anyhow!("No remote URL found"))
        }
    }
    
    /// Get repository state
    fn get_repository_state(&self, repo: &Repository) -> Result<RepositoryState> {
        let mut opts = StatusOptions::new();
        opts.include_untracked(true);
        
        let statuses = repo.statuses(Some(&mut opts))?;
        
        let mut staged_changes = 0;
        let mut unstaged_changes = 0;
        let mut untracked_files = 0;
        
        for entry in statuses.iter() {
            let status = entry.status();
            
            if status.is_index_new() || status.is_index_modified() || status.is_index_deleted() {
                staged_changes += 1;
            }
            
            if status.is_wt_new() {
                untracked_files += 1;
            } else if status.is_wt_modified() || status.is_wt_deleted() {
                unstaged_changes += 1;
            }
        }
        
        let working_directory_clean = staged_changes == 0 && unstaged_changes == 0 && untracked_files == 0;
        
        // Get last commit info
        let (last_commit_hash, last_commit_timestamp) = if let Ok(head) = repo.head() {
            if let Ok(commit) = head.peel_to_commit() {
                let hash = Some(commit.id().to_string());
                let timestamp = Some(DateTime::from_timestamp(commit.time().seconds(), 0).unwrap_or_else(Utc::now));
                (hash, timestamp)
            } else {
                (None, None)
            }
        } else {
            (None, None)
        };
        
        // Get stash count (simplified)
        let stash_count = 0; // Would need more complex implementation
        
        // Calculate repository size (simplified)
        let repository_size_bytes = self.calculate_repository_size(repo.path())?;
        
        Ok(RepositoryState {
            working_directory_clean,
            staged_changes,
            unstaged_changes,
            untracked_files,
            ahead_commits: 0, // Would need remote comparison
            behind_commits: 0, // Would need remote comparison
            stash_count,
            last_commit_hash,
            last_commit_timestamp,
            repository_size_bytes,
        })
    }
    
    /// Calculate repository size
    fn calculate_repository_size(&self, git_dir: &Path) -> Result<u64> {
        let mut size = 0;
        if git_dir.exists() {
            size = self.dir_size(git_dir)?;
        }
        Ok(size)
    }
    
    /// Calculate directory size recursively
    fn dir_size(&self, path: &Path) -> Result<u64> {
        let mut size = 0;
        if path.is_dir() {
            for entry in std::fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    size += self.dir_size(&path)?;
                } else {
                    size += entry.metadata()?.len();
                }
            }
        }
        Ok(size)
    }
    
    /// Get repository metadata
    fn get_repository_metadata(&self, repo: &Repository) -> Result<RepositoryMetadata> {
        // Get contributors (simplified)
        let contributors = self.get_contributors(repo)?;
        
        // Get tags
        let tags = self.get_tags(repo)?;
        
        // Get creation date (first commit)
        let created_at = self.get_first_commit_date(repo);
        
        // Get last activity (last commit)
        let last_activity = if let Ok(head) = repo.head() {
            if let Ok(commit) = head.peel_to_commit() {
                Some(DateTime::from_timestamp(commit.time().seconds(), 0).unwrap_or_else(Utc::now))
            } else {
                None
            }
        } else {
            None
        };
        
        Ok(RepositoryMetadata {
            description: None,
            tags,
            contributors,
            languages: HashMap::new(), // Would be populated by language detection
            license: None, // Would be detected from LICENSE file
            created_at,
            last_activity,
            custom: HashMap::new(),
        })
    }
    
    /// Get repository contributors (simplified)
    fn get_contributors(&self, repo: &Repository) -> Result<Vec<String>> {
        let mut contributors = std::collections::HashSet::new();
        
        let mut revwalk = repo.revwalk()?;
        revwalk.push_head()?;
        
        for oid in revwalk.take(100) { // Limit to last 100 commits for performance
            let oid = oid?;
            if let Ok(commit) = repo.find_commit(oid) {
                if let Some(author) = commit.author().name() {
                    contributors.insert(author.to_string());
                }
            }
        }
        
        Ok(contributors.into_iter().collect())
    }
    
    /// Get repository tags
    fn get_tags(&self, repo: &Repository) -> Result<Vec<String>> {
        let mut tags = Vec::new();
        
        repo.tag_foreach(|oid, name| {
            if let Ok(name_str) = std::str::from_utf8(name) {
                if name_str.starts_with("refs/tags/") {
                    tags.push(name_str.strip_prefix("refs/tags/").unwrap_or(name_str).to_string());
                }
            }
            true
        })?;
        
        Ok(tags)
    }
    
    /// Get first commit date
    fn get_first_commit_date(&self, repo: &Repository) -> Option<DateTime<Utc>> {
        // This is a simplified implementation - finding the first commit can be expensive
        if let Ok(head) = repo.head() {
            if let Ok(commit) = head.peel_to_commit() {
                return Some(DateTime::from_timestamp(commit.time().seconds(), 0).unwrap_or_else(Utc::now));
            }
        }
        None
    }
    
    /// Calculate repository statistics
    fn calculate_repository_statistics(&self, repo: &Repository) -> Result<RepositoryStatistics> {
        let total_commits = self.get_commit_count(repo)?;
        let total_files = self.get_file_count(repo)?;
        let total_lines_of_code = 0; // Simplified - would need file analysis
        let active_contributors = self.get_contributors(repo)?.len();
        
        // Calculate commit frequency (simplified)
        let commit_frequency = if let Some(created_at) = self.get_first_commit_date(repo) {
            let days_since_creation = (Utc::now() - created_at).num_days().max(1) as f64;
            total_commits as f64 / days_since_creation
        } else {
            0.0
        };
        
        // Calculate average commit size (simplified)
        let average_commit_size = if total_commits > 0 {
            total_lines_of_code as f64 / total_commits as f64
        } else {
            0.0
        };
        
        // Calculate activity score (simplified metric)
        let activity_score = (commit_frequency * 0.4 + active_contributors as f64 * 0.3 + (total_commits as f64 / 100.0) * 0.3).min(1.0);
        
        Ok(RepositoryStatistics {
            total_commits,
            total_files,
            total_lines_of_code,
            commit_frequency,
            active_contributors,
            average_commit_size,
            activity_score,
        })
    }
    
    /// Get total commit count
    fn get_commit_count(&self, repo: &Repository) -> Result<usize> {
        let mut revwalk = repo.revwalk()?;
        revwalk.push_head()?;
        Ok(revwalk.count())
    }
    
    /// Get file count
    fn get_file_count(&self, repo: &Repository) -> Result<usize> {
        if let Ok(head) = repo.head() {
            if let Ok(tree) = head.peel_to_tree() {
                return Ok(tree.len());
            }
        }
        Ok(0)
    }
    
    /// Get repository count
    pub fn get_repository_count(&self) -> usize {
        self.repositories.len()
    }
    
    /// Get repository by ID
    pub fn get_repository(&self, repo_id: &str) -> Option<&TrackedRepository> {
        self.repositories.get(repo_id)
    }
    
    /// Get all repositories
    pub fn get_all_repositories(&self) -> Vec<&TrackedRepository> {
        self.repositories.values().collect()
    }
    
    /// Remove repository from tracking
    pub fn remove_repository(&mut self, repo_id: &str) -> Result<()> {
        if let Some(repository) = self.repositories.remove(repo_id) {
            self.path_to_id.remove(&repository.path);
            self.health_status.remove(repo_id);
            info!("Removed repository from tracking: {}", repo_id);
        }
        Ok(())
    }
    
    /// Get repository health
    pub fn get_repository_health(&self, repo_id: &str) -> Option<&RepositoryHealth> {
        self.health_status.get(repo_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repository_tracker_config_default() {
        let config = RepositoryTrackerConfig::default();
        assert_eq!(config.max_repositories, 100);
        assert_eq!(config.scan_interval_seconds, 300);
        assert!(config.enable_auto_discovery);
    }
    
    #[test]
    fn test_health_status_equality() {
        assert_eq!(HealthStatus::Healthy, HealthStatus::Healthy);
        assert_ne!(HealthStatus::Healthy, HealthStatus::Warning);
    }
    
    #[test]
    fn test_issue_severity_ordering() {
        assert!(IssueSeverity::Critical > IssueSeverity::High);
        assert!(IssueSeverity::High > IssueSeverity::Medium);
        assert!(IssueSeverity::Medium > IssueSeverity::Low);
    }
    
    #[test]
    fn test_repository_state_serialization() {
        let state = RepositoryState {
            working_directory_clean: true,
            staged_changes: 0,
            unstaged_changes: 0,
            untracked_files: 0,
            ahead_commits: 0,
            behind_commits: 0,
            stash_count: 0,
            last_commit_hash: None,
            last_commit_timestamp: None,
            repository_size_bytes: 1024,
        };
        
        let serialized = serde_json::to_string(&state).unwrap();
        let deserialized: RepositoryState = serde_json::from_str(&serialized).unwrap();
        assert_eq!(state.working_directory_clean, deserialized.working_directory_clean);
        assert_eq!(state.repository_size_bytes, deserialized.repository_size_bytes);
    }
}
