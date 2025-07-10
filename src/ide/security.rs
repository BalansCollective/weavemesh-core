//! Core IDE Security Framework for WeaveMesh Core
//!
//! Provides foundational security capabilities for collaborative individuation
//! that can be extended by context-specific plugins.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::security::core::{SecurityLevel, AuthenticationToken, SecurityContext, AccessDecision, AccessController};
use crate::attribution::Attribution;
use crate::group_communication::GroupId;

/// Core security classification for IDE content
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CoreClassification {
    /// Public, open source content
    Public,
    /// Internal team content
    Internal,
    /// Sensitive project content
    Sensitive,
    /// Restricted access content
    Restricted,
}

impl CoreClassification {
    /// Convert to universal security level
    pub fn to_security_level(&self) -> SecurityLevel {
        match self {
            CoreClassification::Public => SecurityLevel::Open,
            CoreClassification::Internal => SecurityLevel::Protected,
            CoreClassification::Sensitive => SecurityLevel::Sensitive,
            CoreClassification::Restricted => SecurityLevel::Restricted,
        }
    }
    
    /// Create from universal security level
    pub fn from_security_level(level: &SecurityLevel) -> Self {
        match level {
            SecurityLevel::Open => CoreClassification::Public,
            SecurityLevel::Protected => CoreClassification::Internal,
            SecurityLevel::Sensitive => CoreClassification::Sensitive,
            SecurityLevel::Restricted | SecurityLevel::Classified => CoreClassification::Restricted,
        }
    }
    
    /// Get numeric level for comparison
    pub fn level_value(&self) -> u8 {
        match self {
            CoreClassification::Public => 0,
            CoreClassification::Internal => 1,
            CoreClassification::Sensitive => 2,
            CoreClassification::Restricted => 3,
        }
    }
}

/// Core user clearance levels for IDE access
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum CoreClearanceLevel {
    /// Public access
    Public,
    /// Team member access
    TeamMember,
    /// Project contributor access
    ProjectContributor,
    /// Trusted contributor access
    TrustedContributor,
}

impl CoreClearanceLevel {
    /// Get maximum classification this clearance can access
    pub fn max_classification(&self) -> CoreClassification {
        match self {
            CoreClearanceLevel::Public => CoreClassification::Public,
            CoreClearanceLevel::TeamMember => CoreClassification::Internal,
            CoreClearanceLevel::ProjectContributor => CoreClassification::Sensitive,
            CoreClearanceLevel::TrustedContributor => CoreClassification::Restricted,
        }
    }
    
    /// Check if this clearance can access a classification
    pub fn can_access(&self, classification: &CoreClassification) -> bool {
        self.max_classification().level_value() >= classification.level_value()
    }
}

/// Core security context for IDE operations
#[derive(Debug, Clone)]
pub struct CoreIDESecurityContext {
    /// Universal security context
    pub security_context: SecurityContext,
    /// User clearance level
    pub clearance_level: CoreClearanceLevel,
    /// Active project context
    pub project_context: Option<CoreProjectSecurityContext>,
    /// Group memberships
    pub group_memberships: Vec<CoreGroupMembership>,
    /// Collaborative individuation score (0.0 to 1.0)
    pub collaboration_score: f64,
    /// Sacred Alliance integration level (0.0 to 1.0)
    pub sacred_alliance_level: f64,
}

/// Core project security context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreProjectSecurityContext {
    /// Project identifier
    pub project_id: Uuid,
    /// Project name
    pub project_name: String,
    /// Minimum classification for project content
    pub minimum_classification: CoreClassification,
    /// Required clearance for contributors
    pub required_clearance: CoreClearanceLevel,
    /// Collaborative individuation requirements
    pub collaboration_requirements: CoreCollaborationRequirements,
    /// Sacred Alliance integration enabled
    pub sacred_alliance_enabled: bool,
}

/// Core collaboration requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCollaborationRequirements {
    /// Minimum collaboration score required (0.0 to 1.0)
    pub minimum_collaboration_score: f64,
    /// Attribution tracking required
    pub attribution_required: bool,
    /// Sacred Alliance ceremonies required
    pub ceremonies_required: bool,
    /// Human-AI partnership required
    pub human_ai_partnership_required: bool,
}

