//! Validation types for SI and UI levels
//!
//! Provides types for input and output validation

use crate::content::StructuredContent;
use crate::error::Result;

/// Validator trait for input and output validation
pub trait Validator: Send + Sync {
    /// Validates input content
    fn validate_input(&self, input: &StructuredContent) -> Result<()>;

    /// Validates output content
    fn validate_output(&self, output: &StructuredContent) -> Result<()>;
}

/// Basic validator that accepts any content
pub struct PassThroughValidator;

impl PassThroughValidator {
    /// Creates a new pass-through validator
    pub fn new() -> Self {
        Self
    }
}

impl Default for PassThroughValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl Validator for PassThroughValidator {
    fn validate_input(&self, _input: &StructuredContent) -> Result<()> {
        Ok(())
    }

    fn validate_output(&self, _output: &StructuredContent) -> Result<()> {
        Ok(())
    }
}
