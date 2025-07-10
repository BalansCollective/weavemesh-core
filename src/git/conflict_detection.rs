//! Git Conflict Detection Module for WeaveMesh Core
//!
//! This module provides advanced conflict detection and analysis for git operations,
//! enabling proactive conflict resolution and collaborative decision-making.

use anyhow::Result;
use chrono::{DateTime, Utc};
use git2::{Repository, StatusOptions, DiffOptions};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn, error};
use uuid::Uuid;

use super::{GitManagerConfig, GitOperationType};

/// Git conflict detector for identifying and analyzing conflicts
pub struct GitConflictDetector {
    /// Configuration
    config: ConflictDetectionConfig,
    /// Detected conflicts cache
    conflicts_cache: HashMap<String, Vec<GitConflict>>,
    /// Conflict resolution history
    resolution_history: Vec<ConflictResolutionRecord>,
    /// Conflict patterns
    conflict_patterns: HashMap<String, ConflictPattern>,
}

/// Configuration for conflict detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictDetectionConfig {
    /// Enable proactive conflict detection
    pub enable_proactive_detection: bool,
    /// Conflict detection sensitivity (0.0 to 1.0)
    pub detection_sensitivity: f64,
    /// Enable semantic conflict detection
    pub enable_semantic_detection: bool,
    /// Cache size for conflict data
    pub cache_size: usize,
    /// Conflict analysis timeout in seconds
    pub analysis_timeout_seconds: u64,
    /// Enable conflict prediction
    pub enable_prediction: bool,
    /// Minimum confidence for conflict prediction
    pub prediction_confidence_threshold: f64,
}

impl Default for ConflictDetectionConfig {
    fn default() -> Self {
        Self {
            enable_proactive_detection: true,
            detection_sensitivity: 0.7,
            enable_semantic_detection: true,
            cache_size: 1000,
            analysis_timeout_seconds: 60,
            enable_prediction: true,
            prediction_confidence_threshold: 0.6,
        }
    }
}

/// Git conflict information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitConflict {
    /// Conflict identifier
    pub conflict_id: String,
    /// Conflict type
    pub conflict_type: ConflictType,
    /// Conflict severity
    pub severity: ConflictSeverity,
    /// File path with conflict
    pub file_path: String,
    /// Conflict location in file
    pub location: ConflictLocation,
    /// Conflict description
    pub description: String,
    /// Conflicting branches/commits
    pub conflicting_refs: Vec<String>,
    /// Conflict content
    pub conflict_content: ConflictContent,
    /// Suggested resolutions
    pub suggested_resolutions: Vec<ConflictResolution>,
    /// Conflict metadata
    pub metadata: HashMap<String, String>,
    /// Detection timestamp
    pub detected_at: DateTime<Utc>,
    /// Resolution status
    pub resolution_status: ConflictResolutionStatus,
}

/// Types of git conflicts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ConflictType {
    /// Content conflict in file
    ContentConflict,
    /// File was deleted in one branch, modified in another
    DeleteModify,
    /// File was added in both branches with different content
    AddAdd,
    /// File was renamed differently in both branches
    RenameRename,
    /// File mode conflict (permissions)
    ModeConflict,
    /// Submodule conflict
    SubmoduleConflict,
    /// Semantic conflict (code logic conflicts)
    SemanticConflict,
    /// Structural conflict (architecture changes)
    StructuralConflict,
    /// Attribution conflict (ownership disputes)
    AttributionConflict,
    /// Dependency conflict (version mismatches)
    DependencyConflict,
}

/// Conflict severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConflictSeverity {
    /// Minor conflict, easily resolvable
    Minor,
    /// Moderate conflict, requires attention
    Moderate,
    /// Major conflict, requires careful resolution
    Major,
    /// Critical conflict, may break functionality
    Critical,
    /// Blocking conflict, prevents operation
    Blocking,
}

