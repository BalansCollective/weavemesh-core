//! Git State Tracking Module for WeaveMesh Core
//!
//! This module provides real-time git state synchronization and tracking
//! for collaborative development environments.

use anyhow::Result;
use chrono::{DateTime, Utc};
use git2::{Repository, StatusOptions};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn, error};
use uuid::Uuid;

use crate::attribution::Attribution;
use super::{GitManagerConfig, GitOperationType};

/// Git state tracker for real-time synchronization
pub struct GitStateTracker {
    /// Configuration
    config: StateTrackingConfig,
    /// Repository states
    repository_states: HashMap<String, RepositoryState>,
    /// State change events
    state_events: Vec<StateChangeEvent>,
    /// State synchronization status
    sync_status: HashMap<String, SyncStatus>,
    /// State watchers
    watchers: Vec<StateWatcher>,
}

/// Configuration for git state tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateTrackingConfig {
    /// Enable state tracking
    pub enable_tracking: bool,
    /// State sync interval in seconds
    pub sync_interval_seconds: u64,
    /// Enable real-time monitoring
    pub enable_realtime_monitoring: bool,
    /// Maximum state events to keep
    pub max_state_events: usize,
    /// Enable state persistence
    pub enable_persistence: bool,
    /// State cache size
    pub cache_size: usize,
    /// Enable conflict detection
    pub enable_conflict_detection: bool,
}

impl Default for StateTrackingConfig {
    fn default() -> Self {
        Self {
            enable_tracking: true,
            sync_interval_seconds: 30,
            enable_realtime_monitoring: true,
            max_state_events: 10000,
            enable_persistence: true,
            cache_size: 1000,
            enable_conflict_detection: true,
        }
    }
}

/// Repository state information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryState {
    /// Repository identifier
    pub repository_id: String,
    /// Repository path
    pub repository_path: PathBuf,
    /// Current HEAD commit
    pub head_commit: Option<String>,
    /// Current branch
    pub current_branch: Option<String>,
    /// Repository status
    pub status: GitRepositoryStatus,
    /// Working directory state
    pub working_directory: WorkingDirectoryState,
    /// Branch information
    pub branches: Vec<BranchInfo>,
    /// Remote information
    pub remotes: Vec<RemoteInfo>,
    /// Stash information
    pub stashes: Vec<StashInfo>,
    /// Tags information
    pub tags: Vec<TagInfo>,
    /// Last update timestamp
    pub last_updated: DateTime<Utc>,
    /// State metadata
    pub metadata: HashMap<String, String>,
}

/// Git repository status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GitRepositoryStatus {
    /// Repository is clean
    Clean,
    /// Repository has uncommitted changes
    Dirty,
    /// Repository is in merge state
    Merging,
    /// Repository is in rebase state
    Rebasing,
    /// Repository is in cherry-pick state
    CherryPicking,
    /// Repository is in revert state
    Reverting,
    /// Repository is in bisect state
    Bisecting,
    /// Repository state is unknown
    Unknown,
}

/// Working directory state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkingDirectoryState {
    /// Modified files
    pub modified_files: Vec<FileStatus>,
    /// Staged files
    pub staged_files: Vec<FileStatus>,
    /// Untracked files
    pub untracked_files: Vec<String>,
    /// Ignored files
    pub ignored_files: Vec<String>,
    /// Conflicted files
    pub conflicted_files: Vec<String>,
    /// Total file count
    pub total_files: usize,
}

/// File status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileStatus {
    /// File path
    pub path: String,
    /// File status flags
    pub status: FileStatusFlags,
    /// File size
    pub size: Option<u64>,
    /// Last modified time
    pub modified_time: Option<DateTime<Utc>>,
}

/// File status flags
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileStatusFlags {
    /// File is new
    pub is_new: bool,
    /// File is modified
    pub is_modified: bool,
    /// File is deleted
    pub is_deleted: bool,
    /// File is renamed
    pub is_renamed: bool,
    /// File is typechanged
    pub is_typechanged: bool,
    /// File is ignored
    pub is_ignored: bool,
    /// File is conflicted
    pub is_conflicted: bool,
}

