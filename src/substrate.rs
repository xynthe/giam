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
