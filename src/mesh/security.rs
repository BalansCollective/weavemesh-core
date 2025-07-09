//! Universal Mesh Security System
//!
//! Provides universal security primitives for mesh networks that can be extended
//! by context-specific plugins. This module contains universal security concepts
//! and a plugin-based architecture for context-specific security policies.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;

/// Universal mesh security system
pub struct SecuritySystem {
    /// Local node ID
    local_node_id: Uuid,
    
    /// Trust relationships with other nodes
    trust_relationships: Arc<RwLock<HashMap<Uuid, TrustRelationship>>>,
    
    /// Security policies
    security_policies: Arc<RwLock<SecurityPolicies>>,
    
    /// Security event log
    security_events: Arc<RwLock<Vec<SecurityEvent>>>,
    
    /// Security providers for context-specific policies
    providers: Vec<Box<dyn SecurityProvider>>,
    
    /// Security configuration
    config: SecurityConfig,
    
    /// Running state
    is_running: Arc<RwLock<bool>>,
}

/// Trust relationship between nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustRelationship {
    /// Partner node ID
    pub partner_id: Uuid,
    
    /// Current trust level
    pub trust_level: TrustLevel,
    
    /// Trust history and evolution
    pub trust_history: Vec<TrustEvent>,
    
    /// Shared credentials and authentication tokens
    pub shared_credentials: SharedCredentials,
    
    /// Trust verification mechanisms
    pub verification_methods: Vec<TrustVerificationMethod>,
    
    /// Trust boundaries and limitations
    pub trust_boundaries: TrustBoundaries,
    
    /// When this relationship was established
    pub established_at: DateTime<Utc>,
    
    /// Last trust verification
    pub last_verified: DateTime<Utc>,
}

/// Universal trust levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrustLevel {
    /// Unknown trust level
    Unknown,
    
    /// Basic trust - minimal verification
    Basic,
    
    /// Verified trust - identity confirmed
    Verified,
    
    /// Trusted - proven reliability
    Trusted,
    
    /// Highly trusted - extensive positive history
    HighlyTrusted,
}

/// Trust event in the relationship history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustEvent {
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
    
    /// Type of trust event
    pub event_type: TrustEventType,
    
    /// Event description
    pub description: String,
    
    /// Trust level before the event
    pub trust_before: TrustLevel,
    
    /// Trust level after the event
    pub trust_after: TrustLevel,
    
    /// Evidence or context for the event
    pub evidence: Vec<String>,
    
    /// Event metadata
    pub metadata: HashMap<String, String>,
}

/// Types of trust events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TrustEventType {
    /// Initial trust establishment
    Establishment,
    
    /// Trust increase due to positive interaction
    Enhancement,
    
    /// Trust decrease due to negative interaction
    Degradation,
    
    /// Trust verification through challenge-response
    Verification,
    
    /// Trust repair after violation
    Repair,
    
    /// Trust revocation due to serious violation
    Revocation,
    
    /// Trust renewal after period of inactivity
    Renewal,
    
    /// Trust update from external source
    ExternalUpdate,
}

/// Shared credentials for secure communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedCredentials {
    /// Shared symmetric keys for encryption
    pub symmetric_keys: HashMap<String, EncryptedKey>,
    
    /// Public key fingerprints for verification
    pub public_key_fingerprints: HashMap<String, String>,
    
    /// Authentication tokens
    pub auth_tokens: HashMap<String, AuthToken>,
    
    /// Credential rotation schedule
    pub rotation_schedule: CredentialRotationSchedule,
    
    /// Last credential update
    pub last_updated: DateTime<Utc>,
}

/// Encrypted key with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedKey {
    /// Encrypted key data
    pub encrypted_data: String,
    
    /// Key algorithm
    pub algorithm: String,
    
    /// Key creation time
    pub created_at: DateTime<Utc>,
    
    /// Key expiration time
    pub expires_at: Option<DateTime<Utc>>,
    
    /// Key usage permissions
    pub permissions: Vec<String>,
}

