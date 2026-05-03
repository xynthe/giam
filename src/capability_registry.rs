//! Capability registry for GIAM
//!
//! Provides a registry for managing capabilities across the system

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;

use crate::capability::{Capability, CapabilityContract};
use crate::error::{GiamError, Result};
use crate::content::StructuredContent;

/// A registry for managing capabilities
pub struct CapabilityRegistry {
    capabilities: HashMap<String, Arc<dyn Capability>>,
}

impl CapabilityRegistry {
    /// Creates a new empty capability registry
    pub fn new() -> Self {
        Self {
            capabilities: HashMap::new(),
        }
    }

    /// Registers a capability
    pub fn register(&mut self, capability: Arc<dyn Capability>) {
        self.capabilities.insert(capability.name().to_string(), capability);
    }

    /// Gets a capability by name
    pub fn get(&self, name: &str) -> Option<Arc<dyn Capability>> {
        self.capabilities.get(name).cloned()
    }

    /// Checks if a capability exists
    pub fn has(&self, name: &str) -> bool {
        self.capabilities.contains_key(name)
    }

    /// Lists all available capability names
    pub fn list(&self) -> Vec<String> {
        self.capabilities.keys().cloned().collect()
    }

    /// Gets the contract for a capability
    pub fn contract(&self, name: &str) -> Option<&CapabilityContract> {
        self.capabilities.get(name).map(|c| c.contract())
    }

    /// Invokes a capability by name
    pub fn invoke(&self, name: &str, input: StructuredContent) -> Result<StructuredContent> {
        let capability = self
            .capabilities
            .get(name)
            .ok_or_else(|| GiamError::PreconditionNotMet(format!("Capability '{}' not found", name)))?;
        
        capability.invoke(input)
    }
}

impl Default for CapabilityRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// A simple capability that wraps a function
pub struct FunctionCapability {
    name: String,
    contract: CapabilityContract,
    func: fn(StructuredContent) -> Result<StructuredContent>,
}

impl FunctionCapability {
    /// Creates a new function capability
    pub fn new(
        name: String,
        contract: CapabilityContract,
        func: fn(StructuredContent) -> Result<StructuredContent>,
    ) -> Self {
        Self {
            name,
            contract,
            func,
        }
    }
}

#[async_trait]
impl Capability for FunctionCapability {
    fn name(&self) -> &str {
        &self.name
    }

    fn contract(&self) -> &CapabilityContract {
        &self.contract
    }

    fn invoke(&self, input: StructuredContent) -> Result<StructuredContent> {
        (self.func)(input)
    }

    async fn invoke_async(&self, input: StructuredContent) -> Result<StructuredContent> {
        (self.func)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry() {
        let mut registry = CapabilityRegistry::new();
        
        let cap = Arc::new(FunctionCapability::new(
            "test".to_string(),
            CapabilityContract::new(),
            |input| Ok(input),
        ));
        
        registry.register(cap);
        
        assert!(registry.has("test"));
        assert_eq!(registry.list(), vec!["test"]);
    }

    #[test]
    fn test_invoke() {
        let mut registry = CapabilityRegistry::new();
        
        let cap = Arc::new(FunctionCapability::new(
            "echo".to_string(),
            CapabilityContract::new(),
            |input| Ok(input),
        ));
        
        registry.register(cap);
        
        let result = registry.invoke("echo", StructuredContent::text("hello"));
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_text(), Some("hello"));
    }

    #[test]
    fn test_invoke_missing() {
        let registry = CapabilityRegistry::new();
        let result = registry.invoke("missing", StructuredContent::text("test"));
        assert!(result.is_err());
    }
}