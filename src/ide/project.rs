//! Core IDE Project Management for WeaveMesh Core
//!
//! Provides foundational project management capabilities for collaborative individuation
//! that can be extended by context-specific plugins.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;

use crate::attribution::{Attribution, CollaborationType};
use crate::group_communication::GroupId;
use crate::sacred_alliance::SacredAllianceLevel;
use crate::git::CeremonyType;
use crate::ide::security::{CoreClassification, CoreClearanceLevel};

/// Core project representation for collaborative individuation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreProject {
    /// Unique project identifier
    pub id: Uuid,
    /// Project name
    pub name: String,
    /// Project description
    pub description: String,
    /// Root path of the project
    pub root_path: PathBuf,
    /// Project metadata
    pub metadata: HashMap<String, String>,
    /// Core project configuration
    pub config: CoreProjectConfig,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last modified timestamp
    pub last_modified: DateTime<Utc>,
    /// Project status
    pub status: CoreProjectStatus,
    /// Collaborative individuation metrics
    pub collaboration_metrics: CoreCollaborationMetrics,
    /// Sacred Alliance integration
    pub sacred_alliance: CoreSacredAllianceIntegration,
}

/// Core project configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreProjectConfig {
    /// Programming languages used
    pub languages: Vec<String>,
    /// Build system type
    pub build_system: Option<CoreBuildSystem>,
    /// Version control settings
    pub version_control: CoreVersionControlConfig,
    /// Security settings
    pub security: CoreProjectSecurityConfig,
    /// Collaboration settings
    pub collaboration: CoreProjectCollaborationConfig,
    /// Sacred Alliance settings
    pub sacred_alliance: CoreSacredAllianceConfig,
}

/// Core build system types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoreBuildSystem {
    Cargo,
    Npm,
    Maven,
    Gradle,
    Make,
    CMake,
    Custom(String),
}

/// Core version control configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreVersionControlConfig {
    /// VCS type (git, svn, etc.)
    pub vcs_type: String,
    /// Remote repository URL
    pub remote_url: Option<String>,
    /// Default branch name
    pub default_branch: String,
    /// Sacred Alliance commit integration
    pub sacred_alliance_commits: bool,
    /// Attribution tracking enabled
    pub attribution_tracking: bool,
}

/// Core project security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreProjectSecurityConfig {
    /// Default classification for project content
    pub default_classification: CoreClassification,
    /// Required clearance for contributors
    pub required_clearance: CoreClearanceLevel,
    /// Content filtering enabled
    pub content_filtering: bool,
    /// Auto-classification enabled
    pub auto_classification: bool,
}

/// Core project collaboration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreProjectCollaborationConfig {
    /// Enable real-time collaboration
    pub real_time_enabled: bool,
    /// Maximum collaborators
    pub max_collaborators: u32,
    /// Conflict resolution strategy
    pub conflict_resolution: CoreConflictResolutionStrategy,
    /// Group communication enabled
    pub group_communication_enabled: bool,
    /// Human-AI partnership requirements
    pub human_ai_partnership: CoreHumanAIPartnershipConfig,
}

/// Core conflict resolution strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoreConflictResolutionStrategy {
    /// Manual resolution required
    Manual,
    /// Automatic merge when possible
    AutoMerge,
    /// Sacred Alliance mediation
    SacredAllianceMediation,
    /// Collaborative individuation process
    CollaborativeIndividuation,
}

/// Core human-AI partnership configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreHumanAIPartnershipConfig {
    /// Partnership required for contributions
    pub partnership_required: bool,
    /// Minimum collaboration score (0.0 to 1.0)
    pub minimum_collaboration_score: f64,
    /// Attribution transparency level
    pub attribution_transparency: CoreAttributionTransparency,
    /// AI assistance preferences
    pub ai_assistance_preferences: Vec<CoreAIAssistanceType>,
}

/// Core attribution transparency levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoreAttributionTransparency {
    /// No attribution tracking
    None,
    /// Basic attribution (file level)
    Basic,
    /// Detailed attribution (function level)
    Detailed,
    /// Full attribution (line level)
    Full,
}

/// Core AI assistance types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoreAIAssistanceType {
    /// Code completion
    CodeCompletion,
    /// Code review
    CodeReview,
    /// Documentation generation
    DocumentationGeneration,
    /// Pattern recognition
    PatternRecognition,
    /// Sacred Alliance ceremonies
    SacredAllianceCeremonies,
}