/// Authentication token with expiration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthToken {
    /// Token value (encrypted)
    pub token: String,
    
    /// Token expiration time
    pub expires_at: DateTime<Utc>,
    
    /// Token scope and permissions
    pub scope: Vec<String>,
    
    /// Token issuer
    pub issuer: Uuid,
    
    /// Token type
    pub token_type: TokenType,
    
    /// Token metadata
    pub metadata: HashMap<String, String>,
}

/// Types of authentication tokens
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TokenType {
    /// Bearer token for API access
    Bearer,
    
    /// Session token for ongoing communication
    Session,
    
    /// Refresh token for token renewal
    Refresh,
    
    /// Capability token for specific permissions
    Capability,
    
    /// Context-specific token
    ContextSpecific(String),
}

/// Trust verification methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrustVerificationMethod {
    /// Challenge-response verification
    ChallengeResponse {
        challenge_type: String,
        response_timeout: Duration,
        difficulty_level: u32,
    },
    
    /// Behavioral consistency verification
    BehavioralConsistency {
        behavior_patterns: Vec<String>,
        consistency_threshold: f64,
        observation_period: Duration,
    },
    
    /// Third-party attestation
    ThirdPartyAttestation {
        attestor_id: Uuid,
        attestation_type: String,
        validity_period: Duration,
    },
    
    /// Collaborative task verification
    CollaborativeTask {
        task_type: String,
        success_criteria: Vec<String>,
        completion_timeout: Duration,
    },
    
    /// Cryptographic proof verification
    CryptographicProof {
        proof_type: String,
        algorithm: String,
        verification_data: String,
    },
    
    /// Context-specific verification
    ContextSpecific {
        context: String,
        verification_data: serde_json::Value,
    },
}

/// Trust boundaries and limitations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustBoundaries {
    /// Maximum trust level that can be achieved
    pub max_trust_level: TrustLevel,
    
    /// Domains where trust applies
    pub trusted_domains: Vec<String>,
    
    /// Domains where trust is limited
    pub limited_domains: Vec<String>,
    
    /// Specific capabilities that are trusted
    pub trusted_capabilities: Vec<String>,
    
    /// Time-based trust limitations
    pub time_limitations: Option<TrustTimeLimit>,
    
    /// Conditional trust requirements
    pub conditional_requirements: Vec<TrustCondition>,
    
    /// Geographic or network limitations
    pub network_limitations: Vec<String>,
}

/// Time-based trust limitations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustTimeLimit {
    /// Trust expiration time
    pub expires_at: DateTime<Utc>,
    
    /// Trust renewal requirements
    pub renewal_requirements: Vec<String>,
    
    /// Automatic renewal conditions
    pub auto_renewal_conditions: Vec<String>,
    
    /// Grace period for renewal
    pub grace_period: Duration,
}

/// Conditional trust requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustCondition {
    /// Condition description
    pub condition: String,
    
    /// Required evidence for condition
    pub required_evidence: Vec<String>,
    
    /// Condition verification method
    pub verification_method: String,
    
    /// Condition priority
    pub priority: ConditionPriority,
    
    /// Condition metadata
    pub metadata: HashMap<String, String>,
}

/// Priority levels for trust conditions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConditionPriority {
    /// Low priority - optional
    Low,
    
    /// Medium priority - recommended
    Medium,
    
    /// High priority - important
    High,
    
    /// Critical priority - required
    Critical,
}

/// Universal security policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPolicies {
    /// Authentication requirements
    pub authentication_requirements: HashMap<String, AuthenticationPolicy>,
    
    /// Authorization rules
    pub authorization_rules: Vec<AuthorizationRule>,
    
    /// Encryption requirements
    pub encryption_requirements: EncryptionPolicy,
    
    /// Access control policies
    pub access_control: AccessControlPolicy,
    
    /// Security monitoring settings
    pub monitoring_settings: MonitoringPolicy,
    
    /// Context-specific policies
    pub context_policies: HashMap<String, serde_json::Value>,
}

/// Authentication policy for different operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationPolicy {
    /// Required authentication methods
    pub required_methods: Vec<AuthenticationMethod>,
    
    /// Multi-factor authentication requirements
    pub mfa_required: bool,
    
    /// Token expiration time
    pub token_expiration: Duration,
    
    /// Maximum failed attempts
    pub max_failed_attempts: u32,
    
    /// Lockout duration after failed attempts
    pub lockout_duration: Duration,
}

