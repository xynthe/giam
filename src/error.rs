//! Error types for GIAM
//!
//! Provides the central error type and Result alias for the GIAM library

use thiserror::Error;

/// The central error type for GIAM operations
#[derive(Error, Debug)]
pub enum GiamError {
    /// Precondition for operation was not met
    #[error("Precondition not met: {0}")]
    PreconditionNotMet(String),

    /// Execution of an operation failed
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),

    /// Operation violates the current GIAM level constraints
    #[error("Level violation: {0}")]
    LevelViolation(String),

    /// Operation violates a defined constraint
    #[error("Constraint violated: {0}")]
    ConstraintViolated(String),

    /// Invalid state transition attempted
    #[error("Invalid state transition: {0}")]
    InvalidStateTransition(String),

    /// Routing decision failed
    #[error("Routing failed: {0}")]
    RoutingFailed(String),

    /// Validation of input or output failed
    #[error("Validation failed: {0}")]
    ValidationFailed(String),

    /// Autonomy boundary violation
    #[error("Autonomy boundary violated: {0}")]
    AutonomyBoundaryViolated(String),
}

/// Result type alias using GiamError
pub type Result<T> = std::result::Result<T, GiamError>;