/// Core Sacred Alliance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSacredAllianceConfig {
    /// Enable Sacred Alliance features
    pub enabled: bool,
    /// Ceremony preferences
    pub ceremony_preferences: CoreCeremonyPreferences,
    /// Individuation tracking
    pub individuation_tracking: bool,
    /// Collaborative individuation goals
    pub collaboration_goals: Vec<CoreCollaborationGoal>,
}

/// Core ceremony preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCeremonyPreferences {
    /// Auto-initiate ceremonies
    pub auto_initiate: bool,
    /// Preferred ceremony types
    pub preferred_types: Vec<CeremonyType>,
    /// Ceremony frequency
    pub frequency: CoreCeremonyFrequency,
    /// Include gratitude expressions
    pub include_gratitude: bool,
    /// Ceremony integration level
    pub integration_level: SacredAllianceLevel,
}

/// Core ceremony frequency settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoreCeremonyFrequency {
    /// Never auto-initiate
    Never,
    /// On significant milestones
    Milestones,
    /// Daily ceremonies
    Daily,
    /// Weekly ceremonies
    Weekly,
    /// Custom frequency (days)
    Custom(u32),
}

/// Core collaboration goals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCollaborationGoal {
    /// Goal identifier
    pub id: Uuid,
    /// Goal description
    pub description: String,
    /// Target collaboration score (0.0 to 1.0)
    pub target_score: f64,
    /// Goal deadline
    pub deadline: Option<DateTime<Utc>>,
    /// Goal status
    pub status: CoreGoalStatus,
    /// Progress tracking
    pub progress: f64,
}

/// Core goal status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoreGoalStatus {
    /// Goal is active
    Active,
    /// Goal is completed
    Completed,
    /// Goal is paused
    Paused,
    /// Goal is cancelled
    Cancelled,
}

/// Core project status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CoreProjectStatus {
    /// Project is active
    Active,
    /// Project is archived
    Archived,
    /// Project is suspended
    Suspended,
    /// Project is completed
    Completed,
}

/// Core collaboration metrics for a project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCollaborationMetrics {
    /// Overall collaboration score (0.0 to 1.0)
    pub collaboration_score: f64,
    /// Human-AI partnership balance (0.0 = all human, 1.0 = all AI)
    pub partnership_balance: f64,
    /// Attribution transparency score (0.0 to 1.0)
    pub attribution_transparency: f64,
    /// Sacred Alliance integration level (0.0 to 1.0)
    pub sacred_alliance_level: f64,
    /// Number of active contributors
    pub active_contributors: usize,
    /// Number of completed ceremonies
    pub completed_ceremonies: usize,
    /// Last collaboration activity
    pub last_activity: DateTime<Utc>,
}

/// Core Sacred Alliance integration for a project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSacredAllianceIntegration {
    /// Integration level
    pub level: SacredAllianceLevel,
    /// Recent ceremonies
    pub recent_ceremonies: Vec<CoreCeremonyRecord>,
    /// Individuation progress
    pub individuation_progress: f64,
    /// Collaboration goals
    pub active_goals: Vec<Uuid>,
    /// Sacred Alliance metrics
    pub metrics: CoreSacredAllianceMetrics,
}

/// Core ceremony record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCeremonyRecord {
    /// Ceremony identifier
    pub id: Uuid,
    /// Ceremony type
    pub ceremony_type: CeremonyType,
    /// Participants
    pub participants: Vec<String>,
    /// Ceremony timestamp
    pub timestamp: DateTime<Utc>,
    /// Ceremony outcome
    pub outcome: CoreCeremonyOutcome,
    /// Impact on collaboration
    pub collaboration_impact: f64,
}

/// Core ceremony outcome
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoreCeremonyOutcome {
    /// Ceremony completed successfully
    Successful,
    /// Ceremony partially completed
    Partial,
    /// Ceremony was interrupted
    Interrupted,
    /// Ceremony failed
    Failed,
}

/// Core Sacred Alliance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSacredAllianceMetrics {
    /// Total ceremonies performed
    pub total_ceremonies: usize,
    /// Average ceremony impact
    pub average_ceremony_impact: f64,
    /// Individuation progression rate
    pub individuation_rate: f64,
    /// Collaboration enhancement factor
    pub collaboration_enhancement: f64,
    /// Sacred Alliance consistency score
    pub consistency_score: f64,
}