/// Authentication methods
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuthenticationMethod {
    /// Password-based authentication
    Password,
    
    /// Public key authentication
    PublicKey,
    
    /// Token-based authentication
    Token,
    
    /// Certificate-based authentication
    Certificate,
    
    /// Biometric authentication
    Biometric,
    
    /// Hardware token authentication
    HardwareToken,
    
    /// Context-specific authentication
    ContextSpecific(String),
}

/// Authorization rule for access control
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationRule {
    /// Rule identifier
    pub rule_id: String,
    
    /// Resource pattern this rule applies to
    pub resource_pattern: String,
    
    /// Required permissions
    pub required_permissions: Vec<String>,
    
    /// Required trust level
    pub required_trust_level: TrustLevel,
    
    /// Additional conditions
    pub conditions: Vec<String>,
    
    /// Rule priority
    pub priority: u32,
    
    /// Rule metadata
    pub metadata: HashMap<String, String>,
}

/// Encryption policy settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionPolicy {
    /// Required encryption algorithms
    pub required_algorithms: Vec<String>,
    
    /// Minimum key sizes
    pub minimum_key_sizes: HashMap<String, u32>,
    
    /// Key rotation frequency
    pub key_rotation_frequency: Duration,
    
    /// Encryption requirements by data type
    pub data_type_requirements: HashMap<String, EncryptionRequirement>,
    
    /// Perfect forward secrecy requirements
    pub pfs_required: bool,
}

/// Encryption requirement for specific data types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionRequirement {
    /// Encryption algorithm
    pub algorithm: String,
    
    /// Key size
    pub key_size: u32,
    
    /// Additional parameters
    pub parameters: HashMap<String, String>,
    
    /// Encryption mode
    pub mode: String,
}

/// Access control policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControlPolicy {
    /// Default access level
    pub default_access: AccessLevel,
    
    /// Resource-specific access rules
    pub resource_rules: HashMap<String, AccessLevel>,
    
    /// Role-based access control
    pub role_based_access: HashMap<String, Vec<String>>,
    
    /// Time-based access restrictions
    pub time_restrictions: Vec<TimeRestriction>,
    
    /// Network-based access restrictions
    pub network_restrictions: Vec<NetworkRestriction>,
}

/// Access levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum AccessLevel {
    /// No access
    None,
    
    /// Read-only access
    Read,
    
    /// Read and write access
    ReadWrite,
    
    /// Full administrative access
    Admin,
    
    /// Context-specific access level
    ContextSpecific(String),
}

/// Time-based access restriction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRestriction {
    /// Restriction description
    pub description: String,
    
    /// Allowed time windows
    pub allowed_windows: Vec<TimeWindow>,
    
    /// Timezone for time calculations
    pub timezone: String,
    
    /// Exception conditions
    pub exceptions: Vec<String>,
}

/// Time window for access
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeWindow {
    /// Start time (24-hour format)
    pub start_time: String,
    
    /// End time (24-hour format)
    pub end_time: String,
    
    /// Days of week (0=Sunday, 6=Saturday)
    pub days_of_week: Vec<u8>,
    
    /// Date range (optional)
    pub date_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
}

/// Network-based access restriction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkRestriction {
    /// Restriction type
    pub restriction_type: NetworkRestrictionType,
    
    /// Network patterns
    pub network_patterns: Vec<String>,
    
    /// Restriction description
    pub description: String,
    
    /// Exception conditions
    pub exceptions: Vec<String>,
}

/// Types of network restrictions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NetworkRestrictionType {
    /// Allow only specified networks
    AllowList,
    
    /// Block specified networks
    BlockList,
    
    /// Geographic restrictions
    Geographic,
    
    /// Protocol restrictions
    Protocol,
}

/// Security monitoring policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringPolicy {
    /// Enable security event logging
    pub enable_logging: bool,
    
    /// Log retention period
    pub log_retention_period: Duration,
    
    /// Events to monitor
    pub monitored_events: Vec<SecurityEventType>,
    
    /// Alert thresholds
    pub alert_thresholds: HashMap<String, AlertThreshold>,
    
    /// Monitoring frequency
    pub monitoring_frequency: Duration,
    
    /// External monitoring integrations
    pub external_integrations: Vec<String>,
}

