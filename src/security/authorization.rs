//! Authorization module for WeaveMesh Core
//! 
//! Implements role-based access control and environment-specific permissions.

use crate::security::{SecurityLevel, SecurityContext, Environment, AuthenticationTier};
use crate::WeaveMeshError;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// User role in the system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Role {
    /// Open source user - read-only access to public content
    OpenSourceUser,
    /// Company employee - access to internal content
    Employee,
    /// Project team member - access to client-specific content
    ProjectMember { project_ids: Vec<String> },
    /// Compliance officer - access to compliance-regulated content
    ComplianceOfficer { standards: Vec<String> },
    /// Security officer - access to security-related content
    SecurityOfficer,
    /// Administrator - full access within organization
    Administrator,
    /// Defense personnel - access to classified content
    DefensePersonnel { clearance_level: String },
}

impl Role {
    /// Get the maximum security level this role can access
    pub fn max_security_level(&self) -> SecurityLevel {
        match self {
            Role::OpenSourceUser => SecurityLevel::Open,
            Role::Employee => SecurityLevel::Internal,
            Role::ProjectMember { .. } => SecurityLevel::Client,
            Role::ComplianceOfficer { .. } => SecurityLevel::Compliance,
            Role::SecurityOfficer => SecurityLevel::Compliance,
            Role::Administrator => SecurityLevel::Compliance,
            Role::DefensePersonnel { .. } => SecurityLevel::Classified,
        }
    }
    
    /// Check if this role can access a specific environment
    pub fn can_access_environment(&self, environment: &Environment) -> bool {
        match (self, environment) {
            // Open source users can only access open environments
            (Role::OpenSourceUser, Environment::Open) => true,
            (Role::OpenSourceUser, _) => false,
            
            // Employees can access open and internal environments
            (Role::Employee, Environment::Open | Environment::Internal { .. }) => true,
            (Role::Employee, _) => false,
            
            // Project members can access environments for their projects
            (Role::ProjectMember { project_ids }, Environment::Client { client_id, .. }) => {
                project_ids.contains(client_id)
            }
            (Role::ProjectMember { .. }, Environment::Open | Environment::Internal { .. }) => true,
            (Role::ProjectMember { .. }, _) => false,
            
            // Compliance officers can access compliance environments
            (Role::ComplianceOfficer { standards }, Environment::Medical { compliance_standards, .. }) => {
                compliance_standards.iter().any(|std| {
                    standards.contains(&std.to_string())
                })
            }
            (Role::ComplianceOfficer { standards }, Environment::GDPR { .. }) => {
                standards.contains(&"GDPR".to_string())
            }
            (Role::ComplianceOfficer { .. }, Environment::Open | Environment::Internal { .. } | Environment::Client { .. }) => true,
            (Role::ComplianceOfficer { .. }, _) => false,
            
            // Security officers have broad access
            (Role::SecurityOfficer, Environment::Defense { .. }) => false, // Except defense
            (Role::SecurityOfficer, _) => true,
            
            // Administrators have broad access within their organization
            (Role::Administrator, Environment::Defense { .. }) => false, // Except defense
            (Role::Administrator, _) => true,
            
            // Defense personnel can access defense environments
            (Role::DefensePersonnel { .. }, _) => true, // Full access for defense personnel
        }
    }
    
    /// Get display name for the role
    pub fn display_name(&self) -> String {
        match self {
            Role::OpenSourceUser => "Open Source User".to_string(),
            Role::Employee => "Employee".to_string(),
            Role::ProjectMember { project_ids } => {
                format!("Project Member ({})", project_ids.join(", "))
            }
            Role::ComplianceOfficer { standards } => {
                format!("Compliance Officer ({})", standards.join(", "))
            }
            Role::SecurityOfficer => "Security Officer".to_string(),
            Role::Administrator => "Administrator".to_string(),
            Role::DefensePersonnel { clearance_level } => {
                format!("Defense Personnel ({})", clearance_level)
            }
        }
    }
}

/// Permission for specific operations
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    /// Read access to content
    Read,
    /// Write access to content
    Write,
    /// Execute spells/operations
    Execute,
    /// Manage users and permissions
    Manage,
    /// Access audit logs
    Audit,
    /// Configure security settings
    SecurityConfig,
    /// Access classified information
    ClassifiedAccess,
}

/// Authorization policy for a resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationPolicy {
    /// Required security level
    pub required_security_level: SecurityLevel,
    /// Required roles
    pub required_roles: Vec<Role>,
    /// Required permissions
    pub required_permissions: Vec<Permission>,
    /// Environment restrictions
    pub environment_restrictions: Vec<Environment>,
    /// Additional conditions
    pub conditions: HashMap<String, String>,
}

