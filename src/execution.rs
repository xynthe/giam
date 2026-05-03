//! Execution engine for GIAM
//!
//! Provides an engine for executing plans using capabilities

use serde::{Deserialize, Serialize};

use crate::capability_registry::CapabilityRegistry;
use crate::content::StructuredContent;
use crate::error::Result;
use crate::planning::{ExecutionPlan, ExecutionResult, ExecutionStep};

/// Configuration for execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionConfig {
    pub max_parallel_steps: usize,
    pub timeout_seconds: u64,
    pub continue_on_error: bool,
}

impl Default for ExecutionConfig {
    fn default() -> Self {
        Self {
            max_parallel_steps: 10,
            timeout_seconds: 300,
            continue_on_error: false,
        }
    }
}

/// Execution engine for running plans
pub struct ExecutionEngine {
    registry: CapabilityRegistry,
    config: ExecutionConfig,
}

impl ExecutionEngine {
    /// Creates a new execution engine
    pub fn new(registry: CapabilityRegistry) -> Self {
        Self {
            registry,
            config: ExecutionConfig::default(),
        }
    }

    /// Creates a new execution engine with config
    pub fn with_config(registry: CapabilityRegistry, config: ExecutionConfig) -> Self {
        Self { registry, config }
    }

    /// Executes a plan
    pub fn execute(&self, plan: &ExecutionPlan) -> Result<ExecutionResult> {
        let order = plan.topological_order();

        let mut outputs: Vec<StructuredContent> = Vec::new();

        for step_idx in order {
            let step = &plan.steps[step_idx];

            let input = if step.depends_on.is_empty() {
                StructuredContent::text("default input".to_string())
            } else {
                let dep_outputs: Vec<_> = step
                    .depends_on
                    .iter()
                    .filter(|&&i| i < outputs.len())
                    .map(|&i| outputs[i].clone())
                    .collect();

                if dep_outputs.is_empty() {
                    StructuredContent::text("default input".to_string())
                } else if dep_outputs.len() == 1 {
                    dep_outputs[0].clone()
                } else {
                    StructuredContent::text(format!("combined {} inputs", dep_outputs.len()))
                }
            };

            match self.registry.invoke(&step.capability, input) {
                Ok(output) => outputs.push(output),
                Err(e) if self.config.continue_on_error => {
                    outputs.push(StructuredContent::text(format!("error: {}", e)));
                }
                Err(e) => {
                    return Ok(ExecutionResult::failure(format!(
                        "Step {} failed: {}",
                        step_idx, e
                    )));
                }
            }
        }

        let final_output = outputs
            .last()
            .cloned()
            .unwrap_or_else(|| StructuredContent::text("no output".to_string()));

        Ok(ExecutionResult::success(final_output))
    }

    /// Executes a single step
    pub fn execute_step(
        &self,
        step: &ExecutionStep,
        input: StructuredContent,
    ) -> Result<StructuredContent> {
        self.registry.invoke(&step.capability, input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capability::{Capability, CapabilityContract};

    #[test]
    fn test_execution_engine() {
        let mut registry = CapabilityRegistry::new();

        let cap = crate::capability_registry::FunctionCapability::new(
            "echo".to_string(),
            CapabilityContract::new(),
            |input| Ok(input),
        );

        registry.register(std::sync::Arc::new(cap));

        let engine = ExecutionEngine::new(registry);
        let mut plan = ExecutionPlan::new();
        plan.add_step("echo".to_string(), vec![]);

        let result = engine.execute(&plan);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execution_with_dependencies() {
        let mut registry = CapabilityRegistry::new();

        let cap = crate::capability_registry::FunctionCapability::new(
            "double".to_string(),
            CapabilityContract::new(),
            |input| {
                if let Some(text) = input.as_text() {
                    Ok(StructuredContent::text(format!("{} {}", text, text)))
                } else {
                    Ok(input)
                }
            },
        );

        registry.register(std::sync::Arc::new(cap));

        let engine = ExecutionEngine::new(registry);
        let mut plan = ExecutionPlan::new();
        plan.add_step("double".to_string(), vec![]);
        plan.add_step("double".to_string(), vec![0]);

        let result = engine.execute(&plan);
        assert!(result.is_ok());
    }
}