/// Conflict location in file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictLocation {
    /// Start line number
    pub start_line: usize,
    /// End line number
    pub end_line: usize,
    /// Start column (optional)
    pub start_column: Option<usize>,
    /// End column (optional)
    pub end_column: Option<usize>,
    /// Function or section name (if applicable)
    pub context: Option<String>,
}

/// Conflict content details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictContent {
    /// Content from "ours" side
    pub ours: String,
    /// Content from "theirs" side
    pub theirs: String,
    /// Common ancestor content (if available)
    pub base: Option<String>,
    /// Conflict markers present
    pub has_markers: bool,
    /// Content type (text, binary, etc.)
    pub content_type: ContentType,
}

/// Types of file content
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContentType {
    /// Text content
    Text,
    /// Binary content
    Binary,
    /// Image content
    Image,
    /// Configuration file
    Configuration,
    /// Source code
    SourceCode,
    /// Documentation
    Documentation,
    /// Unknown content type
    Unknown,
}

/// Conflict resolution suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictResolution {
    /// Resolution identifier
    pub resolution_id: String,
    /// Resolution type
    pub resolution_type: ResolutionType,
    /// Resolution description
    pub description: String,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Resolution steps
    pub steps: Vec<ResolutionStep>,
    /// Estimated effort
    pub estimated_effort: ResolutionEffort,
    /// Risk assessment
    pub risk_level: RiskLevel,
    /// Required expertise
    pub required_expertise: Vec<String>,
}

/// Types of conflict resolutions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ResolutionType {
    /// Accept our changes
    AcceptOurs,
    /// Accept their changes
    AcceptTheirs,
    /// Manual merge required
    ManualMerge,
    /// Automatic merge possible
    AutoMerge,
    /// Rewrite section
    Rewrite,
    /// Split into multiple files
    Split,
    /// Defer resolution
    Defer,
    /// Escalate to expert
    Escalate,
}

/// Resolution step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolutionStep {
    /// Step identifier
    pub step_id: String,
    /// Step description
    pub description: String,
    /// Step type
    pub step_type: StepType,
    /// Step parameters
    pub parameters: HashMap<String, String>,
    /// Step order
    pub order: usize,
    /// Is step optional
    pub optional: bool,
}

/// Types of resolution steps
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StepType {
    /// Git command
    GitCommand,
    /// File edit
    FileEdit,
    /// Code review
    CodeReview,
    /// Test execution
    TestExecution,
    /// Documentation update
    DocumentationUpdate,
    /// Ceremony initiation
    CeremonyInitiation,
    /// Manual intervention
    ManualIntervention,
}

/// Resolution effort estimation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResolutionEffort {
    /// Minimal effort (< 5 minutes)
    Minimal,
    /// Low effort (5-30 minutes)
    Low,
    /// Medium effort (30 minutes - 2 hours)
    Medium,
    /// High effort (2-8 hours)
    High,
    /// Very high effort (> 8 hours)
    VeryHigh,
}

/// Risk level for resolution
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum RiskLevel {
    /// Very low risk
    VeryLow,
    /// Low risk
    Low,
    /// Medium risk
    Medium,
    /// High risk
    High,
    /// Very high risk
    VeryHigh,
}

/// Conflict resolution status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConflictResolutionStatus {
    /// Conflict detected but not resolved
    Detected,
    /// Resolution in progress
    InProgress,
    /// Conflict resolved
    Resolved,
    /// Resolution failed
    Failed,
    /// Resolution deferred
    Deferred,
    /// Conflict escalated
    Escalated,
}

/// Conflict pattern for learning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictPattern {
    /// Pattern identifier
    pub pattern_id: String,
    /// Pattern name
    pub name: String,
    /// Conflict types in pattern
    pub conflict_types: Vec<ConflictType>,
    /// Common file patterns
    pub file_patterns: Vec<String>,
    /// Typical resolution approaches
    pub typical_resolutions: Vec<ResolutionType>,
    /// Pattern frequency
    pub frequency: usize,
    /// Success rate of resolutions
    pub success_rate: f64,
    /// Pattern confidence
    pub confidence: f64,
}