/// Branch type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BranchType {
    /// Local branch
    Local,
    /// Remote branch
    Remote,
}

/// Branch information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchInfo {
    /// Branch name
    pub name: String,
    /// Branch type (local/remote)
    pub branch_type: BranchType,
    /// Is current branch
    pub is_current: bool,
    /// HEAD commit
    pub head_commit: Option<String>,
    /// Upstream branch
    pub upstream: Option<String>,
    /// Ahead/behind counts
    pub ahead_behind: Option<(usize, usize)>,
}

/// Remote information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteInfo {
    /// Remote name
    pub name: String,
    /// Remote URL
    pub url: String,
    /// Fetch URL
    pub fetch_url: Option<String>,
    /// Push URL
    pub push_url: Option<String>,
}

/// Stash information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StashInfo {
    /// Stash index
    pub index: usize,
    /// Stash message
    pub message: String,
    /// Stash commit OID
    pub oid: String,
    /// Stash timestamp
    pub timestamp: DateTime<Utc>,
}

/// Tag information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagInfo {
    /// Tag name
    pub name: String,
    /// Tag target OID
    pub target_oid: String,
    /// Tag message (for annotated tags)
    pub message: Option<String>,
    /// Tag tagger
    pub tagger: Option<String>,
    /// Tag timestamp
    pub timestamp: Option<DateTime<Utc>>,
}

/// State change event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateChangeEvent {
    /// Event identifier
    pub event_id: String,
    /// Repository identifier
    pub repository_id: String,
    /// Event type
    pub event_type: StateChangeType,
    /// Event description
    pub description: String,
    /// Previous state (if applicable)
    pub previous_state: Option<String>,
    /// New state
    pub new_state: String,
    /// Files affected
    pub affected_files: Vec<String>,
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
    /// Event metadata
    pub metadata: HashMap<String, String>,
    /// Attribution information
    pub attribution: Option<Attribution>,
}

/// Types of state changes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum StateChangeType {
    /// Repository status changed
    StatusChange,
    /// Branch changed
    BranchChange,
    /// Commit added
    CommitAdded,
    /// Files modified
    FilesModified,
    /// Files staged
    FilesStaged,
    /// Files unstaged
    FilesUnstaged,
    /// Merge started
    MergeStarted,
    /// Merge completed
    MergeCompleted,
    /// Rebase started
    RebaseStarted,
    /// Rebase completed
    RebaseCompleted,
    /// Stash created
    StashCreated,
    /// Stash applied
    StashApplied,
    /// Tag created
    TagCreated,
    /// Remote added
    RemoteAdded,
    /// Remote removed
    RemoteRemoved,
    /// Conflict detected
    ConflictDetected,
    /// Conflict resolved
    ConflictResolved,
}

/// Synchronization status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStatus {
    /// Repository identifier
    pub repository_id: String,
    /// Last sync timestamp
    pub last_sync: DateTime<Utc>,
    /// Sync status
    pub status: SyncState,
    /// Sync errors
    pub errors: Vec<String>,
    /// Sync duration in milliseconds
    pub duration_ms: Option<u64>,
    /// Next sync scheduled time
    pub next_sync: Option<DateTime<Utc>>,
}

/// Synchronization state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SyncState {
    /// Sync is up to date
    UpToDate,
    /// Sync is in progress
    InProgress,
    /// Sync failed
    Failed,
    /// Sync is pending
    Pending,
    /// Sync is disabled
    Disabled,
}

/// State watcher for monitoring changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateWatcher {
    /// Watcher identifier
    pub watcher_id: String,
    /// Repository path to watch
    pub repository_path: PathBuf,
    /// Watch configuration
    pub config: WatcherConfig,
    /// Watcher status
    pub status: WatcherStatus,
    /// Last check timestamp
    pub last_check: Option<DateTime<Utc>>,
}

/// Watcher configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatcherConfig {
    /// Enable file system watching
    pub enable_fs_watching: bool,
    /// Watch interval in seconds
    pub watch_interval_seconds: u64,
    /// Events to watch for
    pub watch_events: Vec<StateChangeType>,
    /// Enable notifications
    pub enable_notifications: bool,
}

