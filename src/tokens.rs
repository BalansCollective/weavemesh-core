//! Token Policy Interface for WeaveMesh Core
//!
//! This module provides basic interfaces for external token systems to consume
//! attribution data. It maintains strict separation between objective
//! measurement (attribution) and subjective value assignment (tokens).

use crate::Attribution;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Token amount type - using f64 for precision in calculations
pub type TokenAmount = f64;

/// Unique identifier for token policies
pub type PolicyId = Uuid;

/// Unique identifier for contributors in token systems
pub type ContributorId = String;

/// Token allocation result from a policy calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenAllocation {
    /// Token amounts allocated to each contributor
    pub allocations: HashMap<ContributorId, TokenAmount>,
    /// Reasoning for each allocation decision
    pub reasoning: Vec<AllocationReason>,
    /// Metadata about the allocation process
    pub metadata: TokenMetadata,
    /// Timestamp when allocation was calculated
    pub calculated_at: DateTime<Utc>,
    /// Policy that generated this allocation
    pub policy_id: PolicyId,
}

/// Reasoning for a specific token allocation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationReason {
    /// Contributor this reasoning applies to
    pub contributor: ContributorId,
    /// Human-readable explanation
    pub explanation: String,
    /// Factors that contributed to the allocation
    pub factors: HashMap<String, f64>,
    /// Confidence in this allocation (0.0 to 1.0)
    pub confidence: f64,
}

/// Metadata about token allocation process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenMetadata {
    /// Total tokens allocated in this batch
    pub total_allocated: TokenAmount,
    /// Number of attribution events processed
    pub events_processed: usize,
    /// Time period covered by this allocation
    pub time_period: Option<(DateTime<Utc>, DateTime<Utc>)>,
    /// Policy version used for calculation
    pub policy_version: String,
    /// Any warnings or notes about the allocation
    pub warnings: Vec<String>,
}

/// Core trait for token policy implementations
pub trait TokenPolicy: Send + Sync {
    /// Calculate token allocations based on attribution events
    fn calculate_tokens(&self, events: &[Attribution]) -> Result<TokenAllocation>;
    
    /// Get the policy name for identification
    fn get_policy_name(&self) -> &str;
    
    /// Get the policy version for tracking changes
    fn get_policy_version(&self) -> &str;
    
    /// Get policy description for human understanding
    fn get_policy_description(&self) -> &str;
    
    /// Get the maximum token dependency this policy allows (0.0 to 1.0)
    /// This is a safeguard to prevent tokens from becoming primary reality
    fn get_max_token_dependency(&self) -> f64 {
        0.2 // Default 20% maximum dependency
    }
    
    /// Check if this policy requires business value correlation
    fn requires_business_value_correlation(&self) -> bool {
        true // Default to requiring correlation with practical outcomes
    }
}

/// Simple token policy implementation for testing
#[derive(Debug)]
pub struct SimpleTokenPolicy {
    name: String,
    version: String,
    description: String,
    tokens_per_contribution: TokenAmount,
}

impl SimpleTokenPolicy {
    pub fn new(
        name: String,
        version: String,
        description: String,
        tokens_per_contribution: TokenAmount,
    ) -> Self {
        Self {
            name,
            version,
            description,
            tokens_per_contribution,
        }
    }
}

