//! Self-evolution types for HI level
//!
//! Provides types for recursive self-improvement capabilities

use std::collections::HashMap;

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
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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

    /// Returns default metrics
    pub fn default_metrics() -> Self {
        Self::new(0.5, 0.5, 0.5)
    }
}

/// A skill representing an evolved capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    /// Unique identifier
    pub id: Uuid,
    /// Human-readable name
    pub name: String,
    /// The source of this skill
    pub source: SkillSource,
    /// Metrics for this skill
    pub metrics: SkillMetrics,
    /// Version number
    pub version: u32,
}

impl Skill {
    /// Creates a new skill
    pub fn new(name: String, source: SkillSource, metrics: SkillMetrics) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            source,
            metrics,
            version: 1,
        }
    }

    /// Creates an evolved version of this skill
    pub fn evolve(&self, new_metrics: SkillMetrics) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: self.name.clone(),
            source: SkillSource::Evolved,
            metrics: new_metrics,
            version: self.version + 1,
        }
    }
}

/// Status of the evolution process
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvolutionStatus {
    Idle,
    Evaluating,
    Generating,
    Selecting,
    Evolving,
    Complete,
    Failed,
}

/// The evolution loop handles the evaluate-generate-select cycle for self-evolution
pub struct EvolutionLoop {
    status: EvolutionStatus,
    current_metrics: SkillMetrics,
    best_skill: Option<Skill>,
    iterations: u32,
}

impl EvolutionLoop {
    pub fn new() -> Self {
        Self {
            status: EvolutionStatus::Idle,
            current_metrics: SkillMetrics::default_metrics(),
            best_skill: None,
            iterations: 0,
        }
    }

    pub fn status(&self) -> EvolutionStatus {
        self.status
    }

    pub fn current_metrics(&self) -> SkillMetrics {
        self.current_metrics
    }

    pub fn best_skill(&self) -> Option<&Skill> {
        self.best_skill.as_ref()
    }

    pub fn iterations(&self) -> u32 {
        self.iterations
    }

    /// Run one iteration of the evolution loop
    pub fn step(&mut self, generate: fn() -> Vec<Skill>, select: fn(&[Skill]) -> Skill) -> bool {
        self.iterations += 1;

        self.status = EvolutionStatus::Generating;
        let candidates = generate();

        if candidates.is_empty() {
            self.status = EvolutionStatus::Failed;
            return false;
        }

        self.status = EvolutionStatus::Selecting;
        let selected = select(&candidates);

        if self.best_skill.is_none()
            || selected.metrics.overall() > self.best_skill.as_ref().unwrap().metrics.overall()
        {
            self.best_skill = Some(selected);
            self.current_metrics = self.best_skill.as_ref().unwrap().metrics;
            self.status = EvolutionStatus::Evolving;
        }

        self.status = EvolutionStatus::Complete;
        true
    }

    /// Run multiple iterations until threshold or max iterations
    pub fn evolve_until(
        &mut self,
        generate: fn() -> Vec<Skill>,
        select: fn(&[Skill]) -> Skill,
        threshold: f64,
        max_iterations: u32,
    ) -> bool {
        for _ in 0..max_iterations {
            self.step(generate, select);

            if let Some(skill) = &self.best_skill {
                if skill.metrics.overall() >= threshold {
                    return true;
                }
            }
        }
        false
    }
}

impl Default for EvolutionLoop {
    fn default() -> Self {
        Self::new()
    }
}

/// Skill repository for storing and retrieving skills
pub struct SkillRepository {
    skills: HashMap<Uuid, Skill>,
    by_name: HashMap<String, Vec<Uuid>>,
}

impl SkillRepository {
    pub fn new() -> Self {
        Self {
            skills: HashMap::new(),
            by_name: HashMap::new(),
        }
    }

    pub fn add(&mut self, skill: Skill) {
        let id = skill.id;
        let name = skill.name.clone();
        self.skills.insert(id, skill);
        self.by_name.entry(name).or_default().push(id);
    }

    pub fn get(&self, id: Uuid) -> Option<&Skill> {
        self.skills.get(&id)
    }

    pub fn find_by_name(&self, name: &str) -> Option<&Skill> {
        self.by_name
            .get(name)
            .and_then(|ids| ids.first())
            .and_then(|id| self.skills.get(id))
    }

    pub fn all(&self) -> Vec<&Skill> {
        self.skills.values().collect()
    }

    pub fn by_source(&self, source: SkillSource) -> Vec<&Skill> {
        self.skills
            .values()
            .filter(|s| s.source == source)
            .collect()
    }
}

impl Default for SkillRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_creation() {
        let skill = Skill::new(
            "test".to_string(),
            SkillSource::BuiltIn,
            SkillMetrics::default_metrics(),
        );
        assert_eq!(skill.version, 1);
    }

    #[test]
    fn test_skill_evolve() {
        let skill = Skill::new(
            "test".to_string(),
            SkillSource::BuiltIn,
            SkillMetrics::default_metrics(),
        );
        let evolved = skill.evolve(SkillMetrics::new(0.8, 0.8, 0.8));
        assert_eq!(evolved.version, 2);
        assert_eq!(evolved.source, SkillSource::Evolved);
    }

    #[test]
    fn test_skill_repository() {
        let mut repo = SkillRepository::new();
        let skill = Skill::new(
            "test".to_string(),
            SkillSource::BuiltIn,
            SkillMetrics::default_metrics(),
        );
        repo.add(skill);

        assert!(repo.find_by_name("test").is_some());
    }
}
