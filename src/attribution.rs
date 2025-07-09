//! Universal Attribution System
//! 
//! This module provides basic attribution tracking for collaborative work
//! in WeaveMesh, supporting various collaboration patterns while maintaining
//! simplicity and extensibility through plugins.

use std::collections::HashMap;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Unique identifier for attribution records
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AttributionId(Uuid);

impl AttributionId {
    /// Create a new attribution ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Create an attribution ID from a string
    pub fn from_string(s: &str) -> Result<Self, uuid::Error> {
        Ok(Self(Uuid::parse_str(s)?))
    }

    /// Get the string representation
    pub fn as_string(&self) -> String {
        self.0.to_string()
    }
}

impl Default for AttributionId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for AttributionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Types of collaboration patterns
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CollaborationType {
    /// Human-led work with possible AI assistance
    HumanLed,
    /// AI-led work with possible human guidance
    AILed,
    /// Equal collaboration between human and AI
    CoCreated,
    /// Pair programming style collaboration
    PairProgramming,
    /// Individual work (human only)
    Individual,
    /// Automated work (AI only)
    Automated,
    /// Custom collaboration type
    Custom(String),
}

/// Basic attribution information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribution {
    /// Unique identifier for this attribution
    pub id: AttributionId,
    
    /// Human contributor identifier
    pub human_contributor: Option<String>,
    
    /// AI contributor identifier  
    pub ai_contributor: Option<String>,
    
    /// Type of collaboration
    pub collaboration_type: CollaborationType,
    
    /// Confidence in attribution (0.0 to 1.0)
    pub confidence: f32,
    
    /// Timestamp of attribution
    pub timestamp: DateTime<Utc>,
    
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl Attribution {
    /// Create a new attribution
    pub fn new(
        human_contributor: Option<String>,
        ai_contributor: Option<String>,
        collaboration_type: CollaborationType,
        confidence: f32,
    ) -> Self {
        Self {
            id: AttributionId::new(),
            human_contributor,
            ai_contributor,
            collaboration_type,
            confidence: confidence.clamp(0.0, 1.0),
            timestamp: Utc::now(),
            metadata: HashMap::new(),
        }
    }
    
    /// Create a simple human attribution
    pub fn new_human(contributor: String) -> Self {
        Self::new(
            Some(contributor),
            None,
            CollaborationType::Individual,
            1.0,
        )
    }
    
    /// Create a simple AI attribution
    pub fn new_ai(contributor: String) -> Self {
        Self::new(
            None,
            Some(contributor),
            CollaborationType::Automated,
            1.0,
        )
    }
    
    /// Create a collaborative attribution
    pub fn new_collaborative(
        human_contributor: String,
        ai_contributor: String,
        collaboration_type: CollaborationType,
        confidence: f32,
    ) -> Self {
        Self::new(
            Some(human_contributor),
            Some(ai_contributor),
            collaboration_type,
            confidence,
        )
    }
    
    /// Check if this attribution represents collaborative work
    pub fn is_collaborative(&self) -> bool {
        matches!(
            self.collaboration_type,
            CollaborationType::CoCreated
                | CollaborationType::PairProgramming
                | CollaborationType::HumanLed
                | CollaborationType::AILed
        )
    }
    
    /// Check if this attribution has both human and AI contributors
    pub fn has_both_contributors(&self) -> bool {
        self.human_contributor.is_some() && self.ai_contributor.is_some()
    }
    
    /// Add metadata to the attribution
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
    
    /// Get metadata value
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
    
    /// Update confidence level
    pub fn update_confidence(&mut self, confidence: f32) {
        self.confidence = confidence.clamp(0.0, 1.0);
    }
    
    /// Validate attribution completeness
    pub fn validate(&self) -> Result<(), AttributionError> {
        // Check if we have at least one contributor
        if self.human_contributor.is_none() && self.ai_contributor.is_none() {
            return Err(AttributionError::MissingContributor);
        }
        
        // Check if collaborative work has appropriate contributors
        if self.is_collaborative() && !self.has_both_contributors() {
            return Err(AttributionError::IncompleteCollaboration);
        }
        
        // Check confidence bounds
        if self.confidence < 0.0 || self.confidence > 1.0 {
            return Err(AttributionError::InvalidConfidence(self.confidence));
        }
        
        Ok(())
    }
}