/// Conflict resolution record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictResolutionRecord {
    /// Record identifier
    pub record_id: String,
    /// Original conflict
    pub conflict: GitConflict,
    /// Resolution applied
    pub resolution: ConflictResolution,
    /// Resolution outcome
    pub outcome: ResolutionOutcome,
    /// Time to resolve
    pub resolution_time_minutes: u64,
    /// Participants involved
    pub participants: Vec<String>,
    /// Lessons learned
    pub lessons_learned: Vec<String>,
    /// Record timestamp
    pub recorded_at: DateTime<Utc>,
}

/// Outcome of conflict resolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolutionOutcome {
    /// Was resolution successful
    pub success: bool,
    /// Outcome description
    pub description: String,
    /// Quality score (0.0 to 1.0)
    pub quality_score: f64,
    /// Side effects detected
    pub side_effects: Vec<String>,
    /// Follow-up actions needed
    pub follow_up_actions: Vec<String>,
}

impl GitConflictDetector {
    /// Create a new git conflict detector
    pub fn new(git_config: &GitManagerConfig) -> Result<Self> {
        let config = ConflictDetectionConfig::default();
        
        info!("Initializing git conflict detector");
        
        Ok(Self {
            config,
            conflicts_cache: HashMap::new(),
            resolution_history: Vec::new(),
            conflict_patterns: HashMap::new(),
        })
    }
    
    /// Detect conflicts in repository
    pub async fn detect_conflicts(&mut self, repository_path: &Path) -> Result<Vec<GitConflict>> {
        debug!("Detecting conflicts in repository: {:?}", repository_path);
        
        let cache_key = repository_path.to_string_lossy().to_string();
        
        // Check cache first
        if let Some(cached_conflicts) = self.conflicts_cache.get(&cache_key) {
            debug!("Using cached conflict detection results");
            return Ok(cached_conflicts.clone());
        }
        
        let repo = Repository::open(repository_path)?;
        let mut conflicts = Vec::new();
        
        // Detect different types of conflicts
        conflicts.extend(self.detect_merge_conflicts(&repo).await?);
        conflicts.extend(self.detect_status_conflicts(&repo).await?);
        
        if self.config.enable_semantic_detection {
            conflicts.extend(self.detect_semantic_conflicts(&repo).await?);
        }
        
        if self.config.enable_proactive_detection {
            conflicts.extend(self.detect_potential_conflicts(&repo).await?);
        }
        
        // Analyze and enhance conflicts
        for conflict in &mut conflicts {
            self.analyze_conflict(conflict, &repo).await?;
            self.generate_resolutions(conflict).await?;
        }
        
        // Cache results
        if self.conflicts_cache.len() >= self.config.cache_size {
            // Remove oldest entry (simplified LRU)
            if let Some(first_key) = self.conflicts_cache.keys().next().cloned() {
                self.conflicts_cache.remove(&first_key);
            }
        }
        self.conflicts_cache.insert(cache_key, conflicts.clone());
        
        info!("Detected {} conflicts in repository", conflicts.len());
        Ok(conflicts)
    }
    
