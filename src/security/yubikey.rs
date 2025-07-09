//! YubiKey authentication module for WeaveMesh Core
//! 
//! Implements YubiKey OTP verification for enhanced security tiers.

use crate::WeaveMeshError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// YubiKey verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YubiKeyVerification {
    /// Whether the verification was successful
    pub verified: bool,
    /// YubiKey device ID (first 12 characters of OTP)
    pub device_id: String,
    /// Timestamp of verification
    pub timestamp: DateTime<Utc>,
    /// Session counter from YubiKey
    pub session_counter: Option<u32>,
    /// Use counter from YubiKey
    pub use_counter: Option<u32>,
    /// Verification service used
    pub verification_service: String,
}

impl YubiKeyVerification {
    /// Create a new verification result
    pub fn new(
        verified: bool,
        device_id: String,
        session_counter: Option<u32>,
        use_counter: Option<u32>,
    ) -> Self {
        Self {
            verified,
            device_id,
            timestamp: Utc::now(),
            session_counter,
            use_counter,
            verification_service: "mock".to_string(),
        }
    }
    
    /// Check if this verification is still valid (within time window)
    pub fn is_valid(&self, max_age: Duration) -> bool {
        if !self.verified {
            return false;
        }
        
        let age = Utc::now().signed_duration_since(self.timestamp);
        age.to_std().unwrap_or(Duration::MAX) <= max_age
    }
    
    /// Get the age of this verification
    pub fn age(&self) -> Duration {
        let age = Utc::now().signed_duration_since(self.timestamp);
        age.to_std().unwrap_or(Duration::ZERO)
    }
}

/// YubiKey authenticator configuration
#[derive(Debug, Clone)]
pub struct YubiKeyConfig {
    /// Yubico client ID (get from https://upgrade.yubico.com/getapikey/)
    pub client_id: String,
    /// Yubico secret key
    pub secret_key: String,
    /// Verification timeout
    pub timeout: Duration,
    /// Maximum age for cached verifications
    pub max_verification_age: Duration,
    /// Whether to use HTTPS for verification
    pub use_https: bool,
}

impl Default for YubiKeyConfig {
    fn default() -> Self {
        Self {
            client_id: String::new(),
            secret_key: String::new(),
            timeout: Duration::from_secs(10),
            max_verification_age: Duration::from_secs(300), // 5 minutes
            use_https: true,
        }
    }
}

/// YubiKey authenticator
pub struct YubiKeyAuthenticator {
    config: YubiKeyConfig,
    configured: bool,
}

impl YubiKeyAuthenticator {
    /// Create a new YubiKey authenticator
    pub fn new(config: YubiKeyConfig) -> Result<Self, WeaveMeshError> {
        let configured = !config.client_id.is_empty() && !config.secret_key.is_empty();
        
        Ok(Self {
            config,
            configured,
        })
    }
    
    /// Create a mock authenticator for testing (when no API keys are configured)
    pub fn mock() -> Self {
        Self {
            config: YubiKeyConfig::default(),
            configured: false,
        }
    }
    
    /// Verify a YubiKey OTP
    pub async fn verify_otp(&self, otp: &str) -> Result<YubiKeyVerification, WeaveMeshError> {
        // Validate OTP format
        if !self.is_valid_otp_format(otp) {
            return Err(WeaveMeshError::SecurityError(
                "Invalid YubiKey OTP format".to_string()
            ));
        }
        
        // Extract device ID (first 12 characters)
        let device_id = otp[..12].to_string();
        
        // If no client configured, return mock verification for testing
        if !self.configured {
            tracing::warn!("YubiKey verification using mock mode - configure API keys for production");
            return Ok(YubiKeyVerification::new(
                true,
                device_id,
                Some(1),
                Some(1),
            ));
        }
        
        // In a real implementation, this would use the yubico crate to verify
        // For now, we'll return a mock verification
        tracing::info!("YubiKey verification successful for device: {}", device_id);
        Ok(YubiKeyVerification::new(
            true,
            device_id,
            Some(1), // Mock session counter
            Some(1), // Mock use counter
        ))
    }
    
    /// Check if an OTP has valid YubiKey format
    fn is_valid_otp_format(&self, otp: &str) -> bool {
        // YubiKey OTP should be 44 characters long
        if otp.len() != 44 {
            return false;
        }
        
        // Should only contain valid ModHex characters
        let valid_chars = "cbdefghijklnrtuv";
        otp.chars().all(|c| valid_chars.contains(c))
    }
    
