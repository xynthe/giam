//! Goal tracking types for GIAM
//!
//! Provides types for tracking goals and their progress

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::intent::GoalStatus;
use crate::temporal::Timestamp;
use uuid::Uuid;

/// A goal with tracking information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    /// Unique identifier
    pub id: Uuid,
    /// Human-readable description
    pub description: String,
    /// Current status
    pub status: GoalStatus,
    /// Progress from 0.0 to 1.0
    pub progress: f64,
    /// Optional deadline
    pub deadline: Option<Timestamp>,
    /// IDs of goals this goal depends on
    pub dependencies: Vec<Uuid>,
    /// Priority (higher values = higher priority)
    pub priority: u32,
}

impl Goal {
    /// Creates a new goal
    pub fn new(description: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            description,
            status: GoalStatus::Pending,
            progress: 0.0,
            deadline: None,
            dependencies: Vec::new(),
            priority: 0,
        }
    }

    /// Creates a goal with dependencies
    pub fn with_dependencies(description: String, dependencies: Vec<Uuid>) -> Self {
        Self {
            id: Uuid::new_v4(),
            description,
            status: GoalStatus::Pending,
            progress: 0.0,
            deadline: None,
            dependencies,
            priority: 0,
        }
    }

    /// Adds a dependency
    pub fn add_dependency(&mut self, goal_id: Uuid) {
        if !self.dependencies.contains(&goal_id) {
            self.dependencies.push(goal_id);
        }
    }

    /// Checks if all dependencies are satisfied
    pub fn dependencies_satisfied(&self, completed: &HashSet<Uuid>) -> bool {
        self.dependencies.iter().all(|id| completed.contains(id))
    }

    /// Updates the progress
    pub fn set_progress(&mut self, progress: f64) {
        self.progress = progress.clamp(0.0, 1.0);
        if self.progress > 0.0 && self.status == GoalStatus::Pending {
            self.status = GoalStatus::Active;
        }
    }

    /// Marks the goal as completed
    pub fn complete(&mut self) {
        self.progress = 1.0;
        self.status = GoalStatus::Completed;
    }

    /// Marks the goal as failed
    pub fn fail(&mut self) {
        self.status = GoalStatus::Failed;
    }
}
