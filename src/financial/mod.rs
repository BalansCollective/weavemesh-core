//! Universal Financial Framework for WeaveMesh Core
//! 
//! Provides basic financial tracking and cost management primitives
//! that can be used across all contexts while allowing context-specific
//! financial implementations to build on top.

use crate::WeaveMeshError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Universal cost tracking for operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostRecord {
    /// Unique operation identifier
    pub operation_id: String,
    /// Timestamp of the operation
    pub timestamp: DateTime<Utc>,
    /// Cost in base units (e.g., USD cents, tokens, etc.)
    pub cost: u64,
    /// Currency or unit type
    pub currency: String,
    /// Operation type
    pub operation_type: OperationType,
    /// Context where the operation occurred
    pub context: Option<String>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Types of operations for cost tracking
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OperationType {
    /// Communication/messaging operations
    Communication,
    /// Computation/processing operations
    Computation,
    /// Storage operations
    Storage,
    /// Network operations
    Network,
    /// AI/LLM operations
    AI,
    /// Custom operation type
    Custom(String),
}

/// Universal spending limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpendingLimits {
    /// Daily spending limit
    pub daily_limit: Option<u64>,
    /// Weekly spending limit
    pub weekly_limit: Option<u64>,
    /// Monthly spending limit
    pub monthly_limit: Option<u64>,
    /// Per-operation limit
    pub per_operation_limit: Option<u64>,
    /// Currency for limits
    pub currency: String,
    /// Auto-approval threshold
    pub auto_approval_threshold: u64,
}

impl Default for SpendingLimits {
    fn default() -> Self {
        Self {
            daily_limit: Some(1000), // 10.00 in cents
            weekly_limit: Some(5000), // 50.00 in cents
            monthly_limit: Some(20000), // 200.00 in cents
            per_operation_limit: Some(100), // 1.00 in cents
            currency: "USD".to_string(),
            auto_approval_threshold: 50, // 0.50 in cents
        }
    }
}

/// Spending period for analysis
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SpendingPeriod {
    /// Last 24 hours
    Daily,
    /// Last 7 days
    Weekly,
    /// Last 30 days
    Monthly,
    /// Current session
    Session,
    /// All time
    Total,
}

/// Spending summary for a period
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpendingSummary {
    /// Total spent in the period
    pub total_spent: u64,
    /// Number of operations
    pub operation_count: u32,
    /// Average cost per operation
    pub average_cost: u64,
    /// Breakdown by operation type
    pub by_operation_type: HashMap<OperationType, u64>,
    /// Breakdown by context
    pub by_context: HashMap<String, u64>,
    /// Period analyzed
    pub period: SpendingPeriod,
    /// Currency
    pub currency: String,
    /// Period start and end
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
}

/// Cost approval result
#[derive(Debug, Clone)]
pub enum ApprovalResult {
    /// Operation approved automatically
    Approved,
    /// Operation denied due to limits
    Denied { reason: String },
    /// User approval required
    UserApprovalRequired { estimated_cost: u64 },
}

/// Universal financial tracker
pub struct FinancialTracker {
    /// Recorded costs
    costs: Vec<CostRecord>,
    /// Spending limits
    limits: SpendingLimits,
    /// Maximum records to keep in memory
    max_records: usize,
}

impl FinancialTracker {
    /// Create a new financial tracker
    pub fn new(limits: SpendingLimits) -> Self {
        Self {
            costs: Vec::new(),
            limits,
            max_records: 10000,
        }
    }
    
    /// Create a tracker with default limits
    pub fn with_defaults() -> Self {
        Self::new(SpendingLimits::default())
    }
    
    /// Record a cost
    pub fn record_cost(&mut self, record: CostRecord) -> Result<(), WeaveMeshError> {
        self.costs.push(record);
        
        // Keep only the most recent records
        if self.costs.len() > self.max_records {
            self.costs.remove(0);
        }
        
        Ok(())
    }
    