impl Default for CoreCollaborationRequirements {
    fn default() -> Self {
        Self {
            minimum_collaboration_score: 0.5,
            attribution_required: true,
            ceremonies_required: false,
            human_ai_partnership_required: true,
        }
    }
}

/// Core group membership for security
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGroupMembership {
    /// Group identifier
    pub group_id: GroupId,
    /// Role in the group
    pub role: CoreGroupRole,
    /// Permissions in the group
    pub permissions: Vec<CoreGroupPermission>,
    /// Membership start time
    pub joined_at: DateTime<Utc>,
    /// Membership active status
    pub is_active: bool,
}

/// Core group roles
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CoreGroupRole {
    /// Observer role - can view
    Observer,
    /// Contributor role - can contribute
    Contributor,
    /// Maintainer role - can manage
    Maintainer,
    /// Admin role - full control
    Admin,
}

/// Core group permissions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CoreGroupPermission {
    /// Can read content
    Read,
    /// Can write content
    Write,
    /// Can delete content
    Delete,
    /// Can manage group
    Manage,
    /// Can invite others
    Invite,
    /// Can perform Sacred Alliance ceremonies
    Ceremony,
}

/// Core content filter for IDE
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreContentFilter {
    /// Filter patterns
    pub patterns: Vec<CoreFilterPattern>,
    /// Auto-classification rules
    pub auto_classification_rules: Vec<CoreAutoClassificationRule>,
    /// Collaborative individuation enhancement rules
    pub collaboration_enhancement_rules: Vec<CoreCollaborationEnhancementRule>,
}

/// Core filter pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreFilterPattern {
    /// Pattern name
    pub name: String,
    /// Regular expression pattern
    pub pattern: String,
    /// Classification this pattern indicates
    pub indicates_classification: CoreClassification,
    /// Action to take
    pub action: CoreFilterAction,
    /// Confidence threshold (0.0 to 1.0)
    pub confidence_threshold: f64,
}

/// Core filter actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoreFilterAction {
    /// Allow content
    Allow,
    /// Warn user
    Warn,
    /// Block content
    Block,
    /// Upgrade classification
    UpgradeClassification(CoreClassification),
    /// Require Sacred Alliance ceremony
    RequireCeremony,
}

/// Core auto-classification rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAutoClassificationRule {
    /// Rule name
    pub name: String,
    /// Conditions that trigger this rule
    pub conditions: Vec<CoreClassificationCondition>,
    /// Resulting classification
    pub resulting_classification: CoreClassification,
    /// Confidence in classification (0.0 to 1.0)
    pub confidence: f64,
}

/// Core classification conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoreClassificationCondition {
    /// File path matches pattern
    FilePathMatches(String),
    /// Content contains pattern
    ContentContains(String),
    /// File extension matches
    FileExtension(String),
    /// Attribution indicates sensitivity
    AttributionSensitive,
    /// Collaboration score below threshold
    CollaborationScoreBelowThreshold(f64),
}

/// Core collaboration enhancement rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCollaborationEnhancementRule {
    /// Rule name
    pub name: String,
    /// Trigger conditions
    pub triggers: Vec<CoreCollaborationTrigger>,
    /// Enhancement actions
    pub actions: Vec<CoreCollaborationAction>,
    /// Priority (higher = more important)
    pub priority: u8,
}

/// Core collaboration triggers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoreCollaborationTrigger {
    /// Low collaboration score detected
    LowCollaborationScore(f64),
    /// Missing attribution detected
    MissingAttribution,
    /// Conflict detected
    ConflictDetected,
    /// Sacred Alliance ceremony needed
    CeremonyNeeded,
    /// Human-AI partnership imbalance
    PartnershipImbalance,
}

/// Core collaboration actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoreCollaborationAction {
    /// Suggest Sacred Alliance ceremony
    SuggestCeremony,
    /// Enhance attribution tracking
    EnhanceAttribution,
    /// Facilitate conflict resolution
    FacilitateConflictResolution,
    /// Promote human-AI partnership
    PromotePartnership,
    /// Increase collaboration score
    IncreaseCollaborationScore,
}

