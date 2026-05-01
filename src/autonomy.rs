//! Autonomy boundary types for GIAM
//!
//! Provides types for defining autonomy boundaries at each level

use serde::{Deserialize, Serialize};

/// The autonomy boundary defining what the system can do without human approval
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AutonomyBoundary {
    /// No autonomy - all actions require human approval
    Unbounded,
    /// Review required before execution
    PreExecutionReview,
    /// Actions within bounds allowed, approval required for others
    #[default]
    BoundedWithApproval,
    /// Read-only mode - no actions allowed
    ReadOnly,
}

impl AutonomyBoundary {
    /// Returns the default autonomy boundary for a given level
    pub fn for_level(tier: u8) -> Self {
        match tier {
            0 => Self::ReadOnly,            // AGI
            1 => Self::BoundedWithApproval, // SI
            2 => Self::PreExecutionReview,  // UI
            _ => Self::Unbounded,           // HI and above
        }
    }

    /// Returns whether actions can be taken at this boundary without approval
    pub fn can_act(&self) -> bool {
        matches!(self, Self::Unbounded)
    }

    /// Returns whether review is required before execution
    pub fn requires_review(&self) -> bool {
        matches!(self, Self::PreExecutionReview)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autonomy_boundary_for_level() {
        assert_eq!(AutonomyBoundary::for_level(0), AutonomyBoundary::ReadOnly);
        assert_eq!(
            AutonomyBoundary::for_level(1),
            AutonomyBoundary::BoundedWithApproval
        );
        assert_eq!(
            AutonomyBoundary::for_level(2),
            AutonomyBoundary::PreExecutionReview
        );
        assert_eq!(AutonomyBoundary::for_level(3), AutonomyBoundary::Unbounded);
    }

    #[test]
    fn test_autonomy_can_act() {
        assert!(!AutonomyBoundary::ReadOnly.can_act());
        assert!(!AutonomyBoundary::BoundedWithApproval.can_act());
        assert!(!AutonomyBoundary::PreExecutionReview.can_act());
        assert!(AutonomyBoundary::Unbounded.can_act());
    }
}
