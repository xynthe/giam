//! Provenance tracking for GIAM
//!
//! Provides types for tracking the lineage and causal relationships of thoughts and actions

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A chain of provenance tracking parent-child relationships
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProvenanceChain {
    /// Unique identifier for this chain
    pub id: Uuid,
    /// Parent IDs that this chain derives from
    pub parents: Vec<Uuid>,
}

impl ProvenanceChain {
    /// Creates a new provenance chain
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            parents: Vec::new(),
        }
    }

    /// Creates a provenance chain with a single parent
    pub fn with_parent(parent: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            parents: vec![parent],
        }
    }

    /// Creates a provenance chain with multiple parents
    pub fn with_parents(parents: Vec<Uuid>) -> Self {
        Self {
            id: Uuid::new_v4(),
            parents,
        }
    }

    /// Adds a parent to this chain
    pub fn add_parent(&mut self, parent: Uuid) {
        if !self.parents.contains(&parent) {
            self.parents.push(parent);
        }
    }
}

/// The type of causal link between two entities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LinkType {
    /// Derived from another entity
    Derived,
    /// Directly caused by another entity
    Caused,
    /// Refined or improved from another entity
    Refined,
}

/// A causal link between two entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalLink {
    /// The source entity
    pub source: Uuid,
    /// The target entity
    pub target: Uuid,
    /// The type of link
    pub link_type: LinkType,
}

impl CausalLink {
    /// Creates a new causal link
    pub fn new(source: Uuid, target: Uuid, link_type: LinkType) -> Self {
        Self {
            source,
            target,
            link_type,
        }
    }
}
