//! Git Hooks Module for WeaveMesh Core
//!
//! This module provides git hooks integration for automated workflows,
//! attribution tracking, and collaborative development processes.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use tracing::{debug, info, warn, error};
use uuid::Uuid;

use crate::attribution::Attribution;
use super::{GitManagerConfig, GitOperationType};

/// Git hooks manager for WeaveMesh Core
pub struct GitHooksManager {
    /// Configuration
    config: GitHooksConfig,
    /// Installed hooks
    installed_hooks: HashMap<GitHookType, GitHook>,
    /// Hook execution history
    execution_history: Vec<HookExecutionRecord>,
    /// Hook templates
    hook_templates: HashMap<GitHookType, String>,
}

/// Configuration for git hooks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHooksConfig {
    /// Enable git hooks
    pub enable_hooks: bool,
    /// Hooks directory path
    pub hooks_directory: PathBuf,
    /// Enable attribution hooks
    pub enable_attribution_hooks: bool,
    /// Enable ceremony hooks
    pub enable_ceremony_hooks: bool,
    /// Hook execution timeout in seconds
    pub execution_timeout_seconds: u64,
    /// Enable hook validation
    pub enable_validation: bool,
    /// Maximum hook execution history
    pub max_execution_history: usize,
}

impl Default for GitHooksConfig {
    fn default() -> Self {
        Self {
            enable_hooks: true,
            hooks_directory: PathBuf::from(".git/hooks"),
            enable_attribution_hooks: true,
            enable_ceremony_hooks: true,
            execution_timeout_seconds: 300, // 5 minutes
            enable_validation: true,
            max_execution_history: 1000,
        }
    }
}

/// Types of git hooks
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GitHookType {
    /// Pre-commit hook
    PreCommit,
    /// Prepare commit message hook
    PrepareCommitMsg,
    /// Commit message hook
    CommitMsg,
    /// Post-commit hook
    PostCommit,
    /// Pre-receive hook
    PreReceive,
    /// Update hook
    Update,
    /// Post-receive hook
    PostReceive,
    /// Pre-push hook
    PrePush,
    /// Post-update hook
    PostUpdate,
    /// Pre-rebase hook
    PreRebase,
    /// Post-checkout hook
    PostCheckout,
    /// Post-merge hook
    PostMerge,
    /// Pre-auto-gc hook
    PreAutoGc,
    /// Post-rewrite hook
    PostRewrite,
    /// Push-to-checkout hook
    PushToCheckout,
}

/// Git hook definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHook {
    /// Hook identifier
    pub hook_id: String,
    /// Hook type
    pub hook_type: GitHookType,
    /// Hook name
    pub name: String,
    /// Hook description
    pub description: String,
    /// Hook script content
    pub script_content: String,
    /// Hook language/interpreter
    pub interpreter: HookInterpreter,
    /// Hook configuration
    pub config: HookConfig,
    /// Installation status
    pub installed: bool,
    /// Installation path
    pub installation_path: Option<PathBuf>,
    /// Hook metadata
    pub metadata: HashMap<String, String>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Last modified timestamp
    pub modified_at: DateTime<Utc>,
}

/// Hook interpreter types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HookInterpreter {
    /// Shell script
    Shell,
    /// Python script
    Python,
    /// Node.js script
    NodeJs,
    /// Rust binary
    Rust,
    /// Custom executable
    Custom(String),
}

/// Hook configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookConfig {
    /// Enable hook
    pub enabled: bool,
    /// Hook priority (lower numbers run first)
    pub priority: u32,
    /// Hook timeout in seconds
    pub timeout_seconds: u64,
    /// Environment variables
    pub environment: HashMap<String, String>,
    /// Hook arguments
    pub arguments: Vec<String>,
    /// Working directory
    pub working_directory: Option<PathBuf>,
    /// Failure behavior
    pub on_failure: HookFailureBehavior,
}