/// Alert threshold configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThreshold {
    /// Threshold value
    pub threshold: f64,
    
    /// Time window for threshold calculation
    pub time_window: Duration,
    
    /// Alert severity
    pub severity: SecuritySeverity,
    
    /// Alert actions
    pub actions: Vec<String>,
}

/// Security event in the mesh
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    /// Event ID
    pub event_id: Uuid,
    
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
    
    /// Event type
    pub event_type: SecurityEventType,
    
    /// Involved nodes
    pub involved_nodes: Vec<Uuid>,
    
    /// Event description
    pub description: String,
    
    /// Severity level
    pub severity: SecuritySeverity,
    
    /// Response actions taken
    pub response_actions: Vec<String>,
    
    /// Event resolution status
    pub resolution_status: ResolutionStatus,
    
    /// Event metadata
    pub metadata: HashMap<String, String>,
    
    /// Related events
    pub related_events: Vec<Uuid>,
}

/// Types of security events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SecurityEventType {
    /// Trust establishment
    TrustEstablishment,
    
    /// Trust violation
    TrustViolation,
    
    /// Authentication attempt
    AuthenticationAttempt,
    
    /// Authentication failure
    AuthenticationFailure,
    
    /// Authorization check
    AuthorizationCheck,
    
    /// Authorization failure
    AuthorizationFailure,
    
    /// Unauthorized access attempt
    UnauthorizedAccess,
    
    /// Encryption key rotation
    KeyRotation,
    
    /// Security policy violation
    PolicyViolation,
    
    /// Suspicious activity detected
    SuspiciousActivity,
    
    /// Security configuration change
    ConfigurationChange,
    
    /// Context-specific security event
    ContextSpecific {
        context: String,
        event_subtype: String,
    },
}

/// Security event severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecuritySeverity {
    /// Informational event
    Info,
    
    /// Low severity - monitoring required
    Low,
    
    /// Medium severity - attention required
    Medium,
    
    /// High severity - immediate action required
    High,
    
    /// Critical severity - emergency response required
    Critical,
}

/// Event resolution status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ResolutionStatus {
    /// Event is open and unresolved
    Open,
    
    /// Event is being investigated
    InProgress,
    
    /// Event has been resolved
    Resolved,
    
    /// Event resolution was unsuccessful
    Failed,
    
    /// Event was escalated to higher authority
    Escalated,
    
    /// Event was automatically resolved
    AutoResolved,
}

/// Credential rotation schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialRotationSchedule {
    /// Rotation frequency
    pub rotation_frequency: Duration,
    
    /// Rotation triggers
    pub rotation_triggers: Vec<String>,
    
    /// Rotation procedures
    pub rotation_procedures: Vec<String>,
    
    /// Emergency rotation conditions
    pub emergency_rotation: Vec<String>,
    
    /// Next scheduled rotation
    pub next_rotation: DateTime<Utc>,
}

/// Security system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Maximum security events to keep in memory
    pub max_events_in_memory: usize,
    
    /// Security event retention period
    pub event_retention_period: Duration,
    
    /// Enable security monitoring
    pub enable_monitoring: bool,
    
    /// Default trust level for new nodes
    pub default_trust_level: TrustLevel,
    
    /// Trust verification frequency
    pub trust_verification_frequency: Duration,
    
    /// Context-specific configuration
    pub context_config: HashMap<String, serde_json::Value>,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            max_events_in_memory: 10000,
            event_retention_period: Duration::from_secs(86400 * 30), // 30 days
            enable_monitoring: true,
            default_trust_level: TrustLevel::Unknown,
            trust_verification_frequency: Duration::from_secs(3600), // 1 hour
            context_config: HashMap::new(),
        }
    }
}

impl SecuritySystem {
    /// Create a new security system
    pub fn new(
        local_node_id: Uuid,
        config: Option<SecurityConfig>,
    ) -> Self {
        let config = config.unwrap_or_default();
        
        info!("Initializing security system for node: {}", local_node_id);
        
        Self {
            local_node_id,
            trust_relationships: Arc::new(RwLock::new(HashMap::new())),
            security_policies: Arc::new(RwLock::new(SecurityPolicies::default())),
            security_events: Arc::new(RwLock::new(Vec::new())),
            providers: Vec::new(),
            config,
            is_running: Arc::new(RwLock::new(false)),
        }
    }
    
