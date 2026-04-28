//! Planning types for SI and UI levels
//!
//! Provides types for execution planning

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A single step in an execution plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionStep {
    /// Step index
    pub id: usize,
    /// The capability to invoke
    pub capability: String,
    /// Dependencies on other steps (by index)
    pub depends_on: Vec<usize>,
}

/// A complete execution plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPlan {
    /// Unique identifier
    pub id: Uuid,
    /// The steps in this plan
    pub steps: Vec<ExecutionStep>,
}

impl ExecutionPlan {
    /// Creates a new execution plan
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            steps: Vec::new(),
        }
    }

    /// Adds a step to the plan
    pub fn add_step(&mut self, capability: String, depends_on: Vec<usize>) {
        let id = self.steps.len();
        self.steps.push(ExecutionStep {
            id,
            capability,
            depends_on,
        });
    }
}

impl Default for ExecutionPlan {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of executing a plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    /// Whether execution was successful
    pub success: bool,
    /// The output from execution
    pub output: crate::content::StructuredContent,
    /// Error message if failed
    pub error: Option<String>,
}

impl ExecutionResult {
    /// Creates a successful result
    pub fn success(output: crate::content::StructuredContent) -> Self {
        Self {
            success: true,
            output,
            error: None,
        }
    }

    /// Creates a failed result
    pub fn failure(error: String) -> Self {
        Self {
            success: false,
            output: crate::content::StructuredContent::Text(String::new()),
            error: Some(error),
        }
    }
}