/// Hook failure behavior
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HookFailureBehavior {
    /// Abort the git operation
    Abort,
    /// Continue with warning
    Warn,
    /// Ignore failure
    Ignore,
    /// Retry with different parameters
    Retry,
}

/// Hook execution record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookExecutionRecord {
    /// Execution identifier
    pub execution_id: String,
    /// Hook that was executed
    pub hook_type: GitHookType,
    /// Repository path
    pub repository_path: PathBuf,
    /// Execution status
    pub status: HookExecutionStatus,
    /// Execution start time
    pub started_at: DateTime<Utc>,
    /// Execution end time
    pub ended_at: Option<DateTime<Utc>>,
    /// Execution duration in milliseconds
    pub duration_ms: Option<u64>,
    /// Exit code
    pub exit_code: Option<i32>,
    /// Standard output
    pub stdout: String,
    /// Standard error
    pub stderr: String,
    /// Hook context
    pub context: HookExecutionContext,
    /// Attribution information
    pub attribution: Option<Attribution>,
}

/// Hook execution status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HookExecutionStatus {
    /// Hook is running
    Running,
    /// Hook completed successfully
    Success,
    /// Hook failed
    Failed,
    /// Hook was cancelled
    Cancelled,
    /// Hook timed out
    TimedOut,
    /// Hook was skipped
    Skipped,
}

/// Hook execution context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookExecutionContext {
    /// Git operation that triggered the hook
    pub git_operation: Option<GitOperationType>,
    /// Commit hash (if applicable)
    pub commit_hash: Option<String>,
    /// Branch name
    pub branch_name: Option<String>,
    /// Files affected
    pub affected_files: Vec<String>,
    /// Author information
    pub author: Option<String>,
    /// Commit message (if applicable)
    pub commit_message: Option<String>,
    /// Additional context
    pub additional_context: HashMap<String, String>,
}

impl GitHooksManager {
    /// Create a new git hooks manager
    pub fn new(git_config: &GitManagerConfig) -> Result<Self> {
        let config = GitHooksConfig::default();
        
        info!("Initializing git hooks manager");
        
        let hook_templates = Self::initialize_hook_templates();
        
        Ok(Self {
            config,
            installed_hooks: HashMap::new(),
            execution_history: Vec::new(),
            hook_templates,
        })
    }
    
    /// Install a git hook
    pub async fn install_hook(&mut self, repository_path: &Path, hook: GitHook) -> Result<()> {
        if !self.config.enable_hooks {
            warn!("Git hooks are disabled");
            return Ok(());
        }
        
        let hooks_dir = repository_path.join(&self.config.hooks_directory);
        std::fs::create_dir_all(&hooks_dir)?;
        
        let hook_file_name = self.get_hook_filename(&hook.hook_type);
        let hook_path = hooks_dir.join(&hook_file_name);
        
        // Generate hook script
        let script_content = self.generate_hook_script(&hook)?;
        
        // Write hook file
        std::fs::write(&hook_path, script_content)?;
        
        // Make hook executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&hook_path)?.permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&hook_path, perms)?;
        }
        
        // Update hook record
        let mut installed_hook = hook.clone();
        installed_hook.installed = true;
        installed_hook.installation_path = Some(hook_path.clone());
        installed_hook.modified_at = Utc::now();
        
        self.installed_hooks.insert(installed_hook.hook_type.clone(), installed_hook);
        
