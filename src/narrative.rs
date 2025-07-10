//! Wurtz-Style Narrative Generation
//! 
//! Hi, you're trying to make complex things simple, floating in communication space.
//! 
//! This module provides tools for transforming technical documentation into
//! accessible, engaging narratives using patterns extracted from Bill Wurtz's
//! "history of the entire world, i guess".

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Core narrative patterns extracted from Bill Wurtz
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WurtzPattern {
    pub context_establishment: String,
    pub complexity_acknowledgment: String,
    pub rapid_transitions: Vec<String>,
    pub positive_reframing: Vec<String>,
    pub pattern_recognition: Vec<String>,
    pub curiosity_hooks: Vec<String>,
}

impl Default for WurtzPattern {
    fn default() -> Self {
        Self {
            context_establishment: "Hi, you're {context}, floating in {space}".to_string(),
            complexity_acknowledgment: "Some of it's {simple}. Actually, most of it's {complex}".to_string(),
            rapid_transitions: vec![
                "Great news! {thing} just {happened}".to_string(),
                "Oh wait, now {thing} is {state}".to_string(),
                "Whoops! {thing} just {action}".to_string(),
            ],
            positive_reframing: vec![
                "Great news! {achievement}".to_string(),
                "Congratulations! {success}".to_string(),
                "Perfect! Now {next_step}".to_string(),
            ],
            pattern_recognition: vec![
                "You could make a {thing} out of this".to_string(),
                "That's pretty {adjective}, I would say".to_string(),
                "This is {assessment}".to_string(),
            ],
            curiosity_hooks: vec![
                "What's {thing}? {explanation}".to_string(),
                "Want to {action}? {solution}".to_string(),
                "How's {thing}? {status}".to_string(),
            ],
        }
    }
}

/// Narrative generator for transforming technical content
#[derive(Debug)]
pub struct WurtzNarrativeGenerator {
    patterns: WurtzPattern,
    context_templates: HashMap<String, String>,
}

impl Default for WurtzNarrativeGenerator {
    fn default() -> Self {
        let mut context_templates = HashMap::new();
        
        // Technical contexts
        context_templates.insert(
            "distributed_systems".to_string(),
            "writing distributed software, floating in complexity".to_string()
        );
        context_templates.insert(
            "git_collaboration".to_string(),
            "collaborating on code, floating in version control space".to_string()
        );
        context_templates.insert(
            "family_communication".to_string(),
            "in a family, floating in emotional space".to_string()
        );
        context_templates.insert(
            "ai_collaboration".to_string(),
            "working with AI, floating in human-machine space".to_string()
        );
        
        Self {
            patterns: WurtzPattern::default(),
            context_templates,
        }
    }
}

impl WurtzNarrativeGenerator {
    /// Create a new narrative generator
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Generate a context establishment opening
    pub fn establish_context(&self, context_type: &str) -> String {
        if let Some(context) = self.context_templates.get(context_type) {
            format!("Hi, you're {}.", context)
        } else {
            format!("Hi, you're trying to understand {}, floating in complexity space.", context_type)
        }
    }
    
    /// Generate complexity acknowledgment
    pub fn acknowledge_complexity(&self, simple_part: &str, complex_part: &str) -> String {
        format!("Some of it's {}. Actually, most of it's {}.", simple_part, complex_part)
    }
    
    /// Generate positive reframing
    pub fn positive_reframe(&self, achievement: &str) -> String {
        format!("Great news! {}", achievement)
    }
    
    /// Generate curiosity hook
    pub fn curiosity_hook(&self, thing: &str, explanation: &str) -> String {
        format!("What's {}? {}", thing, explanation)
    }
    
    /// Generate pattern recognition
    pub fn pattern_recognition(&self, thing: &str) -> String {
        format!("You could make a {} out of this.", thing)
    }
    
    /// Transform technical documentation into Wurtz-style narrative
    pub fn transform_documentation(&self, content: &TechnicalContent) -> String {
        let mut narrative = String::new();
        
        // Context establishment
        narrative.push_str(&self.establish_context(&content.domain));
        narrative.push_str("\n\n");
        
        // Complexity acknowledgment
        if let (Some(simple), Some(complex)) = (&content.simple_aspect, &content.complex_aspect) {
            narrative.push_str(&self.acknowledge_complexity(simple, complex));
            narrative.push_str("\n\n");
        }
        
        // Main content with rapid transitions
        for section in &content.sections {
            match section.section_type {
                SectionType::Problem => {
                    narrative.push_str(&format!("Want to {}? ", section.goal.as_deref().unwrap_or("solve this")));
                }
                SectionType::Solution => {
                    narrative.push_str(&self.positive_reframe(&section.content));
                    narrative.push_str(" ");
                }
                SectionType::Explanation => {
                    if let Some(term) = &section.key_term {
                        narrative.push_str(&self.curiosity_hook(term, &section.content));
                    } else {
                        narrative.push_str(&section.content);
                    }
                    narrative.push_str(" ");
                }
                SectionType::Conclusion => {
                    if let Some(bigger_picture) = &section.bigger_picture {
                        narrative.push_str(&self.pattern_recognition(bigger_picture));
                    }
                }
            }
            narrative.push_str("\n\n");
        }
        
        narrative
    }
    