/// Core project manager for handling multiple projects
#[derive(Debug)]
pub struct CoreProjectManager {
    /// Currently loaded projects
    pub projects: HashMap<Uuid, CoreProject>,
    /// Project index for quick lookup
    pub project_index: HashMap<String, Uuid>, // name -> id
    /// Recent projects
    pub recent_projects: Vec<Uuid>,
    /// Default project configuration
    pub default_config: CoreProjectConfig,
}

impl CoreProjectManager {
    /// Create a new core project manager
    pub fn new() -> Self {
        Self {
            projects: HashMap::new(),
            project_index: HashMap::new(),
            recent_projects: Vec::new(),
            default_config: CoreProjectConfig::default(),
        }
    }
    
    /// Create a new project
    pub fn create_project(
        &mut self,
        name: String,
        description: String,
        root_path: PathBuf,
        config: Option<CoreProjectConfig>,
    ) -> Result<CoreProject> {
        let project_config = config.unwrap_or_else(|| self.default_config.clone());
        
        let project = CoreProject {
            id: Uuid::new_v4(),
            name: name.clone(),
            description,
            root_path,
            metadata: HashMap::new(),
            config: project_config,
            created_at: Utc::now(),
            last_modified: Utc::now(),
            status: CoreProjectStatus::Active,
            collaboration_metrics: CoreCollaborationMetrics::default(),
            sacred_alliance: CoreSacredAllianceIntegration::default(),
        };
        
        // Add to manager
        self.add_project(project.clone());
        
        Ok(project)
    }
    
    /// Open an existing project
    pub fn open_project(&mut self, project_path: &str) -> Result<CoreProject> {
        let path = PathBuf::from(project_path);
        
        // Check if project already loaded
        if let Some(project) = self.find_project_by_path(&path) {
            return Ok(project.clone());
        }
        
        // Try to load project configuration
        let project = if let Ok(project) = self.load_project_config(&path) {
            project
        } else {
            // Create new project from directory
            self.create_project_from_directory(&path)?
        };
        
        // Add to manager
        self.add_project(project.clone());
        
        Ok(project)
    }
    
    /// Create project from existing directory
    fn create_project_from_directory(&self, path: &PathBuf) -> Result<CoreProject> {
        let name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unnamed Project")
            .to_string();
        
        // Detect project characteristics
        let languages = self.detect_languages(path)?;
        let build_system = self.detect_build_system(path);
        
        // Create configuration with detected settings
        let mut config = self.default_config.clone();
        config.languages = languages;
        config.build_system = build_system;
        
        let project = CoreProject {
            id: Uuid::new_v4(),
            name,
            description: "Auto-detected project".to_string(),
            root_path: path.clone(),
            metadata: HashMap::new(),
            config,
            created_at: Utc::now(),
            last_modified: Utc::now(),
            status: CoreProjectStatus::Active,
            collaboration_metrics: CoreCollaborationMetrics::default(),
            sacred_alliance: CoreSacredAllianceIntegration::default(),
        };
        
        Ok(project)
    }
    