impl TokenPolicy for SimpleTokenPolicy {
    fn calculate_tokens(&self, events: &[Attribution]) -> Result<TokenAllocation> {
        let mut allocations = HashMap::new();
        let mut reasoning = Vec::new();
        
        for event in events {
            // Allocate tokens based on collaboration type and confidence
            let base_tokens = self.tokens_per_contribution;
            let confidence_multiplier = event.confidence as f64;
            let tokens = base_tokens * confidence_multiplier;
            
            // Allocate to human contributor if present
            if let Some(ref human_id) = event.human_contributor {
                *allocations.entry(human_id.clone()).or_insert(0.0) += tokens;
                
                reasoning.push(AllocationReason {
                    contributor: human_id.clone(),
                    explanation: format!(
                        "Allocated {} tokens for {} collaboration with {:.1}% confidence",
                        tokens,
                        format!("{:?}", event.collaboration_type),
                        confidence_multiplier * 100.0
                    ),
                    factors: {
                        let mut factors = HashMap::new();
                        factors.insert("base_tokens".to_string(), base_tokens);
                        factors.insert("confidence".to_string(), confidence_multiplier);
                        factors
                    },
                    confidence: confidence_multiplier,
                });
            }
            
            // Allocate to AI contributor if present
            if let Some(ref ai_id) = event.ai_contributor {
                *allocations.entry(ai_id.clone()).or_insert(0.0) += tokens * 0.5; // AI gets 50%
                
                reasoning.push(AllocationReason {
                    contributor: ai_id.clone(),
                    explanation: format!(
                        "Allocated {} tokens (50% of human allocation) for AI contribution",
                        tokens * 0.5
                    ),
                    factors: {
                        let mut factors = HashMap::new();
                        factors.insert("base_tokens".to_string(), tokens);
                        factors.insert("ai_multiplier".to_string(), 0.5);
                        factors
                    },
                    confidence: confidence_multiplier,
                });
            }
        }
        
        let total_allocated: TokenAmount = allocations.values().sum();
        
        Ok(TokenAllocation {
            allocations,
            reasoning,
            metadata: TokenMetadata {
                total_allocated,
                events_processed: events.len(),
                time_period: None,
                policy_version: self.version.clone(),
                warnings: Vec::new(),
            },
            calculated_at: Utc::now(),
            policy_id: Uuid::new_v4(),
        })
    }
    
    fn get_policy_name(&self) -> &str {
        &self.name
    }
    
    fn get_policy_version(&self) -> &str {
        &self.version
    }
    
    fn get_policy_description(&self) -> &str {
        &self.description
    }
}

/// Token system error types
#[derive(thiserror::Error, Debug)]
pub enum TokenError {
    #[error("Token policy validation failed: {0}")]
    PolicyValidationFailed(String),
    
    #[error("Token calculation failed: {0}")]
    CalculationFailed(String),
    
    #[error("Policy registration failed: {0}")]
    PolicyRegistrationFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Attribution, CollaborationType};

    #[test]
    fn test_token_allocation_creation() {
        let mut allocations = HashMap::new();
        allocations.insert("contributor1".to_string(), 100.0);
        allocations.insert("contributor2".to_string(), 50.0);
        
        let allocation = TokenAllocation {
            allocations,
            reasoning: vec![],
            metadata: TokenMetadata {
                total_allocated: 150.0,
                events_processed: 5,
                time_period: None,
                policy_version: "1.0".to_string(),
                warnings: vec![],
            },
            calculated_at: Utc::now(),
            policy_id: Uuid::new_v4(),
        };
        
        assert_eq!(allocation.metadata.total_allocated, 150.0);
        assert_eq!(allocation.allocations.len(), 2);
    }
    
    #[test]
    fn test_simple_token_policy() {
        let policy = SimpleTokenPolicy::new(
            "Test Policy".to_string(),
            "1.0".to_string(),
            "A simple test policy".to_string(),
            10.0,
        );
        
        let attribution = Attribution::new(
            Some("human1".to_string()),
            Some("ai1".to_string()),
            CollaborationType::CoCreated,
            0.8,
        );
        
        let allocation = policy.calculate_tokens(&[attribution]).unwrap();
        
        assert_eq!(allocation.metadata.events_processed, 1);
        assert!(allocation.allocations.contains_key("human1"));
        assert!(allocation.allocations.contains_key("ai1"));
        
        // Human should get 8.0 tokens (10.0 * 0.8 confidence)
        assert_eq!(allocation.allocations["human1"], 8.0);
        // AI should get 4.0 tokens (50% of human allocation)
        assert_eq!(allocation.allocations["ai1"], 4.0);
    }
}
