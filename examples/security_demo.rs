//! Security Framework Demonstration
//! 
//! This example demonstrates the universal security primitives in WeaveMesh Core,
//! showing how authentication, authorization, and auditing work together to provide
//! secure communication across different contexts.

use weavemesh_core::{
    AuthenticationLevel, SecurityContext,
    SecurityValidator, SecurityEvent, SecurityEventType, SecurityResult, 
    MemorySecurityAuditor, SecurityAuditor,
    WeaveMeshBuilder,
};
use weavemesh_core::security::SecurityLevel;
use std::collections::HashMap;
use chrono::{Duration, Utc};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🔐 WeaveMesh Core Security Framework Demo");
    println!("==========================================\n");

    // 1. Demonstrate Security Levels
    demonstrate_security_levels();
    
    // 2. Demonstrate Authentication
    demonstrate_authentication();
    
    // 3. Demonstrate Access Control
    demonstrate_access_control();
    
    // 4. Demonstrate Security Auditing
    demonstrate_security_auditing();
    
    // 5. Demonstrate Integration with WeaveMesh
    demonstrate_weavemesh_integration().await?;

    println!("\n✅ Security framework demonstration completed successfully!");
    Ok(())
}

fn demonstrate_security_levels() {
    println!("1. Security Level Hierarchy");
    println!("---------------------------");
    
    let levels = vec![
        SecurityLevel::Open,
        SecurityLevel::Internal,
        SecurityLevel::Client,
        SecurityLevel::Compliance,
        SecurityLevel::Classified,
    ];
    
    for (i, level) in levels.iter().enumerate() {
        println!("Level {}: {:?} (strength: {})", i, level, level.required_auth_strength());
    }
    
    // Demonstrate access hierarchy
    println!("\nAccess Hierarchy:");
    println!("- Classified can access Open: {}", SecurityLevel::Classified.can_access(&SecurityLevel::Open));
    println!("- Open can access Classified: {}", SecurityLevel::Open.can_access(&SecurityLevel::Classified));
    println!("- Client can access Internal: {}", SecurityLevel::Client.can_access(&SecurityLevel::Internal));
    println!();
}

fn demonstrate_authentication() {
    println!("2. Authentication Levels");
    println!("-----------------------");
    
    let future_time = Utc::now() + Duration::hours(1);
    
    // Create different authentication levels
    let basic_auth = AuthenticationLevel::Basic {
        identity: "alice@example.com".to_string(),
        expires_at: future_time,
    };
    
    let enhanced_auth = AuthenticationLevel::Enhanced {
        identity: "bob@example.com".to_string(),
        factors: vec!["password".to_string(), "yubikey".to_string()],
        expires_at: future_time,
    };
    
    let maximum_auth = AuthenticationLevel::Maximum {
        identity: "charlie@example.com".to_string(),
        factors: vec!["password".to_string(), "yubikey".to_string(), "biometric".to_string()],
        additional_verification: {
            let mut map = HashMap::new();
            map.insert("clearance_level".to_string(), "top_secret".to_string());
            map
        },
        expires_at: future_time,
    };
    
    let auths = vec![
        ("Basic", &basic_auth),
        ("Enhanced", &enhanced_auth),
        ("Maximum", &maximum_auth),
    ];
    
    for (name, auth) in auths {
        println!("{} Authentication:", name);
        println!("  - Identity: {:?}", auth.identity());
        println!("  - Strength: {}", auth.strength());
        println!("  - Can access Client: {}", auth.can_access_level(&SecurityLevel::Client));
        println!("  - Can access Compliance: {}", auth.can_access_level(&SecurityLevel::Compliance));
        println!("  - Valid: {}", auth.is_valid());
        println!();
    }
}

