//! Self-evolution types for HI level
//!
//! Provides types for recursive self-improvement capabilities

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The source of a skill
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkillSource {
    /// Built-in capability
    BuiltIn,
    /// Synthesized from existing capabilities
    Synthesized,
    /// Evolved through self-modification
    Evolved,
}

/// Metrics for evaluating skills
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMetrics {
    /// Effectiveness score (0.0-1.0)
    pub effectiveness: f64,
    /// Efficiency score (0.0-1.0)
    pub efficiency: f64,
    /// Generalization score (0.0-1.0)
    pub generalization: f64,
}

impl SkillMetrics {
    /// Creates new skill metrics
    pub fn new(effectiveness: f64, efficiency: f64, generalization: f64) -> Self {
        Self {
            effectiveness: effectiveness.clamp(0.0, 1.0),
            efficiency: efficiency.clamp(0.0, 1.0),
            generalization: generalization.clamp(0.0, 1.0),
        }
    }

    /// Returns the overall score as the average of all metrics
    pub fn overall(&self) -> f64 {
        (self.effectiveness + self.efficiency + self.generalization) / 3.0
    }
}

/// A skill representing an evolved capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    /// Unique identifier
    pub id: Uuid,
    /// The source of this skill
    pub source: SkillSource,
    /// Metrics for this skill
    pub metrics: SkillMetrics,
}

impl Skill {
    /// Creates a new skill
    pub fn new(source: SkillSource, metrics: SkillMetrics) -> Self {
        Self {
            id: Uuid::new_v4(),
            source,
            metrics,
        }
    }
}

/// Placeholder for the evolution loop
///
/// The EvolutionLoop handles the evaluate-generate-select cycle for self-evolution:
/// - evaluate: Assess current capabilities
/// - generate: Create new potential skills
/// - select: Choose the best skill to adopt
pub struct EvolutionLoop {
    /// Evaluation function
    pub evaluate: fn() -> SkillMetrics,
    /// Generation function
    pub generate: fn() -> Vec<Skill>,
    /// Selection function
    pub select: fn(skills: &[Skill]) -> Skill,
}

impl EvolutionLoop {
    /// Creates a new evolution loop
    pub fn new(
        evaluate: fn() -> SkillMetrics,
        generate: fn() -> Vec<Skill>,
        select: fn(skills: &[Skill]) -> Skill,
    ) -> Self {
        Self {
            evaluate,
            generate,
            select,
        }
    }
}
