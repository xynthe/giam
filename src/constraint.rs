//! Constraint types for GIAM
//!
//! Provides types for defining system constraints

use serde::{Deserialize, Serialize};

use crate::content::StructuredContent;
use uuid::Uuid;

/// Types of constraints that can be applied
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConstraintType {
    /// Maximum resource limits
    ResourceLimit,
    /// Budget constraints
    BudgetLimit,
    /// Time constraints
    TimeLimit,
    /// Ethical boundaries
    EthicalBoundary,
    /// Actions requiring approval
    ApprovalRequired,
}

/// A token representing a constraint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintToken {
    /// Unique identifier
    pub id: Uuid,
    /// The type of constraint
    pub constraint_type: ConstraintType,
    /// The constraint payload
    pub payload: StructuredContent,
}

impl ConstraintToken {
    /// Creates a new constraint token
    pub fn new(constraint_type: ConstraintType, payload: StructuredContent) -> Self {
        Self {
            id: Uuid::new_v4(),
            constraint_type,
            payload,
        }
    }
}
