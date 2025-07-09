//! HTTP Server Interface for WeaveMesh Core
//!
//! This module provides HTTP/WebSocket gateway functionality for weaver nodes,
//! enabling web browsers, mobile apps, and other HTTP-based frontends to
//! access WeaveMesh collaborative individuation capabilities.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// HTTP server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpConfig {
    /// Port to bind the HTTP server to
    pub port: u16,
    /// Host to bind to (default: 0.0.0.0 for all interfaces)
    pub host: String,
    /// Enable CORS for web frontends
    pub enable_cors: bool,
    /// Maximum request size in bytes
    pub max_request_size: usize,
    /// Request timeout in seconds
    pub request_timeout: u64,
    /// WebSocket configuration
    pub websocket: WebSocketConfig,
    /// Rate limiting configuration
    pub rate_limiting: RateLimitConfig,
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            port: 8080,
            host: "0.0.0.0".to_string(),
            enable_cors: true,
            max_request_size: 16 * 1024 * 1024, // 16MB
            request_timeout: 30,
            websocket: WebSocketConfig::default(),
            rate_limiting: RateLimitConfig::default(),
        }
    }
}

/// WebSocket configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketConfig {
    /// Maximum message size in bytes
    pub max_message_size: usize,
    /// Ping interval in seconds
    pub ping_interval: u64,
    /// Connection timeout in seconds
    pub connection_timeout: u64,
}

impl Default for WebSocketConfig {
    fn default() -> Self {
        Self {
            max_message_size: 1024 * 1024, // 1MB
            ping_interval: 30,
            connection_timeout: 300, // 5 minutes
        }
    }
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Requests per minute per IP
    pub requests_per_minute: u32,
    /// Burst allowance
    pub burst_size: u32,
    /// Enable rate limiting
    pub enabled: bool,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_minute: 60,
            burst_size: 10,
            enabled: true,
        }
    }
}

/// HTTP API error response
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    /// Error code
    pub code: String,
    /// Human-readable error message
    pub message: String,
    /// Additional error details
    pub details: Option<HashMap<String, serde_json::Value>>,
    /// Request ID for tracking
    pub request_id: Option<Uuid>,
}

impl ApiError {
    pub fn new(code: &str, message: &str) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
            details: None,
            request_id: None,
        }
    }
    
    pub fn with_details(mut self, details: HashMap<String, serde_json::Value>) -> Self {
        self.details = Some(details);
        self
    }
    
    pub fn with_request_id(mut self, request_id: Uuid) -> Self {
        self.request_id = Some(request_id);
        self
    }
}

/// Standard API response wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// Response data
    pub data: T,
    /// Request ID for tracking
    pub request_id: Uuid,
    /// Response timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl<T> ApiResponse<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            request_id: Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
        }
    }
    
    pub fn with_request_id(mut self, request_id: Uuid) -> Self {
        self.request_id = request_id;
        self
    }
}

/// Health check response
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    /// Service status
    pub status: String,
    /// Service version
    pub version: String,
    /// Zenoh connection status
    pub zenoh_connected: bool,
    /// Active group connections
    pub active_groups: u32,
    /// Active WebSocket connections
    pub active_websockets: u32,
    /// Uptime in seconds
    pub uptime: u64,
}

/// Group information for API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupInfo {
    /// Group ID
    pub id: String,
    /// Group display name
    pub name: String,
    /// Group type
    pub group_type: GroupType,
    /// Number of active members
    pub active_members: u32,
    /// Group creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last activity timestamp
    pub last_activity: chrono::DateTime<chrono::Utc>,
}

/// Group member information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupMember {
    /// Member ID
    pub id: String,
    /// Display name
    pub name: String,
    /// Member role
    pub role: GroupRole,
    /// Online status
    pub online: bool,
    /// Last seen timestamp
    pub last_seen: chrono::DateTime<chrono::Utc>,
}

/// Types of groups supported
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GroupType {
    /// Family group
    Family,
    /// Business team
    Team,
    /// Organization/company
    Organization,
    /// Project group
    Project,
    /// Community group
    Community,
    /// Educational group
    Educational,
}

/// Group member roles
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GroupRole {
    /// Group administrator
    Admin,
    /// Manager/supervisor
    Manager,
    /// Regular member
    Member,
    /// Guest user
    Guest,
    /// Observer (read-only)
    Observer,
    /// Parent/guardian (family groups)
    Parent,
    /// Child (family groups)
    Child,
    /// Team lead (business groups)
    TeamLead,
    /// Developer (business groups)
    Developer,
    /// Designer (business groups)
    Designer,
    /// Product manager (business groups)
    ProductManager,
}

/// Chat message for group communication
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    /// Message ID
    pub id: Uuid,
    /// Group ID
    pub group_id: String,
    /// Sender ID
    pub sender_id: String,
    /// Sender display name
    pub sender_name: String,
    /// Message content
    pub content: String,
    /// Message timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Message type
    pub message_type: MessageType,
    /// Reply to message ID (if this is a reply)
    pub reply_to: Option<Uuid>,
}

/// Types of chat messages
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageType {
    /// Regular text message
    Text,
    /// Weaver AI response
    WeaverResponse,
    /// System notification
    System,
    /// File attachment
    Attachment,
    /// Voice message
    Voice,
}

/// Request to send a chat message
#[derive(Debug, Deserialize)]
pub struct SendMessageRequest {
    /// Message content
    pub content: String,
    /// Message type (defaults to Text)
    pub message_type: Option<MessageType>,
    /// Reply to message ID (optional)
    pub reply_to: Option<Uuid>,
}

/// Weaver AI request
#[derive(Debug, Deserialize)]
pub struct WeaverRequest {
    /// User's message to Weaver
    pub message: String,
    /// Conversation context (optional)
    pub context: Option<String>,
    /// Requested response style
    pub style: Option<WeaverStyle>,
}

/// Weaver AI response styles
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WeaverStyle {
    /// Helpful family assistant
    FamilyAssistant,
    /// Educational tutor
    Tutor,
    /// Creative collaborator
    Creative,
    /// Technical advisor
    Technical,
    /// Therapeutic support
    Therapeutic,
}

/// Weaver AI response
#[derive(Debug, Serialize, Deserialize)]
pub struct WeaverResponse {
    /// Response content
    pub content: String,
    /// Response style used
    pub style: WeaverStyle,
    /// Confidence in response (0.0 to 1.0)
    pub confidence: f64,
    /// Sources or references used
    pub sources: Vec<String>,
    /// Suggested follow-up questions
    pub follow_ups: Vec<String>,
    /// Cost of this response in tokens
    pub cost_tokens: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_config_default() {
        let config = HttpConfig::default();
        assert_eq!(config.port, 8080);
        assert_eq!(config.host, "0.0.0.0");
        assert!(config.enable_cors);
    }

    #[test]
    fn test_api_error_creation() {
        let error = ApiError::new("BAD_REQUEST", "Invalid input");
        assert_eq!(error.code, "BAD_REQUEST");
        assert_eq!(error.message, "Invalid input");
        assert!(error.details.is_none());
    }

    #[test]
    fn test_api_response_creation() {
        let response = ApiResponse::new("test data");
        assert_eq!(response.data, "test data");
        assert!(response.request_id != Uuid::nil());
    }

    #[test]
    fn test_group_role_serialization() {
        let role = GroupRole::Parent;
        let serialized = serde_json::to_string(&role).unwrap();
        let deserialized: GroupRole = serde_json::from_str(&serialized).unwrap();
        assert_eq!(role, deserialized);
    }
}
