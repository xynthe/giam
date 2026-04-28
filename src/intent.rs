//! Intent types for GIAM
//!
//! Provides types for representing goals and intentions

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// An intent representing a goal or objective
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    /// Unique identifier for this intent
    pub id: Uuid,
    /// Human-readable description of the intent
    pub description: String,
    /// Priority level (higher = more important)
    pub priority: u32,
}

impl Intent {
    /// Creates a new intent
    pub fn new(description: String, priority: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            description,
            priority,
        }
    }

    /// Creates a new intent with a generated ID
    pub fn with_description(description: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            description: description.into(),
            priority: 0,
        }
    }
}

/// The status of a goal
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GoalStatus {
    /// Goal has not been started
    Pending,
    /// Goal is currently being pursued
    Active,
    /// Goal is blocked and cannot proceed
    Blocked,
    /// Goal has been completed
    Completed,
    /// Goal has failed
    Failed,
}

impl GoalStatus {
    /// Returns whether this is a terminal status
    pub fn is_terminal(&self) -> bool {
        matches!(self, GoalStatus::Completed | GoalStatus::Failed)
    }
}
