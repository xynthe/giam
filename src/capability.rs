//! Capability contract and interface for GIAM
//!
//! Defines how capabilities are represented and invoked

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::content::StructuredContent;
use crate::error::Result;

/// The capability contract defining preconditions, effects, and failure modes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityContract {
    /// Preconditions that must be satisfied before invocation
    pub preconditions: Vec<String>,
    /// Effects produced by successful invocation
    pub effects: Vec<String>,
    /// Possible failure modes and their conditions
    pub failure_modes: Vec<String>,
}

impl CapabilityContract {
    /// Creates a new empty capability contract
    pub fn new() -> Self {
        Self {
            preconditions: Vec::new(),
            effects: Vec::new(),
            failure_modes: Vec::new(),
        }
    }

    /// Creates a capability contract with the given components
    pub fn with(
        preconditions: Vec<String>,
        effects: Vec<String>,
        failure_modes: Vec<String>,
    ) -> Self {
        Self {
            preconditions,
            effects,
            failure_modes,
        }
    }
}

impl Default for CapabilityContract {
    fn default() -> Self {
        Self::new()
    }
}

/// The capability interface for implementing GIAM capabilities
#[async_trait]
pub trait Capability: Send + Sync {
    /// Returns the name of this capability
    fn name(&self) -> &str;

    /// Returns the contract for this capability
    fn contract(&self) -> &CapabilityContract;

    /// Invokes the capability with the given input
    fn invoke(&self, input: StructuredContent) -> Result<StructuredContent>;

    /// Async invocation of the capability
    async fn invoke_async(&self, input: StructuredContent) -> Result<StructuredContent>;
}