/// Watcher status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WatcherStatus {
    /// Watcher is active
    Active,
    /// Watcher is paused
    Paused,
    /// Watcher is stopped
    Stopped,
    /// Watcher has error
    Error,
}

impl GitStateTracker {
    /// Create a new git state tracker
    pub fn new(git_config: &GitManagerConfig) -> Result<Self> {
        let config = StateTrackingConfig::default();
        
        info!("Initializing git state tracker");
        
        Ok(Self {
            config,
            repository_states: HashMap::new(),
            state_events: Vec::new(),
            sync_status: HashMap::new(),
            watchers: Vec::new(),
        })
    }
    
    /// Update repository state
    pub async fn update_repository_state(&mut self, repository_path: &Path) -> Result<()> {
        if !self.config.enable_tracking {
            return Ok(());
        }
        
        debug!("Updating repository state: {:?}", repository_path);
        
        let repository_id = self.generate_repository_id(repository_path);
        let repo = Repository::open(repository_path)?;
        
        // Get current state
        let current_state = self.collect_repository_state(&repo, repository_path).await?;
        
        // Check for changes
        if let Some(previous_state) = self.repository_states.get(&repository_id) {
            let changes = self.detect_state_changes(previous_state, &current_state).await?;
            
            // Record state change events
            for change in changes {
                self.record_state_change_event(change).await?;
            }
        }
        
        // Update stored state
        self.repository_states.insert(repository_id.clone(), current_state);
        
        // Update sync status
        self.update_sync_status(&repository_id, SyncState::UpToDate, None).await?;
        
        debug!("Repository state updated: {}", repository_id);
        Ok(())
    }
    
    /// Collect current repository state
    async fn collect_repository_state(&self, repo: &Repository, repository_path: &Path) -> Result<RepositoryState> {
        let repository_id = self.generate_repository_id(repository_path);
        
        // Get HEAD commit
        let head_commit = repo.head()
            .ok()
            .and_then(|head| head.target())
            .map(|oid| oid.to_string());
        
        // Get current branch
        let current_branch = repo.head()
            .ok()
            .and_then(|head| head.shorthand().map(|s| s.to_string()));
        
        // Get repository status
        let repo_status = self.determine_repository_status(repo)?;
        
        // Get working directory state
        let working_directory = self.collect_working_directory_state(repo).await?;
        
        // Get branch information
        let branches = self.collect_branch_info(repo).await?;
        
        // Get remote information
        let remotes = self.collect_remote_info(repo).await?;
        
        // Get stash information
        let stashes = self.collect_stash_info(repo).await?;
        
        // Get tag information
        let tags = self.collect_tag_info(repo).await?;
        
        Ok(RepositoryState {
            repository_id,
            repository_path: repository_path.to_path_buf(),
            head_commit,
            current_branch,
            status: repo_status,
            working_directory,
            branches,
            remotes,
            stashes,
            tags,
            last_updated: Utc::now(),
            metadata: HashMap::new(),
        })
    }
    
    /// Determine repository status
    fn determine_repository_status(&self, repo: &Repository) -> Result<GitRepositoryStatus> {
        match repo.state() {
            git2::RepositoryState::Clean => Ok(GitRepositoryStatus::Clean),
            git2::RepositoryState::Merge => Ok(GitRepositoryStatus::Merging),
            git2::RepositoryState::Revert | git2::RepositoryState::RevertSequence => Ok(GitRepositoryStatus::Reverting),
            git2::RepositoryState::CherryPick | git2::RepositoryState::CherryPickSequence => Ok(GitRepositoryStatus::CherryPicking),
            git2::RepositoryState::Bisect => Ok(GitRepositoryStatus::Bisecting),
            git2::RepositoryState::Rebase | git2::RepositoryState::RebaseInteractive | git2::RepositoryState::RebaseMerge => Ok(GitRepositoryStatus::Rebasing),
            _ => {
                // Check if working directory is dirty
                let mut opts = StatusOptions::new();
                opts.include_untracked(true);
                opts.include_ignored(false);
                
                let statuses = repo.statuses(Some(&mut opts))?;
                if statuses.is_empty() {
                    Ok(GitRepositoryStatus::Clean)
                } else {
                    Ok(GitRepositoryStatus::Dirty)
                }
            }
        }
    }
    