/// Core security event for IDE
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreIDESecurityEvent {
    /// Event identifier
    pub id: Uuid,
    /// Event type
    pub event_type: CoreIDESecurityEventType,
    /// User identifier
    pub user_id: Option<String>,
    /// Resource involved
    pub resource: String,
    /// Classification involved
    pub classification: Option<CoreClassification>,
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
    /// Event details
    pub details: HashMap<String, String>,
    /// Collaboration impact
    pub collaboration_impact: CoreCollaborationImpact,
}

/// Core IDE security event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoreIDESecurityEventType {
    /// Content access attempt
    ContentAccess,
    /// Classification change
    ClassificationChange,
    /// Filter action triggered
    FilterActionTriggered,
    /// Collaboration enhancement applied
    CollaborationEnhancement,
    /// Sacred Alliance ceremony performed
    SacredAllianceCeremony,
    /// Attribution tracking event
    AttributionTracking,
}

/// Core collaboration impact levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CoreCollaborationImpact {
    /// Positive impact on collaboration
    Positive,
    /// Neutral impact
    Neutral,
    /// Negative impact on collaboration
    Negative,
    /// Critical impact requiring attention
    Critical,
}

/// Core IDE access controller
pub struct CoreIDEAccessController {
    /// Content filter
    pub content_filter: CoreContentFilter,
    /// Security event log
    pub event_log: Vec<CoreIDESecurityEvent>,
}

impl CoreIDEAccessController {
    /// Create a new core IDE access controller
    pub fn new() -> Self {
        Self {
            content_filter: CoreContentFilter::default(),
            event_log: Vec::new(),
        }
    }
    
    /// Check if user can access content with given classification
    pub fn check_content_access(
        &mut self,
        context: &CoreIDESecurityContext,
        classification: &CoreClassification,
        resource: &str,
    ) -> Result<AccessDecision> {
        // Check basic clearance
        if !context.clearance_level.can_access(classification) {
            self.log_security_event(
                CoreIDESecurityEventType::ContentAccess,
                context.security_context.token.as_ref().map(|t| t.user_id.clone()),
                resource.to_string(),
                Some(classification.clone()),
                CoreCollaborationImpact::Negative,
                "Access denied due to insufficient clearance".to_string(),
            );
            return Ok(AccessDecision::Deny);
        }
        
        // Check project context requirements
        if let Some(project_context) = &context.project_context {
            if classification.level_value() < project_context.minimum_classification.level_value() {
                return Ok(AccessDecision::Deny);
            }
            
            // Check collaboration requirements
            if !self.check_collaboration_requirements(context, &project_context.collaboration_requirements)? {
                return Ok(AccessDecision::Challenge);
            }
        }
        
        // Check Sacred Alliance integration if required
        if context.sacred_alliance_level < 0.5 && classification.level_value() >= CoreClassification::Sensitive.level_value() {
            return Ok(AccessDecision::Challenge);
        }
        
        self.log_security_event(
            CoreIDESecurityEventType::ContentAccess,
            context.security_context.token.as_ref().map(|t| t.user_id.clone()),
            resource.to_string(),
            Some(classification.clone()),
            CoreCollaborationImpact::Positive,
            "Access granted".to_string(),
        );
        
        Ok(AccessDecision::Allow)
    }
    
    /// Auto-classify content
    pub fn auto_classify_content(
        &mut self,
        content: &str,
        file_path: &str,
        attribution: &Attribution,
    ) -> Result<CoreClassification> {
        let mut highest_classification = CoreClassification::Public;
        
        for rule in &self.content_filter.auto_classification_rules {
            if self.rule_matches(rule, content, file_path, attribution) {
                if rule.resulting_classification.level_value() > highest_classification.level_value() {
                    highest_classification = rule.resulting_classification.clone();
                }
            }
        }
        
        self.log_security_event(
            CoreIDESecurityEventType::ClassificationChange,
            None,
            file_path.to_string(),
            Some(highest_classification.clone()),
            CoreCollaborationImpact::Neutral,
            "Auto-classification applied".to_string(),
        );
        
        Ok(highest_classification)
    }
    