/// Context information for attribution analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributionContext {
    /// Source of the change (editor, CLI, API, etc.)
    pub source: String,
    
    /// Time since last human interaction (seconds)
    pub time_since_human: Option<u64>,
    
    /// Time since last AI interaction (seconds)
    pub time_since_ai: Option<u64>,
    
    /// Size of the change (arbitrary units)
    pub change_size: u32,
    
    /// Additional context metadata
    pub metadata: HashMap<String, String>,
}

impl AttributionContext {
    /// Create a new attribution context
    pub fn new(source: String) -> Self {
        Self {
            source,
            time_since_human: None,
            time_since_ai: None,
            change_size: 0,
            metadata: HashMap::new(),
        }
    }
    
    /// Add metadata to the context
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
    
    /// Set timing information
    pub fn with_timing(mut self, time_since_human: Option<u64>, time_since_ai: Option<u64>) -> Self {
        self.time_since_human = time_since_human;
        self.time_since_ai = time_since_ai;
        self
    }
    
    /// Set change size
    pub fn with_change_size(mut self, size: u32) -> Self {
        self.change_size = size;
        self
    }
}

/// Configuration for attribution detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributionConfig {
    /// Minimum confidence threshold for attribution
    pub min_confidence: f32,
    
    /// Time window for collaboration detection (seconds)
    pub collaboration_window: u64,
    
    /// Patterns that indicate human-led work
    pub human_indicators: Vec<String>,
    
    /// Patterns that indicate AI-led work
    pub ai_indicators: Vec<String>,
    
    /// Patterns that indicate collaborative work
    pub collaboration_indicators: Vec<String>,
}

impl Default for AttributionConfig {
    fn default() -> Self {
        Self {
            min_confidence: 0.7,
            collaboration_window: 300, // 5 minutes
            human_indicators: vec![
                "manual".to_string(),
                "user".to_string(),
                "keyboard".to_string(),
                "mouse".to_string(),
                "interactive".to_string(),
            ],
            ai_indicators: vec![
                "ai".to_string(),
                "generated".to_string(),
                "auto".to_string(),
                "assistant".to_string(),
                "automated".to_string(),
            ],
            collaboration_indicators: vec![
                "pair".to_string(),
                "collaborative".to_string(),
                "shared".to_string(),
                "review".to_string(),
                "session".to_string(),
            ],
        }
    }
}

/// Result of attribution analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributionAnalysis {
    /// Detected attribution
    pub attribution: Attribution,
    
    /// Confidence breakdown by factor
    pub confidence_factors: HashMap<String, f32>,
    
    /// Reasoning for the attribution
    pub reasoning: Vec<String>,
    
    /// Suggestions for improving attribution accuracy
    pub suggestions: Vec<String>,
}

/// Basic attribution engine
pub struct BasicAttributionEngine {
    /// Configuration for attribution detection
    config: AttributionConfig,
    
    /// Historical attribution data
    history: Vec<Attribution>,
}

impl BasicAttributionEngine {
    /// Create a new attribution engine
    pub fn new(config: AttributionConfig) -> Self {
        Self {
            config,
            history: Vec::new(),
        }
    }
    
    /// Create with default configuration
    pub fn default() -> Self {
        Self::new(AttributionConfig::default())
    }
    