    /// Add a security provider for context-specific policies
    pub fn add_provider(&mut self, provider: Box<dyn SecurityProvider>) {
        info!("Adding security provider: {}", provider.name());
        self.providers.push(provider);
    }
    
    /// Start the security system
    pub async fn start(&mut self) -> Result<()> {
        let mut is_running = self.is_running.write().await;
        if *is_running {
            return Ok(());
        }
        
        *is_running = true;
        drop(is_running);
        
        // Initialize security providers
        for provider in &mut self.providers {
            provider.initialize(&self.config).await?;
        }
        
        info!("Security system started for node {}", self.local_node_id);
        Ok(())
    }
    
    /// Stop the security system
    pub async fn stop(&mut self) -> Result<()> {
        let mut is_running = self.is_running.write().await;
        if !*is_running {
            return Ok(());
        }
        
        *is_running = false;
        drop(is_running);
        
        // Cleanup security providers
        for provider in &mut self.providers {
            provider.cleanup().await?;
        }
        
        info!("Security system stopped for node {}", self.local_node_id);
        Ok(())
    }
    
    /// Establish trust relationship with another node
    pub async fn establish_trust(
        &self,
        partner_id: Uuid,
        initial_trust_level: TrustLevel,
        verification_methods: Vec<TrustVerificationMethod>,
    ) -> Result<()> {
        let trust_relationship = TrustRelationship {
            partner_id,
            trust_level: initial_trust_level.clone(),
            trust_history: vec![TrustEvent {
                timestamp: Utc::now(),
                event_type: TrustEventType::Establishment,
                description: "Initial trust establishment".to_string(),
                trust_before: TrustLevel::Unknown,
                trust_after: initial_trust_level,
                evidence: Vec::new(),
                metadata: HashMap::new(),
            }],
            shared_credentials: SharedCredentials::default(),
            verification_methods,
            trust_boundaries: TrustBoundaries::default(),
            established_at: Utc::now(),
            last_verified: Utc::now(),
        };
        
        let mut relationships = self.trust_relationships.write().await;
        relationships.insert(partner_id, trust_relationship);
        drop(relationships);
        
        // Log security event
        self.log_security_event(SecurityEvent {
            event_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type: SecurityEventType::TrustEstablishment,
            involved_nodes: vec![self.local_node_id, partner_id],
            description: "Trust relationship established".to_string(),
            severity: SecuritySeverity::Info,
            response_actions: Vec::new(),
            resolution_status: ResolutionStatus::Resolved,
            metadata: HashMap::new(),
            related_events: Vec::new(),
        }).await;
        
        info!("Established trust relationship with node {}", partner_id);
        Ok(())
    }
    
    /// Verify trust relationship
    pub async fn verify_trust(&self, partner_id: Uuid) -> Result<bool> {
        let relationships = self.trust_relationships.read().await;
        if let Some(relationship) = relationships.get(&partner_id) {
            // Check if trust level is sufficient
            Ok(matches!(
                relationship.trust_level,
                TrustLevel::Basic | TrustLevel::Verified | TrustLevel::Trusted | TrustLevel::HighlyTrusted
            ))
        } else {
            Ok(false)
        }
    }
    
    /// Check authorization for a resource
    pub async fn check_authorization(
        &self,
        node_id: Uuid,
        resource: &str,
        action: &str,
    ) -> Result<bool> {
        let policies = self.security_policies.read().await;
        
        // Get trust level
        let trust_level = self.get_trust_level(node_id).await;
        
        // Check authorization rules
        for rule in &policies.authorization_rules {
            if resource.contains(&rule.resource_pattern) {
                if trust_level >= rule.required_trust_level {
                    return Ok(true);
                }
            }
        }
        
        // Check with security providers
        for provider in &self.providers {
            if provider.check_authorization(node_id, resource, action).await? {
                return Ok(true);
            }
        }
        
        Ok(false)
    }
    