impl AuthorizationPolicy {
    /// Create a new authorization policy
    pub fn new(required_security_level: SecurityLevel) -> Self {
        Self {
            required_security_level,
            required_roles: Vec::new(),
            required_permissions: Vec::new(),
            environment_restrictions: Vec::new(),
            conditions: HashMap::new(),
        }
    }
    
    /// Add a required role
    pub fn require_role(mut self, role: Role) -> Self {
        self.required_roles.push(role);
        self
    }
    
    /// Add a required permission
    pub fn require_permission(mut self, permission: Permission) -> Self {
        self.required_permissions.push(permission);
        self
    }
    
    /// Add an environment restriction
    pub fn restrict_to_environment(mut self, environment: Environment) -> Self {
        self.environment_restrictions.push(environment);
        self
    }
    
    /// Add a condition
    pub fn add_condition(mut self, key: String, value: String) -> Self {
        self.conditions.insert(key, value);
        self
    }
}

/// User authorization information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAuthorization {
    /// User's email
    pub user_email: String,
    /// User's organization
    pub organization: Option<String>,
    /// User's roles
    pub roles: Vec<Role>,
    /// User's permissions
    pub permissions: HashSet<Permission>,
    /// Environments user can access
    pub accessible_environments: Vec<Environment>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl UserAuthorization {
    /// Create a new user authorization
    pub fn new(user_email: String, organization: Option<String>) -> Self {
        Self {
            user_email,
            organization,
            roles: Vec::new(),
            permissions: HashSet::new(),
            accessible_environments: Vec::new(),
            metadata: HashMap::new(),
        }
    }
    
    /// Add a role to the user
    pub fn add_role(mut self, role: Role) -> Self {
        self.roles.push(role);
        self
    }
    
    /// Add a permission to the user
    pub fn add_permission(mut self, permission: Permission) -> Self {
        self.permissions.insert(permission);
        self
    }
    
    /// Add an accessible environment
    pub fn add_environment(mut self, environment: Environment) -> Self {
        self.accessible_environments.push(environment);
        self
    }
    
    /// Check if user has a specific role
    pub fn has_role(&self, role: &Role) -> bool {
        self.roles.contains(role)
    }
    
    /// Check if user has a specific permission
    pub fn has_permission(&self, permission: &Permission) -> bool {
        self.permissions.contains(permission)
    }
    
    /// Check if user can access an environment
    pub fn can_access_environment(&self, environment: &Environment) -> bool {
        // Check if user has explicit access to this environment
        if self.accessible_environments.contains(environment) {
            return true;
        }
        
        // Check if any of user's roles allow access to this environment
        self.roles.iter().any(|role| role.can_access_environment(environment))
    }
    
    /// Get maximum security level user can access
    pub fn max_security_level(&self) -> SecurityLevel {
        self.roles.iter()
            .map(|role| role.max_security_level())
            .max()
            .unwrap_or(SecurityLevel::Open)
    }
}

/// Authorization manager
pub struct AuthorizationManager {
    /// User authorizations by email
    user_authorizations: HashMap<String, UserAuthorization>,
    /// Default policies for different resource types
    default_policies: HashMap<String, AuthorizationPolicy>,
}

impl AuthorizationManager {
    /// Create a new authorization manager
    pub fn new() -> Self {
        let mut manager = Self {
            user_authorizations: HashMap::new(),
            default_policies: HashMap::new(),
        };
        
        // Set up default policies
        manager.setup_default_policies();
        manager
    }
    
    /// Set up default authorization policies
    fn setup_default_policies(&mut self) {
        // Open content policy
        self.default_policies.insert(
            "open_content".to_string(),
            AuthorizationPolicy::new(SecurityLevel::Open)
                .require_permission(Permission::Read)
        );
        
        // Internal content policy
        self.default_policies.insert(
            "internal_content".to_string(),
            AuthorizationPolicy::new(SecurityLevel::Internal)
                .require_role(Role::Employee)
                .require_permission(Permission::Read)
        );
        
        // Client content policy
        self.default_policies.insert(
            "client_content".to_string(),
            AuthorizationPolicy::new(SecurityLevel::Client)
                .require_permission(Permission::Read)
        );
        
        // Compliance content policy
        self.default_policies.insert(
            "compliance_content".to_string(),
            AuthorizationPolicy::new(SecurityLevel::Compliance)
                .require_role(Role::ComplianceOfficer { standards: vec!["HIPAA".to_string(), "GDPR".to_string()] })
                .require_permission(Permission::Read)
        );
        
        // Security configuration policy
        self.default_policies.insert(
            "security_config".to_string(),
            AuthorizationPolicy::new(SecurityLevel::Internal)
                .require_role(Role::SecurityOfficer)
                .require_permission(Permission::SecurityConfig)
        );
        
        // Audit access policy
        self.default_policies.insert(
            "audit_access".to_string(),
            AuthorizationPolicy::new(SecurityLevel::Internal)
                .require_permission(Permission::Audit)
        );
    }
    