    /// Analyze context and determine attribution
    pub fn analyze(&mut self, context: AttributionContext) -> Result<AttributionAnalysis, AttributionError> {
        let mut confidence_factors = HashMap::new();
        let mut reasoning = Vec::new();
        
        // Analyze source patterns
        let (human_score, ai_score, collab_score) = self.analyze_source_patterns(&context);
        confidence_factors.insert("source_patterns".to_string(), human_score.max(ai_score).max(collab_score));
        
        // Analyze timing patterns
        let timing_score = self.analyze_timing_patterns(&context);
        confidence_factors.insert("timing_patterns".to_string(), timing_score);
        
        // Determine collaboration type and confidence
        let (collaboration_type, confidence) = self.determine_collaboration_type(
            human_score,
            ai_score,
            collab_score,
            &context,
            &mut reasoning,
        );
        
        // Create attribution
        let attribution = Attribution::new(
            self.extract_human_contributor(&context),
            self.extract_ai_contributor(&context),
            collaboration_type,
            confidence,
        );
        
        // Validate attribution
        attribution.validate()?;
        
        // Generate suggestions
        let suggestions = self.generate_suggestions(&attribution, &confidence_factors);
        
        let analysis = AttributionAnalysis {
            attribution: attribution.clone(),
            confidence_factors,
            reasoning,
            suggestions,
        };
        
        // Store in history
        self.history.push(attribution);
        
        // Limit history size
        if self.history.len() > 1000 {
            self.history.remove(0);
        }
        
        Ok(analysis)
    }
    
    /// Analyze source patterns to determine contributor likelihood
    fn analyze_source_patterns(&self, context: &AttributionContext) -> (f32, f32, f32) {
        let mut human_score: f32 = 0.0;
        let mut ai_score: f32 = 0.0;
        let mut collab_score: f32 = 0.0;
        
        // Check human indicators
        for indicator in &self.config.human_indicators {
            if context.source.contains(indicator) || 
               context.metadata.values().any(|v| v.contains(indicator)) {
                human_score += 0.3;
            }
        }
        
        // Check AI indicators
        for indicator in &self.config.ai_indicators {
            if context.source.contains(indicator) || 
               context.metadata.values().any(|v| v.contains(indicator)) {
                ai_score += 0.3;
            }
        }
        
        // Check collaboration indicators
        for indicator in &self.config.collaboration_indicators {
            if context.source.contains(indicator) || 
               context.metadata.values().any(|v| v.contains(indicator)) {
                collab_score += 0.4;
            }
        }
        
        (human_score.min(1.0), ai_score.min(1.0), collab_score.min(1.0))
    }
    
    /// Analyze timing patterns for collaboration detection
    fn analyze_timing_patterns(&self, context: &AttributionContext) -> f32 {
        let mut score = 0.5; // Neutral baseline
        
        // Recent human interaction increases human likelihood
        if let Some(time_since_human) = context.time_since_human {
            if time_since_human < self.config.collaboration_window {
                score += 0.3 * (1.0 - time_since_human as f32 / self.config.collaboration_window as f32);
            }
        }
        
        // Recent AI interaction increases AI likelihood
        if let Some(time_since_ai) = context.time_since_ai {
            if time_since_ai < self.config.collaboration_window {
                score += 0.3 * (1.0 - time_since_ai as f32 / self.config.collaboration_window as f32);
            }
        }
        
        // Both recent interactions suggest collaboration
        if context.time_since_human.unwrap_or(u64::MAX) < self.config.collaboration_window &&
           context.time_since_ai.unwrap_or(u64::MAX) < self.config.collaboration_window {
            score += 0.4;
        }
        
        score.min(1.0)
    }
    
