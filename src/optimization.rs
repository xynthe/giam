//! Optimization types for ULI level
//!
//! Provides types for global optimization

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::error::Result;

/// The optimization objective
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Objective {
    /// Minimize the target
    Minimize,
    /// Maximize the target
    Maximize,
}

/// The type of resource being optimized
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceType {
    /// Compute resources
    Compute,
    /// Memory resources
    Memory,
    /// Network resources
    Network,
    /// Time resources
    Time,
}

/// The result of an optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationResult {
    /// The improvement ratio
    pub improvement: f64,
    /// The efficiency ratio (actual / optimal)
    pub efficiency_ratio: f64,
    /// The optimized value
    pub optimized_value: f64,
    /// Number of iterations to converge
    pub iterations: u32,
}

impl OptimizationResult {
    /// Creates a new optimization result
    pub fn new(
        improvement: f64,
        efficiency_ratio: f64,
        optimized_value: f64,
        iterations: u32,
    ) -> Self {
        Self {
            improvement,
            efficiency_ratio: efficiency_ratio.clamp(0.0, 1.0),
            optimized_value,
            iterations,
        }
    }
}

/// Target for optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationTarget {
    /// The objective to optimize
    pub objective: Objective,
    /// The type of resource
    pub resource: ResourceType,
    /// The target value
    pub target_value: f64,
}

impl OptimizationTarget {
    /// Creates a new optimization target
    pub fn new(objective: Objective, resource: ResourceType, target_value: f64) -> Self {
        Self {
            objective,
            resource,
            target_value,
        }
    }
}

/// A resource allocation entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAllocation {
    pub resource_id: String,
    pub allocated_amount: f64,
    pub priority: u32,
}

/// Global optimizer for ULI level
pub struct GlobalOptimizer {
    allocations: HashMap<String, Vec<ResourceAllocation>>,
}

impl GlobalOptimizer {
    pub fn new() -> Self {
        Self {
            allocations: HashMap::new(),
        }
    }

    /// Optimize resource allocation for a target
    pub fn optimize(
        &mut self,
        target: &OptimizationTarget,
        available: f64,
    ) -> Result<OptimizationResult> {
        let optimal = match target.objective {
            Objective::Minimize => available * 0.8,
            Objective::Maximize => available * 1.2,
        };

        let improvement = (optimal - target.target_value) / target.target_value.abs();
        let efficiency = (optimal / available).clamp(0.0, 1.0);

        Ok(OptimizationResult::new(
            improvement.abs(),
            efficiency,
            optimal,
            10,
        ))
    }

    /// Allocate resources across multiple tasks
    pub fn allocate(&mut self, task_id: &str, allocations: Vec<ResourceAllocation>) {
        self.allocations.insert(task_id.to_string(), allocations);
    }

    /// Get current allocations for a task
    pub fn get_allocations(&self, task_id: &str) -> Option<&Vec<ResourceAllocation>> {
        self.allocations.get(task_id)
    }

    /// Rebalance allocations based on priority
    pub fn rebalance(&mut self) {
        let mut allocs: Vec<_> = self.allocations.values_mut().flatten().collect();
        allocs.sort_by(|a, b| b.priority.cmp(&a.priority));

        let total: f64 = allocs.iter().map(|a| a.allocated_amount).sum();
        let share = total / allocs.len() as f64;

        for alloc in allocs {
            alloc.allocated_amount = share;
        }
    }
}

impl Default for GlobalOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimization_result() {
        let result = OptimizationResult::new(0.15, 0.9, 100.0, 10);
        assert_eq!(result.iterations, 10);
        assert!((result.efficiency_ratio - 0.9).abs() < 0.001);
    }

    #[test]
    fn test_global_optimizer() {
        let mut optimizer = GlobalOptimizer::new();

        let target = OptimizationTarget::new(Objective::Minimize, ResourceType::Compute, 50.0);
        let result = optimizer.optimize(&target, 100.0);

        assert!(result.is_ok());
    }

    #[test]
    fn test_resource_allocation() {
        let mut optimizer = GlobalOptimizer::new();

        optimizer.allocate(
            "task1",
            vec![
                ResourceAllocation {
                    resource_id: "cpu".to_string(),
                    allocated_amount: 50.0,
                    priority: 1,
                },
                ResourceAllocation {
                    resource_id: "memory".to_string(),
                    allocated_amount: 100.0,
                    priority: 2,
                },
            ],
        );

        let allocs = optimizer.get_allocations("task1");
        assert!(allocs.is_some());
        assert_eq!(allocs.unwrap().len(), 2);
    }
}
