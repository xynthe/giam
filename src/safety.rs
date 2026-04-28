//! Safety policy types for GIAM
//!
//! Provides types for safety boundaries

use serde::{Deserialize, Serialize};

/// An action that can be controlled by safety policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    /// Unique identifier
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Whether this action is reversible
    pub reversible: bool,
}

impl Action {
    /// Creates a new action
    pub fn new(id: String, name: String, reversible: bool) -> Self {
        Self {
            id,
            name,
            reversible,
        }
    }
}

/// Safety policy controlling allowed actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyPolicy {
    /// Actions that are explicitly allowed
    pub allowed_actions: Vec<Action>,
    /// Actions that are blocked
    pub blocked_actions: Vec<Action>,
    /// Actions requiring human approval
    pub approval_required: Vec<Action>,
}

impl SafetyPolicy {
    /// Creates a new safety policy
    pub fn new() -> Self {
        Self {
            allowed_actions: Vec::new(),
            blocked_actions: Vec::new(),
            approval_required: Vec::new(),
        }
    }

    /// Checks if an action is allowed
    pub fn is_allowed(&self, action_id: &str) -> bool {
        self.allowed_actions.iter().any(|a| &a.id == action_id)
    }

    /// Checks if an action requires approval
    pub fn requires_approval(&self, action_id: &str) -> bool {
        self.approval_required.iter().any(|a| &a.id == action_id)
    }
}

impl Default for SafetyPolicy {
    fn default() -> Self {
        Self::new()
    }
}
