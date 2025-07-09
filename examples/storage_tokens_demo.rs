//! Storage and Tokens Demo
//!
//! This example demonstrates the new storage and token modules working together
//! to store attribution data and calculate token allocations.

use weavemesh_core::{
    Attribution, CollaborationType, MemoryStorage, Storage, SimpleTokenPolicy, TokenPolicy,
    serialize_json, deserialize_json, StorageAccessControl,
};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔧 WeaveMesh Core - Storage and Tokens Demo");
    println!("============================================");

    // Create storage and token policy
    let mut storage = MemoryStorage::new();
    let token_policy = SimpleTokenPolicy::new(
        "Demo Policy".to_string(),
        "1.0".to_string(),
        "A demonstration token policy".to_string(),
        100.0, // 100 tokens per contribution
    );

    println!("\n📊 Creating sample attribution data...");
    
    // Create some sample attributions
    let attributions = vec![
        Attribution::new(
            Some("alice".to_string()),
            Some("ai-assistant".to_string()),
            CollaborationType::CoCreated,
            0.9,
        ),
        Attribution::new(
            Some("bob".to_string()),
            Some("ai-assistant".to_string()),
            CollaborationType::HumanLed,
            0.8,
        ),
        Attribution::new(
            Some("charlie".to_string()),
            None,
            CollaborationType::Individual,
            1.0,
        ),
    ];

    println!("✅ Created {} attribution records", attributions.len());

    // Serialize and store the attribution data
    println!("\n💾 Storing attribution data...");
    let serialized_data = serialize_json(&attributions)?;
    
    let resource_id = storage.store_resource(
        "attribution_data.json".to_string(),
        serialized_data.into_bytes(),
        "application/json".to_string(),
        StorageAccessControl::default(),
        vec!["attribution".to_string(), "demo".to_string()],
    ).await?;

    println!("✅ Stored attribution data with ID: {}", resource_id);

    // Retrieve and deserialize the data
    println!("\n📥 Retrieving attribution data...");
    let retrieved_data = storage.get_resource_content(&resource_id).await?;
    let data_str = String::from_utf8(retrieved_data)?;
    let retrieved_attributions: Vec<Attribution> = deserialize_json(&data_str)?;
    
    println!("✅ Retrieved {} attribution records", retrieved_attributions.len());

    // Calculate token allocations
    println!("\n🪙 Calculating token allocations...");
    let allocation = token_policy.calculate_tokens(&retrieved_attributions)?;
    
    println!("✅ Token allocation completed!");
    println!("   Policy: {}", token_policy.get_policy_name());
    println!("   Total tokens allocated: {}", allocation.metadata.total_allocated);
    println!("   Events processed: {}", allocation.metadata.events_processed);

    // Display allocations
    println!("\n💰 Token Allocations:");
    for (contributor, amount) in &allocation.allocations {
        println!("   {} → {} tokens", contributor, amount);
    }

    // Display reasoning
    println!("\n🧠 Allocation Reasoning:");
    for reason in &allocation.reasoning {
        println!("   {}: {}", reason.contributor, reason.explanation);
        println!("      Confidence: {:.1}%", reason.confidence * 100.0);
    }

    // Store the token allocation
    println!("\n💾 Storing token allocation...");
    let allocation_data = serialize_json(&allocation)?;
    let allocation_id = storage.store_resource(
        "token_allocation.json".to_string(),
        allocation_data.into_bytes(),
        "application/json".to_string(),
        StorageAccessControl::default(),
        vec!["tokens".to_string(), "allocation".to_string(), "demo".to_string()],
    ).await?;

    println!("✅ Stored token allocation with ID: {}", allocation_id);

    // Show storage statistics
    println!("\n📈 Storage Statistics:");
    let stats = storage.get_stats();
    println!("   Total resources: {}", stats.total_resources);
    println!("   Total size: {} bytes", stats.total_size);

    // List all resources
    println!("\n📋 All Stored Resources:");
    let resources = storage.list_resources(None);
    for resource in resources {
        println!("   {} ({})", resource.name, resource.content_type);
        println!("      ID: {}", resource.resource_id);
        println!("      Size: {} bytes", resource.size);
        println!("      Tags: {:?}", resource.tags);
    }

    println!("\n🎉 Demo completed successfully!");
    Ok(())
}
