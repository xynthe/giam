//! Memory layer types for GIAM
//!
//! Provides types for hierarchical memory management

use serde::{Deserialize, Serialize};

use crate::temporal::TimeWindow;

/// Memory layers representing different time scales of retention
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryLayer {
    /// Immediate memory (seconds)
    Immediate,
    /// Working memory (hours)
    Working,
    /// Episodic memory (days)
    Episodic,
    /// Semantic memory (persistent)
    Semantic,
}

/// A query for searching memory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryQuery {
    /// The search pattern
    pub pattern: String,
    /// The memory layer to search
    pub layer: MemoryLayer,
    /// The time window to search within
    pub window: TimeWindow,
}

impl MemoryQuery {
    /// Creates a new memory query
    pub fn new(pattern: String, layer: MemoryLayer, window: TimeWindow) -> Self {
        Self {
            pattern,
            layer,
            window,
        }
    }
}
