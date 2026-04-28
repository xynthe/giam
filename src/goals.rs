//! Goal tracking types for GIAM
//!
//! Provides types for tracking goals and their progress

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
        }
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
