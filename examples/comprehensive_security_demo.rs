//! Comprehensive Security Framework Demo
//! 
//! This example demonstrates the full security capabilities of WeaveMesh Core,
//! including authentication tiers, environment-based access control, and
//! LLM processing tier selection.

use weavemesh_core::{
    AuthenticationTier, SecurityContext, Environment,
    LLMTier, ComplianceStandard, WeaveMeshError,
};
use weavemesh_core::security::SecurityLevel;
use chrono::{Duration, Utc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔐 WeaveMesh Core - Comprehensive Security Framework Demo");
    println!("========================================================\n");

    // Demo 1: Authentication Tiers
    demo_authentication_tiers().await?;
    
    // Demo 2: Environment-Based Access Control
    demo_environment_access_control().await?;
    
    // Demo 3: LLM Processing Tiers
    demo_llm_processing_tiers().await?;
    
    // Demo 4: Compliance Standards
    demo_compliance_standards().await?;
    
    // Demo 5: Security Context Validation
    demo_security_context_validation().await?;

    println!("\n✅ All security demos completed successfully!");
    Ok(())
}

async fn demo_authentication_tiers() -> Result<(), WeaveMeshError> {
    println!("🔑 Demo 1: Authentication Tiers");
    println!("--------------------------------");

    // No authentication (open source)
    let none_auth = AuthenticationTier::None;
    println!("None Authentication:");
    println!("  Max Security Level: {:?}", none_auth.max_security_level());
    println!("  Can access Open: {}", none_auth.can_access_level(&SecurityLevel::Open));
    println!("  Can access Internal: {}", none_auth.can_access_level(&SecurityLevel::Internal));

    // Basic OAuth authentication
    let basic_auth = AuthenticationTier::BasicAuth {
        oauth_token: "oauth_token_123".to_string(),
        user_email: "user@company.com".to_string(),
        expires_at: Utc::now() + Duration::hours(1),
    };
    println!("\nBasic Authentication:");
    println!("  User: {}", basic_auth.user_email().unwrap_or("N/A"));
    println!("  Max Security Level: {:?}", basic_auth.max_security_level());
    println!("  Can access Internal: {}", basic_auth.can_access_level(&SecurityLevel::Internal));
    println!("  Can access Client: {}", basic_auth.can_access_level(&SecurityLevel::Client));

    // Enhanced authentication with YubiKey
    let yubikey_verification = weavemesh_core::security::YubiKeyVerification::new(
        true,
        "ccccccfhcjln".to_string(),
        Some(1),
        Some(1),
    );
    
    let enhanced_auth = AuthenticationTier::EnhancedAuth {
        oauth_token: "oauth_token_456".to_string(),
        user_email: "secure.user@company.com".to_string(),
        yubikey_verification,
        expires_at: Utc::now() + Duration::hours(1),
    };
    println!("\nEnhanced Authentication (with YubiKey):");
    println!("  User: {}", enhanced_auth.user_email().unwrap_or("N/A"));
    println!("  Max Security Level: {:?}", enhanced_auth.max_security_level());
    println!("  Has YubiKey: {}", enhanced_auth.has_yubikey());
    println!("  Can access Compliance: {}", enhanced_auth.can_access_level(&SecurityLevel::Compliance));

    println!();
    Ok(())
}

async fn demo_environment_access_control() -> Result<(), WeaveMeshError> {
    println!("🏢 Demo 2: Environment-Based Access Control");
    println!("--------------------------------------------");

    // Open environment
    let open_env = Environment::Open;
    println!("Open Environment:");
    println!("  Required Security Level: {:?}", open_env.required_security_level());

    // Internal company environment
    let internal_env = Environment::Internal {
        organization_id: "acme-corp".to_string(),
    };
    println!("\nInternal Environment:");
    println!("  Organization: acme-corp");
    println!("  Required Security Level: {:?}", internal_env.required_security_level());

    // Client-specific environment
    let client_env = Environment::Client {
        organization_id: "acme-corp".to_string(),
        client_id: "client-123".to_string(),
    };
    println!("\nClient Environment:");
    println!("  Organization: acme-corp");
    println!("  Client: client-123");
    println!("  Required Security Level: {:?}", client_env.required_security_level());

    // Medical compliance environment
    let medical_env = Environment::Medical {
        organization_id: "healthcare-corp".to_string(),
        compliance_standards: vec![
            ComplianceStandard::HIPAA,
            ComplianceStandard::GDPR,
        ],
    };
    println!("\nMedical Environment:");
    println!("  Organization: healthcare-corp");
    println!("  Compliance Standards: HIPAA, GDPR");
    println!("  Required Security Level: {:?}", medical_env.required_security_level());

    // Defense environment
    let defense_env = Environment::Defense {
        organization_id: "defense-contractor".to_string(),
        classification_level: "SECRET".to_string(),
        clearance_required: "SECRET".to_string(),
    };
    println!("\nDefense Environment:");
    println!("  Organization: defense-contractor");
    println!("  Classification: SECRET");
    println!("  Required Security Level: {:?}", defense_env.required_security_level());

    println!();
    Ok(())
}