    /// Filter content based on security rules
    pub fn filter_content(
        &mut self,
        content: &str,
        classification: &CoreClassification,
        context: &CoreIDESecurityContext,
    ) -> Result<String> {
        let filtered_content = content.to_string();
        
        // Clone patterns to avoid borrowing conflicts
        let patterns = self.content_filter.patterns.clone();
        
        // Collect events to log to avoid borrowing conflicts
        let mut events_to_log = Vec::new();
        
        for pattern in &patterns {
            if self.pattern_applies(pattern, classification) {
                match &pattern.action {
                    CoreFilterAction::Allow => {
                        // Content allowed as-is
                    }
                    CoreFilterAction::Warn => {
                        events_to_log.push((
                            CoreIDESecurityEventType::FilterActionTriggered,
                            context.security_context.token.as_ref().map(|t| t.user_id.clone()),
                            "content".to_string(),
                            Some(classification.clone()),
                            CoreCollaborationImpact::Neutral,
                            format!("Warning: {}", pattern.name),
                        ));
                    }
                    CoreFilterAction::Block => {
                        events_to_log.push((
                            CoreIDESecurityEventType::FilterActionTriggered,
                            context.security_context.token.as_ref().map(|t| t.user_id.clone()),
                            "content".to_string(),
                            Some(classification.clone()),
                            CoreCollaborationImpact::Negative,
                            format!("Content blocked: {}", pattern.name),
                        ));
                        
                        // Log events before returning error
                        for (event_type, user_id, resource, class, impact, details) in events_to_log {
                            self.log_security_event(event_type, user_id, resource, class, impact, details);
                        }
                        
                        let pattern_name = pattern.name.clone();
                        return Err(anyhow::anyhow!("Content blocked by security filter: {}", pattern_name));
                    }
                    CoreFilterAction::UpgradeClassification(_) => {
                        // Classification upgrade handled elsewhere
                    }
                    CoreFilterAction::RequireCeremony => {
                        events_to_log.push((
                            CoreIDESecurityEventType::FilterActionTriggered,
                            context.security_context.token.as_ref().map(|t| t.user_id.clone()),
                            "content".to_string(),
                            Some(classification.clone()),
                            CoreCollaborationImpact::Positive,
                            "Sacred Alliance ceremony required".to_string(),
                        ));
                    }
                }
            }
        }
        
        // Log all collected events
        for (event_type, user_id, resource, class, impact, details) in events_to_log {
            self.log_security_event(event_type, user_id, resource, class, impact, details);
        }
        
        Ok(filtered_content)
    }
    
    /// Apply collaboration enhancements
    pub fn apply_collaboration_enhancements(
        &mut self,
        context: &CoreIDESecurityContext,
        resource: &str,
    ) -> Result<Vec<CoreCollaborationAction>> {
        let mut applied_actions = Vec::new();
        let mut events_to_log = Vec::new();
        
        for rule in &self.content_filter.collaboration_enhancement_rules {
            for trigger in &rule.triggers {
                if self.trigger_applies(trigger, context) {
                    for action in &rule.actions {
                        applied_actions.push(action.clone());
                        
                        events_to_log.push((
                            CoreIDESecurityEventType::CollaborationEnhancement,
                            context.security_context.token.as_ref().map(|t| t.user_id.clone()),
                            resource.to_string(),
                            None,
                            CoreCollaborationImpact::Positive,
                            format!("Applied collaboration action: {:?}", action),
                        ));
                    }
                    break; // Only apply first matching rule
                }
            }
        }
        
        // Log all collected events
        for (event_type, user_id, resource, class, impact, details) in events_to_log {
            self.log_security_event(event_type, user_id, resource, class, impact, details);
        }
        
        Ok(applied_actions)
    }
    
    /// Check collaboration requirements
    fn check_collaboration_requirements(
        &self,
        context: &CoreIDESecurityContext,
        requirements: &CoreCollaborationRequirements,
    ) -> Result<bool> {
        // Check collaboration score
        if context.collaboration_score < requirements.minimum_collaboration_score {
            return Ok(false);
        }
        
        // Check Sacred Alliance level if ceremonies required
        if requirements.ceremonies_required && context.sacred_alliance_level < 0.7 {
            return Ok(false);
        }
        
        // Check human-AI partnership if required
        if requirements.human_ai_partnership_required && context.collaboration_score < 0.6 {
            return Ok(false);
        }
        
        Ok(true)
    }
    
