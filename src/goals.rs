//! Goal tracking types for GIAM
//!
//! Provides types for tracking goals and their progress

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::intent::GoalStatus;
use crate::temporal::Timestamp;
use uuid::Uuid;

/// A milestone within a goal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    pub id: Uuid,
    pub description: String,
    pub completed: bool,
    pub target_progress: f64,
}

impl Milestone {
    pub fn new(description: String, target_progress: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            description,
            completed: false,
            target_progress: target_progress.clamp(0.0, 1.0),
        }
    }
}

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
    /// Sub-goals
    pub sub_goals: Vec<Uuid>,
    /// Milestones within this goal
    pub milestones: Vec<Milestone>,
    /// Parent goal ID (if this is a sub-goal)
    pub parent: Option<Uuid>,
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
            sub_goals: Vec::new(),
            milestones: Vec::new(),
            parent: None,
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
            sub_goals: Vec::new(),
            milestones: Vec::new(),
            parent: None,
        }
    }

    /// Creates a sub-goal
    pub fn sub_goal(description: String, parent_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            description,
            status: GoalStatus::Pending,
            progress: 0.0,
            deadline: None,
            dependencies: Vec::new(),
            priority: 0,
            sub_goals: Vec::new(),
            milestones: Vec::new(),
            parent: Some(parent_id),
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

    /// Adds a milestone
    pub fn add_milestone(&mut self, milestone: Milestone) {
        self.milestones.push(milestone);
    }

    /// Updates progress based on milestone completion
    pub fn update_from_milestones(&mut self) {
        if self.milestones.is_empty() {
            return;
        }
        let completed = self.milestones.iter().filter(|m| m.completed).count();
        self.progress = completed as f64 / self.milestones.len() as f64;
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
