//! Core security primitives for WeaveMesh
//! 
//! Implements universal security concepts that can be adapted to different contexts
//! through the plugin architecture.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Universal security levels for any context
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub enum SecurityLevel {
    /// Public information, no authentication required
    Open,
    /// Basic protected information, simple authentication required
    Protected,
    /// Sensitive information, enhanced authentication required
    Sensitive,
    /// Restricted information, strong authentication required
    Restricted,
    /// Classified information, maximum authentication required
    Classified,
}

impl SecurityLevel {
    /// Check if this security level allows access to another level
    pub fn can_access(&self, other: &SecurityLevel) -> bool {
        self >= other
    }
    
    /// Get a numeric representation for comparison
    pub fn level_value(&self) -> u8 {
        match self {
            SecurityLevel::Open => 0,
            SecurityLevel::Protected => 1,
            SecurityLevel::Sensitive => 2,
            SecurityLevel::Restricted => 3,
            SecurityLevel::Classified => 4,
        }
    }
}

/// Universal authentication strength levels
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub enum AuthenticationStrength {
    /// No authentication required
    None,
    /// Basic authentication (password, token)
    Basic,
    /// Multi-factor authentication
    MultiFactor,
    /// Hardware-backed authentication
    Hardware,
    /// Maximum security authentication
    Maximum,
}

impl AuthenticationStrength {
    /// Get the maximum security level this authentication strength can access
    pub fn max_security_level(&self) -> SecurityLevel {
        match self {
            AuthenticationStrength::None => SecurityLevel::Open,
            AuthenticationStrength::Basic => SecurityLevel::Protected,
            AuthenticationStrength::MultiFactor => SecurityLevel::Sensitive,
            AuthenticationStrength::Hardware => SecurityLevel::Restricted,
            AuthenticationStrength::Maximum => SecurityLevel::Classified,
        }
    }
    
    /// Check if this authentication strength can access a security level
    pub fn can_access_level(&self, level: &SecurityLevel) -> bool {
        self.max_security_level().can_access(level)
    }
}

/// Universal authentication token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationToken {
    /// Unique identifier for this token
    pub token_id: String,
    /// Authentication strength of this token
    pub strength: AuthenticationStrength,
    /// User identifier
    pub user_id: String,
    /// Token expiration time (Unix timestamp)
    pub expires_at: i64,
    /// Additional claims/metadata
    pub claims: HashMap<String, String>,
}

impl AuthenticationToken {
    /// Create a new authentication token
    pub fn new(
        token_id: String,
        strength: AuthenticationStrength,
        user_id: String,
        expires_at: i64,
    ) -> Self {
        Self {
            token_id,
            strength,
            user_id,
            expires_at,
            claims: HashMap::new(),
        }
    }
    
    /// Check if the token is still valid (not expired)
    pub fn is_valid(&self) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;
        now < self.expires_at
    }
    
    /// Check if this token can access a security level
    pub fn can_access_level(&self, level: &SecurityLevel) -> bool {
        self.is_valid() && self.strength.can_access_level(level)
    }
    
    /// Add a claim to the token
    pub fn add_claim(&mut self, key: String, value: String) {
        self.claims.insert(key, value);
    }
    
    /// Get a claim from the token
    pub fn get_claim(&self, key: &str) -> Option<&String> {
        self.claims.get(key)
    }
}

/// Universal security context for operations
#[derive(Debug, Clone)]
pub struct SecurityContext {
    /// Current authentication token
    pub token: Option<AuthenticationToken>,
    /// Context-specific metadata
    pub metadata: HashMap<String, String>,
}

impl SecurityContext {
    /// Create a new security context
    pub fn new(token: Option<AuthenticationToken>) -> Self {
        Self {
            token,
            metadata: HashMap::new(),
        }
    }
    
    /// Create an unauthenticated security context
    pub fn unauthenticated() -> Self {
        Self::new(None)
    }
    
    /// Check if this context can access a security level
    pub fn can_access_level(&self, level: &SecurityLevel) -> bool {
        match &self.token {
            Some(token) => token.can_access_level(level),
            None => level == &SecurityLevel::Open,
        }
    }
    
    /// Get the maximum security level this context can access
    pub fn max_security_level(&self) -> SecurityLevel {
        match &self.token {
            Some(token) => token.strength.max_security_level(),
            None => SecurityLevel::Open,
        }
    }
    
    /// Add metadata to the context
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
    
    /// Get metadata from the context
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
    
    /// Validate that this security context is properly configured
    pub fn validate(&self) -> Result<(), String> {
        if let Some(token) = &self.token {
            if !token.is_valid() {
                return Err("Authentication token expired".to_string());
            }
        }
        Ok(())
    }
}

/// Universal access control decision
#[derive(Debug, Clone, PartialEq)]
pub enum AccessDecision {
    /// Access granted
    Allow,
    /// Access denied
    Deny,
    /// Access requires additional verification
    Challenge,
}

/// Universal access control interface
pub trait AccessController {
    /// Check if access should be granted for a security context and level
    fn check_access(&self, context: &SecurityContext, level: &SecurityLevel) -> AccessDecision;
    
    /// Validate a security context
    fn validate_context(&self, context: &SecurityContext) -> Result<(), String>;
}

/// Default access controller implementation
#[derive(Debug, Clone)]
pub struct DefaultAccessController;

impl AccessController for DefaultAccessController {
    fn check_access(&self, context: &SecurityContext, level: &SecurityLevel) -> AccessDecision {
        if context.can_access_level(level) {
            AccessDecision::Allow
        } else {
            AccessDecision::Deny
        }
    }
    