    /// Detect merge conflicts
    async fn detect_merge_conflicts(&self, repo: &Repository) -> Result<Vec<GitConflict>> {
        let mut conflicts = Vec::new();
        
        // Check if repository is in merge state
        if repo.state() == git2::RepositoryState::Merge {
            let index = repo.index()?;
            
            for entry in index.iter() {
                if index.has_conflicts() {
                    if let Some(path_str) = std::str::from_utf8(&entry.path).ok() {
                        if let Some(_conflict) = index.get_path(std::path::Path::new(path_str), 0) {
                            let file_path = String::from_utf8_lossy(&entry.path).to_string();
                        
                        let git_conflict = GitConflict {
                            conflict_id: Uuid::new_v4().to_string(),
                            conflict_type: ConflictType::ContentConflict,
                            severity: ConflictSeverity::Major,
                            file_path: file_path.clone(),
                            location: ConflictLocation {
                                start_line: 0,
                                end_line: 0,
                                start_column: None,
                                end_column: None,
                                context: None,
                            },
                            description: format!("Merge conflict in {}", file_path),
                            conflicting_refs: vec!["HEAD".to_string(), "MERGE_HEAD".to_string()],
                            conflict_content: ConflictContent {
                                ours: String::new(),
                                theirs: String::new(),
                                base: None,
                                has_markers: true,
                                content_type: self.determine_content_type(&file_path),
                            },
                            suggested_resolutions: Vec::new(),
                            metadata: HashMap::new(),
                            detected_at: Utc::now(),
                            resolution_status: ConflictResolutionStatus::Detected,
                        };
                        
                        conflicts.push(git_conflict);
                        }
                    }
                }
            }
        }
        
        Ok(conflicts)
    }
    
    /// Detect status conflicts
    async fn detect_status_conflicts(&self, repo: &Repository) -> Result<Vec<GitConflict>> {
        let mut conflicts = Vec::new();
        
        let mut opts = StatusOptions::new();
        opts.include_untracked(false);
        opts.include_ignored(false);
        
        let statuses = repo.statuses(Some(&mut opts))?;
        
        for entry in statuses.iter() {
            let status = entry.status();
            
            if status.is_conflicted() {
                if let Some(path) = entry.path() {
                    let git_conflict = GitConflict {
                        conflict_id: Uuid::new_v4().to_string(),
                        conflict_type: self.determine_conflict_type_from_status(status),
                        severity: ConflictSeverity::Major,
                        file_path: path.to_string(),
                        location: ConflictLocation {
                            start_line: 0,
                            end_line: 0,
                            start_column: None,
                            end_column: None,
                            context: None,
                        },
                        description: format!("Status conflict in {}", path),
                        conflicting_refs: Vec::new(),
                        conflict_content: ConflictContent {
                            ours: String::new(),
                            theirs: String::new(),
                            base: None,
                            has_markers: false,
                            content_type: self.determine_content_type(path),
                        },
                        suggested_resolutions: Vec::new(),
                        metadata: HashMap::new(),
                        detected_at: Utc::now(),
                        resolution_status: ConflictResolutionStatus::Detected,
                    };
                    
                    conflicts.push(git_conflict);
                }
            }
        }
        
        Ok(conflicts)
    }
    
    /// Detect semantic conflicts (simplified implementation)
    async fn detect_semantic_conflicts(&self, repo: &Repository) -> Result<Vec<GitConflict>> {
        let mut conflicts = Vec::new();
        
        // This would involve more sophisticated analysis of code semantics
        // For now, we'll implement a simplified version that looks for common patterns
        
        // Check for potential function signature conflicts
        // Check for variable naming conflicts
        // Check for import/dependency conflicts
        
        // Placeholder implementation
        debug!("Semantic conflict detection not fully implemented");
        
        Ok(conflicts)
    }
    
    /// Detect potential conflicts proactively
    async fn detect_potential_conflicts(&self, repo: &Repository) -> Result<Vec<GitConflict>> {
        let mut conflicts = Vec::new();
        
        // This would analyze patterns and predict potential conflicts
        // Based on file change patterns, contributor patterns, etc.
        
        // Placeholder implementation
        debug!("Proactive conflict detection not fully implemented");
        
        Ok(conflicts)
    }
    
    /// Analyze conflict for additional details
    async fn analyze_conflict(&self, conflict: &mut GitConflict, repo: &Repository) -> Result<()> {
        // Read file content to analyze conflict markers
        if let Ok(content) = std::fs::read_to_string(&conflict.file_path) {
            if content.contains("<<<<<<<") && content.contains(">>>>>>>") {
                conflict.conflict_content.has_markers = true;
                
                // Parse conflict markers to extract content
                self.parse_conflict_markers(&mut conflict.conflict_content, &content);
                
                // Determine conflict location
                self.determine_conflict_location(conflict, &content);
            }
        }
        
        // Assess severity based on file importance and conflict complexity
        conflict.severity = self.assess_conflict_severity(conflict);
        
        Ok(())
    }
    
