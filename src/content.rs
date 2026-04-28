//! Content types for GIAM
//!
//! Provides structured content types for representing various data formats

use serde::{Deserialize, Serialize};

/// Structured content with type discrimination
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum StructuredContent {
    /// Plain text content
    Text(String),
    /// JSON-structured content
    Json(serde_json::Value),
    /// Binary data
    Binary(Vec<u8>),
}

impl StructuredContent {
    /// Creates a new text content
    pub fn text<S: Into<String>>(s: S) -> Self {
        StructuredContent::Text(s.into())
    }

    /// Creates a new JSON content
    pub fn json(value: serde_json::Value) -> Self {
        StructuredContent::Json(value)
    }

    /// Creates a new binary content
    pub fn binary(data: Vec<u8>) -> Self {
        StructuredContent::Binary(data)
    }

    /// Returns the content as text, if applicable
    pub fn as_text(&self) -> Option<&str> {
        match self {
            StructuredContent::Text(s) => Some(s),
            _ => None,
        }
    }

    /// Returns the content as JSON value, if applicable
    pub fn as_json(&self) -> Option<&serde_json::Value> {
        match self {
            StructuredContent::Json(v) => Some(v),
            _ => None,
        }
    }

    /// Returns the content as binary, if applicable
    pub fn as_binary(&self) -> Option<&Vec<u8>> {
        match self {
            StructuredContent::Binary(b) => Some(b),
            _ => None,
        }
    }
}

impl From<String> for StructuredContent {
    fn from(s: String) -> Self {
        StructuredContent::Text(s)
    }
}

impl From<&str> for StructuredContent {
    fn from(s: &str) -> Self {
        StructuredContent::Text(s.to_string())
    }
}