    /// Get trust level with partner
    pub async fn get_trust_level(&self, partner_id: Uuid) -> TrustLevel {
        let relationships = self.trust_relationships.read().await;
        relationships.get(&partner_id)
            .map(|r| r.trust_level.clone())
            .unwrap_or(TrustLevel::Unknown)
    }
    
    /// Log security event
    pub async fn log_security_event(&self, event: SecurityEvent) {
        let mut events = self.security_events.write().await;
        events.push(event.clone());
        
        // Maintain event history size limit
        if events.len() > self.config.max_events_in_memory {
            let excess = events.len() - self.config.max_events_in_memory;
            events.drain(0..excess);
        }
        drop(events);
        
        // Process with security providers
        for provider in &self.providers {
            if let Err(e) = provider.handle_security_event(&event).await {
                warn!("Security provider {} failed to handle event: {}", provider.name(), e);
            }
        }
        
        debug!("Logged security event: {} ({})", event.event_id, event.event_type.category());
    }
    
    /// Get security events
    pub async fn get_security_events(&self, filter: Option<SecurityEventFilter>) -> Vec<SecurityEvent> {
        let events = self.security_events.read().await;
        match filter {
            Some(f) => events.iter()
                .filter(|event| f.matches(event))
                .cloned()
                .collect(),
            None => events.clone(),
        }
    }
    
    /// Update security policies
    pub async fn update_policies(&self, policies: SecurityPolicies) -> Result<()> {
        let mut current_policies = self.security_policies.write().await;
        *current_policies = policies;
        
        info!("Updated security policies for node {}", self.local_node_id);
        Ok(())
    }
    
    /// Get security configuration
    pub fn get_config(&self) -> &SecurityConfig {
        &self.config
    }
    
    /// Update security configuration
    pub fn update_config(&mut self, config: SecurityConfig) {
        self.config = config;
        debug!("Updated security system configuration");
    }
}

/// Filter for security events
#[derive(Debug, Clone)]
pub struct SecurityEventFilter {
    /// Event types to include
    pub event_types: Option<Vec<SecurityEventType>>,
    
    /// Severity levels to include
    pub severities: Option<Vec<SecuritySeverity>>,
    
    /// Time range
    pub time_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    
    /// Involved nodes
    pub involved_nodes: Option<Vec<Uuid>>,
    
    /// Resolution status
    pub resolution_status: Option<Vec<ResolutionStatus>>,
}

impl SecurityEventFilter {
    /// Check if an event matches this filter
    pub fn matches(&self, event: &SecurityEvent) -> bool {
        if let Some(ref types) = self.event_types {
            if !types.contains(&event.event_type) {
                return false;
            }
        }
        
        if let Some(ref severities) = self.severities {
            if !severities.contains(&event.severity) {
                return false;
            }
        }
        
        if let Some((start, end)) = self.time_range {
            if event.timestamp < start || event.timestamp > end {
                return false;
            }
        }
        
        if let Some(ref nodes) = self.involved_nodes {
            if !event.involved_nodes.iter().any(|n| nodes.contains(n)) {
                return false;
            }
        }
        
        if let Some(ref statuses) = self.resolution_status {
            if !statuses.contains(&event.resolution_status) {
                return false;
            }
        }
        
        true
    }
}

impl SecurityEventType {
    /// Get the event category for routing
    pub fn category(&self) -> &'static str {
        match self {
            SecurityEventType::TrustEstablishment => "trust",
            SecurityEventType::TrustViolation => "trust",
            SecurityEventType::AuthenticationAttempt => "auth",
            SecurityEventType::AuthenticationFailure => "auth",
            SecurityEventType::AuthorizationCheck => "authz",
            SecurityEventType::AuthorizationFailure => "authz",
            SecurityEventType::UnauthorizedAccess => "access",
            SecurityEventType::KeyRotation => "crypto",
            SecurityEventType::PolicyViolation => "policy",
            SecurityEventType::SuspiciousActivity => "monitoring",
            SecurityEventType::ConfigurationChange => "config",
            SecurityEventType::ContextSpecific { context, .. } => "context",
        }
    }
}

