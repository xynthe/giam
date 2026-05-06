//! Validation types for SI and UI levels
//!
//! Provides types for input and output validation

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::content::StructuredContent;
use crate::error::{GiamError, Result};

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

/// Schema for content validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentSchema {
    allowed_types: HashSet<String>,
    max_size: Option<usize>,
    required_fields: Vec<String>,
}

impl ContentSchema {
    pub fn new() -> Self {
        Self {
            allowed_types: HashSet::new(),
            max_size: None,
            required_fields: Vec::new(),
        }
    }

    pub fn with_allowed_type(mut self, type_name: impl Into<String>) -> Self {
        self.allowed_types.insert(type_name.into());
        self
    }

    pub fn with_max_size(mut self, size: usize) -> Self {
        self.max_size = Some(size);
        self
    }

    pub fn with_required_field(mut self, field: impl Into<String>) -> Self {
        self.required_fields.push(field.into());
        self
    }
}

impl Default for ContentSchema {
    fn default() -> Self {
        Self::new()
    }
}

/// Schema-based validator
pub struct SchemaValidator {
    input_schema: ContentSchema,
    output_schema: ContentSchema,
}

impl SchemaValidator {
    pub fn new(input_schema: ContentSchema, output_schema: ContentSchema) -> Self {
        Self {
            input_schema,
            output_schema,
        }
    }

    fn validate_content(&self, content: &StructuredContent, schema: &ContentSchema) -> Result<()> {
        let type_name = match content {
            StructuredContent::Text(_) => "Text",
            StructuredContent::Json(_) => "Json",
            StructuredContent::Binary(_) => "Binary",
        };

        if !schema.allowed_types.is_empty() && !schema.allowed_types.contains(type_name) {
            return Err(GiamError::ValidationFailed(format!(
                "Content type '{}' not in allowed types",
                type_name
            )));
        }

        if let Some(max_size) = schema.max_size {
            let size: usize = match content {
                StructuredContent::Text(s) => s.len(),
                StructuredContent::Json(v) => v.to_string().len(),
                StructuredContent::Binary(b) => b.len(),
            };
            if size > max_size {
                return Err(GiamError::ValidationFailed(format!(
                    "Content size {} exceeds max {}",
                    size, max_size
                )));
            }
        }

        Ok(())
    }
}

impl Validator for SchemaValidator {
    fn validate_input(&self, input: &StructuredContent) -> Result<()> {
        self.validate_content(input, &self.input_schema)
    }

    fn validate_output(&self, output: &StructuredContent) -> Result<()> {
        self.validate_content(output, &self.output_schema)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pass_through() {
        let validator = PassThroughValidator::new();
        assert!(validator
            .validate_input(&StructuredContent::text("test"))
            .is_ok());
    }

    #[test]
    fn test_schema_validator() {
        let input_schema = ContentSchema::new()
            .with_allowed_type("Text")
            .with_max_size(1000);

        let output_schema = ContentSchema::new().with_allowed_type("Json");

        let validator = SchemaValidator::new(input_schema, output_schema);

        assert!(validator
            .validate_input(&StructuredContent::text("short"))
            .is_ok());
        assert!(validator
            .validate_output(&StructuredContent::json(
                serde_json::json!({"key": "value"})
            ))
            .is_ok());
    }
}