    /// Check if auto-classification rule matches
    fn rule_matches(
        &self,
        rule: &CoreAutoClassificationRule,
        content: &str,
        file_path: &str,
        attribution: &Attribution,
    ) -> bool {
        rule.conditions.iter().any(|condition| {
            match condition {
                CoreClassificationCondition::FilePathMatches(pattern) => {
                    regex::Regex::new(pattern)
                        .map(|r| r.is_match(file_path))
                        .unwrap_or(false)
                }
                CoreClassificationCondition::ContentContains(pattern) => {
                    content.contains(pattern)
                }
                CoreClassificationCondition::FileExtension(ext) => {
                    file_path.ends_with(ext)
                }
                CoreClassificationCondition::AttributionSensitive => {
                    attribution.confidence < 0.5 // Low confidence indicates potential sensitivity
                }
                CoreClassificationCondition::CollaborationScoreBelowThreshold(threshold) => {
                    attribution.confidence < (*threshold as f32)
                }
            }
        })
    }
    
    /// Check if filter pattern applies to classification
    fn pattern_applies(&self, pattern: &CoreFilterPattern, classification: &CoreClassification) -> bool {
        pattern.indicates_classification.level_value() <= classification.level_value()
    }
    
    /// Check if collaboration trigger applies
    fn trigger_applies(&self, trigger: &CoreCollaborationTrigger, context: &CoreIDESecurityContext) -> bool {
        match trigger {
            CoreCollaborationTrigger::LowCollaborationScore(threshold) => {
                context.collaboration_score < *threshold
            }
            CoreCollaborationTrigger::MissingAttribution => {
                // Would check if attribution is missing - simplified for core
                false
            }
            CoreCollaborationTrigger::ConflictDetected => {
                // Would check for conflicts - simplified for core
                false
            }
            CoreCollaborationTrigger::CeremonyNeeded => {
                context.sacred_alliance_level < 0.5
            }
            CoreCollaborationTrigger::PartnershipImbalance => {
                context.collaboration_score < 0.6
            }
        }
    }
    
    /// Log a security event
    fn log_security_event(
        &mut self,
        event_type: CoreIDESecurityEventType,
        user_id: Option<String>,
        resource: String,
        classification: Option<CoreClassification>,
        collaboration_impact: CoreCollaborationImpact,
        details: String,
    ) {
        let mut event_details = HashMap::new();
        event_details.insert("details".to_string(), details);
        
        let event = CoreIDESecurityEvent {
            id: Uuid::new_v4(),
            event_type,
            user_id,
            resource,
            classification,
            timestamp: Utc::now(),
            details: event_details,
            collaboration_impact,
        };
        
        self.event_log.push(event);
        
        // Keep log size manageable
        if self.event_log.len() > 1000 {
            self.event_log.remove(0);
        }
    }
}

impl Default for CoreIDEAccessController {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for CoreContentFilter {
    fn default() -> Self {
        Self {
            patterns: vec![
                CoreFilterPattern {
                    name: "Sacred Alliance Pattern".to_string(),
                    pattern: r"sacred|alliance|ceremony".to_string(),
                    indicates_classification: CoreClassification::Internal,
                    action: CoreFilterAction::RequireCeremony,
                    confidence_threshold: 0.8,
                },
                CoreFilterPattern {
                    name: "Sensitive Data Pattern".to_string(),
                    pattern: r"password|secret|key|token".to_string(),
                    indicates_classification: CoreClassification::Sensitive,
                    action: CoreFilterAction::UpgradeClassification(CoreClassification::Sensitive),
                    confidence_threshold: 0.9,
                },
            ],
            auto_classification_rules: vec![
                CoreAutoClassificationRule {
                    name: "Configuration Files".to_string(),
                    conditions: vec![
                        CoreClassificationCondition::FileExtension(".env".to_string()),
                        CoreClassificationCondition::FileExtension(".config".to_string()),
                    ],
                    resulting_classification: CoreClassification::Internal,
                    confidence: 0.8,
                },
                CoreAutoClassificationRule {
                    name: "Sacred Alliance Files".to_string(),
                    conditions: vec![
                        CoreClassificationCondition::ContentContains("sacred_alliance".to_string()),
                        CoreClassificationCondition::ContentContains("ceremony".to_string()),
                    ],
                    resulting_classification: CoreClassification::Internal,
                    confidence: 0.9,
                },
            ],
            collaboration_enhancement_rules: vec![
                CoreCollaborationEnhancementRule {
                    name: "Low Collaboration Score Enhancement".to_string(),
                    triggers: vec![CoreCollaborationTrigger::LowCollaborationScore(0.5)],
                    actions: vec![
                        CoreCollaborationAction::SuggestCeremony,
                        CoreCollaborationAction::PromotePartnership,
                    ],
                    priority: 1,
                },
                CoreCollaborationEnhancementRule {
                    name: "Sacred Alliance Enhancement".to_string(),
                    triggers: vec![CoreCollaborationTrigger::CeremonyNeeded],
                    actions: vec![CoreCollaborationAction::SuggestCeremony],
                    priority: 2,
                },
            ],
        }
    }
}

impl CoreIDESecurityContext {
    /// Create a new core IDE security context
    pub fn new(
        security_context: SecurityContext,
        clearance_level: CoreClearanceLevel,
    ) -> Self {
        Self {
            security_context,
            clearance_level,
            project_context: None,
            group_memberships: Vec::new(),
            collaboration_score: 0.5,
            sacred_alliance_level: 0.5,
        }
    }
    