/// Trait for context-specific security providers
#[async_trait::async_trait]
pub trait SecurityProvider: Send + Sync {
    /// Provider name
    fn name(&self) -> &str;
    
    /// Initialize the security provider
    async fn initialize(&mut self, config: &SecurityConfig) -> Result<()>;
    
    /// Cleanup provider resources
    async fn cleanup(&mut self) -> Result<()>;
    
    /// Handle a security event
    async fn handle_security_event(&self, event: &SecurityEvent) -> Result<()>;
    
    /// Check authorization for a resource
    async fn check_authorization(&self, node_id: Uuid, resource: &str, action: &str) -> Result<bool>;
    
    /// Validate trust relationship
    async fn validate_trust(&self, relationship: &TrustRelationship) -> Result<bool>;
    
    /// Get provider-specific security policies
    fn get_security_policies(&self) -> Vec<String>;
}

// Default implementations
impl Default for SecurityPolicies {
    fn default() -> Self {
        Self {
            authentication_requirements: HashMap::new(),
            authorization_rules: Vec::new(),
            encryption_requirements: EncryptionPolicy::default(),
            access_control: AccessControlPolicy::default(),
            monitoring_settings: MonitoringPolicy::default(),
            context_policies: HashMap::new(),
        }
    }
}

impl Default for SharedCredentials {
    fn default() -> Self {
        Self {
            symmetric_keys: HashMap::new(),
            public_key_fingerprints: HashMap::new(),
            auth_tokens: HashMap::new(),
            rotation_schedule: CredentialRotationSchedule::default(),
            last_updated: Utc::now(),
        }
    }
}

impl Default for TrustBoundaries {
    fn default() -> Self {
        Self {
            max_trust_level: TrustLevel::Trusted,
            trusted_domains: Vec::new(),
            limited_domains: Vec::new(),
            trusted_capabilities: Vec::new(),
            time_limitations: None,
            conditional_requirements: Vec::new(),
            network_limitations: Vec::new(),
        }
    }
}

impl Default for EncryptionPolicy {
    fn default() -> Self {
        Self {
            required_algorithms: vec!["AES-256-GCM".to_string()],
            minimum_key_sizes: {
                let mut sizes = HashMap::new();
                sizes.insert("AES".to_string(), 256);
                sizes.insert("RSA".to_string(), 2048);
                sizes
            },
            key_rotation_frequency: Duration::from_secs(86400), // 24 hours
            data_type_requirements: HashMap::new(),
            pfs_required: true,
        }
    }
}

impl Default for AccessControlPolicy {
    fn default() -> Self {
        Self {
            default_access: AccessLevel::None,
            resource_rules: HashMap::new(),
            role_based_access: HashMap::new(),
            time_restrictions: Vec::new(),
            network_restrictions: Vec::new(),
        }
    }
}

impl Default for MonitoringPolicy {
    fn default() -> Self {
        Self {
            enable_logging: true,
            log_retention_period: Duration::from_secs(86400 * 30), // 30 days
            monitored_events: vec![
                SecurityEventType::TrustViolation,
                SecurityEventType::AuthenticationFailure,
                SecurityEventType::AuthorizationFailure,
                SecurityEventType::UnauthorizedAccess,
                SecurityEventType::PolicyViolation,
                SecurityEventType::SuspiciousActivity,
            ],
            alert_thresholds: HashMap::new(),
            monitoring_frequency: Duration::from_secs(60), // 1 minute
            external_integrations: Vec::new(),
        }
    }
}

impl Default for CredentialRotationSchedule {
    fn default() -> Self {
        Self {
            rotation_frequency: Duration::from_secs(86400), // 24 hours
            rotation_triggers: vec![
                "time_based".to_string(),
                "usage_threshold".to_string(),
                "security_event".to_string(),
            ],
            rotation_procedures: vec![
                "generate_new_key".to_string(),
                "distribute_key".to_string(),
                "revoke_old_key".to_string(),
            ],
            emergency_rotation: vec![
                "immediate_revocation".to_string(),
                "emergency_key_generation".to_string(),
                "security_alert".to_string(),
            ],
            next_rotation: Utc::now() + Duration::from_secs(86400),
        }
    }
}

impl Default for TrustLevel {
    fn default() -> Self {
        TrustLevel::Unknown
    }
}