async fn demo_llm_processing_tiers() -> Result<(), WeaveMeshError> {
    println!("🤖 Demo 3: LLM Processing Tiers");
    println!("--------------------------------");

    for security_level in [
        SecurityLevel::Open,
        SecurityLevel::Internal,
        SecurityLevel::Client,
        SecurityLevel::Compliance,
        SecurityLevel::Classified,
    ] {
        println!("\nSecurity Level: {:?}", security_level);
        
        let allowed_tiers = LLMTier::allowed_for_security_level(&security_level);
        println!("  Allowed LLM Tiers:");
        for tier in &allowed_tiers {
            println!("    - {:?}", tier);
        }
        
        let recommended_tier = LLMTier::recommended_for_security_level(&security_level);
        println!("  Recommended: {:?}", recommended_tier);
        
        // Explain the reasoning
        match security_level {
            SecurityLevel::Open => println!("  Rationale: Public content can use external LLMs"),
            SecurityLevel::Internal => println!("  Rationale: Company data requires on-premises processing"),
            SecurityLevel::Client => println!("  Rationale: Client data needs air-gapped processing"),
            SecurityLevel::Compliance => println!("  Rationale: Regulated data requires manual review only"),
            SecurityLevel::Classified => println!("  Rationale: Classified data requires manual review only"),
        }
    }

    println!();
    Ok(())
}

async fn demo_compliance_standards() -> Result<(), WeaveMeshError> {
    println!("📋 Demo 4: Compliance Standards");
    println!("--------------------------------");

    let standards = vec![
        ComplianceStandard::GDPR,
        ComplianceStandard::HIPAA,
        ComplianceStandard::SOX,
        ComplianceStandard::ITAR,
        ComplianceStandard::Custom("ISO-27001".to_string()),
    ];

    for standard in standards {
        println!("Compliance Standard: {}", standard);
        match standard {
            ComplianceStandard::GDPR => {
                println!("  Description: General Data Protection Regulation (EU)");
                println!("  Scope: Personal data processing");
                println!("  Requirements: Consent, data minimization, right to erasure");
            }
            ComplianceStandard::HIPAA => {
                println!("  Description: Health Insurance Portability and Accountability Act (US)");
                println!("  Scope: Protected health information");
                println!("  Requirements: Encryption, access controls, audit logs");
            }
            ComplianceStandard::SOX => {
                println!("  Description: Sarbanes-Oxley Act (US)");
                println!("  Scope: Financial reporting");
                println!("  Requirements: Internal controls, audit trails");
            }
            ComplianceStandard::ITAR => {
                println!("  Description: International Traffic in Arms Regulations (US)");
                println!("  Scope: Defense-related technology");
                println!("  Requirements: Export controls, access restrictions");
            }
            ComplianceStandard::Custom(name) => {
                println!("  Description: Custom compliance standard");
                println!("  Name: {}", name);
                println!("  Requirements: Organization-specific");
            }
        }
        println!();
    }

    Ok(())
}

async fn demo_security_context_validation() -> Result<(), WeaveMeshError> {
    println!("🛡️ Demo 5: Security Context Validation");
    println!("---------------------------------------");

    // Valid context: Basic auth for internal environment
    let basic_auth = AuthenticationTier::BasicAuth {
        oauth_token: "token".to_string(),
        user_email: "user@acme-corp.com".to_string(),
        expires_at: Utc::now() + Duration::hours(1),
    };
    
    let internal_env = Environment::Internal {
        organization_id: "acme-corp".to_string(),
    };
    
    let valid_context = SecurityContext::new(
        basic_auth,
        internal_env,
        Some("acme-corp".to_string()),
    );
    
    println!("Valid Security Context:");
    println!("  Authentication: BasicAuth");
    println!("  Environment: Internal");
    println!("  Organization: acme-corp");
    println!("  Can access Internal: {}", valid_context.can_access_level(&SecurityLevel::Internal));
    println!("  Can access Client: {}", valid_context.can_access_level(&SecurityLevel::Client));
    
    match valid_context.validate() {
        Ok(()) => println!("  Validation: ✅ PASSED"),
        Err(e) => println!("  Validation: ❌ FAILED - {}", e),
    }

    // Invalid context: No auth for compliance environment
    let no_auth = AuthenticationTier::None;
    let compliance_env = Environment::Medical {
        organization_id: "healthcare-corp".to_string(),
        compliance_standards: vec![ComplianceStandard::HIPAA],
    };
    
    let invalid_context = SecurityContext::new(
        no_auth,
        compliance_env,
        Some("healthcare-corp".to_string()),
    );
    
    println!("\nInvalid Security Context:");
    println!("  Authentication: None");
    println!("  Environment: Medical (HIPAA)");
    println!("  Organization: healthcare-corp");
    println!("  Can access Compliance: {}", invalid_context.can_access_level(&SecurityLevel::Compliance));
    
    match invalid_context.validate() {
        Ok(()) => println!("  Validation: ✅ PASSED"),
        Err(e) => println!("  Validation: ❌ FAILED - {}", e),
    }

    // Demonstrate LLM tier selection
    println!("\nLLM Tier Selection:");
    let allowed_tiers = valid_context.allowed_llm_tiers();
    println!("  Valid context allowed tiers: {:?}", allowed_tiers);
    println!("  Valid context recommended tier: {:?}", valid_context.recommended_llm_tier());
    
    let invalid_allowed_tiers = invalid_context.allowed_llm_tiers();
    println!("  Invalid context allowed tiers: {:?}", invalid_allowed_tiers);
    println!("  Invalid context recommended tier: {:?}", invalid_context.recommended_llm_tier());

    println!();
    Ok(())
}