    /// Collect working directory state
    async fn collect_working_directory_state(&self, repo: &Repository) -> Result<WorkingDirectoryState> {
        let mut opts = StatusOptions::new();
        opts.include_untracked(true);
        opts.include_ignored(true);
        
        let statuses = repo.statuses(Some(&mut opts))?;
        
        let mut modified_files = Vec::new();
        let mut staged_files = Vec::new();
        let mut untracked_files = Vec::new();
        let mut ignored_files = Vec::new();
        let mut conflicted_files = Vec::new();
        
        for entry in statuses.iter() {
            if let Some(path) = entry.path() {
                let status = entry.status();
                let file_status_flags = FileStatusFlags {
                    is_new: status.is_wt_new() || status.is_index_new(),
                    is_modified: status.is_wt_modified() || status.is_index_modified(),
                    is_deleted: status.is_wt_deleted() || status.is_index_deleted(),
                    is_renamed: status.is_wt_renamed() || status.is_index_renamed(),
                    is_typechanged: status.is_wt_typechange() || status.is_index_typechange(),
                    is_ignored: status.is_ignored(),
                    is_conflicted: status.is_conflicted(),
                };
                
                let file_status = FileStatus {
                    path: path.to_string(),
                    status: file_status_flags.clone(),
                    size: None, // Would need additional file system call
                    modified_time: None, // Would need additional file system call
                };
                
                if file_status_flags.is_conflicted {
                    conflicted_files.push(path.to_string());
                } else if status.is_ignored() {
                    ignored_files.push(path.to_string());
                } else if status.is_wt_new() {
                    untracked_files.push(path.to_string());
                } else if status.is_index_modified() || status.is_index_new() || status.is_index_deleted() {
                    staged_files.push(file_status.clone());
                }
                
                if status.is_wt_modified() || status.is_wt_deleted() || status.is_wt_typechange() {
                    modified_files.push(file_status);
                }
            }
        }
        
        Ok(WorkingDirectoryState {
            modified_files,
            staged_files,
            untracked_files,
            ignored_files,
            conflicted_files,
            total_files: statuses.len(),
        })
    }
    
    /// Collect branch information
    async fn collect_branch_info(&self, repo: &Repository) -> Result<Vec<BranchInfo>> {
        let mut branches = Vec::new();
        
        let branch_iter = repo.branches(Some(git2::BranchType::Local))?;
        for branch_result in branch_iter {
            if let Ok((branch, git2_branch_type)) = branch_result {
                if let Some(name) = branch.name()? {
                    let is_current = branch.is_head();
                    let head_commit = branch.get().target().map(|oid| oid.to_string());
                    let upstream = branch.upstream().ok()
                        .and_then(|upstream| {
                            upstream.name().ok()
                                .and_then(|name| name.map(|s| s.to_string()))
                        });
                    
                    // Convert git2::BranchType to our BranchType
                    let branch_type = match git2_branch_type {
                        git2::BranchType::Local => BranchType::Local,
                        git2::BranchType::Remote => BranchType::Remote,
                    };
                    
                    branches.push(BranchInfo {
                        name: name.to_string(),
                        branch_type,
                        is_current,
                        head_commit,
                        upstream,
                        ahead_behind: None, // Would need additional calculation
                    });
                }
            }
        }
        
        Ok(branches)
    }
    
    /// Collect remote information
    async fn collect_remote_info(&self, repo: &Repository) -> Result<Vec<RemoteInfo>> {
        let mut remotes = Vec::new();
        
        for remote_name in repo.remotes()?.iter() {
            if let Some(name) = remote_name {
                if let Ok(remote) = repo.find_remote(name) {
                    remotes.push(RemoteInfo {
                        name: name.to_string(),
                        url: remote.url().unwrap_or("").to_string(),
                        fetch_url: remote.url().map(|s| s.to_string()),
                        push_url: remote.pushurl().map(|s| s.to_string()),
                    });
                }
            }
        }
        
        Ok(remotes)
    }
    
