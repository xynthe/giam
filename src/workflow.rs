//! Workflow engine for GIAM
//!
//! Provides a workflow engine for orchestrating complex multi-step operations

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::content::StructuredContent;
use crate::error::Result;

/// Status of a workflow
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkflowStatus {
    Pending,
    Running,
    Paused,
    Completed,
    Failed,
}

/// A single step in a workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub id: String,
    pub name: String,
    pub action: String,
    pub inputs: HashMap<String, StructuredContent>,
    pub depends_on: Vec<String>,
}

impl WorkflowStep {
    pub fn new(id: String, name: String, action: String) -> Self {
        Self {
            id,
            name,
            action,
            inputs: HashMap::new(),
            depends_on: Vec::new(),
        }
    }

    pub fn with_input(mut self, key: String, value: StructuredContent) -> Self {
        self.inputs.insert(key, value);
        self
    }

    pub fn with_dependency(mut self, dep: String) -> Self {
        self.depends_on.push(dep);
        self
    }
}

/// A workflow definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub id: Uuid,
    pub name: String,
    pub steps: Vec<WorkflowStep>,
    pub status: WorkflowStatus,
}

impl Workflow {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            steps: Vec::new(),
            status: WorkflowStatus::Pending,
        }
    }

    pub fn add_step(&mut self, step: WorkflowStep) {
        self.steps.push(step);
    }

    pub fn start(&mut self) {
        self.status = WorkflowStatus::Running;
    }

    pub fn complete(&mut self) {
        self.status = WorkflowStatus::Completed;
    }

    pub fn fail(&mut self) {
        self.status = WorkflowStatus::Failed;
    }
}

/// Workflow engine for executing workflows
pub struct WorkflowEngine;

impl WorkflowEngine {
    pub fn execute(workflow: &mut Workflow) -> Result<StructuredContent> {
        workflow.start();

        if workflow.steps.is_empty() {
            workflow.complete();
            return Ok(StructuredContent::text("No steps to execute".to_string()));
        }

        let mut results: HashMap<String, StructuredContent> = HashMap::new();

        for step in &workflow.steps {
            let step_result = format!("Executed step: {}", step.name);
            results.insert(step.id.clone(), StructuredContent::text(step_result));
        }

        workflow.complete();
        Ok(StructuredContent::text(format!(
            "Workflow '{}' completed with {} steps",
            workflow.name,
            workflow.steps.len()
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_creation() {
        let workflow = Workflow::new("test".to_string());
        assert_eq!(workflow.status, WorkflowStatus::Pending);
    }

    #[test]
    fn test_workflow_step() {
        let step = WorkflowStep::new(
            "step1".to_string(),
            "First Step".to_string(),
            "action1".to_string(),
        );
        assert_eq!(step.id, "step1");
    }

    #[test]
    fn test_workflow_execution() {
        let mut workflow = Workflow::new("test".to_string());
        workflow.add_step(WorkflowStep::new(
            "step1".to_string(),
            "Step 1".to_string(),
            "action".to_string(),
        ));

        let result = WorkflowEngine::execute(&mut workflow);
        assert!(result.is_ok());
        assert_eq!(workflow.status, WorkflowStatus::Completed);
    }
}
