//! Financial Framework Demo
//! 
//! Demonstrates the basic financial tracking and cost management
//! capabilities of WeaveMesh Core.

use std::collections::HashMap;
use weavemesh_core::{
    FinancialManager, OperationType, SpendingLimits, SpendingPeriod,
    ApprovalResult, SimpleCostEstimator,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🏦 WeaveMesh Financial Framework Demo");
    println!("=====================================\n");

    // Create a financial manager with custom limits
    let limits = SpendingLimits {
        daily_limit: Some(500), // $5.00 daily limit
        weekly_limit: Some(2000), // $20.00 weekly limit
        monthly_limit: Some(8000), // $80.00 monthly limit
        per_operation_limit: Some(200), // $2.00 per operation
        currency: "USD".to_string(),
        auto_approval_threshold: 25, // $0.25 auto-approval
    };

    let mut manager = FinancialManager::new(
        limits,
        Box::new(SimpleCostEstimator::new()),
    );

    println!("📊 Current spending limits:");
    let current_limits = manager.get_limits();
    println!("  Daily: ${:.2}", current_limits.daily_limit.unwrap_or(0) as f64 / 100.0);
    println!("  Weekly: ${:.2}", current_limits.weekly_limit.unwrap_or(0) as f64 / 100.0);
    println!("  Monthly: ${:.2}", current_limits.monthly_limit.unwrap_or(0) as f64 / 100.0);
    println!("  Per-operation: ${:.2}", current_limits.per_operation_limit.unwrap_or(0) as f64 / 100.0);
    println!("  Auto-approval threshold: ${:.2}\n", current_limits.auto_approval_threshold as f64 / 100.0);

    // Simulate various operations
    let operations = vec![
        (OperationType::Communication, "chat-session", "Basic messaging"),
        (OperationType::AI, "code-generation", "Generate Rust code"),
        (OperationType::Computation, "data-processing", "Process user data"),
        (OperationType::Storage, "file-backup", "Backup project files"),
        (OperationType::Network, "mesh-sync", "Synchronize with mesh"),
    ];

    println!("🔄 Simulating operations...\n");

    for (i, (op_type, context, description)) in operations.iter().enumerate() {
        let operation_id = format!("op-{}", i + 1);
        let metadata = HashMap::new();

        // Estimate cost and check approval
        match manager.estimate_and_check(op_type, Some(context), &metadata) {
            Ok((estimated_cost, approval)) => {
                println!("Operation: {}", description);
                println!("  Type: {:?}", op_type);
                println!("  Estimated cost: ${:.2}", estimated_cost as f64 / 100.0);
                
                match approval {
                    ApprovalResult::Approved => {
                        println!("  Status: ✅ Approved");
                        
                        // Record the operation
                        if let Err(e) = manager.record_operation(
                            operation_id,
                            op_type.clone(),
                            estimated_cost,
                            Some(context.to_string()),
                            metadata,
                        ) {
                            println!("  Error recording: {}", e);
                        } else {
                            println!("  Recorded: ✅");
                        }
                    }
                    ApprovalResult::UserApprovalRequired { estimated_cost } => {
                        println!("  Status: ⚠️  User approval required (${:.2})", estimated_cost as f64 / 100.0);
                        
                        // For demo, auto-approve
                        if let Err(e) = manager.record_operation(
                            operation_id,
                            op_type.clone(),
                            estimated_cost,
                            Some(context.to_string()),
                            metadata,
                        ) {
                            println!("  Error recording: {}", e);
                        } else {
                            println!("  Auto-approved and recorded: ✅");
                        }
                    }
                    ApprovalResult::Denied { reason } => {
                        println!("  Status: ❌ Denied - {}", reason);
                    }
                }
            }
            Err(e) => {
                println!("Operation: {}", description);
                println!("  Error: {}", e);
            }
        }
        println!();
    }

    // Show spending summary
    println!("📈 Spending Summary");
    println!("==================");

    match manager.get_summary(SpendingPeriod::Daily) {
        Ok(summary) => {
            println!("Daily spending:");
            println!("  Total: ${:.2}", summary.total_spent as f64 / 100.0);
            println!("  Operations: {}", summary.operation_count);
            println!("  Average per operation: ${:.2}", summary.average_cost as f64 / 100.0);
            
            println!("\nBreakdown by operation type:");
            for (op_type, cost) in &summary.by_operation_type {
                println!("  {:?}: ${:.2}", op_type, *cost as f64 / 100.0);
            }
            
            if !summary.by_context.is_empty() {
                println!("\nBreakdown by context:");
                for (context, cost) in &summary.by_context {
                    println!("  {}: ${:.2}", context, *cost as f64 / 100.0);
                }
            }
        }
        Err(e) => {
            println!("Error getting summary: {}", e);
        }
    }

    println!("\n🎯 Demo completed successfully!");
    println!("The financial framework provides:");
    println!("  • Cost estimation for different operation types");
    println!("  • Automatic approval/denial based on spending limits");
    println!("  • Detailed tracking and reporting");
    println!("  • Flexible configuration for different contexts");

    Ok(())
}