    /// Determine the most likely collaboration type and confidence
    fn determine_collaboration_type(
        &self,
        human_score: f32,
        ai_score: f32,
        collab_score: f32,
        _context: &AttributionContext,
        reasoning: &mut Vec<String>,
    ) -> (CollaborationType, f32) {
        // If collaboration indicators are strong, it's likely collaborative
        if collab_score > 0.6 {
            reasoning.push("Strong collaboration indicators detected".to_string());
            return (CollaborationType::CoCreated, collab_score);
        }
        
        // If both human and AI scores are significant, it's collaborative
        if human_score > 0.4 && ai_score > 0.4 {
            reasoning.push("Both human and AI indicators present".to_string());
            return (CollaborationType::PairProgramming, (human_score + ai_score) / 2.0);
        }
        
        // Determine primary contributor
        if human_score > ai_score {
            reasoning.push(format!("Human indicators stronger ({:.2} vs {:.2})", human_score, ai_score));
            
            if ai_score > 0.2 {
                reasoning.push("AI assistance detected".to_string());
                return (CollaborationType::HumanLed, human_score);
            } else {
                return (CollaborationType::Individual, human_score);
            }
        } else if ai_score > human_score {
            reasoning.push(format!("AI indicators stronger ({:.2} vs {:.2})", ai_score, human_score));
            
            if human_score > 0.2 {
                reasoning.push("Human guidance detected".to_string());
                return (CollaborationType::AILed, ai_score);
            } else {
                return (CollaborationType::Automated, ai_score);
            }
        } else {
            // Equal scores suggest collaboration
            reasoning.push("Equal human and AI indicators".to_string());
            return (CollaborationType::CoCreated, (human_score + ai_score) / 2.0);
        }
    }
    
    /// Extract human contributor identifier from context
    fn extract_human_contributor(&self, context: &AttributionContext) -> Option<String> {
        // Try to extract from metadata
        if let Some(user) = context.metadata.get("user") {
            return Some(user.clone());
        }
        
        if let Some(author) = context.metadata.get("author") {
            return Some(author.clone());
        }
        
        // Try to extract from source
        if context.source.contains("human") || context.source.contains("user") {
            return Some("human_user".to_string());
        }
        
        None
    }
    
    /// Extract AI contributor identifier from context
    fn extract_ai_contributor(&self, context: &AttributionContext) -> Option<String> {
        // Try to extract from metadata
        if let Some(ai) = context.metadata.get("ai_assistant") {
            return Some(ai.clone());
        }
        
        if let Some(model) = context.metadata.get("model") {
            return Some(model.clone());
        }
        
        // Try to extract from source
        if context.source.contains("ai") || context.source.contains("assistant") {
            return Some("ai_assistant".to_string());
        }
        
        None
    }
    
    /// Generate suggestions for improving attribution accuracy
    fn generate_suggestions(
        &self,
        attribution: &Attribution,
        confidence_factors: &HashMap<String, f32>,
    ) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        // Low confidence suggestions
        if attribution.confidence < self.config.min_confidence {
            suggestions.push("Consider adding more context metadata for better attribution".to_string());
        }
        
        // Missing contributor suggestions
        if attribution.human_contributor.is_none() {
            suggestions.push("Add human contributor identification".to_string());
        }
        
        if attribution.ai_contributor.is_none() {
            suggestions.push("Add AI contributor identification".to_string());
        }
        
        // Low factor scores
        for (factor, score) in confidence_factors {
            if *score < 0.3 {
                suggestions.push(format!("Improve {} detection patterns", factor));
            }
        }
        
        suggestions
    }
    
    /// Get attribution history
    pub fn get_history(&self) -> &[Attribution] {
        &self.history
    }
    
    /// Get attribution statistics
    pub fn get_statistics(&self) -> AttributionStatistics {
        let total = self.history.len();
        let mut collaboration_types = HashMap::new();
        let mut total_confidence = 0.0;
        
        for attribution in &self.history {
            let count = collaboration_types.entry(format!("{:?}", attribution.collaboration_type)).or_insert(0);
            *count += 1;
            total_confidence += attribution.confidence;
        }
        
        AttributionStatistics {
            total_attributions: total,
            average_confidence: if total > 0 { total_confidence / total as f32 } else { 0.0 },
            collaboration_type_distribution: collaboration_types,
        }
    }
}

/// Attribution statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributionStatistics {
    /// Total number of attributions made
    pub total_attributions: usize,
    
    /// Average confidence across all attributions
    pub average_confidence: f32,
    
    /// Distribution of collaboration types
    pub collaboration_type_distribution: HashMap<String, usize>,
}

/// Attribution-related errors
#[derive(Debug, thiserror::Error)]
pub enum AttributionError {
    #[error("Missing contributor information")]
    MissingContributor,
    
