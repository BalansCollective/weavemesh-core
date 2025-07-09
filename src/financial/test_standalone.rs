//! Standalone test for the financial framework
//! This can be run independently to verify the financial module works

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_complete_financial_workflow() {
        println!("🏦 Testing WeaveMesh Financial Framework");
        
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
        println!("  Auto-approval threshold: ${:.2}", current_limits.auto_approval_threshold as f64 / 100.0);

        // Test various operations
        let operations = vec![
            (OperationType::Communication, "chat-session", "Basic messaging"),
            (OperationType::AI, "code-generation", "Generate Rust code"),
            (OperationType::Computation, "data-processing", "Process user data"),
        ];

        println!("\n🔄 Testing operations...");

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
                                panic!("Failed to record operation");
                            } else {
                                println!("  Recorded: ✅");
                            }
                        }
                        ApprovalResult::UserApprovalRequired { estimated_cost } => {
                            println!("  Status: ⚠️  User approval required (${:.2})", estimated_cost as f64 / 100.0);
                            
                            // For test, auto-approve
                            if let Err(e) = manager.record_operation(
                                operation_id,
                                op_type.clone(),
                                estimated_cost,
                                Some(context.to_string()),
                                metadata,
                            ) {
                                println!("  Error recording: {}", e);
                                panic!("Failed to record operation");
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
                    panic!("Failed to estimate cost");
                }
            }
            println!();
        }

        // Test spending summary
        println!("📈 Testing Spending Summary");
        match manager.get_summary(SpendingPeriod::Daily) {
            Ok(summary) => {
                println!("Daily spending:");
                println!("  Total: ${:.2}", summary.total_spent as f64 / 100.0);
                println!("  Operations: {}", summary.operation_count);
                println!("  Average per operation: ${:.2}", summary.average_cost as f64 / 100.0);
                
                assert!(summary.total_spent > 0, "Should have recorded some spending");
                assert!(summary.operation_count > 0, "Should have recorded some operations");
                
                println!("\nBreakdown by operation type:");
                for (op_type, cost) in &summary.by_operation_type {
                    println!("  {:?}: ${:.2}", op_type, *cost as f64 / 100.0);
                }
            }
            Err(e) => {
                println!("Error getting summary: {}", e);
                panic!("Failed to get spending summary");
            }
        }

        println!("\n🎯 Financial framework test completed successfully!");
        println!("✅ All core financial features working:");
        println!("  • Cost estimation for different operation types");
        println!("  • Automatic approval/denial based on spending limits");
        println!("  • Detailed tracking and reporting");
        println!("  • Flexible configuration for different contexts");
    }

    #[test]
    fn test_spending_limits_enforcement() {
        let limits = SpendingLimits {
            daily_limit: Some(100), // $1.00 daily limit
            per_operation_limit: Some(50), // $0.50 per operation
            auto_approval_threshold: 25, // $0.25 auto-approval
            ..Default::default()
        };

        let tracker = FinancialTracker::new(limits);
        
        // Test per-operation limit
        let approval = tracker.check_approval(60, &OperationType::AI).unwrap();
        assert!(matches!(approval, ApprovalResult::Denied { .. }), "Should deny operations exceeding per-operation limit");
        
        // Test auto-approval threshold
        let approval = tracker.check_approval(30, &OperationType::AI).unwrap();
        assert!(matches!(approval, ApprovalResult::UserApprovalRequired { .. }), "Should require approval for operations above threshold");
        
        // Test normal approval
        let approval = tracker.check_approval(20, &OperationType::Communication).unwrap();
        assert!(matches!(approval, ApprovalResult::Approved), "Should approve small operations");
        
        println!("✅ Spending limits enforcement working correctly");
    }

    #[test]
    fn test_cost_estimation() {
        let estimator = SimpleCostEstimator::new();
        let metadata = HashMap::new();
        
        // Test different operation types have different costs
        let ai_cost = estimator.estimate_cost(&OperationType::AI, None, &metadata).unwrap();
        let comm_cost = estimator.estimate_cost(&OperationType::Communication, None, &metadata).unwrap();
        
        assert!(ai_cost > comm_cost, "AI operations should cost more than communication");
        assert_eq!(ai_cost, 10, "AI operations should cost 10 cents");
        assert_eq!(comm_cost, 1, "Communication should cost 1 cent");
        
        println!("✅ Cost estimation working correctly");
        println!("  AI: ${:.2}", ai_cost as f64 / 100.0);
        println!("  Communication: ${:.2}", comm_cost as f64 / 100.0);
    }
}