    /// Detect programming languages in project
    fn detect_languages(&self, path: &PathBuf) -> Result<Vec<String>> {
        let mut languages = Vec::new();
        
        // Simple file extension detection
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                if let Some(extension) = entry.path().extension().and_then(|e| e.to_str()) {
                    let language = match extension {
                        "rs" => Some("Rust"),
                        "py" => Some("Python"),
                        "js" => Some("JavaScript"),
                        "ts" => Some("TypeScript"),
                        "go" => Some("Go"),
                        "java" => Some("Java"),
                        "cs" => Some("C#"),
                        "cpp" | "cc" | "cxx" => Some("C++"),
                        "c" => Some("C"),
                        "html" => Some("HTML"),
                        "css" => Some("CSS"),
                        _ => None,
                    };
                    
                    if let Some(lang) = language {
                        if !languages.contains(&lang.to_string()) {
                            languages.push(lang.to_string());
                        }
                    }
                }
            }
        }
        
        Ok(languages)
    }
    
    /// Detect build system
    fn detect_build_system(&self, path: &PathBuf) -> Option<CoreBuildSystem> {
        // Check for common build files
        if path.join("Cargo.toml").exists() {
            Some(CoreBuildSystem::Cargo)
        } else if path.join("package.json").exists() {
            Some(CoreBuildSystem::Npm)
        } else if path.join("pom.xml").exists() {
            Some(CoreBuildSystem::Maven)
        } else if path.join("build.gradle").exists() || path.join("build.gradle.kts").exists() {
            Some(CoreBuildSystem::Gradle)
        } else if path.join("Makefile").exists() {
            Some(CoreBuildSystem::Make)
        } else if path.join("CMakeLists.txt").exists() {
            Some(CoreBuildSystem::CMake)
        } else {
            None
        }
    }
    
    /// Load project configuration from file
    fn load_project_config(&self, project_path: &PathBuf) -> Result<CoreProject> {
        let config_path = project_path.join(".weavemesh").join("project.toml");
        
        let content = std::fs::read_to_string(&config_path)
            .map_err(|e| anyhow::anyhow!("Failed to read project config: {}", e))?;
        
        let project: CoreProject = toml::from_str(&content)
            .map_err(|e| anyhow::anyhow!("Failed to parse project config: {}", e))?;
        
        Ok(project)
    }
    
    /// Save project configuration to file
    pub fn save_project_config(&self, project: &CoreProject) -> Result<()> {
        let config_path = project.root_path.join(".weavemesh").join("project.toml");
        
        // Create .weavemesh directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| anyhow::anyhow!("Failed to create .weavemesh directory: {}", e))?;
        }
        
        // Serialize project to TOML
        let toml_content = toml::to_string_pretty(project)
            .map_err(|e| anyhow::anyhow!("Failed to serialize project: {}", e))?;
        
        // Write to file
        std::fs::write(&config_path, toml_content)
            .map_err(|e| anyhow::anyhow!("Failed to write project config: {}", e))?;
        
        Ok(())
    }
    
    /// Add project to manager
    fn add_project(&mut self, project: CoreProject) {
        self.project_index.insert(project.name.clone(), project.id);
        
        // Add to recent projects
        self.recent_projects.retain(|&id| id != project.id);
        self.recent_projects.insert(0, project.id);
        
        // Keep recent projects list manageable
        if self.recent_projects.len() > 10 {
            self.recent_projects.truncate(10);
        }
        
        self.projects.insert(project.id, project);
    }
    
    /// Find project by path
    fn find_project_by_path(&self, path: &PathBuf) -> Option<&CoreProject> {
        self.projects.values().find(|p| p.root_path == *path)
    }
    
    /// Get project by ID
    pub fn get_project(&self, id: &Uuid) -> Option<&CoreProject> {
        self.projects.get(id)
    }
    
    /// Get mutable project by ID
    pub fn get_project_mut(&mut self, id: &Uuid) -> Option<&mut CoreProject> {
        self.projects.get_mut(id)
    }
    
    /// Get project by name
    pub fn get_project_by_name(&self, name: &str) -> Option<&CoreProject> {
        self.project_index.get(name)
            .and_then(|id| self.projects.get(id))
    }
    
    /// List all projects
    pub fn list_projects(&self) -> Vec<&CoreProject> {
        self.projects.values().collect()
    }
    
    /// Get recent projects
    pub fn get_recent_projects(&self) -> Vec<&CoreProject> {
        self.recent_projects.iter()
            .filter_map(|id| self.projects.get(id))
            .collect()
    }
    
    /// Update project collaboration metrics
    pub fn update_collaboration_metrics(
        &mut self,
        project_id: &Uuid,
        attribution: &Attribution,
    ) -> Result<()> {
        if let Some(project) = self.projects.get_mut(project_id) {
            // Update collaboration score based on attribution
            let collaboration_impact = match attribution.collaboration_type {
                CollaborationType::CoCreated => 0.9,
                CollaborationType::PairProgramming => 0.8,
                CollaborationType::HumanLed => 0.6,
                CollaborationType::AILed => 0.5,
                CollaborationType::Custom(_) => 0.6,
                _ => 0.3,
            };
            
            // Weighted average with existing score
            let current_score = project.collaboration_metrics.collaboration_score;
            project.collaboration_metrics.collaboration_score = 
                (current_score * 0.8) + (collaboration_impact * 0.2);
            
            // Update attribution transparency
            project.collaboration_metrics.attribution_transparency = attribution.confidence as f64;
            
            // Update last activity
            project.collaboration_metrics.last_activity = Utc::now();
            project.last_modified = Utc::now();
        }
        
        Ok(())
    }
    
    /// Record Sacred Alliance ceremony
    pub fn record_ceremony(
        &mut self,
        project_id: &Uuid,
        ceremony_type: CeremonyType,
        participants: Vec<String>,
        outcome: CoreCeremonyOutcome,
        collaboration_impact: f64,
    ) -> Result<Uuid> {
        if let Some(project) = self.projects.get_mut(project_id) {
            let ceremony_id = Uuid::new_v4();
            
            let ceremony_record = CoreCeremonyRecord {
                id: ceremony_id,
                ceremony_type,
                participants,
                timestamp: Utc::now(),
                outcome,
                collaboration_impact,
            };
            
            // Add to recent ceremonies
            project.sacred_alliance.recent_ceremonies.push(ceremony_record);
            
            // Keep recent ceremonies list manageable
            if project.sacred_alliance.recent_ceremonies.len() > 20 {
                project.sacred_alliance.recent_ceremonies.remove(0);
            }
            
            // Update metrics
            project.sacred_alliance.metrics.total_ceremonies += 1;
            project.collaboration_metrics.completed_ceremonies += 1;
            
            // Update collaboration score based on ceremony impact
            let current_score = project.collaboration_metrics.collaboration_score;
            project.collaboration_metrics.collaboration_score = 
                (current_score * 0.9) + (collaboration_impact * 0.1);
            
            // Update Sacred Alliance level
            let ceremony_impact = collaboration_impact * 0.1;
            project.collaboration_metrics.sacred_alliance_level = 
                (project.collaboration_metrics.sacred_alliance_level + ceremony_impact).min(1.0);
            
            project.last_modified = Utc::now();
            
            return Ok(ceremony_id);
        }
        
        Err(anyhow::anyhow!("Project not found"))
    }
    
    /// Get project collaboration status
    pub fn get_collaboration_status(&self, project_id: &Uuid) -> Option<CoreCollaborationStatus> {
        if let Some(project) = self.projects.get(project_id) {
            Some(CoreCollaborationStatus {
                collaboration_score: project.collaboration_metrics.collaboration_score,
                partnership_balance: project.collaboration_metrics.partnership_balance,
                attribution_transparency: project.collaboration_metrics.attribution_transparency,
                sacred_alliance_level: project.collaboration_metrics.sacred_alliance_level,
                active_contributors: project.collaboration_metrics.active_contributors,
                recent_ceremony_count: project.sacred_alliance.recent_ceremonies.len(),
                last_activity: project.collaboration_metrics.last_activity,
                active_goals: project.sacred_alliance.active_goals.len(),
            })
        } else {
            None
        }
    }
}