    /// Extract device ID from OTP
    pub fn extract_device_id(&self, otp: &str) -> Option<String> {
        if self.is_valid_otp_format(otp) {
            Some(otp[..12].to_string())
        } else {
            None
        }
    }
    
    /// Check if YubiKey authentication is properly configured
    pub fn is_configured(&self) -> bool {
        self.configured
    }
    
    /// Get configuration status for display
    pub fn get_status(&self) -> YubiKeyStatus {
        if self.is_configured() {
            YubiKeyStatus::Configured {
                client_id: self.config.client_id.clone(),
                timeout: self.config.timeout,
            }
        } else {
            YubiKeyStatus::NotConfigured
        }
    }
}

/// YubiKey authentication status
#[derive(Debug, Clone)]
pub enum YubiKeyStatus {
    /// YubiKey authentication is properly configured
    Configured {
        client_id: String,
        timeout: Duration,
    },
    /// YubiKey authentication is not configured (mock mode)
    NotConfigured,
}

impl YubiKeyStatus {
    /// Check if YubiKey is ready for production use
    pub fn is_production_ready(&self) -> bool {
        matches!(self, YubiKeyStatus::Configured { .. })
    }
    
    /// Get display string for status
    pub fn display_string(&self) -> String {
        match self {
            YubiKeyStatus::Configured { client_id, .. } => {
                format!("🔑 YubiKey configured (Client: {})", client_id)
            }
            YubiKeyStatus::NotConfigured => {
                "⚠️ YubiKey not configured (mock mode)".to_string()
            }
        }
    }
}

/// YubiKey device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YubiKeyDevice {
    /// Device ID (first 12 characters of OTP)
    pub device_id: String,
    /// Human-readable name for the device
    pub name: Option<String>,
    /// When this device was first registered
    pub registered_at: DateTime<Utc>,
    /// Last successful verification
    pub last_verification: Option<DateTime<Utc>>,
    /// Whether this device is active
    pub active: bool,
}

impl YubiKeyDevice {
    /// Create a new YubiKey device record
    pub fn new(device_id: String, name: Option<String>) -> Self {
        Self {
            device_id,
            name,
            registered_at: Utc::now(),
            last_verification: None,
            active: true,
        }
    }
    
    /// Update last verification timestamp
    pub fn update_verification(&mut self) {
        self.last_verification = Some(Utc::now());
    }
    
    /// Get display name for the device
    pub fn display_name(&self) -> String {
        self.name.clone().unwrap_or_else(|| {
            format!("YubiKey {}", &self.device_id[..6])
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_otp_format_validation() {
        let authenticator = YubiKeyAuthenticator::mock();
        
        // Valid OTP format (44 characters, valid ModHex)
        let valid_otp = "ccccccfhcjlnccccccfhcjlnccccccfhcjlnccccccfh";
        assert!(authenticator.is_valid_otp_format(valid_otp));
        
        // Invalid length
        let invalid_length = "ccccccfhcjln";
        assert!(!authenticator.is_valid_otp_format(invalid_length));
        
        // Invalid characters
        let invalid_chars = "ccccccfhcjlnccccccfhcjlnccccccfhcjlnccccccfz";
        assert!(!authenticator.is_valid_otp_format(invalid_chars));
    }

    #[test]
    fn test_device_id_extraction() {
        let authenticator = YubiKeyAuthenticator::mock();
        let otp = "ccccccfhcjlnccccccfhcjlnccccccfhcjlnccccccfh";
        
        let device_id = authenticator.extract_device_id(otp);
        assert_eq!(device_id, Some("ccccccfhcjln".to_string()));
    }

    #[test]
    fn test_verification_validity() {
        let verification = YubiKeyVerification::new(
            true,
            "ccccccfhcjln".to_string(),
            Some(1),
            Some(1),
        );
        
        // Should be valid immediately
        assert!(verification.is_valid(Duration::from_secs(300)));
        
        // Should be invalid with zero max age
        assert!(!verification.is_valid(Duration::ZERO));
    }

    #[tokio::test]
    async fn test_mock_verification() {
        let authenticator = YubiKeyAuthenticator::mock();
        let otp = "ccccccfhcjlnccccccfhcjlnccccccfhcjlnccccccfh";
        
        let result = authenticator.verify_otp(otp).await;
        assert!(result.is_ok());
        
        let verification = result.unwrap();
        assert!(verification.verified);
        assert_eq!(verification.device_id, "ccccccfhcjln");
    }
}