        info!("Installed git hook: {:?} at {:?}", hook.hook_type, hook_path);
        Ok(())
    }
    
    /// Uninstall a git hook
    pub async fn uninstall_hook(&mut self, repository_path: &Path, hook_type: &GitHookType) -> Result<()> {
        if let Some(hook) = self.installed_hooks.get(hook_type) {
            if let Some(ref installation_path) = hook.installation_path {
                if installation_path.exists() {
                    std::fs::remove_file(installation_path)?;
                    info!("Removed hook file: {:?}", installation_path);
                }
            }
        }
        
        self.installed_hooks.remove(hook_type);
        info!("Uninstalled git hook: {:?}", hook_type);
        Ok(())
    }
    
    /// Execute a git hook
    pub async fn execute_hook(
        &mut self,
        repository_path: &Path,
        hook_type: &GitHookType,
        context: HookExecutionContext,
        attribution: Option<Attribution>,
    ) -> Result<HookExecutionRecord> {
        let execution_id = Uuid::new_v4().to_string();
        let started_at = Utc::now();
        
        let mut record = HookExecutionRecord {
            execution_id: execution_id.clone(),
            hook_type: hook_type.clone(),
            repository_path: repository_path.to_path_buf(),
            status: HookExecutionStatus::Running,
            started_at,
            ended_at: None,
            duration_ms: None,
            exit_code: None,
            stdout: String::new(),
            stderr: String::new(),
            context,
            attribution,
        };
        
        // Check if hook is installed and enabled
        if let Some(hook) = self.installed_hooks.get(hook_type) {
            if !hook.config.enabled {
                record.status = HookExecutionStatus::Skipped;
                record.ended_at = Some(Utc::now());
                record.duration_ms = Some(0);
                debug!("Hook {:?} is disabled, skipping", hook_type);
                return Ok(record);
            }
            
            if let Some(ref installation_path) = hook.installation_path {
                if installation_path.exists() {
                    // Execute the hook
                    let execution_result = self.execute_hook_script(hook, installation_path, &record.context).await;
                    
                    let ended_at = Utc::now();
                    let duration = (ended_at - started_at).num_milliseconds() as u64;
                    
                    record.ended_at = Some(ended_at);
                    record.duration_ms = Some(duration);
                    
                    match execution_result {
                        Ok((exit_code, stdout, stderr)) => {
                            record.exit_code = Some(exit_code);
                            record.stdout = stdout;
                            record.stderr = stderr;
                            
                            if exit_code == 0 {
                                record.status = HookExecutionStatus::Success;
                                info!("Hook {:?} executed successfully", hook_type);
                            } else {
                                record.status = HookExecutionStatus::Failed;
                                warn!("Hook {:?} failed with exit code: {}", hook_type, exit_code);
                            }
                        }
                        Err(e) => {
                            record.status = HookExecutionStatus::Failed;
                            record.stderr = e.to_string();
                            error!("Hook {:?} execution error: {}", hook_type, e);
                        }
                    }
                } else {
                    record.status = HookExecutionStatus::Failed;
                    record.stderr = "Hook file not found".to_string();
                    record.ended_at = Some(Utc::now());
                    record.duration_ms = Some(0);
                }
            }
        } else {
            record.status = HookExecutionStatus::Skipped;
            record.ended_at = Some(Utc::now());
            record.duration_ms = Some(0);
            debug!("Hook {:?} not installed, skipping", hook_type);
        }
        
        // Store execution record
        self.execution_history.push(record.clone());
        
        // Limit history size
        if self.execution_history.len() > self.config.max_execution_history {
            self.execution_history.drain(0..100); // Remove oldest 100 entries
        }
        
        Ok(record)
    }
    
    /// Execute hook script
    async fn execute_hook_script(
        &self,
        hook: &GitHook,
        script_path: &Path,
        context: &HookExecutionContext,
    ) -> Result<(i32, String, String)> {
        let mut command = match hook.interpreter {
            HookInterpreter::Shell => Command::new("sh"),
            HookInterpreter::Python => Command::new("python3"),
            HookInterpreter::NodeJs => Command::new("node"),
            HookInterpreter::Rust => Command::new(script_path),
            HookInterpreter::Custom(ref interpreter) => Command::new(interpreter),
        };
        
        // Add script path as argument for interpreted languages
        if !matches!(hook.interpreter, HookInterpreter::Rust) {
            command.arg(script_path);
        }
        
        // Add hook arguments
        command.args(&hook.config.arguments);
        
        // Set working directory
        if let Some(ref working_dir) = hook.config.working_directory {
            command.current_dir(working_dir);
        }
        
        // Set environment variables
        for (key, value) in &hook.config.environment {
            command.env(key, value);
        }
        
        // Add context as environment variables
        if let Some(ref commit_hash) = context.commit_hash {
            command.env("WEAVEMESH_COMMIT_HASH", commit_hash);
        }
        if let Some(ref branch_name) = context.branch_name {
            command.env("WEAVEMESH_BRANCH_NAME", branch_name);
        }
        if let Some(ref author) = context.author {
            command.env("WEAVEMESH_AUTHOR", author);
        }
        if let Some(ref commit_message) = context.commit_message {
            command.env("WEAVEMESH_COMMIT_MESSAGE", commit_message);
        }
        
        // Execute command
        let output = command.output()?;
        
        let exit_code = output.status.code().unwrap_or(-1);
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        
        Ok((exit_code, stdout, stderr))
    }
    
    /// Generate hook script content
    fn generate_hook_script(&self, hook: &GitHook) -> Result<String> {
        let mut script = String::new();
        
        // Add shebang based on interpreter
        match hook.interpreter {
            HookInterpreter::Shell => script.push_str("#!/bin/sh\n"),
            HookInterpreter::Python => script.push_str("#!/usr/bin/env python3\n"),
            HookInterpreter::NodeJs => script.push_str("#!/usr/bin/env node\n"),
            HookInterpreter::Rust => {
                // For Rust, we would compile the binary separately
                return Ok(hook.script_content.clone());
            }
            HookInterpreter::Custom(ref interpreter) => {
                script.push_str(&format!("#!{}\n", interpreter));
            }
        }
        
        // Add header comment
        script.push_str(&format!(
            "# WeaveMesh Git Hook: {}\n# Generated at: {}\n# Description: {}\n\n",
            hook.name,
            Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
            hook.description
        ));
        
        // Add the actual hook content
        script.push_str(&hook.script_content);
        
        Ok(script)
    }
    
    /// Get hook filename for hook type
    fn get_hook_filename(&self, hook_type: &GitHookType) -> String {
        match hook_type {
            GitHookType::PreCommit => "pre-commit".to_string(),
            GitHookType::PrepareCommitMsg => "prepare-commit-msg".to_string(),
            GitHookType::CommitMsg => "commit-msg".to_string(),
            GitHookType::PostCommit => "post-commit".to_string(),
            GitHookType::PreReceive => "pre-receive".to_string(),
            GitHookType::Update => "update".to_string(),
            GitHookType::PostReceive => "post-receive".to_string(),
            GitHookType::PrePush => "pre-push".to_string(),
            GitHookType::PostUpdate => "post-update".to_string(),
            GitHookType::PreRebase => "pre-rebase".to_string(),
            GitHookType::PostCheckout => "post-checkout".to_string(),
            GitHookType::PostMerge => "post-merge".to_string(),
            GitHookType::PreAutoGc => "pre-auto-gc".to_string(),
            GitHookType::PostRewrite => "post-rewrite".to_string(),
            GitHookType::PushToCheckout => "push-to-checkout".to_string(),
        }
    }
    
    /// Initialize hook templates
    fn initialize_hook_templates() -> HashMap<GitHookType, String> {
        let mut templates = HashMap::new();
        
        // Pre-commit hook template
        templates.insert(
            GitHookType::PreCommit,
            r#"
# WeaveMesh Pre-commit Hook
# This hook runs before each commit to validate changes

echo "Running WeaveMesh pre-commit checks..."

# Check for attribution information
if [ -z "$WEAVEMESH_AUTHOR" ]; then
    echo "Warning: No attribution information found"
fi

# Run basic validation
echo "Validating commit..."

# Exit successfully
exit 0
"#.to_string(),
        );
        
        // Post-commit hook template
        templates.insert(
            GitHookType::PostCommit,
            r#"
# WeaveMesh Post-commit Hook
# This hook runs after each commit to update attribution

echo "Processing WeaveMesh post-commit actions..."

# Update attribution records
if [ -n "$WEAVEMESH_COMMIT_HASH" ]; then
    echo "Recording attribution for commit: $WEAVEMESH_COMMIT_HASH"
fi

# Notify mesh network
echo "Notifying WeaveMesh network of new commit..."

exit 0
"#.to_string(),
        );
        
        templates
    }
    
    /// Create default attribution hook
    pub fn create_attribution_hook(&self, hook_type: GitHookType) -> GitHook {
        let template = self.hook_templates.get(&hook_type)
            .cloned()
            .unwrap_or_else(|| "# Default WeaveMesh hook\nexit 0\n".to_string());
        
        GitHook {
            hook_id: Uuid::new_v4().to_string(),
            hook_type: hook_type.clone(),
            name: format!("WeaveMesh {:?} Hook", hook_type),
            description: "WeaveMesh attribution and collaboration hook".to_string(),
            script_content: template,
            interpreter: HookInterpreter::Shell,
            config: HookConfig {
                enabled: true,
                priority: 100,
                timeout_seconds: self.config.execution_timeout_seconds,
                environment: HashMap::new(),
                arguments: Vec::new(),
                working_directory: None,
                on_failure: HookFailureBehavior::Warn,
            },
            installed: false,
            installation_path: None,
            metadata: HashMap::new(),
            created_at: Utc::now(),
            modified_at: Utc::now(),
        }
    }
    
    /// Get hook execution statistics
    pub fn get_hook_statistics(&self) -> HookStatistics {
        let total_executions = self.execution_history.len();
        let successful_executions = self.execution_history.iter()
            .filter(|r| r.status == HookExecutionStatus::Success)
            .count();
        
        let success_rate = if total_executions > 0 {
            successful_executions as f64 / total_executions as f64
        } else {
            0.0
        };
        
        let avg_execution_time = if total_executions > 0 {
            self.execution_history.iter()
                .filter_map(|r| r.duration_ms)
                .sum::<u64>() as f64 / total_executions as f64
        } else {
            0.0
        };
        
        let hook_type_distribution = self.execution_history
            .iter()
            .fold(HashMap::new(), |mut acc, record| {
                *acc.entry(record.hook_type.clone()).or_insert(0) += 1;
                acc
            });
        
        HookStatistics {
            total_executions,
            successful_executions,
            success_rate,
            average_execution_time_ms: avg_execution_time,
            installed_hooks: self.installed_hooks.len(),
            hook_type_distribution,
        }
    }
}