    /// Parse conflict markers in file content
    fn parse_conflict_markers(&self, conflict_content: &mut ConflictContent, content: &str) {
        let lines: Vec<&str> = content.lines().collect();
        let mut in_conflict = false;
        let mut ours_content = String::new();
        let mut theirs_content = String::new();
        let mut current_section = "ours";
        
        for line in lines {
            if line.starts_with("<<<<<<<") {
                in_conflict = true;
                current_section = "ours";
            } else if line.starts_with("=======") {
                current_section = "theirs";
            } else if line.starts_with(">>>>>>>") {
                in_conflict = false;
            } else if in_conflict {
                match current_section {
                    "ours" => {
                        ours_content.push_str(line);
                        ours_content.push('\n');
                    }
                    "theirs" => {
                        theirs_content.push_str(line);
                        theirs_content.push('\n');
                    }
                    _ => {}
                }
            }
        }
        
        conflict_content.ours = ours_content;
        conflict_content.theirs = theirs_content;
    }
    
    /// Determine conflict location in file
    fn determine_conflict_location(&self, conflict: &mut GitConflict, content: &str) {
        let lines: Vec<&str> = content.lines().collect();
        
        for (i, line) in lines.iter().enumerate() {
            if line.starts_with("<<<<<<<") {
                conflict.location.start_line = i + 1;
            } else if line.starts_with(">>>>>>>") {
                conflict.location.end_line = i + 1;
                break;
            }
        }
    }
    
    /// Assess conflict severity
    fn assess_conflict_severity(&self, conflict: &GitConflict) -> ConflictSeverity {
        let mut severity_score = 0;
        
        // File importance
        if conflict.file_path.contains("main") || conflict.file_path.contains("core") {
            severity_score += 2;
        }
        
        // Conflict type
        match conflict.conflict_type {
            ConflictType::ContentConflict => severity_score += 1,
            ConflictType::SemanticConflict => severity_score += 3,
            ConflictType::StructuralConflict => severity_score += 3,
            ConflictType::DeleteModify => severity_score += 2,
            _ => severity_score += 1,
        }
        
        // Content size
        let content_size = conflict.conflict_content.ours.len() + conflict.conflict_content.theirs.len();
        if content_size > 1000 {
            severity_score += 2;
        } else if content_size > 100 {
            severity_score += 1;
        }
        
        match severity_score {
            0..=1 => ConflictSeverity::Minor,
            2..=3 => ConflictSeverity::Moderate,
            4..=5 => ConflictSeverity::Major,
            6..=7 => ConflictSeverity::Critical,
            _ => ConflictSeverity::Blocking,
        }
    }
    