    /// Generate onboarding flow
    pub fn generate_onboarding(&self, steps: &[OnboardingStep]) -> String {
        let mut narrative = String::new();
        
        narrative.push_str("Welcome! You're about to embark on a journey, floating in learning space.\n\n");
        narrative.push_str("Some of it's familiar. Actually, most of it's new.\n");
        narrative.push_str("Don't worry, we'll get you from here to there without the usual frustration.\n\n");
        
        for (i, step) in steps.iter().enumerate() {
            narrative.push_str(&format!("Step {}: {}\n", i + 1, step.action));
            narrative.push_str(&self.positive_reframe(&step.success_message));
            narrative.push_str("\n\n");
        }
        
        narrative.push_str("Congratulations! You're now ready to build amazing things.\n");
        narrative.push_str(&self.pattern_recognition("whole new system"));
        
        narrative
    }
    
    /// Generate error message
    pub fn generate_error_message(&self, error: &ErrorContext) -> String {
        let mut message = String::new();
        
        message.push_str(&format!("Whoops! {} just {}.\n\n", error.component, error.what_happened));
        message.push_str(&format!("Don't worry, this happens when {}.\n", error.explanation));
        message.push_str(&format!("Want to fix it? Try {}.\n", error.solution));
        
        if let Some(alternative) = &error.alternative {
            message.push_str(&format!("Still broken? Try {}.\n", alternative));
        }
        
        message.push_str("\n");
        message.push_str(&self.positive_reframe("Everything should be working again"));
        
        message
    }
}

/// Structure for technical content to be transformed
#[derive(Debug, Clone)]
pub struct TechnicalContent {
    pub domain: String,
    pub simple_aspect: Option<String>,
    pub complex_aspect: Option<String>,
    pub sections: Vec<ContentSection>,
}

#[derive(Debug, Clone)]
pub struct ContentSection {
    pub section_type: SectionType,
    pub content: String,
    pub goal: Option<String>,
    pub key_term: Option<String>,
    pub bigger_picture: Option<String>,
}

#[derive(Debug, Clone)]
pub enum SectionType {
    Problem,
    Solution,
    Explanation,
    Conclusion,
}

#[derive(Debug, Clone)]
pub struct OnboardingStep {
    pub action: String,
    pub success_message: String,
}

#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub component: String,
    pub what_happened: String,
    pub explanation: String,
    pub solution: String,
    pub alternative: Option<String>,
}

/// Collaborative individuation embodiment through narrative
/// 
/// This module embodies collaborative individuation by:
/// - Individual Voice: Maintains Bill Wurtz's unique perspective and humor
/// - Collective Service: Makes complex shared knowledge accessible to everyone
/// - Emergent Wisdom: Creates understanding through personality-driven explanation
/// - Sacred Alliance: Individual creativity serving collective understanding
pub struct CollaborativeIndividuationNarrative;

impl CollaborativeIndividuationNarrative {
    /// Generate narrative that embodies collaborative individuation principles
    pub fn generate_embodiment_narrative(domain: &str) -> String {
        let generator = WurtzNarrativeGenerator::new();
        
        let mut narrative = String::new();
        narrative.push_str(&generator.establish_context(domain));
        narrative.push_str("\n\n");
        
        narrative.push_str("Pretty cool, huh? Some of it's individual creativity. ");
        narrative.push_str("Actually, most of it's collective wisdom.\n\n");
        
        narrative.push_str("Want to see how individual voices can serve collective understanding? ");
        narrative.push_str(&generator.positive_reframe("We built a framework for that"));
        narrative.push_str("\n\n");
        
        narrative.push_str(&generator.curiosity_hook(
            "collaborative individuation",
            "Individual creativity serving collective understanding through unique perspectives"
        ));
        narrative.push_str("\n\n");
        
        narrative.push_str("This is how we make complex systems human-scale ");
        narrative.push_str("while preserving the depth that makes them powerful.\n\n");
        
        narrative.push_str(&generator.pattern_recognition("sacred alliance between human creativity and collective wisdom"));
        
        narrative
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_establishment() {
        let generator = WurtzNarrativeGenerator::new();
        let context = generator.establish_context("distributed_systems");
        assert!(context.contains("Hi, you're"));
        assert!(context.contains("distributed software"));
    }

    #[test]
    fn test_complexity_acknowledgment() {
        let generator = WurtzNarrativeGenerator::new();
        let ack = generator.acknowledge_complexity("working", "distributed systems");
        assert!(ack.contains("Some of it's working"));
        assert!(ack.contains("most of it's distributed systems"));
    }

    #[test]
    fn test_collaborative_individuation_embodiment() {
        let narrative = CollaborativeIndividuationNarrative::generate_embodiment_narrative("ai_collaboration");
        assert!(narrative.contains("Hi, you're"));
        assert!(narrative.contains("collaborative individuation"));
        assert!(narrative.contains("sacred alliance"));
    }
}
