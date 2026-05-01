//! Measurement metrics for GIAM
//!
//! Provides types for quantifying system performance at each level

use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::core::GiamLevel;

/// Metrics for evaluating a GIAM level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelMetrics {
    /// Autonomy index (0.0-1.0)
    pub autonomy_index: f64,
    /// Temporal horizon in seconds
    pub temporal_horizon: Duration,
    /// Execution breadth (number of parallel tasks)
    pub execution_breadth: usize,
    /// Adaptability score (0.0-1.0)
    pub adaptability_score: f64,
}

impl LevelMetrics {
    /// Creates new level metrics
    pub fn new(
        autonomy_index: f64,
        temporal_horizon: Duration,
        execution_breadth: usize,
        adaptability_score: f64,
    ) -> Self {
        Self {
            autonomy_index: autonomy_index.clamp(0.0, 1.0),
            temporal_horizon,
            execution_breadth,
            adaptability_score: adaptability_score.clamp(0.0, 1.0),
        }
    }

    /// Returns the default metrics for a given level
    pub fn for_level(level: GiamLevel) -> Self {
        let (autonomy_index, temporal_horizon, execution_breadth, adaptability_score) = match level
        {
            GiamLevel::Agi => (0.1, Duration::from_secs(3600), 1, 0.2),
            GiamLevel::Si => (0.3, Duration::from_secs(14400), 5, 0.4),
            GiamLevel::Ui => (0.5, Duration::from_secs(604800), 10, 0.6),
            GiamLevel::Hi => (0.7, Duration::from_secs(0), 20, 0.8),
            GiamLevel::Spi => (0.8, Duration::from_secs(0), 100, 0.85),
            GiamLevel::Uli => (0.9, Duration::from_secs(0), 1000, 0.9),
            GiamLevel::Ti => (1.0, Duration::from_secs(0), usize::MAX, 1.0),
        };
        Self::new(
            autonomy_index,
            temporal_horizon,
            execution_breadth,
            adaptability_score,
        )
    }

    /// Returns the overall score as a weighted average
    pub fn overall_score(&self) -> f64 {
        (self.autonomy_index + self.adaptability_score) / 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_metrics_for_level() {
        let agi = LevelMetrics::for_level(GiamLevel::Agi);
        assert_eq!(agi.autonomy_index, 0.1);
        assert!((agi.adaptability_score - 0.2).abs() < 0.01);

        let ti = LevelMetrics::for_level(GiamLevel::Ti);
        assert_eq!(ti.autonomy_index, 1.0);
        assert!((ti.adaptability_score - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_overall_score() {
        let metrics = LevelMetrics::new(0.5, Duration::from_secs(100), 5, 0.8);
        let expected = (0.5 + 0.8) / 2.0;
        assert!((metrics.overall_score() - expected).abs() < 0.01);
    }
}