    /// Generate resolution suggestions
    async fn generate_resolutions(&self, conflict: &mut GitConflict) -> Result<()> {
        let mut resolutions = Vec::new();
        
        // Generate basic resolutions
        resolutions.push(ConflictResolution {
            resolution_id: Uuid::new_v4().to_string(),
            resolution_type: ResolutionType::AcceptOurs,
            description: "Accept our changes".to_string(),
            confidence: 0.7,
            steps: vec![
                ResolutionStep {
                    step_id: Uuid::new_v4().to_string(),
                    description: "Accept our version of the file".to_string(),
                    step_type: StepType::GitCommand,
                    parameters: {
                        let mut params = HashMap::new();
                        params.insert("command".to_string(), "checkout --ours".to_string());
                        params.insert("file".to_string(), conflict.file_path.clone());
                        params
                    },
                    order: 1,
                    optional: false,
                }
            ],
            estimated_effort: ResolutionEffort::Minimal,
            risk_level: RiskLevel::Low,
            required_expertise: vec!["git".to_string()],
        });
        
        resolutions.push(ConflictResolution {
            resolution_id: Uuid::new_v4().to_string(),
            resolution_type: ResolutionType::AcceptTheirs,
            description: "Accept their changes".to_string(),
            confidence: 0.7,
            steps: vec![
                ResolutionStep {
                    step_id: Uuid::new_v4().to_string(),
                    description: "Accept their version of the file".to_string(),
                    step_type: StepType::GitCommand,
                    parameters: {
                        let mut params = HashMap::new();
                        params.insert("command".to_string(), "checkout --theirs".to_string());
                        params.insert("file".to_string(), conflict.file_path.clone());
                        params
                    },
                    order: 1,
                    optional: false,
                }
            ],
            estimated_effort: ResolutionEffort::Minimal,
            risk_level: RiskLevel::Low,
            required_expertise: vec!["git".to_string()],
        });
        
        // Generate manual merge resolution for complex conflicts
        if conflict.severity >= ConflictSeverity::Major {
            resolutions.push(ConflictResolution {
                resolution_id: Uuid::new_v4().to_string(),
                resolution_type: ResolutionType::ManualMerge,
                description: "Manually merge the conflicting changes".to_string(),
                confidence: 0.9,
                steps: vec![
                    ResolutionStep {
                        step_id: Uuid::new_v4().to_string(),
                        description: "Review conflicting changes".to_string(),
                        step_type: StepType::CodeReview,
                        parameters: HashMap::new(),
                        order: 1,
                        optional: false,
                    },
                    ResolutionStep {
                        step_id: Uuid::new_v4().to_string(),
                        description: "Edit file to resolve conflicts".to_string(),
                        step_type: StepType::FileEdit,
                        parameters: {
                            let mut params = HashMap::new();
                            params.insert("file".to_string(), conflict.file_path.clone());
                            params
                        },
                        order: 2,
                        optional: false,
                    },
                    ResolutionStep {
                        step_id: Uuid::new_v4().to_string(),
                        description: "Test the merged changes".to_string(),
                        step_type: StepType::TestExecution,
                        parameters: HashMap::new(),
                        order: 3,
                        optional: true,
                    },
                ],
                estimated_effort: ResolutionEffort::Medium,
                risk_level: RiskLevel::Medium,
                required_expertise: vec!["domain_knowledge".to_string(), "code_review".to_string()],
            });
        }
        
        conflict.suggested_resolutions = resolutions;
        Ok(())
    }
    
    /// Determine conflict type from git status
    fn determine_conflict_type_from_status(&self, status: git2::Status) -> ConflictType {
        if status.is_index_deleted() && status.is_wt_modified() {
            ConflictType::DeleteModify
        } else if status.is_index_new() && status.is_wt_new() {
            ConflictType::AddAdd
        } else {
            ConflictType::ContentConflict
        }
    }
    
    /// Determine content type from file path
    fn determine_content_type(&self, file_path: &str) -> ContentType {
        let extension = std::path::Path::new(file_path)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
        
        match extension {
            "rs" | "py" | "js" | "ts" | "java" | "cpp" | "c" | "h" => ContentType::SourceCode,
            "md" | "txt" | "rst" => ContentType::Documentation,
            "json" | "yaml" | "yml" | "toml" | "ini" | "conf" => ContentType::Configuration,
            "png" | "jpg" | "jpeg" | "gif" | "svg" => ContentType::Image,
            _ => {
                if file_path.ends_with(".bin") || file_path.contains("binary") {
                    ContentType::Binary
                } else {
                    ContentType::Text
                }
            }
        }
    }
    
    /// Get total conflicts detected
    pub fn get_total_conflicts_detected(&self) -> usize {
        self.resolution_history.len() + 
        self.conflicts_cache.values().map(|conflicts| conflicts.len()).sum::<usize>()
    }
    
