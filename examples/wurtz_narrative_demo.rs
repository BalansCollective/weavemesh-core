//! Wurtz-Style Narrative Demo
//! 
//! Hi, you're about to see how complex technical concepts become accessible, 
//! floating in narrative space.

use weavemesh_core::narrative::{
    WurtzNarrativeGenerator, TechnicalContent, ContentSection, SectionType,
    OnboardingStep, ErrorContext, CollaborativeIndividuationNarrative,
};

fn main() {
    println!("🎭 Wurtz-Style Narrative Integration Demo");
    println!("=========================================\n");

    // Demo 1: Basic narrative generation
    demo_basic_narrative();
    
    // Demo 2: Technical documentation transformation
    demo_technical_documentation();
    
    // Demo 3: Onboarding flow generation
    demo_onboarding_flow();
    
    // Demo 4: Error message generation
    demo_error_messages();
    
    // Demo 5: Collaborative individuation embodiment
    demo_collaborative_individuation();
}

fn demo_basic_narrative() {
    println!("📖 Demo 1: Basic Narrative Patterns");
    println!("-----------------------------------");
    
    let generator = WurtzNarrativeGenerator::new();
    
    // Context establishment
    let context = generator.establish_context("distributed_systems");
    println!("Context: {}", context);
    
    // Complexity acknowledgment
    let complexity = generator.acknowledge_complexity("working", "distributed systems");
    println!("Complexity: {}", complexity);
    
    // Positive reframing
    let positive = generator.positive_reframe("Your nodes are talking to each other");
    println!("Positive: {}", positive);
    
    // Curiosity hook
    let curiosity = generator.curiosity_hook(
        "mesh networking", 
        "Nodes that find each other and share information automatically"
    );
    println!("Curiosity: {}", curiosity);
    
    // Pattern recognition
    let pattern = generator.pattern_recognition("distributed system");
    println!("Pattern: {}", pattern);
    
    println!();
}

fn demo_technical_documentation() {
    println!("📚 Demo 2: Technical Documentation Transformation");
    println!("------------------------------------------------");
    
    let generator = WurtzNarrativeGenerator::new();
    
    let content = TechnicalContent {
        domain: "distributed_systems".to_string(),
        simple_aspect: Some("sending messages".to_string()),
        complex_aspect: Some("distributed consensus and fault tolerance".to_string()),
        sections: vec![
            ContentSection {
                section_type: SectionType::Problem,
                content: "make nodes communicate reliably".to_string(),
                goal: Some("build a mesh network".to_string()),
                key_term: None,
                bigger_picture: None,
            },
            ContentSection {
                section_type: SectionType::Solution,
                content: "We invented Zenoh-based mesh networking".to_string(),
                goal: None,
                key_term: None,
                bigger_picture: None,
            },
            ContentSection {
                section_type: SectionType::Explanation,
                content: "A protocol that lets nodes discover each other and share information automatically".to_string(),
                goal: None,
                key_term: Some("Zenoh".to_string()),
                bigger_picture: None,
            },
            ContentSection {
                section_type: SectionType::Conclusion,
                content: "Now you have a self-organizing network".to_string(),
                goal: None,
                key_term: None,
                bigger_picture: Some("distributed AI collaboration platform".to_string()),
            },
        ],
    };
    
    let narrative = generator.transform_documentation(&content);
    println!("{}", narrative);
}

fn demo_onboarding_flow() {
    println!("🚀 Demo 3: Onboarding Flow Generation");
    println!("-------------------------------------");
    
    let generator = WurtzNarrativeGenerator::new();
    
    let steps = vec![
        OnboardingStep {
            action: "Install WeaveMesh-Core".to_string(),
            success_message: "The library is ready to use".to_string(),
        },
        OnboardingStep {
            action: "Create your first node".to_string(),
            success_message: "Your node is alive and ready to connect".to_string(),
        },
        OnboardingStep {
            action: "Start the mesh discovery".to_string(),
            success_message: "Your node is now part of the mesh".to_string(),
        },
        OnboardingStep {
            action: "Send your first message".to_string(),
            success_message: "Communication is flowing through the mesh".to_string(),
        },
    ];
    
    let onboarding = generator.generate_onboarding(&steps);
    println!("{}", onboarding);
}

fn demo_error_messages() {
    println!("⚠️  Demo 4: Error Message Generation");
    println!("-----------------------------------");
    
    let generator = WurtzNarrativeGenerator::new();
    
    let error = ErrorContext {
        component: "Zenoh session".to_string(),
        what_happened: "failed to connect".to_string(),
        explanation: "the network configuration is pointing to a non-existent router".to_string(),
        solution: "checking your Zenoh router address in the config".to_string(),
        alternative: Some("using the default local configuration".to_string()),
    };
    
    let error_message = generator.generate_error_message(&error);
    println!("{}", error_message);
}

fn demo_collaborative_individuation() {
    println!("🤝 Demo 5: Collaborative Individuation Embodiment");
    println!("------------------------------------------------");
    
    let narrative = CollaborativeIndividuationNarrative::generate_embodiment_narrative("ai_collaboration");
    println!("{}", narrative);
    
    println!("🎯 Key Principles Demonstrated:");
    println!("• Individual Voice: Bill Wurtz's unique style maintained");
    println!("• Collective Service: Complex concepts made accessible");
    println!("• Emergent Wisdom: Understanding through personality");
    println!("• Sacred Alliance: Creativity serving collective understanding");
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use weavemesh_core::narrative::WurtzNarrativeGenerator;

    #[test]
    fn test_narrative_generation() {
        let generator = WurtzNarrativeGenerator::new();
        
        // Test context establishment
        let context = generator.establish_context("distributed_systems");
        assert!(context.contains("Hi, you're"));
        assert!(context.contains("distributed software"));
        
        // Test complexity acknowledgment
        let complexity = generator.acknowledge_complexity("simple", "complex");
        assert!(complexity.contains("Some of it's simple"));
        assert!(complexity.contains("most of it's complex"));
        
        // Test positive reframing
        let positive = generator.positive_reframe("test achievement");
        assert!(positive.contains("Great news!"));
        assert!(positive.contains("test achievement"));
    }
    
    #[test]
    fn test_collaborative_individuation_narrative() {
        let narrative = CollaborativeIndividuationNarrative::generate_embodiment_narrative("test_domain");
        assert!(narrative.contains("Hi, you're"));
        assert!(narrative.contains("collaborative individuation"));
        assert!(narrative.contains("sacred alliance"));
    }
    
    #[test]
    fn test_onboarding_generation() {
        let generator = WurtzNarrativeGenerator::new();
        let steps = vec![
            OnboardingStep {
                action: "Test action".to_string(),
                success_message: "Test success".to_string(),
            }
        ];
        
        let onboarding = generator.generate_onboarding(&steps);
        assert!(onboarding.contains("Welcome!"));
        assert!(onboarding.contains("Test action"));
        assert!(onboarding.contains("Great news!"));
        assert!(onboarding.contains("Test success"));
    }
    
    #[test]
    fn test_error_message_generation() {
        let generator = WurtzNarrativeGenerator::new();
        let error = ErrorContext {
            component: "test component".to_string(),
            what_happened: "broke".to_string(),
            explanation: "test explanation".to_string(),
            solution: "test solution".to_string(),
            alternative: Some("test alternative".to_string()),
        };
        
        let message = generator.generate_error_message(&error);
        assert!(message.contains("Whoops!"));
        assert!(message.contains("test component"));
        assert!(message.contains("broke"));
        assert!(message.contains("test solution"));
        assert!(message.contains("test alternative"));
    }
}
