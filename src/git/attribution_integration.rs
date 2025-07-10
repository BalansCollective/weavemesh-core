//! Git Attribution Integration Module for WeaveMesh Core
//!
//! This module provides attribution tracking and analysis for git operations,
//! integrating with WeaveMesh's unified attribution system.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn, error};

use crate::attribution::{Attribution, AttributionContext, CollaborationType};
use super::{GitOperationType, GitManagerConfig};

/// Git attribution engine for tracking contributions in git operations
pub struct GitAttributionEngine {
    /// Configuration
    config: GitAttributionConfig,
    /// Attribution cache
    attribution_cache: HashMap<String, GitAttributionAnalysis>,
    /// Operation history
    operation_history: Vec<GitAttributionRecord>,
}

/// Configuration for git attribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitAttributionConfig {
    /// Enable detailed attribution tracking
    pub enable_detailed_tracking: bool,
    /// Attribution cache size
    pub cache_size: usize,
    /// Attribution analysis timeout in seconds
    pub analysis_timeout_seconds: u64,
    /// Enable automatic attribution inference
    pub enable_auto_inference: bool,
    /// Minimum contribution threshold for attribution
    pub min_contribution_threshold: f64,
}

impl Default for GitAttributionConfig {
    fn default() -> Self {
        Self {
            enable_detailed_tracking: true,
            cache_size: 1000,
            analysis_timeout_seconds: 30,
            enable_auto_inference: true,
            min_contribution_threshold: 0.1,
        }
    }
}

/// Git-specific attribution context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitAttributionContext {
    /// Base attribution context
    pub base_context: AttributionContext,
    /// Git operation type
    pub operation_type: GitOperationType,
    /// Repository path
    pub repository_path: PathBuf,
    /// Branch name
    pub branch_name: String,
    /// Commit hash (if applicable)
    pub commit_hash: Option<String>,
    /// Files affected
    pub affected_files: Vec<String>,
    /// Lines changed
    pub lines_changed: Option<GitLinesChanged>,
    /// Git metadata
    pub git_metadata: HashMap<String, String>,
}

/// Lines changed in git operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitLinesChanged {
    /// Lines added
    pub added: usize,
    /// Lines deleted
    pub deleted: usize,
    /// Lines modified
    pub modified: usize,
    /// Total lines affected
    pub total: usize,
}

/// Git attribution analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitAttributionAnalysis {
    /// Analysis identifier
    pub analysis_id: String,
    /// Analyzed attribution
    pub attribution: Attribution,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Analysis timestamp
    pub analyzed_at: DateTime<Utc>,
    /// Contributing factors
    pub factors: Vec<GitAttributionFactor>,
    /// Recommendations
    pub recommendations: Vec<String>,
    /// Analysis metadata
    pub metadata: HashMap<String, String>,
}

/// Factors contributing to git attribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitAttributionFactor {
    /// Factor type
    pub factor_type: GitAttributionFactorType,
    /// Factor weight (0.0 to 1.0)
    pub weight: f64,
    /// Factor description
    pub description: String,
    /// Factor evidence
    pub evidence: Vec<String>,
}

/// Types of git attribution factors
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GitAttributionFactorType {
    /// Code authorship
    CodeAuthorship,
    /// Commit frequency
    CommitFrequency,
    /// File ownership
    FileOwnership,
    /// Review participation
    ReviewParticipation,
    /// Issue resolution
    IssueResolution,
    /// Documentation contribution
    DocumentationContribution,
    /// Test coverage
    TestCoverage,
    /// Architecture decisions
    ArchitectureDecisions,
    /// Bug fixes
    BugFixes,
    /// Feature implementation
    FeatureImplementation,
}

/// Git attribution record for historical tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitAttributionRecord {
    /// Record identifier
    pub record_id: String,
    /// Git operation type
    pub operation_type: GitOperationType,
    /// Repository path
    pub repository_path: PathBuf,
    /// Attribution assigned
    pub attribution: Attribution,
    /// Operation timestamp
    pub timestamp: DateTime<Utc>,
    /// Operation parameters
    pub parameters: HashMap<String, String>,
    /// Analysis confidence
    pub confidence: f64,
    /// Record metadata
    pub metadata: HashMap<String, String>,
}

impl GitAttributionEngine {
    /// Create a new git attribution engine
    pub fn new(git_config: &GitManagerConfig) -> Result<Self> {
        let config = GitAttributionConfig::default();
        
        info!("Initializing git attribution engine");
        
        Ok(Self {
            config,
            attribution_cache: HashMap::new(),
            operation_history: Vec::new(),
        })
    }
    
