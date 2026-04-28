//! Routing for GIAM intent handling
//!
//! Provides types and traits for routing intents to appropriate handlers

use serde::{Deserialize, Serialize};

use crate::core::GiamLevel;
use crate::Intent;

/// A routing decision based on intent analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingDecision {
    /// The GIAM level the intent should be handled at
    pub level: GiamLevel,
    /// Confidence in the routing decision
    pub confidence: f64,
    /// Human-readable rationale for the decision
    pub rationale: String,
}

impl RoutingDecision {
    /// Creates a new routing decision
    pub fn new(level: GiamLevel, confidence: f64, rationale: String) -> Self {
        Self {
            level,
            confidence: confidence.clamp(0.0, 1.0),
            rationale,
        }
    }
}

/// Router trait for determining how intents should be handled
pub trait Router: Send + Sync {
    /// Routes an intent and returns the routing decision
    fn route(&self, intent: &Intent) -> RoutingDecision;
}
