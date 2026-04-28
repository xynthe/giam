//! Optimization types for ULI level
//!
//! Provides types for global optimization

use serde::{Deserialize, Serialize};

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
}

impl OptimizationResult {
    /// Creates a new optimization result
    pub fn new(improvement: f64, efficiency_ratio: f64) -> Self {
        Self {
            improvement,
            efficiency_ratio: efficiency_ratio.clamp(0.0, 1.0),
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