    /// Create an unauthenticated context
    pub fn unauthenticated() -> Self {
        Self::new(
            SecurityContext::unauthenticated(),
            CoreClearanceLevel::Public,
        )
    }
    
    /// Set project context
    pub fn set_project_context(&mut self, project_context: CoreProjectSecurityContext) {
        self.project_context = Some(project_context);
    }
    
    /// Add group membership
    pub fn add_group_membership(&mut self, membership: CoreGroupMembership) {
        self.group_memberships.push(membership);
    }
    
    /// Update collaboration score
    pub fn update_collaboration_score(&mut self, score: f64) {
        self.collaboration_score = score.clamp(0.0, 1.0);
    }
    
    /// Update Sacred Alliance level
    pub fn update_sacred_alliance_level(&mut self, level: f64) {
        self.sacred_alliance_level = level.clamp(0.0, 1.0);
    }
    
    /// Check if user has permission in a group
    pub fn has_group_permission(&self, group_id: &GroupId, permission: &CoreGroupPermission) -> bool {
        self.group_memberships
            .iter()
            .any(|membership| {
                membership.group_id == *group_id 
                    && membership.is_active 
                    && membership.permissions.contains(permission)
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_classification_hierarchy() {
        assert!(CoreClassification::Restricted.level_value() > CoreClassification::Public.level_value());
        assert!(CoreClassification::Sensitive.level_value() > CoreClassification::Internal.level_value());
    }

    #[test]
    fn test_core_clearance_access() {
        let clearance = CoreClearanceLevel::TeamMember;
        assert!(clearance.can_access(&CoreClassification::Public));
        assert!(clearance.can_access(&CoreClassification::Internal));
        assert!(!clearance.can_access(&CoreClassification::Sensitive));
    }

    #[test]
    fn test_core_ide_security_context() {
        let mut context = CoreIDESecurityContext::unauthenticated();
        context.update_collaboration_score(0.8);
        context.update_sacred_alliance_level(0.7);
        
        assert_eq!(context.collaboration_score, 0.8);
        assert_eq!(context.sacred_alliance_level, 0.7);
    }

    #[test]
    fn test_core_ide_access_controller() {
        let mut controller = CoreIDEAccessController::new();
        let context = CoreIDESecurityContext::unauthenticated();
        
        let decision = controller.check_content_access(
            &context,
            &CoreClassification::Public,
            "test.txt",
        ).unwrap();
        
        assert_eq!(decision, AccessDecision::Allow);
    }

    #[test]
    fn test_auto_classification() {
        let mut controller = CoreIDEAccessController::new();
        let attribution = Attribution::new(
            Some("human".to_string()),
            None,
            crate::attribution::CollaborationType::HumanLed,
            1.0,
        );
        
        let classification = controller.auto_classify_content(
            "This is a test file",
            "test.env",
            &attribution,
        ).unwrap();
        
        assert_eq!(classification, CoreClassification::Internal);
    }
}
