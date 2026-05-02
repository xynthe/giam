//! Planning types for SI and UI levels
//!
//! Provides types for execution planning

use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{GiamError, Result};

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

    /// Validates the execution plan for correctness
    pub fn validate(&self) -> Result<()> {
        if self.steps.is_empty() {
            return Err(GiamError::ValidationFailed("Plan has no steps".to_string()));
        }

        let step_ids: HashSet<usize> = self.steps.iter().map(|s| s.id).collect();
        if step_ids.len() != self.steps.len() {
            return Err(GiamError::ValidationFailed(
                "Plan has duplicate step IDs".to_string(),
            ));
        }

        for step in &self.steps {
            for dep in &step.depends_on {
                if !step_ids.contains(dep) {
                    return Err(GiamError::ValidationFailed(format!(
                        "Step {} depends on non-existent step {}",
                        step.id, dep
                    )));
                }
                if *dep >= step.id {
                    return Err(GiamError::ValidationFailed(format!(
                        "Step {} has circular or forward dependency on step {}",
                        step.id, dep
                    )));
                }
            }
        }

        Ok(())
    }

    /// Returns the steps in topological order (dependencies first)
    pub fn topological_order(&self) -> Vec<usize> {
        let mut result = Vec::new();
        let mut visited = HashSet::new();
        let mut in_progress = HashSet::new();

        fn visit(
            step_id: usize,
            steps: &[ExecutionStep],
            visited: &mut HashSet<usize>,
            in_progress: &mut HashSet<usize>,
            result: &mut Vec<usize>,
        ) {
            if visited.contains(&step_id) {
                return;
            }
            if in_progress.contains(&step_id) {
                return;
            }
            in_progress.insert(step_id);

            if let Some(step) = steps.get(step_id) {
                for &dep in &step.depends_on {
                    visit(dep, steps, visited, in_progress, result);
                }
            }

            in_progress.remove(&step_id);
            visited.insert(step_id);
            result.push(step_id);
        }

        for i in 0..self.steps.len() {
            visit(i, &self.steps, &mut visited, &mut in_progress, &mut result);
        }

        result
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