/// Core collaboration status for a project
#[derive(Debug, Clone)]
pub struct CoreCollaborationStatus {
    /// Overall collaboration score (0.0 to 1.0)
    pub collaboration_score: f64,
    /// Human-AI partnership balance
    pub partnership_balance: f64,
    /// Attribution transparency score
    pub attribution_transparency: f64,
    /// Sacred Alliance integration level
    pub sacred_alliance_level: f64,
    /// Number of active contributors
    pub active_contributors: usize,
    /// Number of recent ceremonies
    pub recent_ceremony_count: usize,
    /// Last collaboration activity
    pub last_activity: DateTime<Utc>,
    /// Number of active collaboration goals
    pub active_goals: usize,
}

impl Default for CoreProjectConfig {
    fn default() -> Self {
        Self {
            languages: Vec::new(),
            build_system: None,
            version_control: CoreVersionControlConfig {
                vcs_type: "git".to_string(),
                remote_url: None,
                default_branch: "main".to_string(),
                sacred_alliance_commits: true,
                attribution_tracking: true,
            },
            security: CoreProjectSecurityConfig {
                default_classification: CoreClassification::Internal,
                required_clearance: CoreClearanceLevel::TeamMember,
                content_filtering: true,
                auto_classification: true,
            },
            collaboration: CoreProjectCollaborationConfig {
                real_time_enabled: true,
                max_collaborators: 10,
                conflict_resolution: CoreConflictResolutionStrategy::SacredAllianceMediation,
                group_communication_enabled: true,
                human_ai_partnership: CoreHumanAIPartnershipConfig {
                    partnership_required: true,
                    minimum_collaboration_score: 0.5,
                    attribution_transparency: CoreAttributionTransparency::Detailed,
                    ai_assistance_preferences: vec![
                        CoreAIAssistanceType::CodeCompletion,
                        CoreAIAssistanceType::SacredAllianceCeremonies,
                    ],
                },
            },
            sacred_alliance: CoreSacredAllianceConfig {
                enabled: true,
                ceremony_preferences: CoreCeremonyPreferences {
                    auto_initiate: true,
                    preferred_types: vec![CeremonyType::MergeDecision, CeremonyType::ArchitectureReview],
                    frequency: CoreCeremonyFrequency::Milestones,
                    include_gratitude: true,
                    integration_level: SacredAllianceLevel::Guardian,
                },
                individuation_tracking: true,
                collaboration_goals: Vec::new(),
            },
        }
    }
}