    /// Register a user authorization
    pub fn register_user(&mut self, user_auth: UserAuthorization) {
        self.user_authorizations.insert(user_auth.user_email.clone(), user_auth);
    }
    
    /// Get user authorization
    pub fn get_user_authorization(&self, user_email: &str) -> Option<&UserAuthorization> {
        self.user_authorizations.get(user_email)
    }
    
    /// Create default authorization for a user based on their authentication
    pub fn create_default_authorization(&self, auth: &AuthenticationTier) -> UserAuthorization {
        let user_email = auth.user_email().unwrap_or("unknown").to_string();
        let organization = user_email.split('@').nth(1).map(|s| s.to_string());
        
        let mut user_auth = UserAuthorization::new(user_email, organization.clone());
        
        // Assign default role based on authentication tier
        match auth {
            AuthenticationTier::None => {
                user_auth = user_auth
                    .add_role(Role::OpenSourceUser)
                    .add_permission(Permission::Read)
                    .add_environment(Environment::Open);
            }
            AuthenticationTier::BasicAuth { .. } => {
                user_auth = user_auth
                    .add_role(Role::Employee)
                    .add_permission(Permission::Read)
                    .add_permission(Permission::Write)
                    .add_permission(Permission::Execute)
                    .add_environment(Environment::Open);
                
                if let Some(org) = &organization {
                    user_auth = user_auth.add_environment(Environment::Internal {
                        organization_id: org.clone(),
                    });
                }
            }
            AuthenticationTier::EnhancedAuth { .. } => {
                user_auth = user_auth
                    .add_role(Role::Employee)
                    .add_role(Role::ProjectMember { project_ids: Vec::new() })
                    .add_permission(Permission::Read)
                    .add_permission(Permission::Write)
                    .add_permission(Permission::Execute)
                    .add_environment(Environment::Open);
                
                if let Some(org) = &organization {
                    user_auth = user_auth
                        .add_environment(Environment::Internal {
                            organization_id: org.clone(),
                        });
                }
            }
            AuthenticationTier::MilitaryAuth { .. } => {
                user_auth = user_auth
                    .add_role(Role::DefensePersonnel { clearance_level: "SECRET".to_string() })
                    .add_permission(Permission::Read)
                    .add_permission(Permission::Write)
                    .add_permission(Permission::Execute)
                    .add_permission(Permission::ClassifiedAccess)
                    .add_environment(Environment::Open);
                
                if let Some(org) = &organization {
                    user_auth = user_auth
                        .add_environment(Environment::Internal {
                            organization_id: org.clone(),
                        })
                        .add_environment(Environment::Defense {
                            organization_id: org.clone(),
                            classification_level: "SECRET".to_string(),
                            clearance_required: "SECRET".to_string(),
                        });
                }
            }
        }
        
        user_auth
    }
    