    /// Check if an operation is approved within spending limits
    pub fn check_approval(
        &self,
        estimated_cost: u64,
        operation_type: &OperationType,
    ) -> Result<ApprovalResult, WeaveMeshError> {
        // Check per-operation limit
        if let Some(limit) = self.limits.per_operation_limit {
            if estimated_cost > limit {
                return Ok(ApprovalResult::Denied {
                    reason: format!("Cost {} exceeds per-operation limit {}", estimated_cost, limit),
                });
            }
        }
        
        // Check daily limit
        if let Some(daily_limit) = self.limits.daily_limit {
            let daily_spent = self.get_spending_for_period(SpendingPeriod::Daily)?;
            if daily_spent + estimated_cost > daily_limit {
                return Ok(ApprovalResult::Denied {
                    reason: format!("Would exceed daily limit: {} + {} > {}", daily_spent, estimated_cost, daily_limit),
                });
            }
        }
        
        // Check if user approval is required
        if estimated_cost > self.limits.auto_approval_threshold {
            return Ok(ApprovalResult::UserApprovalRequired { estimated_cost });
        }
        
        Ok(ApprovalResult::Approved)
    }
    
    /// Get total spending for a period
    pub fn get_spending_for_period(&self, period: SpendingPeriod) -> Result<u64, WeaveMeshError> {
        let now = Utc::now();
        let cutoff = match period {
            SpendingPeriod::Daily => now - chrono::Duration::days(1),
            SpendingPeriod::Weekly => now - chrono::Duration::weeks(1),
            SpendingPeriod::Monthly => now - chrono::Duration::days(30),
            SpendingPeriod::Session => {
                // For session, we'll use the last hour as a simple approximation
                now - chrono::Duration::hours(1)
            }
            SpendingPeriod::Total => DateTime::<Utc>::MIN_UTC,
        };
        
        let total = self.costs
            .iter()
            .filter(|record| record.timestamp >= cutoff)
            .map(|record| record.cost)
            .sum();
        
        Ok(total)
    }
    
    /// Get detailed spending summary for a period
    pub fn get_spending_summary(&self, period: SpendingPeriod) -> Result<SpendingSummary, WeaveMeshError> {
        let now = Utc::now();
        let (cutoff, period_start) = match period {
            SpendingPeriod::Daily => (now - chrono::Duration::days(1), now - chrono::Duration::days(1)),
            SpendingPeriod::Weekly => (now - chrono::Duration::weeks(1), now - chrono::Duration::weeks(1)),
            SpendingPeriod::Monthly => (now - chrono::Duration::days(30), now - chrono::Duration::days(30)),
            SpendingPeriod::Session => (now - chrono::Duration::hours(1), now - chrono::Duration::hours(1)),
            SpendingPeriod::Total => (DateTime::<Utc>::MIN_UTC, DateTime::<Utc>::MIN_UTC),
        };
        
        let relevant_costs: Vec<&CostRecord> = self.costs
            .iter()
            .filter(|record| record.timestamp >= cutoff)
            .collect();
        
        let total_spent: u64 = relevant_costs.iter().map(|r| r.cost).sum();
        let operation_count = relevant_costs.len() as u32;
        let average_cost = if operation_count > 0 { total_spent / operation_count as u64 } else { 0 };
        
        let mut by_operation_type: HashMap<OperationType, u64> = HashMap::new();
        let mut by_context: HashMap<String, u64> = HashMap::new();
        
        for record in &relevant_costs {
            *by_operation_type.entry(record.operation_type.clone()).or_insert(0) += record.cost;
            
            if let Some(context) = &record.context {
                *by_context.entry(context.clone()).or_insert(0) += record.cost;
            }
        }
        
        Ok(SpendingSummary {
            total_spent,
            operation_count,
            average_cost,
            by_operation_type,
            by_context,
            period,
            currency: self.limits.currency.clone(),
            period_start,
            period_end: now,
        })
    }
    
    /// Update spending limits
    pub fn update_limits(&mut self, limits: SpendingLimits) {
        self.limits = limits;
    }
    
    /// Get current spending limits
    pub fn get_limits(&self) -> &SpendingLimits {
        &self.limits
    }
    
    /// Get recent cost records
    pub fn get_recent_costs(&self, limit: usize) -> Vec<&CostRecord> {
        self.costs.iter().rev().take(limit).collect()
    }
    
    /// Clear all cost records
    pub fn clear_records(&mut self) {
        self.costs.clear();
    }
    
    /// Get total number of records
    pub fn record_count(&self) -> usize {
        self.costs.len()
    }
}