    #[error("Incomplete collaboration attribution")]
    IncompleteCollaboration,
    
    #[error("Invalid confidence value: {0}")]
    InvalidConfidence(f32),
    
    #[error("Attribution analysis failed: {0}")]
    AnalysisFailed(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

/// Attribution builder for easy construction
pub struct AttributionBuilder {
    human_contributor: Option<String>,
    ai_contributor: Option<String>,
    collaboration_type: CollaborationType,
    confidence: f32,
    metadata: HashMap<String, String>,
}

impl Default for AttributionBuilder {
    fn default() -> Self {
        Self {
            human_contributor: None,
            ai_contributor: None,
            collaboration_type: CollaborationType::Individual,
            confidence: 1.0,
            metadata: HashMap::new(),
        }
    }
}

impl AttributionBuilder {
    /// Create a new attribution builder
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set human contributor
    pub fn human(mut self, contributor: String) -> Self {
        self.human_contributor = Some(contributor);
        self
    }
    
    /// Set AI contributor
    pub fn ai(mut self, contributor: String) -> Self {
        self.ai_contributor = Some(contributor);
        self
    }
    
    /// Set collaboration type
    pub fn collaboration_type(mut self, collaboration_type: CollaborationType) -> Self {
        self.collaboration_type = collaboration_type;
        self
    }
    
    /// Set confidence
    pub fn confidence(mut self, confidence: f32) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }
    
    /// Add metadata
    pub fn metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
    
    /// Build the attribution
    pub fn build(self) -> Attribution {
        let mut attribution = Attribution::new(
            self.human_contributor,
            self.ai_contributor,
            self.collaboration_type,
            self.confidence,
        );
        attribution.metadata = self.metadata;
        attribution
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attribution_creation() {
        let attribution = Attribution::new_human("alice".to_string());
        assert_eq!(attribution.human_contributor, Some("alice".to_string()));
        assert_eq!(attribution.ai_contributor, None);
        assert_eq!(attribution.collaboration_type, CollaborationType::Individual);
        assert_eq!(attribution.confidence, 1.0);
    }
    
    #[test]
    fn test_attribution_validation() {
        let mut attribution = Attribution::new(None, None, CollaborationType::Individual, 0.5);
        assert!(attribution.validate().is_err());
        
        attribution.human_contributor = Some("alice".to_string());
        assert!(attribution.validate().is_ok());
    }
    
    #[test]
    fn test_attribution_builder() {
        let attribution = AttributionBuilder::new()
            .human("alice".to_string())
            .ai("claude".to_string())
            .collaboration_type(CollaborationType::CoCreated)
            .confidence(0.8)
            .metadata("project".to_string(), "test".to_string())
            .build();
        
        assert_eq!(attribution.human_contributor, Some("alice".to_string()));
        assert_eq!(attribution.ai_contributor, Some("claude".to_string()));
        assert_eq!(attribution.collaboration_type, CollaborationType::CoCreated);
        assert_eq!(attribution.confidence, 0.8);
        assert!(attribution.is_collaborative());
        assert!(attribution.has_both_contributors());
    }
    
    #[test]
    fn test_basic_attribution_engine() {
        let mut engine = BasicAttributionEngine::default();
        
        let mut context = AttributionContext::new("manual edit".to_string());
        context.add_metadata("user".to_string(), "alice".to_string());
        
        let analysis = engine.analyze(context).unwrap();
        assert!(analysis.attribution.human_contributor.is_some());
        assert!(!analysis.reasoning.is_empty());
    }
    
    #[test]
    fn test_collaboration_detection() {
        let mut engine = BasicAttributionEngine::default();
        
        let mut context = AttributionContext::new("pair programming session".to_string());
        context.add_metadata("user".to_string(), "alice".to_string());
        context.add_metadata("ai_assistant".to_string(), "claude".to_string());
        context = context.with_timing(Some(30), Some(45));
        
        let analysis = engine.analyze(context).unwrap();
        assert!(analysis.attribution.is_collaborative());
        assert!(analysis.attribution.has_both_contributors());
    }
}