    /// Check if a security context is authorized for a resource
    pub fn is_authorized(
        &self,
        context: &SecurityContext,
        resource_type: &str,
        policy: Option<&AuthorizationPolicy>,
    ) -> Result<bool, WeaveMeshError> {
        // Get user authorization
        let user_email = context.authentication.user_email()
            .ok_or_else(|| WeaveMeshError::SecurityError("No user email in authentication".to_string()))?;
        
        let user_auth = self.get_user_authorization(user_email)
            .ok_or_else(|| WeaveMeshError::SecurityError("User not found in authorization system".to_string()))?;
        
        // Get policy (use provided or default)
        let policy = policy.or_else(|| self.default_policies.get(resource_type))
            .ok_or_else(|| WeaveMeshError::SecurityError(format!("No policy found for resource type: {}", resource_type)))?;
        
        // Check security level
        if !context.can_access_level(&policy.required_security_level) {
            return Ok(false);
        }
        
        // Check roles
        if !policy.required_roles.is_empty() {
            let has_required_role = policy.required_roles.iter()
                .any(|required_role| user_auth.has_role(required_role));
            
            if !has_required_role {
                return Ok(false);
            }
        }
        
        // Check permissions
        if !policy.required_permissions.is_empty() {
            let has_required_permissions = policy.required_permissions.iter()
                .all(|required_permission| user_auth.has_permission(required_permission));
            
            if !has_required_permissions {
                return Ok(false);
            }
        }
        
        // Check environment restrictions
        if !policy.environment_restrictions.is_empty() {
            let can_access_environment = policy.environment_restrictions.iter()
                .any(|env| user_auth.can_access_environment(env));
            
            if !can_access_environment {
                return Ok(false);
            }
        }
        
        // Check additional conditions
        for (key, expected_value) in &policy.conditions {
            let actual_value = context.metadata.get(key)
                .ok_or_else(|| WeaveMeshError::SecurityError(format!("Missing condition: {}", key)))?;
            
            if actual_value != expected_value {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    /// Get authorization summary for a user
    pub fn get_authorization_summary(&self, user_email: &str) -> Option<AuthorizationSummary> {
        let user_auth = self.get_user_authorization(user_email)?;
        
        Some(AuthorizationSummary {
            user_email: user_auth.user_email.clone(),
            organization: user_auth.organization.clone(),
            max_security_level: user_auth.max_security_level(),
            roles: user_auth.roles.clone(),
            permissions: user_auth.permissions.iter().cloned().collect(),
            accessible_environments: user_auth.accessible_environments.len(),
        })
    }
}

/// Authorization summary for display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationSummary {
    /// User's email
    pub user_email: String,
    /// User's organization
    pub organization: Option<String>,
    /// Maximum security level
    pub max_security_level: SecurityLevel,
    /// User's roles
    pub roles: Vec<Role>,
    /// User's permissions
    pub permissions: Vec<Permission>,
    /// Number of accessible environments
    pub accessible_environments: usize,
}

impl AuthorizationSummary {
    /// Get display string for the summary
    pub fn display_string(&self) -> String {
        let roles_str = self.roles.iter()
            .map(|r| r.display_name())
            .collect::<Vec<_>>()
            .join(", ");
        
        format!(
            "{} ({}): {} - {} roles, {} permissions",
            self.user_email,
            self.organization.as_deref().unwrap_or("No org"),
            match self.max_security_level {
                SecurityLevel::Open => "Open",
                SecurityLevel::Internal => "Internal",
                SecurityLevel::Client => "Client",
                SecurityLevel::Compliance => "Compliance",
                SecurityLevel::Classified => "Classified",
            },
            self.roles.len(),
            self.permissions.len()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::security::YubiKeyVerification;
    use chrono::{Duration, Utc};

    #[test]
    fn test_role_security_levels() {
        assert_eq!(Role::OpenSourceUser.max_security_level(), SecurityLevel::Open);
        assert_eq!(Role::Employee.max_security_level(), SecurityLevel::Internal);
        assert_eq!(Role::ProjectMember { project_ids: vec![] }.max_security_level(), SecurityLevel::Client);
    }

    #[test]
    fn test_user_authorization() {
        let user_auth = UserAuthorization::new("test@company.com".to_string(), Some("company.com".to_string()))
            .add_role(Role::Employee)
            .add_permission(Permission::Read)
            .add_permission(Permission::Write);
        
        assert!(user_auth.has_role(&Role::Employee));
        assert!(user_auth.has_permission(&Permission::Read));
        assert!(!user_auth.has_permission(&Permission::Manage));
    }

    #[test]
    fn test_authorization_manager() {
        let mut auth_manager = AuthorizationManager::new();
        
        let user_auth = UserAuthorization::new("test@company.com".to_string(), Some("company.com".to_string()))
            .add_role(Role::Employee)
            .add_permission(Permission::Read);
        
        auth_manager.register_user(user_auth);
        
        let summary = auth_manager.get_authorization_summary("test@company.com").unwrap();
        assert_eq!(summary.user_email, "test@company.com");
        assert_eq!(summary.max_security_level, SecurityLevel::Internal);
    }

    #[test]
    fn test_default_authorization_creation() {
        let auth_manager = AuthorizationManager::new();
        
        let basic_auth = AuthenticationTier::BasicAuth {
            oauth_token: "token".to_string(),
            user_email: "test@company.com".to_string(),
            expires_at: Utc::now() + Duration::hours(1),
        };
        
        let user_auth = auth_manager.create_default_authorization(&basic_auth);
        assert!(user_auth.has_role(&Role::Employee));
        assert!(user_auth.has_permission(&Permission::Read));
    }
}
