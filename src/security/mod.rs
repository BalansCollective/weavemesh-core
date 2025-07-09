//! Security module for WeaveMesh Core
//! 
//! Implements the tiered security model from the Weaver Security Model,
//! including authentication, authorization, and content filtering.

pub mod authentication;
pub mod authorization;
pub mod yubikey;
pub mod core;

pub use authentication::*;
pub use authorization::*;
pub use yubikey::*;
pub use core::*;

use crate::WeaveMeshError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Security levels in the WeaveMesh system
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub enum SecurityLevel {
    /// Public information, no authentication required
    Open,
    /// Company internal information, basic authentication required
    Internal,
    /// Client-specific information, enhanced authentication required
    Client,
    /// Compliance-regulated information, YubiKey required
    Compliance,
    /// Classified information, military-grade authentication required
    Classified,
}

impl SecurityLevel {
    /// Check if this security level allows access to another level
    pub fn can_access(&self, other: &SecurityLevel) -> bool {
        self >= other
    }
    
    /// Get the minimum authentication tier type required for this security level
    pub fn required_auth_tier_type(&self) -> &'static str {
        match self {
            SecurityLevel::Open => "None",
            SecurityLevel::Internal => "BasicAuth",
            SecurityLevel::Client => "EnhancedAuth",
            SecurityLevel::Compliance => "EnhancedAuth",
            SecurityLevel::Classified => "MilitaryAuth",
        }
    }
}

/// Authentication tiers following the Weaver Security Model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthenticationTier {
    /// No authentication (open source users)
    None,
    
    /// Basic OAuth authentication (company employees)
    BasicAuth {
        oauth_token: String,
        user_email: String,
        expires_at: chrono::DateTime<chrono::Utc>,
    },
    
    /// Enhanced authentication with YubiKey (secure organizations)
    EnhancedAuth {
        oauth_token: String,
        user_email: String,
        yubikey_verification: YubiKeyVerification,
        expires_at: chrono::DateTime<chrono::Utc>,
    },
    
    /// Military-grade authentication (defense/classified)
    MilitaryAuth {
        oauth_token: String,
        user_email: String,
        yubikey_verification: YubiKeyVerification,
        additional_factors: Vec<String>,
        expires_at: chrono::DateTime<chrono::Utc>,
    },
}

impl AuthenticationTier {
    /// Get the maximum security level this authentication tier can access
    pub fn max_security_level(&self) -> SecurityLevel {
        match self {
            AuthenticationTier::None => SecurityLevel::Open,
            AuthenticationTier::BasicAuth { .. } => SecurityLevel::Internal,
            AuthenticationTier::EnhancedAuth { .. } => SecurityLevel::Compliance,
            AuthenticationTier::MilitaryAuth { .. } => SecurityLevel::Classified,
        }
    }
    
    /// Check if this authentication tier can access a security level
    pub fn can_access_level(&self, level: &SecurityLevel) -> bool {
        self.max_security_level().can_access(level)
    }
    
    /// Check if the authentication is still valid (not expired)
    pub fn is_valid(&self) -> bool {
        match self {
            AuthenticationTier::None => true,
            AuthenticationTier::BasicAuth { expires_at, .. } |
            AuthenticationTier::EnhancedAuth { expires_at, .. } |
            AuthenticationTier::MilitaryAuth { expires_at, .. } => {
                chrono::Utc::now() < *expires_at
            }
        }
    }
    
    /// Get the user email if available
    pub fn user_email(&self) -> Option<&str> {
        match self {
            AuthenticationTier::None => None,
            AuthenticationTier::BasicAuth { user_email, .. } |
            AuthenticationTier::EnhancedAuth { user_email, .. } |
            AuthenticationTier::MilitaryAuth { user_email, .. } => Some(user_email),
        }
    }
    
    /// Check if YubiKey is present in this authentication
    pub fn has_yubikey(&self) -> bool {
        matches!(self, 
            AuthenticationTier::EnhancedAuth { .. } | 
            AuthenticationTier::MilitaryAuth { .. }
        )
    }
}

/// Environment types from the Weaver Security Model
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Environment {
    /// Open source environment - public access
    Open,
    /// Internal company environment
    Internal { organization_id: String },
    /// Client-specific environment
    Client { 
        organization_id: String,
        client_id: String,
    },
    /// Medical compliance environment (HIPAA)
    Medical { 
        organization_id: String,
        compliance_standards: Vec<ComplianceStandard>,
    },
    /// GDPR compliance environment
    GDPR { 
        organization_id: String,
        data_processing_basis: String,
    },
    /// Defense/classified environment
    Defense { 
        organization_id: String,
        classification_level: String,
        clearance_required: String,
    },
}

impl Environment {
    /// Get the security level required for this environment
    pub fn required_security_level(&self) -> SecurityLevel {
        match self {
            Environment::Open => SecurityLevel::Open,
            Environment::Internal { .. } => SecurityLevel::Internal,
            Environment::Client { .. } => SecurityLevel::Client,
            Environment::Medical { .. } | Environment::GDPR { .. } => SecurityLevel::Compliance,
            Environment::Defense { .. } => SecurityLevel::Classified,
        }
    }
    
    /// Check if a user can access this environment
    pub fn can_access(&self, auth: &AuthenticationTier, user_org: Option<&str>) -> bool {
        // Check authentication level
        if !auth.can_access_level(&self.required_security_level()) {
            return false;
        }
        
        // Check organization membership
        match self {
            Environment::Open => true,
            Environment::Internal { organization_id } |
            Environment::Client { organization_id, .. } |
            Environment::Medical { organization_id, .. } |
            Environment::GDPR { organization_id, .. } |
            Environment::Defense { organization_id, .. } => {
                user_org == Some(organization_id)
            }
        }
    }
}

