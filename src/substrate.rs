//! Substrate abstraction types for TI level
//!
//! Provides types for substrate-independent execution

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Migration policy for moving between substrates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MigrationPolicy {
    /// Never migrate
    Never,
    /// Migrate when beneficial
    WhenBeneficial,
    /// Always migrate to best available
    AlwaysBest,
    /// Migrate on substrate failure
    Failover,
}

/// A task that can span multiple substrates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossSubstrateTask {
    /// Unique identifier
    pub id: Uuid,
    /// Available backend identifiers
    pub backends: Vec<String>,
    /// How to handle migrations
    pub migration_policy: MigrationPolicy,
}

impl CrossSubstrateTask {
    /// Creates a new cross-substrate task
    pub fn new(backends: Vec<String>, migration_policy: MigrationPolicy) -> Self {
        Self {
            id: Uuid::new_v4(),
            backends,
            migration_policy,
        }
    }
}

/// Resource usage statistics for a substrate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    /// CPU usage percentage
    pub cpu: f64,
    /// Memory usage in bytes
    pub memory: u64,
    /// Network I/O in bytes
    pub network: u64,
}

impl ResourceUsage {
    /// Creates new resource usage stats
    pub fn new(cpu: f64, memory: u64, network: u64) -> Self {
        Self {
            cpu: cpu.clamp(0.0, 100.0),
            memory,
            network,
        }
    }
}

/// Placeholder for substrate backend
pub trait SubstrateBackend: Send + Sync {
    /// Executes a plan and returns the result
    fn execute(
        &self,
        plan: &crate::planning::ExecutionPlan,
    ) -> Result<crate::planning::ExecutionResult, crate::error::GiamError>;

    /// Returns current resource usage
    fn resources(&self) -> ResourceUsage;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migration_policy() {
        let policy = MigrationPolicy::WhenBeneficial;
        assert_eq!(policy, MigrationPolicy::WhenBeneficial);
    }

    #[test]
    fn test_cross_substrate_task() {
        let task = CrossSubstrateTask::new(
            vec!["backend1".to_string(), "backend2".to_string()],
            MigrationPolicy::Failover,
        );

        assert_eq!(task.backends.len(), 2);
        assert_eq!(task.migration_policy, MigrationPolicy::Failover);
    }

    #[test]
    fn test_resource_usage() {
        let usage = ResourceUsage::new(50.0, 1024, 2048);

        assert_eq!(usage.cpu, 50.0);
        assert_eq!(usage.memory, 1024);
        assert_eq!(usage.network, 2048);
    }

    #[test]
    fn test_resource_usage_clamping() {
        let usage = ResourceUsage::new(150.0, 1024, 2048);
        assert_eq!(usage.cpu, 100.0);

        let usage = ResourceUsage::new(-10.0, 1024, 2048);
        assert_eq!(usage.cpu, 0.0);
    }
}