    /// Get conflict statistics
    pub fn get_conflict_statistics(&self) -> ConflictStatistics {
        let total_conflicts = self.get_total_conflicts_detected();
        let resolved_conflicts = self.resolution_history.len();
        
        let resolution_rate = if total_conflicts > 0 {
            resolved_conflicts as f64 / total_conflicts as f64
        } else {
            0.0
        };
        
        let avg_resolution_time = if resolved_conflicts > 0 {
            self.resolution_history.iter()
                .map(|r| r.resolution_time_minutes)
                .sum::<u64>() as f64 / resolved_conflicts as f64
        } else {
            0.0
        };
        
        let conflict_type_distribution = self.conflicts_cache
            .values()
            .flatten()
            .fold(HashMap::new(), |mut acc, conflict| {
                *acc.entry(conflict.conflict_type.clone()).or_insert(0) += 1;
                acc
            });
        
        ConflictStatistics {
            total_conflicts,
            resolved_conflicts,
            resolution_rate,
            average_resolution_time_minutes: avg_resolution_time,
            conflict_type_distribution,
            patterns_learned: self.conflict_patterns.len(),
        }
    }
}

/// Statistics about conflict detection and resolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictStatistics {
    /// Total conflicts detected
    pub total_conflicts: usize,
    /// Number of resolved conflicts
    pub resolved_conflicts: usize,
    /// Resolution rate (0.0 to 1.0)
    pub resolution_rate: f64,
    /// Average time to resolve conflicts in minutes
    pub average_resolution_time_minutes: f64,
    /// Distribution of conflict types
    pub conflict_type_distribution: HashMap<ConflictType, usize>,
    /// Number of patterns learned
    pub patterns_learned: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conflict_detection_config_default() {
        let config = ConflictDetectionConfig::default();
        assert!(config.enable_proactive_detection);
        assert_eq!(config.detection_sensitivity, 0.7);
        assert!(config.enable_semantic_detection);
    }
    
    #[test]
    fn test_conflict_severity_ordering() {
        assert!(ConflictSeverity::Blocking > ConflictSeverity::Critical);
        assert!(ConflictSeverity::Critical > ConflictSeverity::Major);
        assert!(ConflictSeverity::Major > ConflictSeverity::Moderate);
        assert!(ConflictSeverity::Moderate > ConflictSeverity::Minor);
    }
    
    #[test]
    fn test_content_type_determination() {
        let detector = GitConflictDetector::new(&GitManagerConfig::default()).unwrap();
        
        assert_eq!(detector.determine_content_type("main.rs"), ContentType::SourceCode);
        assert_eq!(detector.determine_content_type("README.md"), ContentType::Documentation);
        assert_eq!(detector.determine_content_type("config.json"), ContentType::Configuration);
        assert_eq!(detector.determine_content_type("image.png"), ContentType::Image);
    }
    
    #[test]
    fn test_conflict_serialization() {
        let conflict = GitConflict {
            conflict_id: "test".to_string(),
            conflict_type: ConflictType::ContentConflict,
            severity: ConflictSeverity::Major,
            file_path: "test.rs".to_string(),
            location: ConflictLocation {
                start_line: 1,
                end_line: 10,
                start_column: None,
                end_column: None,
                context: None,
            },
            description: "Test conflict".to_string(),
            conflicting_refs: vec!["main".to_string(), "feature".to_string()],
            conflict_content: ConflictContent {
                ours: "our version".to_string(),
                theirs: "their version".to_string(),
                base: Some("base version".to_string()),
                has_markers: true,
                content_type: ContentType::SourceCode,
            },
            suggested_resolutions: Vec::new(),
            metadata: HashMap::new(),
            detected_at: Utc::now(),
            resolution_status: ConflictResolutionStatus::Detected,
        };
        
        let serialized = serde_json::to_string(&conflict).unwrap();
        let deserialized: GitConflict = serde_json::from_str(&serialized).unwrap();
        assert_eq!(conflict.conflict_id, deserialized.conflict_id);
        assert_eq!(conflict.conflict_type, deserialized.conflict_type);
    }
}