/// Compliance standards
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ComplianceStandard {
    GDPR,
    HIPAA,
    SOX,
    ITAR,
    Custom(String),
}

impl std::fmt::Display for ComplianceStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComplianceStandard::GDPR => write!(f, "GDPR"),
            ComplianceStandard::HIPAA => write!(f, "HIPAA"),
            ComplianceStandard::SOX => write!(f, "SOX"),
            ComplianceStandard::ITAR => write!(f, "ITAR"),
            ComplianceStandard::Custom(name) => write!(f, "{}", name),
        }
    }
}

/// LLM processing tiers from the Weaver Security Model
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LLMTier {
    /// External LLMs (Claude, GPT-4) - for public content only
    External,
    /// On-premises LLMs - for internal content
    OnPremises,
    /// Air-gapped LLMs - for sensitive content
    AirGapped,
    /// Manual review only - for classified content
    ManualReview,
}

impl LLMTier {
    /// Get allowed LLM tiers for a security level
    pub fn allowed_for_security_level(level: &SecurityLevel) -> Vec<LLMTier> {
        match level {
            SecurityLevel::Open => vec![
                LLMTier::External,
                LLMTier::OnPremises,
                LLMTier::AirGapped,
                LLMTier::ManualReview,
            ],
            SecurityLevel::Internal => vec![
                LLMTier::OnPremises,
                LLMTier::AirGapped,
                LLMTier::ManualReview,
            ],
            SecurityLevel::Client => vec![
                LLMTier::AirGapped,
                LLMTier::ManualReview,
            ],
            SecurityLevel::Compliance => vec![
                LLMTier::ManualReview,
            ],
            SecurityLevel::Classified => vec![
                LLMTier::ManualReview,
            ],
        }
    }
    
    /// Get the recommended LLM tier for a security level
    pub fn recommended_for_security_level(level: &SecurityLevel) -> LLMTier {
        match level {
            SecurityLevel::Open => LLMTier::External,
            SecurityLevel::Internal => LLMTier::OnPremises,
            SecurityLevel::Client => LLMTier::AirGapped,
            SecurityLevel::Compliance | SecurityLevel::Classified => LLMTier::ManualReview,
        }
    }
}

/// Security context for operations
#[derive(Debug, Clone)]
pub struct SecurityContext {
    /// Current authentication
    pub authentication: AuthenticationTier,
    /// Current environment
    pub environment: Environment,
    /// User's organization
    pub organization_id: Option<String>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl SecurityContext {
    /// Create a new security context
    pub fn new(
        authentication: AuthenticationTier,
        environment: Environment,
        organization_id: Option<String>,
    ) -> Self {
        Self {
            authentication,
            environment,
            organization_id,
            metadata: HashMap::new(),
        }
    }
    
    /// Check if this context can access a security level
    pub fn can_access_level(&self, level: &SecurityLevel) -> bool {
        self.authentication.can_access_level(level) &&
        self.environment.can_access(&self.authentication, self.organization_id.as_deref())
    }
    
    /// Get allowed LLM tiers for this context
    pub fn allowed_llm_tiers(&self) -> Vec<LLMTier> {
        let max_level = self.authentication.max_security_level();
        LLMTier::allowed_for_security_level(&max_level)
    }
    
    /// Get recommended LLM tier for this context
    pub fn recommended_llm_tier(&self) -> LLMTier {
        let max_level = self.authentication.max_security_level();
        LLMTier::recommended_for_security_level(&max_level)
    }
    
    /// Validate that this security context is properly configured
    pub fn validate(&self) -> Result<(), WeaveMeshError> {
        // Check authentication is valid
        if !self.authentication.is_valid() {
            return Err(WeaveMeshError::SecurityError("Authentication expired".to_string()));
        }
        
        // Check environment access
        if !self.environment.can_access(&self.authentication, self.organization_id.as_deref()) {
            return Err(WeaveMeshError::SecurityError("Insufficient permissions for environment".to_string()));
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    #[test]
    fn test_security_level_hierarchy() {
        assert!(SecurityLevel::Classified.can_access(&SecurityLevel::Open));
        assert!(SecurityLevel::Internal.can_access(&SecurityLevel::Open));
        assert!(!SecurityLevel::Open.can_access(&SecurityLevel::Internal));
    }

    #[test]
    fn test_authentication_tier_access() {
        let basic_auth = AuthenticationTier::BasicAuth {
            oauth_token: "token".to_string(),
            user_email: "user@company.com".to_string(),
            expires_at: Utc::now() + Duration::hours(1),
        };
        
        assert!(basic_auth.can_access_level(&SecurityLevel::Open));
        assert!(basic_auth.can_access_level(&SecurityLevel::Internal));
        assert!(!basic_auth.can_access_level(&SecurityLevel::Client));
    }

    #[test]
    fn test_llm_tier_restrictions() {
        let open_tiers = LLMTier::allowed_for_security_level(&SecurityLevel::Open);
        assert!(open_tiers.contains(&LLMTier::External));
        
        let internal_tiers = LLMTier::allowed_for_security_level(&SecurityLevel::Internal);
        assert!(!internal_tiers.contains(&LLMTier::External));
        assert!(internal_tiers.contains(&LLMTier::OnPremises));
    }
}
