//! Authentication module for WeaveMesh Core
//! 
//! Implements OAuth2 authentication with optional YubiKey enhancement.

use crate::security::{AuthenticationTier, YubiKeyAuthenticator, YubiKeyVerification, YubiKeyConfig};
use crate::WeaveMeshError;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// OAuth2 configuration for authentication
#[derive(Debug, Clone)]
pub struct OAuthConfig {
    /// OAuth2 client ID
    pub client_id: String,
    /// OAuth2 client secret
    pub client_secret: String,
    /// Redirect URL for OAuth flow
    pub redirect_url: String,
    /// Additional scopes to request
    pub scopes: Vec<String>,
}

impl Default for OAuthConfig {
    fn default() -> Self {
        Self {
            client_id: String::new(),
            client_secret: String::new(),
            redirect_url: "http://localhost:8080/auth/callback".to_string(),
            scopes: vec![
                "openid".to_string(),
                "email".to_string(),
                "profile".to_string(),
            ],
        }
    }
}

/// OAuth2 token information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthToken {
    /// Access token
    pub access_token: String,
    /// Refresh token (if available)
    pub refresh_token: Option<String>,
    /// Token expiration time
    pub expires_at: DateTime<Utc>,
    /// Token scopes
    pub scopes: Vec<String>,
}

impl OAuthToken {
    /// Check if the token is still valid
    pub fn is_valid(&self) -> bool {
        Utc::now() < self.expires_at
    }
    
    /// Check if the token expires soon (within 5 minutes)
    pub fn expires_soon(&self) -> bool {
        let five_minutes = Duration::minutes(5);
        Utc::now() + five_minutes >= self.expires_at
    }
    
    /// Get time until expiration
    pub fn time_until_expiration(&self) -> Duration {
        self.expires_at.signed_duration_since(Utc::now())
    }
}

/// User information from OAuth provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// User's email address
    pub email: String,
    /// User's display name
    pub name: Option<String>,
    /// User's profile picture URL
    pub picture: Option<String>,
    /// Whether email is verified
    pub email_verified: bool,
    /// User's organization domain
    pub organization: Option<String>,
}

/// Authentication manager
pub struct AuthenticationManager {
    oauth_config: OAuthConfig,
    yubikey_config: YubiKeyConfig,
    yubikey_authenticator: YubiKeyAuthenticator,
}

impl AuthenticationManager {
    /// Create a new authentication manager
    pub fn new(oauth_config: OAuthConfig, yubikey_config: YubiKeyConfig) -> Result<Self, WeaveMeshError> {
        let yubikey_authenticator = YubiKeyAuthenticator::new(yubikey_config.clone())?;
        
        Ok(Self {
            oauth_config,
            yubikey_config,
            yubikey_authenticator,
        })
    }
    
    /// Create a mock authentication manager for testing
    pub fn mock() -> Self {
        Self {
            oauth_config: OAuthConfig::default(),
            yubikey_config: YubiKeyConfig::default(),
            yubikey_authenticator: YubiKeyAuthenticator::mock(),
        }
    }
    
    /// Create basic authentication tier from OAuth
    pub fn create_basic_auth(
        &self,
        oauth_token: OAuthToken,
        user_info: UserInfo,
    ) -> AuthenticationTier {
        AuthenticationTier::BasicAuth {
            oauth_token: oauth_token.access_token,
            user_email: user_info.email,
            expires_at: oauth_token.expires_at,
        }
    }
    
    /// Enhance authentication with YubiKey
    pub async fn enhance_with_yubikey(
        &self,
        basic_auth: AuthenticationTier,
        yubikey_otp: &str,
    ) -> Result<AuthenticationTier, WeaveMeshError> {
        // Verify YubiKey OTP
        let yubikey_verification = self.yubikey_authenticator.verify_otp(yubikey_otp).await?;
        
        match basic_auth {
            AuthenticationTier::BasicAuth { oauth_token, user_email, expires_at } => {
                Ok(AuthenticationTier::EnhancedAuth {
                    oauth_token,
                    user_email,
                    yubikey_verification,
                    expires_at,
                })
            }
            _ => Err(WeaveMeshError::SecurityError(
                "Can only enhance BasicAuth with YubiKey".to_string()
            )),
        }
    }
    
    /// Create mock authentication for testing
    pub fn create_mock_auth(&self, user_email: &str, with_yubikey: bool) -> Result<AuthenticationTier, WeaveMeshError> {
        let expires_at = Utc::now() + Duration::hours(1);
        
        if with_yubikey {
            let yubikey_verification = YubiKeyVerification::new(
                true,
                "ccccccfhcjln".to_string(),
                Some(1),
                Some(1),
            );
            
            Ok(AuthenticationTier::EnhancedAuth {
                oauth_token: "mock_token".to_string(),
                user_email: user_email.to_string(),
                yubikey_verification,
                expires_at,
            })
        } else {
            Ok(AuthenticationTier::BasicAuth {
                oauth_token: "mock_token".to_string(),
                user_email: user_email.to_string(),
                expires_at,
            })
        }
    }
    