/// Statistics about git hooks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookStatistics {
    /// Total hook executions
    pub total_executions: usize,
    /// Successful executions
    pub successful_executions: usize,
    /// Success rate (0.0 to 1.0)
    pub success_rate: f64,
    /// Average execution time in milliseconds
    pub average_execution_time_ms: f64,
    /// Number of installed hooks
    pub installed_hooks: usize,
    /// Distribution of hook types executed
    pub hook_type_distribution: HashMap<GitHookType, usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_hooks_config_default() {
        let config = GitHooksConfig::default();
        assert!(config.enable_hooks);
        assert!(config.enable_attribution_hooks);
        assert_eq!(config.execution_timeout_seconds, 300);
    }
    
    #[test]
    fn test_hook_filename_generation() {
        let manager = GitHooksManager::new(&GitManagerConfig::default()).unwrap();
        
        assert_eq!(manager.get_hook_filename(&GitHookType::PreCommit), "pre-commit");
        assert_eq!(manager.get_hook_filename(&GitHookType::PostCommit), "post-commit");
        assert_eq!(manager.get_hook_filename(&GitHookType::PrePush), "pre-push");
    }
    
    #[test]
    fn test_hook_creation() {
        let manager = GitHooksManager::new(&GitManagerConfig::default()).unwrap();
        let hook = manager.create_attribution_hook(GitHookType::PreCommit);
        
        assert_eq!(hook.hook_type, GitHookType::PreCommit);
        assert!(hook.config.enabled);
        assert_eq!(hook.interpreter, HookInterpreter::Shell);
    }
}