    /// Collect stash information
    async fn collect_stash_info(&self, _repo: &Repository) -> Result<Vec<StashInfo>> {
        let stashes = Vec::new();
        
        // For now, return empty vector as stash_foreach has ownership issues
        // In a real implementation, we'd use a different git2 API or collect differently
        Ok(stashes)
    }
    
    /// Collect tag information
    async fn collect_tag_info(&self, repo: &Repository) -> Result<Vec<TagInfo>> {
        let mut tags = Vec::new();
        
        repo.tag_foreach(|oid, name| {
            if let Ok(name_str) = std::str::from_utf8(name) {
                tags.push(TagInfo {
                    name: name_str.to_string(),
                    target_oid: oid.to_string(),
                    message: None, // Would need to resolve tag object for message
                    tagger: None,
                    timestamp: None,
                });
            }
            true
        })?;
        
        Ok(tags)
    }
    
    /// Detect state changes between previous and current state
    async fn detect_state_changes(&self, previous: &RepositoryState, current: &RepositoryState) -> Result<Vec<StateChangeEvent>> {
        let mut changes = Vec::new();
        
        // Check status change
        if previous.status != current.status {
            changes.push(StateChangeEvent {
                event_id: Uuid::new_v4().to_string(),
                repository_id: current.repository_id.clone(),
                event_type: StateChangeType::StatusChange,
                description: format!("Repository status changed from {:?} to {:?}", previous.status, current.status),
                previous_state: Some(format!("{:?}", previous.status)),
                new_state: format!("{:?}", current.status),
                affected_files: Vec::new(),
                timestamp: Utc::now(),
                metadata: HashMap::new(),
                attribution: None,
            });
        }
        
        // Check branch change
        if previous.current_branch != current.current_branch {
            changes.push(StateChangeEvent {
                event_id: Uuid::new_v4().to_string(),
                repository_id: current.repository_id.clone(),
                event_type: StateChangeType::BranchChange,
                description: format!("Branch changed from {:?} to {:?}", previous.current_branch, current.current_branch),
                previous_state: previous.current_branch.clone(),
                new_state: current.current_branch.clone().unwrap_or_else(|| "unknown".to_string()),
                affected_files: Vec::new(),
                timestamp: Utc::now(),
                metadata: HashMap::new(),
                attribution: None,
            });
        }
        
        // Check commit change
        if previous.head_commit != current.head_commit {
            changes.push(StateChangeEvent {
                event_id: Uuid::new_v4().to_string(),
                repository_id: current.repository_id.clone(),
                event_type: StateChangeType::CommitAdded,
                description: "New commit detected".to_string(),
                previous_state: previous.head_commit.clone(),
                new_state: current.head_commit.clone().unwrap_or_else(|| "unknown".to_string()),
                affected_files: Vec::new(),
                timestamp: Utc::now(),
                metadata: HashMap::new(),
                attribution: None,
            });
        }
        
        // Check file changes
        let prev_modified_count = previous.working_directory.modified_files.len();
        let curr_modified_count = current.working_directory.modified_files.len();
        
        if prev_modified_count != curr_modified_count {
            changes.push(StateChangeEvent {
                event_id: Uuid::new_v4().to_string(),
                repository_id: current.repository_id.clone(),
                event_type: StateChangeType::FilesModified,
                description: format!("Modified files changed from {} to {}", prev_modified_count, curr_modified_count),
                previous_state: Some(prev_modified_count.to_string()),
                new_state: curr_modified_count.to_string(),
                affected_files: current.working_directory.modified_files.iter().map(|f| f.path.clone()).collect(),
                timestamp: Utc::now(),
                metadata: HashMap::new(),
                attribution: None,
            });
        }
        
        Ok(changes)
    }
    
