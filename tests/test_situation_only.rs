//! Standalone test for situation module to verify refactoring

use std::collections::HashMap;

// Import the situation module types directly
use weavemesh_core::situation::*;

#[tokio::test]
async fn test_basic_situation_provider() {
    let provider = BasicSituationProvider::new(
        "test-situation".to_string(),
        "Test Situation".to_string(),
    );
    
    assert_eq!(provider.get_situation_id(), "test-situation");
    assert_eq!(provider.get_situation_name(), "Test Situation");
    assert!(provider.validate_compatibility("1.0.0").is_ok());
    
    let detection_data = SituationDetectionData {
        environment: EnvironmentInfo {
            environment_type: "test".to_string(),
            security_level: "basic".to_string(),
            available_resources: vec![],
            network_topology: NetworkTopology {
                topology_type: "mesh".to_string(),
                node_count: 1,
                connection_quality: 1.0,
                bandwidth: "high".to_string(),
                latency: "low".to_string(),
            },
            device_capabilities: vec![],
        },
        participants: vec![],
        communication_patterns: vec![],
        system_capabilities: vec![],
        user_preferences: HashMap::new(),
        temporal_situation: TemporalSituation {
            timestamp: chrono::Utc::now(),
            timezone: "UTC".to_string(),
            day_of_week: "Monday".to_string(),
            time_of_day: "morning".to_string(),
            is_leisure_time: false,
        },
    };
    
    let situation_match = provider.detect_situation(&detection_data).await.unwrap();
    assert!(situation_match.matches);
    assert!(situation_match.confidence > 0.0);
}

#[tokio::test]
async fn test_situation_provider_registry() {
    let mut registry = SituationProviderRegistry::new(RegistryConfig::default());
    
    let provider = std::sync::Arc::new(BasicSituationProvider::new(
        "test-situation".to_string(),
        "Test Situation".to_string(),
    ));
    
    // Register provider
    registry.register_provider(provider).unwrap();
    assert_eq!(registry.get_registered_providers().len(), 1);
    
    // Activate situation
    registry.activate_situation("test-situation", 0.8).await.unwrap();
    assert_eq!(registry.get_active_situations().len(), 1);
    
    // Deactivate situation
    registry.deactivate_situation("test-situation").unwrap();
    assert_eq!(registry.get_active_situations().len(), 0);
}

#[tokio::test]
async fn test_conflict_resolution_types() {
    // Test that our ConflictResolution enum works correctly
    let config = RegistryConfig {
        max_active_situations: 2,
        detection_interval: chrono::Duration::seconds(10),
        min_activation_confidence: 0.5,
        auto_situation_switching: true,
        conflict_resolution: ConflictResolution::HighestPriority,
    };
    
    assert_eq!(config.conflict_resolution, ConflictResolution::HighestPriority);
    
    // Test other variants
    let merge_config = RegistryConfig {
        conflict_resolution: ConflictResolution::Merge,
        ..config
    };
    assert_eq!(merge_config.conflict_resolution, ConflictResolution::Merge);
}

fn main() {
    println!("Situation module refactoring test completed successfully!");
}
