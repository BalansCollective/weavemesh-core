//! Simple demonstration of the financial framework core logic
//! This version uses only standard library to show the framework works

use std::collections::HashMap;

// Simple timestamp for demo (using seconds since epoch)
type Timestamp = u64;

fn current_timestamp() -> Timestamp {
    // For demo purposes, use a fixed timestamp
    1704067200 // 2024-01-01 00:00:00 UTC
}

// Simplified financial framework types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OperationType {
    Communication,
    Computation,
    Storage,
    Network,
    AI,
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct CostRecord {
    pub operation_id: String,
    pub timestamp: Timestamp,
    pub cost: u64,
    pub currency: String,
    pub operation_type: OperationType,
    pub context: Option<String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct SpendingLimits {
    pub daily_limit: Option<u64>,
    pub weekly_limit: Option<u64>,
    pub monthly_limit: Option<u64>,
    pub per_operation_limit: Option<u64>,
    pub currency: String,
    pub auto_approval_threshold: u64,
}

impl Default for SpendingLimits {
    fn default() -> Self {
        Self {
            daily_limit: Some(1000),
            weekly_limit: Some(5000),
            monthly_limit: Some(20000),
            per_operation_limit: Some(100),
            currency: "USD".to_string(),
            auto_approval_threshold: 50,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ApprovalResult {
    Approved,
    Denied { reason: String },
    UserApprovalRequired { estimated_cost: u64 },
}

pub struct FinancialTracker {
    costs: Vec<CostRecord>,
    limits: SpendingLimits,
    max_records: usize,
}

impl FinancialTracker {
    pub fn new(limits: SpendingLimits) -> Self {
        Self {
            costs: Vec::new(),
            limits,
            max_records: 10000,
        }
    }
    
    pub fn with_defaults() -> Self {
        Self::new(SpendingLimits::default())
    }
    
    pub fn record_cost(&mut self, record: CostRecord) -> Result<(), String> {
        self.costs.push(record);
        
        if self.costs.len() > self.max_records {
            self.costs.remove(0);
        }
        
        Ok(())
    }
    
    pub fn check_approval(
        &self,
        estimated_cost: u64,
        _operation_type: &OperationType,
    ) -> Result<ApprovalResult, String> {
        if let Some(limit) = self.limits.per_operation_limit {
            if estimated_cost > limit {
                return Ok(ApprovalResult::Denied {
                    reason: format!("Cost {} exceeds per-operation limit {}", estimated_cost, limit),
                });
            }
        }
        
        if let Some(daily_limit) = self.limits.daily_limit {
            let daily_spent = self.get_daily_spending()?;
            if daily_spent + estimated_cost > daily_limit {
                return Ok(ApprovalResult::Denied {
                    reason: format!("Would exceed daily limit: {} + {} > {}", daily_spent, estimated_cost, daily_limit),
                });
            }
        }
        
        if estimated_cost > self.limits.auto_approval_threshold {
            return Ok(ApprovalResult::UserApprovalRequired { estimated_cost });
        }
        
        Ok(ApprovalResult::Approved)
    }
    
    pub fn get_daily_spending(&self) -> Result<u64, String> {
        // For demo, just sum all costs (simplified)
        let total = self.costs
            .iter()
            .map(|record| record.cost)
            .sum();
        
        Ok(total)
    }
    
    pub fn get_limits(&self) -> &SpendingLimits {
        &self.limits
    }
    
    pub fn record_count(&self) -> usize {
        self.costs.len()
    }
    
    pub fn get_summary(&self) -> (u64, u32, HashMap<OperationType, u64>) {
        let total_spent: u64 = self.costs.iter().map(|r| r.cost).sum();
        let operation_count = self.costs.len() as u32;
        
        let mut by_operation_type: HashMap<OperationType, u64> = HashMap::new();
        for record in &self.costs {
            *by_operation_type.entry(record.operation_type.clone()).or_insert(0) += record.cost;
        }
        
        (total_spent, operation_count, by_operation_type)
    }
}

pub trait CostEstimator {
    fn estimate_cost(
        &self,
        operation_type: &OperationType,
        _context: Option<&str>,
        _metadata: &HashMap<String, String>,
    ) -> Result<u64, String>;
}

pub struct SimpleCostEstimator {
    rates: HashMap<OperationType, u64>,
    default_rate: u64,
}

impl SimpleCostEstimator {
    pub fn new() -> Self {
        let mut rates = HashMap::new();
        rates.insert(OperationType::Communication, 1);
        rates.insert(OperationType::Computation, 5);
        rates.insert(OperationType::Storage, 1);
        rates.insert(OperationType::Network, 1);
        rates.insert(OperationType::AI, 10);
        
        Self {
            rates,
            default_rate: 2,
        }
    }
}

impl CostEstimator for SimpleCostEstimator {
    fn estimate_cost(
        &self,
        operation_type: &OperationType,
        _context: Option<&str>,
        _metadata: &HashMap<String, String>,
    ) -> Result<u64, String> {
        let cost = self.rates.get(operation_type).unwrap_or(&self.default_rate);
        Ok(*cost)
    }
}

pub struct FinancialManager {
    tracker: FinancialTracker,
    estimator: Box<dyn CostEstimator>,
}

impl FinancialManager {
    pub fn new(limits: SpendingLimits, estimator: Box<dyn CostEstimator>) -> Self {
        Self {
            tracker: FinancialTracker::new(limits),
            estimator,
        }
    }
    
    pub fn with_defaults() -> Self {
        Self::new(
            SpendingLimits::default(),
            Box::new(SimpleCostEstimator::new()),
        )
    }
    
    pub fn estimate_and_check(
        &self,
        operation_type: &OperationType,
        context: Option<&str>,
        metadata: &HashMap<String, String>,
    ) -> Result<(u64, ApprovalResult), String> {
        let estimated_cost = self.estimator.estimate_cost(operation_type, context, metadata)?;
        let approval = self.tracker.check_approval(estimated_cost, operation_type)?;
        Ok((estimated_cost, approval))
    }
    
    pub fn record_operation(
        &mut self,
        operation_id: String,
        operation_type: OperationType,
        actual_cost: u64,
        context: Option<String>,
        metadata: HashMap<String, String>,
    ) -> Result<(), String> {
        let record = CostRecord {
            operation_id,
            timestamp: current_timestamp(),
            cost: actual_cost,
            currency: self.tracker.limits.currency.clone(),
            operation_type,
            context,
            metadata,
        };
        
        self.tracker.record_cost(record)
    }
    
    pub fn get_summary(&self) -> (u64, u32, HashMap<OperationType, u64>) {
        self.tracker.get_summary()
    }
    
    pub fn get_limits(&self) -> &SpendingLimits {
        self.tracker.get_limits()
    }
}

fn main() -> Result<(), String> {
    println!("🏦 WeaveMesh Financial Framework Simple Demo");
    println!("==========================================\n");

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

    let (total_spent, operation_count, by_operation_type) = manager.get_summary();
    
    println!("Total spending:");
    println!("  Total: ${:.2}", total_spent as f64 / 100.0);
    println!("  Operations: {}", operation_count);
    if operation_count > 0 {
        println!("  Average per operation: ${:.2}", (total_spent / operation_count as u64) as f64 / 100.0);
    }
    
    println!("\nBreakdown by operation type:");
    for (op_type, cost) in &by_operation_type {
        println!("  {:?}: ${:.2}", op_type, *cost as f64 / 100.0);
    }

    println!("\n🎯 Demo completed successfully!");
    println!("The financial framework provides:");
    println!("  • Cost estimation for different operation types");
    println!("  • Automatic approval/denial based on spending limits");
    println!("  • Detailed tracking and reporting");
    println!("  • Flexible configuration for different contexts");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_financial_tracker_creation() {
        let tracker = FinancialTracker::with_defaults();
        assert_eq!(tracker.record_count(), 0);
    }

    #[test]
    fn test_cost_recording() {
        let mut tracker = FinancialTracker::with_defaults();
        
        let record = CostRecord {
            operation_id: "test-op".to_string(),
            timestamp: current_timestamp(),
            cost: 50,
            currency: "USD".to_string(),
            operation_type: OperationType::Communication,
            context: Some("test".to_string()),
            metadata: HashMap::new(),
        };
        
        assert!(tracker.record_cost(record).is_ok());
        assert_eq!(tracker.record_count(), 1);
    }

    #[test]
    fn test_spending_limits() {
        let tracker = FinancialTracker::with_defaults();
        
        // Should approve small operations
        let approval = tracker.check_approval(10, &OperationType::Communication).unwrap();
        assert!(matches!(approval, ApprovalResult::Approved));
        
        // Should require approval for larger operations
        let approval = tracker.check_approval(100, &OperationType::AI).unwrap();
        assert!(matches!(approval, ApprovalResult::UserApprovalRequired { .. }));
        
        // Should deny operations exceeding per-operation limit
        let approval = tracker.check_approval(200, &OperationType::AI).unwrap();
        assert!(matches!(approval, ApprovalResult::Denied { .. }));
    }

    #[test]
    fn test_cost_estimation() {
        let estimator = SimpleCostEstimator::new();
        let metadata = HashMap::new();
        
        let cost = estimator.estimate_cost(&OperationType::AI, None, &metadata).unwrap();
        assert_eq!(cost, 10);
        
        let cost = estimator.estimate_cost(&OperationType::Communication, None, &metadata).unwrap();
        assert_eq!(cost, 1);
    }

    #[test]
    fn test_financial_manager() {
        let mut manager = FinancialManager::with_defaults();
        let metadata = HashMap::new();
        
        let (cost, approval) = manager.estimate_and_check(
            &OperationType::Communication,
            Some("test"),
            &metadata,
        ).unwrap();
        
        assert_eq!(cost, 1);
        assert!(matches!(approval, ApprovalResult::Approved));
        
        // Record the operation
        assert!(manager.record_operation(
            "test-op".to_string(),
            OperationType::Communication,
            cost,
            Some("test".to_string()),
            metadata,
        ).is_ok());
        
        // Check summary
        let (total_spent, operation_count, _) = manager.get_summary();
        assert_eq!(total_spent, 1);
        assert_eq!(operation_count, 1);
    }
}