/// Cost estimation interface
pub trait CostEstimator {
    /// Estimate cost for an operation
    fn estimate_cost(
        &self,
        operation_type: &OperationType,
        context: Option<&str>,
        metadata: &HashMap<String, String>,
    ) -> Result<u64, WeaveMeshError>;
}

/// Simple cost estimator with fixed rates
pub struct SimpleCostEstimator {
    /// Cost per operation type
    rates: HashMap<OperationType, u64>,
    /// Default rate for unknown operations
    default_rate: u64,
}

impl SimpleCostEstimator {
    /// Create a new simple cost estimator
    pub fn new() -> Self {
        let mut rates = HashMap::new();
        rates.insert(OperationType::Communication, 1); // 0.01 USD
        rates.insert(OperationType::Computation, 5); // 0.05 USD
        rates.insert(OperationType::Storage, 1); // 0.01 USD
        rates.insert(OperationType::Network, 1); // 0.01 USD
        rates.insert(OperationType::AI, 10); // 0.10 USD
        
        Self {
            rates,
            default_rate: 2, // 0.02 USD
        }
    }
    
    /// Set rate for an operation type
    pub fn set_rate(&mut self, operation_type: OperationType, rate: u64) {
        self.rates.insert(operation_type, rate);
    }
}

impl Default for SimpleCostEstimator {
    fn default() -> Self {
        Self::new()
    }
}

impl CostEstimator for SimpleCostEstimator {
    fn estimate_cost(
        &self,
        operation_type: &OperationType,
        _context: Option<&str>,
        _metadata: &HashMap<String, String>,
    ) -> Result<u64, WeaveMeshError> {
        let cost = self.rates.get(operation_type).unwrap_or(&self.default_rate);
        Ok(*cost)
    }
}

/// Financial manager combining tracking and estimation
pub struct FinancialManager {
    tracker: FinancialTracker,
    estimator: Box<dyn CostEstimator + Send + Sync>,
}

impl FinancialManager {
    /// Create a new financial manager
    pub fn new(limits: SpendingLimits, estimator: Box<dyn CostEstimator + Send + Sync>) -> Self {
        Self {
            tracker: FinancialTracker::new(limits),
            estimator,
        }
    }
    
    /// Create a manager with defaults
    pub fn with_defaults() -> Self {
        Self::new(
            SpendingLimits::default(),
            Box::new(SimpleCostEstimator::default()),
        )
    }
    
    /// Estimate and check approval for an operation
    pub fn estimate_and_check(
        &self,
        operation_type: &OperationType,
        context: Option<&str>,
        metadata: &HashMap<String, String>,
    ) -> Result<(u64, ApprovalResult), WeaveMeshError> {
        let estimated_cost = self.estimator.estimate_cost(operation_type, context, metadata)?;
        let approval = self.tracker.check_approval(estimated_cost, operation_type)?;
        Ok((estimated_cost, approval))
    }
    
    /// Record a completed operation
    pub fn record_operation(
        &mut self,
        operation_id: String,
        operation_type: OperationType,
        actual_cost: u64,
        context: Option<String>,
        metadata: HashMap<String, String>,
    ) -> Result<(), WeaveMeshError> {
        let record = CostRecord {
            operation_id,
            timestamp: Utc::now(),
            cost: actual_cost,
            currency: self.tracker.limits.currency.clone(),
            operation_type,
            context,
            metadata,
        };
        
        self.tracker.record_cost(record)
    }
    
    /// Get spending summary
    pub fn get_summary(&self, period: SpendingPeriod) -> Result<SpendingSummary, WeaveMeshError> {
        self.tracker.get_spending_summary(period)
    }
    
    /// Update spending limits
    pub fn update_limits(&mut self, limits: SpendingLimits) {
        self.tracker.update_limits(limits);
    }
    
    /// Get current limits
    pub fn get_limits(&self) -> &SpendingLimits {
        self.tracker.get_limits()
    }
}

mod test_standalone;

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
            timestamp: Utc::now(),
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
        let summary = manager.get_summary(SpendingPeriod::Daily).unwrap();
        assert_eq!(summary.total_spent, 1);
        assert_eq!(summary.operation_count, 1);
    }
}