impl Default for TokenType {
    fn default() -> Self {
        TokenType::Bearer
    }
}

impl Default for ConditionPriority {
    fn default() -> Self {
        ConditionPriority::Medium
    }
}

impl Default for AccessLevel {
    fn default() -> Self {
        AccessLevel::None
    }
}

impl Default for SecuritySeverity {
    fn default() -> Self {
        SecuritySeverity::Info
    }
}

impl Default for ResolutionStatus {
    fn default() -> Self {
        ResolutionStatus::Open
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trust_level_ordering() {
        assert!(TrustLevel::HighlyTrusted > TrustLevel::Trusted);
        assert!(TrustLevel::Trusted > TrustLevel::Verified);
        assert!(TrustLevel::Verified > TrustLevel::Basic);
        assert!(TrustLevel::Basic > TrustLevel::Unknown);
    }

    #[test]
    fn test_security_event_creation() {
        let event = SecurityEvent {
            event_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type: SecurityEventType::TrustEstablishment,
            involved_nodes: vec![Uuid::new_v4()],
            description: "Test event".to_string(),
            severity: SecuritySeverity::Info,
            response_actions: Vec::new(),
            resolution_status: ResolutionStatus::Open,
            metadata: HashMap::new(),
            related_events: Vec::new(),
        };
        
        assert_eq!(event.description, "Test event");
        assert!(matches!(event.severity, SecuritySeverity::Info));
        assert_eq!(event.event_type.category(), "trust");
    }

    #[test]
    fn test_security_event_filter() {
        let event = SecurityEvent {
            event_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type: SecurityEventType::AuthenticationFailure,
            involved_nodes: vec![Uuid::new_v4()],
            description: "Auth failure".to_string(),
            severity: SecuritySeverity::High,
            response_actions: Vec::new(),
            resolution_status: ResolutionStatus::Open,
            metadata: HashMap::new(),
            related_events: Vec::new(),
        };
        
        let filter = SecurityEventFilter {
            event_types: Some(vec![SecurityEventType::AuthenticationFailure]),
            severities: None,
            time_range: None,
            involved_nodes: None,
            resolution_status: None,
        };
        
        assert!(filter.matches(&event));
        
        let filter2 = SecurityEventFilter {
            event_types: Some(vec![SecurityEventType::TrustEstablishment]),
            severities: None,
            time_range: None,
            involved_nodes: None,
            resolution_status: None,
        };
        
        assert!(!filter2.matches(&event));
    }

    #[tokio::test]
    async fn test_security_system_creation() {
        let node_id = Uuid::new_v4();
        let security_system = SecuritySystem::new(node_id, None);
        
        assert_eq!(security_system.local_node_id, node_id);
        assert!(!*security_system.is_running.read().await);
        assert_eq!(security_system.providers.len(), 0);
    }

    #[tokio::test]
    async fn test_trust_establishment() {
        let node_id = Uuid::new_v4();
        let partner_id = Uuid::new_v4();
        let security_system = SecuritySystem::new(node_id, None);
        
        security_system.establish_trust(
            partner_id,
            TrustLevel::Basic,
            vec![],
        ).await.unwrap();
        
        let trust_level = security_system.get_trust_level(partner_id).await;
        assert_eq!(trust_level, TrustLevel::Basic);
        
        let is_trusted = security_system.verify_trust(partner_id).await.unwrap();
        assert!(is_trusted);
    }

    #[tokio::test]
    async fn test_authorization_check() {
        let node_id = Uuid::new_v4();
        let partner_id = Uuid::new_v4();
        let security_system = SecuritySystem::new(node_id, None);
        
        // Without trust, authorization should fail
        let authorized = security_system.check_authorization(
            partner_id,
            "test_resource",
            "read",
        ).await.unwrap();
        assert!(!authorized);
        
        // With trust, authorization might succeed (depends on policies)
        security_system.establish_trust(
            partner_id,
            TrustLevel::Trusted,
            vec![],
        ).await.unwrap();
        
        // Still fails without proper authorization rules
        let authorized = security_system.check_authorization(
            partner_id,
            "test_resource",
            "read",
        ).await.unwrap();
        assert!(!authorized);
    }
}