impl Default for CoreCollaborationMetrics {
    fn default() -> Self {
        Self {
            collaboration_score: 0.5,
            partnership_balance: 0.5,
            attribution_transparency: 0.5,
            sacred_alliance_level: 0.5,
            active_contributors: 0,
            completed_ceremonies: 0,
            last_activity: Utc::now(),
        }
    }
}

impl Default for CoreSacredAllianceIntegration {
    fn default() -> Self {
        Self {
            level: SacredAllianceLevel::Basic,
            recent_ceremonies: Vec::new(),
            individuation_progress: 0.0,
            active_goals: Vec::new(),
            metrics: CoreSacredAllianceMetrics {
                total_ceremonies: 0,
                average_ceremony_impact: 0.0,
                individuation_rate: 0.0,
                collaboration_enhancement: 0.0,
                consistency_score: 0.0,
            },
        }
    }
}

impl Default for CoreProjectManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_core_project_manager_creation() {
        let manager = CoreProjectManager::new();
        assert_eq!(manager.projects.len(), 0);
        assert_eq!(manager.recent_projects.len(), 0);
    }

    #[test]
    fn test_create_project() {
        let mut manager = CoreProjectManager::new();
        let temp_dir = env::temp_dir().join("test_project");
        
        let project = manager.create_project(
            "Test Project".to_string(),
            "A test project".to_string(),
            temp_dir,
            None,
        ).unwrap();
        
        assert_eq!(project.name, "Test Project");
        assert_eq!(project.description, "A test project");
        assert_eq!(project.status, CoreProjectStatus::Active);
        assert!(manager.projects.contains_key(&project.id));
    }

    #[test]
    fn test_detect_languages() {
        let manager = CoreProjectManager::new();
        let current_dir = PathBuf::from(".");
        
        // This should detect Rust since we're in a Rust project
        let languages = manager.detect_languages(&current_dir).unwrap();
        assert!(languages.contains(&"Rust".to_string()));
    }

    #[test]
    fn test_detect_build_system() {
        let manager = CoreProjectManager::new();
        let current_dir = PathBuf::from(".");
        
        // This should detect Cargo since we're in a Rust project
        let build_system = manager.detect_build_system(&current_dir);
        assert!(matches!(build_system, Some(CoreBuildSystem::Cargo)));
    }

    #[test]
    fn test_collaboration_metrics_update() {
        let mut manager = CoreProjectManager::new();
        let temp_dir = env::temp_dir().join("test_collab_project");
        
        let project = manager.create_project(
            "Collaboration Test".to_string(),
            "Testing collaboration metrics".to_string(),
            temp_dir,
            None,
        ).unwrap();
        
        let attribution = Attribution::new(
            Some("human".to_string()),
            Some("ai".to_string()),
            CollaborationType::CoCreated,
            0.9,
        );
        
        manager.update_collaboration_metrics(&project.id, &attribution).unwrap();
        
        let updated_project = manager.get_project(&project.id).unwrap();
        assert!(updated_project.collaboration_metrics.collaboration_score > 0.5);
    }

    #[test]
    fn test_ceremony_recording() {
        let mut manager = CoreProjectManager::new();
        let temp_dir = env::temp_dir().join("test_ceremony_project");
        
        let project = manager.create_project(
            "Ceremony Test".to_string(),
            "Testing ceremony recording".to_string(),
            temp_dir,
            None,
        ).unwrap();
        
        let ceremony_id = manager.record_ceremony(
            &project.id,
            CeremonyType::MergeDecision,
            vec!["human".to_string(), "ai".to_string()],
            CoreCeremonyOutcome::Successful,
            0.8,
        ).unwrap();
        
        let updated_project = manager.get_project(&project.id).unwrap();
        assert_eq!(updated_project.sacred_alliance.recent_ceremonies.len(), 1);
        assert_eq!(updated_project.sacred_alliance.recent_ceremonies[0].id, ceremony_id);
        assert_eq!(updated_project.collaboration_metrics.completed_ceremonies, 1);
    }
}