    /// Analyze git operation for attribution
    pub async fn analyze_git_operation(&mut self, context: &GitAttributionContext) -> Result<GitAttributionAnalysis> {
        debug!("Analyzing git operation for attribution: {:?}", context.operation_type);
        
        let analysis_id = uuid::Uuid::new_v4().to_string();
        
        // Check cache first
        let cache_key = self.generate_cache_key(context);
        if let Some(cached_analysis) = self.attribution_cache.get(&cache_key) {
            debug!("Using cached attribution analysis");
            return Ok(cached_analysis.clone());
        }
        
        // Perform attribution analysis
        let factors = self.analyze_attribution_factors(context).await?;
        let attribution = self.synthesize_attribution(context, &factors).await?;
        let confidence = self.calculate_confidence(&factors);
        let recommendations = self.generate_recommendations(context, &factors);
        
        let analysis = GitAttributionAnalysis {
            analysis_id: analysis_id.clone(),
            attribution,
            confidence,
            analyzed_at: Utc::now(),
            factors,
            recommendations,
            metadata: HashMap::new(),
        };
        
        // Cache the analysis
        if self.attribution_cache.len() >= self.config.cache_size {
            // Remove oldest entry (simplified LRU)
            if let Some(first_key) = self.attribution_cache.keys().next().cloned() {
                self.attribution_cache.remove(&first_key);
            }
        }
        self.attribution_cache.insert(cache_key, analysis.clone());
        
        // Record the analysis
        self.record_attribution_analysis(context, &analysis).await?;
        
        info!("Git attribution analysis completed: {} (confidence: {:.2})", analysis_id, confidence);
        Ok(analysis)
    }
    
    /// Analyze factors contributing to attribution
    async fn analyze_attribution_factors(&self, context: &GitAttributionContext) -> Result<Vec<GitAttributionFactor>> {
        let mut factors = Vec::new();
        
        // Analyze code authorship
        if let Some(authorship_factor) = self.analyze_code_authorship(context).await? {
            factors.push(authorship_factor);
        }
        
        // Analyze commit patterns
        if let Some(commit_factor) = self.analyze_commit_patterns(context).await? {
            factors.push(commit_factor);
        }
        
        // Analyze file ownership
        if let Some(ownership_factor) = self.analyze_file_ownership(context).await? {
            factors.push(ownership_factor);
        }
        
        // Analyze operation complexity
        if let Some(complexity_factor) = self.analyze_operation_complexity(context).await? {
            factors.push(complexity_factor);
        }
        
        Ok(factors)
    }
    
    /// Analyze code authorship factor
    async fn analyze_code_authorship(&self, context: &GitAttributionContext) -> Result<Option<GitAttributionFactor>> {
        // Simplified implementation - would analyze git blame, commit history, etc.
        let weight = match context.operation_type {
            GitOperationType::Commit => 0.8,
            GitOperationType::Merge => 0.6,
            GitOperationType::Push => 0.4,
            _ => 0.2,
        };
        
        if weight >= self.config.min_contribution_threshold {
            Ok(Some(GitAttributionFactor {
                factor_type: GitAttributionFactorType::CodeAuthorship,
                weight,
                description: format!("Code authorship for {:?} operation", context.operation_type),
                evidence: vec![
                    format!("Operation type: {:?}", context.operation_type),
                    format!("Files affected: {}", context.affected_files.len()),
                ],
            }))
        } else {
            Ok(None)
        }
    }
    
    /// Analyze commit patterns factor
    async fn analyze_commit_patterns(&self, context: &GitAttributionContext) -> Result<Option<GitAttributionFactor>> {
        // Simplified implementation
        let weight = if context.operation_type == GitOperationType::Commit {
            if let Some(lines_changed) = &context.lines_changed {
                // Weight based on lines changed
                (lines_changed.total as f64 / 1000.0).min(1.0)
            } else {
                0.5
            }
        } else {
            0.1
        };
        
        if weight >= self.config.min_contribution_threshold {
            Ok(Some(GitAttributionFactor {
                factor_type: GitAttributionFactorType::CommitFrequency,
                weight,
                description: "Commit pattern analysis".to_string(),
                evidence: vec![
                    format!("Lines changed: {:?}", context.lines_changed),
                    format!("Branch: {}", context.branch_name),
                ],
            }))
        } else {
            Ok(None)
        }
    }
    
