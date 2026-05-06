//! Routing for GIAM intent handling
//!
//! Provides types and traits for routing intents to appropriate handlers

use std::collections::HashMap;

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

/// Default router using simple heuristics
pub struct DefaultRouter {
    level_mappings: HashMap<String, GiamLevel>,
}

impl DefaultRouter {
    pub fn new() -> Self {
        let mut mappings = HashMap::new();
        mappings.insert("search".to_string(), GiamLevel::Agi);
        mappings.insert("analyze".to_string(), GiamLevel::Si);
        mappings.insert("optimize".to_string(), GiamLevel::Ui);
        mappings.insert("evolve".to_string(), GiamLevel::Hi);
        mappings.insert("coordinate".to_string(), GiamLevel::Spi);
        mappings.insert("global".to_string(), GiamLevel::Uli);
        mappings.insert("transcend".to_string(), GiamLevel::Ti);

        Self {
            level_mappings: mappings,
        }
    }

    pub fn with_mapping(mut self, keyword: &str, level: GiamLevel) -> Self {
        self.level_mappings.insert(keyword.to_string(), level);
        self
    }
}

impl Default for DefaultRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl Router for DefaultRouter {
    fn route(&self, intent: &Intent) -> RoutingDecision {
        let description = intent.description.to_lowercase();

        for (keyword, level) in &self.level_mappings {
            if description.contains(keyword) {
                return RoutingDecision::new(
                    *level,
                    0.9,
                    format!("Matched keyword '{}' for level {}", keyword, level.name()),
                );
            }
        }

        // Default based on priority
        let default_level = if intent.priority > 80 {
            GiamLevel::Ui
        } else if intent.priority > 50 {
            GiamLevel::Si
        } else {
            GiamLevel::Agi
        };

        RoutingDecision::new(
            default_level,
            0.5,
            format!("Default routing based on priority {}", intent.priority),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intent::GoalStatus;

    #[test]
    fn test_routing_decision() {
        let decision = RoutingDecision::new(GiamLevel::Si, 0.8, "test".to_string());
        assert_eq!(decision.level, GiamLevel::Si);
        assert!((decision.confidence - 0.8).abs() < 0.001);
    }

    #[test]
    fn test_default_router() {
        let router = DefaultRouter::new();

        let intent = Intent::new("search for information".to_string(), 30);

        let decision = router.route(&intent);
        assert_eq!(decision.level, GiamLevel::Agi);
    }
}
