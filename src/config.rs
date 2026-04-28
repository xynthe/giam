//! Configuration for GIAM
//!
//! Provides the main configuration type for initializing GIAM systems

use serde::{Deserialize, Serialize};

use crate::core::GiamLevel;

/// Configuration for a GIAM system instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiamConfig {
    /// The initial GIAM level
    pub initial_level: GiamLevel,
    /// The minimum allowed GIAM level
    pub min_level: GiamLevel,
    /// The maximum allowed GIAM level
    pub max_level: GiamLevel,
    /// Whether auto-escalation is enabled
    pub auto_escalation: bool,
}

impl Default for GiamConfig {
    fn default() -> Self {
        Self {
            initial_level: GiamLevel::Agi,
            min_level: GiamLevel::Agi,
            max_level: GiamLevel::Si,
            auto_escalation: false,
        }
    }
}

impl GiamConfig {
    /// Creates a new configuration
    pub fn new(initial_level: GiamLevel, min_level: GiamLevel, max_level: GiamLevel) -> Self {
        Self {
            initial_level,
            min_level,
            max_level,
            auto_escalation: false,
        }
    }

    /// Creates a configuration with auto-escalation enabled
    pub fn with_auto_escalation(mut self, enabled: bool) -> Self {
        self.auto_escalation = enabled;
        self
    }

    /// Validates the configuration
    pub fn is_valid(&self) -> bool {
        self.min_level.tier() <= self.initial_level.tier()
            && self.initial_level.tier() <= self.max_level.tier()
    }
}