    fn validate_context(&self, context: &SecurityContext) -> Result<(), String> {
        context.validate()
    }
}

/// Security event types for auditing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityEvent {
    /// Authentication attempt
    AuthenticationAttempt {
        user_id: String,
        success: bool,
        timestamp: i64,
    },
    /// Access attempt
    AccessAttempt {
        user_id: Option<String>,
        resource: String,
        level: SecurityLevel,
        decision: String, // Allow/Deny/Challenge
        timestamp: i64,
    },
    /// Token creation
    TokenCreated {
        token_id: String,
        user_id: String,
        strength: AuthenticationStrength,
        timestamp: i64,
    },
    /// Token revocation
    TokenRevoked {
        token_id: String,
        reason: String,
        timestamp: i64,
    },
}

impl SecurityEvent {
    /// Get the timestamp of this event
    pub fn timestamp(&self) -> i64 {
        match self {
            SecurityEvent::AuthenticationAttempt { timestamp, .. } |
            SecurityEvent::AccessAttempt { timestamp, .. } |
            SecurityEvent::TokenCreated { timestamp, .. } |
            SecurityEvent::TokenRevoked { timestamp, .. } => *timestamp,
        }
    }
    
    /// Get the user ID associated with this event, if any
    pub fn user_id(&self) -> Option<&str> {
        match self {
            SecurityEvent::AuthenticationAttempt { user_id, .. } => Some(user_id),
            SecurityEvent::AccessAttempt { user_id, .. } => user_id.as_deref(),
            SecurityEvent::TokenCreated { user_id, .. } => Some(user_id),
            SecurityEvent::TokenRevoked { .. } => None,
        }
    }
}

/// Universal security audit trail
pub trait SecurityAuditor {
    /// Record a security event
    fn record_event(&mut self, event: SecurityEvent);
    
    /// Get events for a user
    fn get_user_events(&self, user_id: &str) -> Vec<&SecurityEvent>;
    
    /// Get events in a time range
    fn get_events_in_range(&self, start: i64, end: i64) -> Vec<&SecurityEvent>;
}

/// In-memory security auditor for testing/simple use cases
#[derive(Debug, Default)]
pub struct MemorySecurityAuditor {
    events: Vec<SecurityEvent>,
}

impl SecurityAuditor for MemorySecurityAuditor {
    fn record_event(&mut self, event: SecurityEvent) {
        self.events.push(event);
    }
    
    fn get_user_events(&self, user_id: &str) -> Vec<&SecurityEvent> {
        self.events
            .iter()
            .filter(|event| event.user_id() == Some(user_id))
            .collect()
    }
    
    fn get_events_in_range(&self, start: i64, end: i64) -> Vec<&SecurityEvent> {
        self.events
            .iter()
            .filter(|event| {
                let ts = event.timestamp();
                ts >= start && ts <= end
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_level_hierarchy() {
        assert!(SecurityLevel::Classified.can_access(&SecurityLevel::Open));
        assert!(SecurityLevel::Protected.can_access(&SecurityLevel::Open));
        assert!(!SecurityLevel::Open.can_access(&SecurityLevel::Protected));
    }

    #[test]
    fn test_authentication_strength_access() {
        let basic = AuthenticationStrength::Basic;
        assert!(basic.can_access_level(&SecurityLevel::Open));
        assert!(basic.can_access_level(&SecurityLevel::Protected));
        assert!(!basic.can_access_level(&SecurityLevel::Sensitive));
    }

    #[test]
    fn test_authentication_token_validity() {
        let future_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64 + 3600; // 1 hour from now
        
        let token = AuthenticationToken::new(
            "test-token".to_string(),
            AuthenticationStrength::Basic,
            "user123".to_string(),
            future_time,
        );
        
        assert!(token.is_valid());
        assert!(token.can_access_level(&SecurityLevel::Open));
        assert!(token.can_access_level(&SecurityLevel::Protected));
        assert!(!token.can_access_level(&SecurityLevel::Sensitive));
    }

    #[test]
    fn test_security_context_access() {
        let token = AuthenticationToken::new(
            "test-token".to_string(),
            AuthenticationStrength::MultiFactor,
            "user123".to_string(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64 + 3600,
        );
        
        let context = SecurityContext::new(Some(token));
        
        assert!(context.can_access_level(&SecurityLevel::Open));
        assert!(context.can_access_level(&SecurityLevel::Protected));
        assert!(context.can_access_level(&SecurityLevel::Sensitive));
        assert!(!context.can_access_level(&SecurityLevel::Restricted));
    }

    #[test]
    fn test_default_access_controller() {
        let controller = DefaultAccessController;
        let context = SecurityContext::unauthenticated();
        
        assert_eq!(
            controller.check_access(&context, &SecurityLevel::Open),
            AccessDecision::Allow
        );
        assert_eq!(
            controller.check_access(&context, &SecurityLevel::Protected),
            AccessDecision::Deny
        );
    }

    #[test]
    fn test_security_auditor() {
        let mut auditor = MemorySecurityAuditor::default();
        
        let event = SecurityEvent::AuthenticationAttempt {
            user_id: "user123".to_string(),
            success: true,
            timestamp: 1234567890,
        };
        
        auditor.record_event(event);
        
        let user_events = auditor.get_user_events("user123");
        assert_eq!(user_events.len(), 1);
        
        let range_events = auditor.get_events_in_range(1234567880, 1234567900);
        assert_eq!(range_events.len(), 1);
    }
}