    /// Check if OAuth is configured
    pub fn is_oauth_configured(&self) -> bool {
        !self.oauth_config.client_id.is_empty() && !self.oauth_config.client_secret.is_empty()
    }
    
    /// Check if YubiKey is configured
    pub fn is_yubikey_configured(&self) -> bool {
        self.yubikey_authenticator.is_configured()
    }
    
    /// Get authentication status
    pub fn get_auth_status(&self) -> AuthenticationStatus {
        AuthenticationStatus {
            oauth_configured: self.is_oauth_configured(),
            yubikey_configured: self.is_yubikey_configured(),
            yubikey_status: self.yubikey_authenticator.get_status(),
        }
    }
}

/// Authentication status for display
#[derive(Debug, Clone)]
pub struct AuthenticationStatus {
    /// Whether OAuth is configured
    pub oauth_configured: bool,
    /// Whether YubiKey is configured
    pub yubikey_configured: bool,
    /// YubiKey status details
    pub yubikey_status: crate::security::yubikey::YubiKeyStatus,
}

impl AuthenticationStatus {
    /// Get display string for OAuth status
    pub fn oauth_display_string(&self) -> String {
        if self.oauth_configured {
            "🔐 OAuth configured".to_string()
        } else {
            "⚠️ OAuth not configured (mock mode)".to_string()
        }
    }
    
    /// Get maximum available authentication tier
    pub fn max_available_tier(&self) -> String {
        match (self.oauth_configured, self.yubikey_configured) {
            (false, false) => "Open (No authentication)".to_string(),
            (true, false) => "Internal (OAuth only)".to_string(),
            (false, true) => "Internal (YubiKey without OAuth)".to_string(),
            (true, true) => "Compliance (OAuth + YubiKey)".to_string(),
        }
    }
}

/// Authentication flow state for interactive authentication
#[derive(Debug)]
pub struct AuthenticationFlow {
    /// Current step in the flow
    pub step: AuthFlowStep,
    /// OAuth CSRF token
    pub csrf_token: Option<String>,
    /// Partial authentication result
    pub partial_auth: Option<AuthenticationTier>,
    /// User information
    pub user_info: Option<UserInfo>,
}

/// Steps in the authentication flow
#[derive(Debug, Clone, PartialEq)]
pub enum AuthFlowStep {
    /// Starting authentication
    Start,
    /// Waiting for OAuth callback
    WaitingForOAuth,
    /// OAuth completed, optionally waiting for YubiKey
    OAuthComplete,
    /// Waiting for YubiKey OTP
    WaitingForYubiKey,
    /// Authentication completed
    Complete,
    /// Authentication failed
    Failed(String),
}

impl AuthenticationFlow {
    /// Create a new authentication flow
    pub fn new() -> Self {
        Self {
            step: AuthFlowStep::Start,
            csrf_token: None,
            partial_auth: None,
            user_info: None,
        }
    }
    
    /// Check if the flow is complete
    pub fn is_complete(&self) -> bool {
        matches!(self.step, AuthFlowStep::Complete)
    }
    
    /// Check if the flow has failed
    pub fn is_failed(&self) -> bool {
        matches!(self.step, AuthFlowStep::Failed(_))
    }
    
    /// Get the final authentication result
    pub fn get_result(&self) -> Option<&AuthenticationTier> {
        if self.is_complete() {
            self.partial_auth.as_ref()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oauth_token_validity() {
        let valid_token = OAuthToken {
            access_token: "token".to_string(),
            refresh_token: None,
            expires_at: Utc::now() + Duration::hours(1),
            scopes: vec!["email".to_string()],
        };
        
        assert!(valid_token.is_valid());
        assert!(!valid_token.expires_soon());
        
        let expired_token = OAuthToken {
            access_token: "token".to_string(),
            refresh_token: None,
            expires_at: Utc::now() - Duration::hours(1),
            scopes: vec!["email".to_string()],
        };
        
        assert!(!expired_token.is_valid());
    }

    #[test]
    fn test_mock_authentication() {
        let auth_manager = AuthenticationManager::mock();
        
        let basic_auth = auth_manager.create_mock_auth("test@example.com", false).unwrap();
        assert!(matches!(basic_auth, AuthenticationTier::BasicAuth { .. }));
        
        let enhanced_auth = auth_manager.create_mock_auth("test@example.com", true).unwrap();
        assert!(matches!(enhanced_auth, AuthenticationTier::EnhancedAuth { .. }));
    }

    #[test]
    fn test_authentication_flow() {
        let mut flow = AuthenticationFlow::new();
        assert_eq!(flow.step, AuthFlowStep::Start);
        assert!(!flow.is_complete());
        assert!(!flow.is_failed());
        
        flow.step = AuthFlowStep::Complete;
        assert!(flow.is_complete());
    }
}