    /// Record state change event
    async fn record_state_change_event(&mut self, event: StateChangeEvent) -> Result<()> {
        info!("Recording state change event: {:?} - {}", event.event_type, event.description);
        
        self.state_events.push(event);
        
        // Limit event history
        if self.state_events.len() > self.config.max_state_events {
            self.state_events.drain(0..1000); // Remove oldest 1000 events
        }
        
        Ok(())
    }
    
    /// Update sync status
    async fn update_sync_status(&mut self, repository_id: &str, state: SyncState, errors: Option<Vec<String>>) -> Result<()> {
        let sync_status = SyncStatus {
            repository_id: repository_id.to_string(),
            last_sync: Utc::now(),
            status: state,
            errors: errors.unwrap_or_default(),
            duration_ms: None,
            next_sync: None,
        };
        
        self.sync_status.insert(repository_id.to_string(), sync_status);
        Ok(())
    }
    
    /// Generate repository identifier
    fn generate_repository_id(&self, repository_path: &Path) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        repository_path.hash(&mut hasher);
        format!("repo_{:x}", hasher.finish())
    }
    
    /// Get repository state
    pub fn get_repository_state(&self, repository_path: &Path) -> Option<&RepositoryState> {
        let repository_id = self.generate_repository_id(repository_path);
        self.repository_states.get(&repository_id)
    }
    
    /// Get state change events for repository
    pub fn get_state_events(&self, repository_path: &Path) -> Vec<&StateChangeEvent> {
        let repository_id = self.generate_repository_id(repository_path);
        self.state_events
            .iter()
            .filter(|event| event.repository_id == repository_id)
            .collect()
    }
    
    /// Get state tracking statistics
    pub fn get_state_statistics(&self) -> StateTrackingStatistics {
        let total_repositories = self.repository_states.len();
        let total_events = self.state_events.len();
        
        let event_type_distribution = self.state_events
            .iter()
            .fold(HashMap::new(), |mut acc, event| {
                *acc.entry(event.event_type.clone()).or_insert(0) += 1;
                acc
            });
        
        let sync_status_distribution = self.sync_status
            .values()
            .fold(HashMap::new(), |mut acc, status| {
                *acc.entry(status.status.clone()).or_insert(0) += 1;
                acc
            });
        
        StateTrackingStatistics {
            total_repositories,
            total_events,
            active_watchers: self.watchers.iter().filter(|w| w.status == WatcherStatus::Active).count(),
            event_type_distribution,
            sync_status_distribution,
        }
    }
}

/// Statistics about state tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateTrackingStatistics {
    /// Total repositories being tracked
    pub total_repositories: usize,
    /// Total state change events
    pub total_events: usize,
    /// Number of active watchers
    pub active_watchers: usize,
    /// Distribution of event types
    pub event_type_distribution: HashMap<StateChangeType, usize>,
    /// Distribution of sync statuses
    pub sync_status_distribution: HashMap<SyncState, usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_tracking_config_default() {
        let config = StateTrackingConfig::default();
        assert!(config.enable_tracking);
        assert_eq!(config.sync_interval_seconds, 30);
        assert!(config.enable_realtime_monitoring);
    }
    
    #[test]
    fn test_repository_status_determination() {
        // This would require a real git repository for testing
        // For now, we'll test the enum values
        assert_ne!(GitRepositoryStatus::Clean, GitRepositoryStatus::Dirty);
        assert_ne!(GitRepositoryStatus::Merging, GitRepositoryStatus::Rebasing);
    }
    
    #[test]
    fn test_state_change_event_creation() {
        let event = StateChangeEvent {
            event_id: "test".to_string(),
            repository_id: "repo_123".to_string(),
            event_type: StateChangeType::StatusChange,
            description: "Test event".to_string(),
            previous_state: Some("Clean".to_string()),
            new_state: "Dirty".to_string(),
            affected_files: vec!["test.rs".to_string()],
            timestamp: Utc::now(),
            metadata: HashMap::new(),
            attribution: None,
        };
        
        assert_eq!(event.event_type, StateChangeType::StatusChange);
        assert_eq!(event.repository_id, "repo_123");
    }
}