    /// Analyze file ownership factor
    async fn analyze_file_ownership(&self, context: &GitAttributionContext) -> Result<Option<GitAttributionFactor>> {
        // Simplified implementation
        let weight = if !context.affected_files.is_empty() {
            0.6
        } else {
            0.1
        };
        
        if weight >= self.config.min_contribution_threshold {
            Ok(Some(GitAttributionFactor {
                factor_type: GitAttributionFactorType::FileOwnership,
                weight,
                description: "File ownership analysis".to_string(),
                evidence: context.affected_files.clone(),
            }))
        } else {
            Ok(None)
        }
    }
    
    /// Analyze operation complexity factor
    async fn analyze_operation_complexity(&self, context: &GitAttributionContext) -> Result<Option<GitAttributionFactor>> {
        let weight = match context.operation_type {
            GitOperationType::Merge | GitOperationType::Rebase => 0.9,
            GitOperationType::ConflictResolution => 0.8,
            GitOperationType::Commit => 0.6,
            GitOperationType::Push | GitOperationType::Pull => 0.4,
            _ => 0.2,
        };
        
        if weight >= self.config.min_contribution_threshold {
            Ok(Some(GitAttributionFactor {
                factor_type: GitAttributionFactorType::ArchitectureDecisions,
                weight,
                description: format!("Operation complexity for {:?}", context.operation_type),
                evidence: vec![
                    format!("Operation: {:?}", context.operation_type),
                    format!("Repository: {:?}", context.repository_path),
                ],
            }))
        } else {
            Ok(None)
        }
    }
    
    /// Synthesize attribution from factors
    async fn synthesize_attribution(&self, context: &GitAttributionContext, factors: &[GitAttributionFactor]) -> Result<Attribution> {
        // Determine collaboration type based on operation and factors
        let collaboration_type = match context.operation_type {
            GitOperationType::Merge | GitOperationType::ConflictResolution => CollaborationType::Coordination,
            GitOperationType::Commit => CollaborationType::Individual,
            GitOperationType::Push | GitOperationType::Pull => CollaborationType::Coordination,
            _ => CollaborationType::HumanLed,
        };
        
        // Calculate total weight
        let total_weight: f64 = factors.iter().map(|f| f.weight).sum();
        
        // Create attribution
        let attribution = Attribution::new(
            context.base_context.metadata.get("human_contributor").cloned(),
            context.base_context.metadata.get("ai_contributor").cloned(),
            collaboration_type,
            total_weight.min(1.0) as f32,
        );
        
        Ok(attribution)
    }
    
    /// Calculate confidence score
    fn calculate_confidence(&self, factors: &[GitAttributionFactor]) -> f64 {
        if factors.is_empty() {
            return 0.0;
        }
        
        let total_weight: f64 = factors.iter().map(|f| f.weight).sum();
        let factor_count = factors.len() as f64;
        
        // Confidence based on total weight and number of factors
        (total_weight / factor_count).min(1.0)
    }
    
    /// Generate recommendations
    fn generate_recommendations(&self, context: &GitAttributionContext, factors: &[GitAttributionFactor]) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if factors.is_empty() {
            recommendations.push("Consider adding more detailed commit messages".to_string());
        }
        
        if context.affected_files.is_empty() {
            recommendations.push("Ensure file changes are properly tracked".to_string());
        }
        
        let high_weight_factors = factors.iter().filter(|f| f.weight > 0.7).count();
        if high_weight_factors == 0 {
            recommendations.push("Consider breaking down large operations into smaller commits".to_string());
        }
        