fn demonstrate_access_control() {
    println!("3. Access Control");
    println!("----------------");
    
    let validator = SecurityValidator::new(SecurityLevel::Internal);
    
    // Create different security contexts
    let future_time = Utc::now() + Duration::hours(1);
    
    let basic_auth = AuthenticationLevel::Basic {
        identity: "user@example.com".to_string(),
        expires_at: future_time,
    };
    
    let contexts = vec![
        ("Open Context", SecurityContext::open()),
        ("Basic Auth Context", SecurityContext::new(basic_auth, SecurityLevel::Internal)),
    ];
    
    for (context_name, context) in &contexts {
        println!("{} Context:", context_name);
        println!("  - Is authorized: {}", context.is_authorized());
        println!("  - Can validate: {}", context.validate().is_ok());
        
        // Test some operations
        let operations = vec!["read_data", "write_data", "admin_access"];
        for operation in operations {
            let allowed = validator.is_allowed(context, operation);
            println!("  - Operation '{}': {}", operation, if allowed { "✅ Allowed" } else { "❌ Denied" });
        }
        println!();
    }
}

fn demonstrate_security_auditing() {
    println!("4. Security Auditing");
    println!("-------------------");
    
    let mut auditor = MemorySecurityAuditor::new(100);
    
    // Record various security events
    let events = vec![
        SecurityEvent::new(
            SecurityEventType::Authentication,
            SecurityResult::Allowed,
        ).with_identity("alice@example.com".to_string())
         .with_action("login".to_string()),
        
        SecurityEvent::new(
            SecurityEventType::Authorization,
            SecurityResult::Allowed,
        ).with_identity("alice@example.com".to_string())
         .with_resource("sensitive-document".to_string())
         .with_action("read".to_string()),
        
        SecurityEvent::new(
            SecurityEventType::Authentication,
            SecurityResult::Denied { reason: "Invalid password".to_string() },
        ).with_identity("bob@example.com".to_string())
         .with_action("login".to_string()),
        
        SecurityEvent::new(
            SecurityEventType::Permission,
            SecurityResult::Allowed,
        ).with_identity("alice@example.com".to_string())
         .with_resource("admin-panel".to_string())
         .with_action("access".to_string()),
    ];
    
    for event in events {
        auditor.log_event(event);
    }
    
    // Query audit trail
    println!("Recent Events:");
    for event in auditor.get_recent_events(10) {
        println!("  - {:?} - {:?} - {:?}", event.event_type, event.identity, event.result);
    }
    
    println!("\nEvents for alice@example.com:");
    for event in auditor.get_events_for_identity("alice@example.com") {
        println!("  - {:?} on {:?}", event.action, event.resource);
    }
    println!();
}

async fn demonstrate_weavemesh_integration() -> anyhow::Result<()> {
    println!("5. WeaveMesh Integration");
    println!("-----------------------");
    
    // Create a WeaveMesh instance with security
    let protocol = WeaveMeshBuilder::new()
        .add_capability("security-demo".to_string())
        .with_heartbeat(false) // Disable for demo
        .build()
        .await?;
    
    println!("✅ WeaveMesh protocol created with security framework");
    
    // Create a security context for operations
    let future_time = Utc::now() + Duration::hours(1);
    
    let auth = AuthenticationLevel::Enhanced {
        identity: "demo-user@example.com".to_string(),
        factors: vec!["password".to_string(), "yubikey".to_string()],
        expires_at: future_time,
    };
    
    let security_context = SecurityContext::new(auth, SecurityLevel::Client)
        .with_metadata("context".to_string(), "demo".to_string())
        .with_metadata("session".to_string(), "security-demo".to_string());
    
    println!("✅ Security context created:");
    println!("   - Required Level: {:?}", security_context.required_level);
    println!("   - Is authorized: {}", security_context.is_authorized());
    println!("   - Context: {:?}", security_context.get_metadata("context"));
    
    // Validate the security context
    match security_context.validate() {
        Ok(()) => println!("✅ Security context validation passed"),
        Err(e) => println!("❌ Security context validation failed: {}", e),
    }
    
    // Demonstrate secure message publishing with context
    let mut message_metadata = HashMap::new();
    message_metadata.insert("security_level".to_string(), "client".to_string());
    message_metadata.insert("authenticated_user".to_string(), "demo-user@example.com".to_string());
    
    protocol.publish_message(
        "secure-channel",
        "security-demo-node".to_string(),
        "This is a secure message with authentication context".to_string(),
        message_metadata,
    ).await?;
    
    println!("✅ Secure message published with authentication context");
    
    Ok(())
}