        recommendations
    }
    
    /// Generate cache key for attribution context
    fn generate_cache_key(&self, context: &GitAttributionContext) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        context.operation_type.hash(&mut hasher);
        context.repository_path.hash(&mut hasher);
        context.branch_name.hash(&mut hasher);
        context.affected_files.hash(&mut hasher);
        
        format!("git_attr_{:x}", hasher.finish())
    }
    
    /// Record attribution analysis for historical tracking
    async fn record_attribution_analysis(&mut self, context: &GitAttributionContext, analysis: &GitAttributionAnalysis) -> Result<()> {
        let record = GitAttributionRecord {
            record_id: uuid::Uuid::new_v4().to_string(),
            operation_type: context.operation_type.clone(),
            repository_path: context.repository_path.clone(),
            attribution: analysis.attribution.clone(),
            timestamp: Utc::now(),
            parameters: context.git_metadata.clone(),
            confidence: analysis.confidence,
            metadata: HashMap::new(),
        };
        
        self.operation_history.push(record);
        
        // Limit history size
        if self.operation_history.len() > 10000 {
            self.operation_history.drain(0..1000); // Remove oldest 1000 entries
        }
        
        Ok(())
    }
    
    /// Get attribution history for a repository
    pub fn get_attribution_history(&self, repository_path: &Path) -> Vec<&GitAttributionRecord> {
        self.operation_history
            .iter()
            .filter(|record| record.repository_path == repository_path)
            .collect()
    }
    
    /// Get attribution statistics
    pub fn get_attribution_statistics(&self) -> GitAttributionStatistics {
        let total_records = self.operation_history.len();
        let avg_confidence = if total_records > 0 {
            self.operation_history.iter().map(|r| r.confidence).sum::<f64>() / total_records as f64
        } else {
            0.0
        };
        
        let operation_counts = self.operation_history
            .iter()
            .fold(HashMap::new(), |mut acc, record| {
                *acc.entry(record.operation_type.clone()).or_insert(0) += 1;
                acc
            });
        
        GitAttributionStatistics {
            total_analyses: total_records,
            average_confidence: avg_confidence,
            cache_size: self.attribution_cache.len(),
            operation_distribution: operation_counts,
        }
    }
}

/// Statistics about git attribution engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitAttributionStatistics {
    /// Total attribution analyses performed
    pub total_analyses: usize,
    /// Average confidence score
    pub average_confidence: f64,
    /// Current cache size
    pub cache_size: usize,
    /// Distribution of operations analyzed
    pub operation_distribution: HashMap<GitOperationType, usize>,
}

impl GitAttributionContext {
    /// Create git attribution context from git operation
    pub fn from_git_operation(
        operation_type: &GitOperationType,
        parameters: &HashMap<String, String>,
        repository_path: &Path,
    ) -> Self {
        let mut base_context = AttributionContext::new(
            parameters.get("source").cloned().unwrap_or_else(|| format!("git_{:?}", operation_type))
        );
        
        // Add timing information if available
        if let Some(time_since_human) = parameters.get("time_since_human").and_then(|s| s.parse().ok()) {
            base_context.time_since_human = Some(time_since_human);
        }
        if let Some(time_since_ai) = parameters.get("time_since_ai").and_then(|s| s.parse().ok()) {
            base_context.time_since_ai = Some(time_since_ai);
        }
        
        // Set change size if available
        if let Some(change_size) = parameters.get("change_size").and_then(|s| s.parse().ok()) {
            base_context.change_size = change_size;
        }
        
        // Add all parameters as metadata
        for (key, value) in parameters {
            base_context.add_metadata(key.clone(), value.clone());
        }
        
        Self {
            base_context,
            operation_type: operation_type.clone(),
            repository_path: repository_path.to_path_buf(),
            branch_name: parameters.get("branch").cloned().unwrap_or_else(|| "main".to_string()),
            commit_hash: parameters.get("commit_hash").cloned(),
            affected_files: parameters.get("files")
                .map(|f| f.split(',').map(|s| s.trim().to_string()).collect())
                .unwrap_or_default(),
            lines_changed: None, // Would be populated by git analysis
            git_metadata: parameters.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_attribution_config_default() {
        let config = GitAttributionConfig::default();
        assert!(config.enable_detailed_tracking);
        assert_eq!(config.cache_size, 1000);
        assert_eq!(config.min_contribution_threshold, 0.1);
    }
    
    #[test]
    fn test_git_attribution_factor_serialization() {
        let factor = GitAttributionFactor {
            factor_type: GitAttributionFactorType::CodeAuthorship,
            weight: 0.8,
            description: "Test factor".to_string(),
            evidence: vec!["evidence1".to_string(), "evidence2".to_string()],
        };
        
        let serialized = serde_json::to_string(&factor).unwrap();
        let deserialized: GitAttributionFactor = serde_json::from_str(&serialized).unwrap();
        assert_eq!(factor.factor_type, deserialized.factor_type);
        assert_eq!(factor.weight, deserialized.weight);
    }
    
    #[test]
    fn test_git_attribution_context_creation() {
        let mut parameters = HashMap::new();
        parameters.insert("author".to_string(), "test_user".to_string());
        parameters.insert("branch".to_string(), "feature/test".to_string());
        
        let context = GitAttributionContext::from_git_operation(
            &GitOperationType::Commit,
            &parameters,
            Path::new("/test/repo"),
        );
        
        assert_eq!(context.operation_type, GitOperationType::Commit);
        assert_eq!(context.branch_name, "feature/test");
        assert_eq!(context.base_context.source, "test_source");
    }
}
