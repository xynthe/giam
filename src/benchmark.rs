//! Benchmark types for GIAM
//!
//! Provides types for benchmarking system performance at each level

use serde::{Deserialize, Serialize};

use crate::core::GiamLevel;
use crate::error::Result;

/// Result of running a benchmark
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    /// The name of the benchmark
    pub name: String,
    /// The score achieved (0.0-1.0 or percentage)
    pub score: f64,
    /// Additional metadata about the benchmark run
    pub metadata: Vec<(String, String)>,
}

impl BenchmarkResult {
    /// Creates a new benchmark result
    pub fn new(name: String, score: f64) -> Self {
        Self {
            name,
            score,
            metadata: Vec::new(),
        }
    }

    /// Adds metadata to the result
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.push((key.into(), value.into()));
        self
    }
}

/// The benchmark trait for running benchmarks
pub trait Benchmark: Send + Sync {
    /// Returns the name of this benchmark
    fn name(&self) -> &str;

    /// Runs the benchmark and returns the result
    fn run(&self) -> Result<BenchmarkResult>;
}

/// A benchmark for measuring task completion (SI level)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskCompletionBenchmark {
    pub total_tasks: usize,
    pub completed_tasks: usize,
}

impl TaskCompletionBenchmark {
    pub fn new(total_tasks: usize, completed_tasks: usize) -> Self {
        Self {
            total_tasks,
            completed_tasks,
        }
    }
}

impl Benchmark for TaskCompletionBenchmark {
    fn name(&self) -> &str {
        "task_completion"
    }

    fn run(&self) -> Result<BenchmarkResult> {
        let score = if self.total_tasks > 0 {
            (self.completed_tasks as f64 / self.total_tasks as f64) * 100.0
        } else {
            0.0
        };
        Ok(BenchmarkResult::new(self.name().to_string(), score)
            .with_metadata("total", self.total_tasks.to_string())
            .with_metadata("completed", self.completed_tasks.to_string()))
    }
}

/// A benchmark for measuring goal completion (UI level)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalCompletionBenchmark {
    pub goals_completed: usize,
    pub time_period_seconds: u64,
}

impl GoalCompletionBenchmark {
    pub fn new(goals_completed: usize, time_period_seconds: u64) -> Self {
        Self {
            goals_completed,
            time_period_seconds,
        }
    }
}

impl Benchmark for GoalCompletionBenchmark {
    fn name(&self) -> &str {
        "goal_completion"
    }

    fn run(&self) -> Result<BenchmarkResult> {
        let score = if self.time_period_seconds > 0 {
            self.goals_completed as f64 / self.time_period_seconds as f64 * 3600.0
        } else {
            0.0
        };
        Ok(BenchmarkResult::new(self.name().to_string(), score)
            .with_metadata("goals", self.goals_completed.to_string())
            .with_metadata("time_seconds", self.time_period_seconds.to_string()))
    }
}

/// A benchmark for measuring improvement rate (HI level)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImprovementRateBenchmark {
    pub initial_score: f64,
    pub final_score: f64,
}

impl ImprovementRateBenchmark {
    pub fn new(initial_score: f64, final_score: f64) -> Self {
        Self {
            initial_score,
            final_score,
        }
    }
}

impl Benchmark for ImprovementRateBenchmark {
    fn name(&self) -> &str {
        "improvement_rate"
    }

    fn run(&self) -> Result<BenchmarkResult> {
        let score = self.final_score - self.initial_score;
        Ok(BenchmarkResult::new(self.name().to_string(), score)
            .with_metadata("initial", self.initial_score.to_string())
            .with_metadata("final", self.final_score.to_string()))
    }
}

/// A benchmark for measuring coordination (SPI level)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinationBenchmark {
    pub tasks_coordinated: usize,
}

impl CoordinationBenchmark {
    pub fn new(tasks_coordinated: usize) -> Self {
        Self { tasks_coordinated }
    }
}

impl Benchmark for CoordinationBenchmark {
    fn name(&self) -> &str {
        "coordination"
    }

    fn run(&self) -> Result<BenchmarkResult> {
        Ok(
            BenchmarkResult::new(self.name().to_string(), self.tasks_coordinated as f64)
                .with_metadata("tasks", self.tasks_coordinated.to_string()),
        )
    }
}

/// A benchmark for measuring efficiency (ULI level)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EfficiencyBenchmark {
    pub actual_cost: f64,
    pub optimal_cost: f64,
}

impl EfficiencyBenchmark {
    pub fn new(actual_cost: f64, optimal_cost: f64) -> Self {
        Self {
            actual_cost,
            optimal_cost,
        }
    }
}

impl Benchmark for EfficiencyBenchmark {
    fn name(&self) -> &str {
        "efficiency"
    }

    fn run(&self) -> Result<BenchmarkResult> {
        let score = if self.optimal_cost > 0.0 {
            (self.optimal_cost / self.actual_cost) * 100.0
        } else {
            0.0
        };
        Ok(BenchmarkResult::new(self.name().to_string(), score)
            .with_metadata("actual", self.actual_cost.to_string())
            .with_metadata("optimal", self.optimal_cost.to_string()))
    }
}

/// A benchmark for measuring substrate coverage (TI level)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstrateCoverageBenchmark {
    pub substrates_supported: usize,
    pub total_substrates: usize,
}

impl SubstrateCoverageBenchmark {
    pub fn new(substrates_supported: usize, total_substrates: usize) -> Self {
        Self {
            substrates_supported,
            total_substrates,
        }
    }
}

impl Benchmark for SubstrateCoverageBenchmark {
    fn name(&self) -> &str {
        "substrate_coverage"
    }

    fn run(&self) -> Result<BenchmarkResult> {
        let score = if self.total_substrates > 0 {
            (self.substrates_supported as f64 / self.total_substrates as f64) * 100.0
        } else {
            0.0
        };
        Ok(BenchmarkResult::new(self.name().to_string(), score)
            .with_metadata("supported", self.substrates_supported.to_string())
            .with_metadata("total", self.total_substrates.to_string()))
    }
}

/// Returns the appropriate benchmark for a given GIAM level
pub fn benchmark_for_level(level: GiamLevel) -> Box<dyn Benchmark> {
    match level {
        GiamLevel::Agi => Box::new(TaskCompletionBenchmark::new(100, 0)),
        GiamLevel::Si => Box::new(TaskCompletionBenchmark::new(100, 0)),
        GiamLevel::Ui => Box::new(GoalCompletionBenchmark::new(0, 3600)),
        GiamLevel::Hi => Box::new(ImprovementRateBenchmark::new(0.0, 0.0)),
        GiamLevel::Spi => Box::new(CoordinationBenchmark::new(0)),
        GiamLevel::Uli => Box::new(EfficiencyBenchmark::new(1.0, 1.0)),
        GiamLevel::Ti => Box::new(SubstrateCoverageBenchmark::new(0, 10)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_completion_benchmark() {
        let benchmark = TaskCompletionBenchmark::new(100, 85);
        let result = benchmark.run().unwrap();
        assert_eq!(result.name, "task_completion");
        assert!((result.score - 85.0).abs() < 0.01);
    }

    #[test]
    fn test_efficiency_benchmark() {
        let benchmark = EfficiencyBenchmark::new(100.0, 80.0);
        let result = benchmark.run().unwrap();
        assert_eq!(result.name, "efficiency");
        assert!((result.score - 80.0).abs() < 0.01);
    }

    #[test]
    fn test_substrate_coverage_benchmark() {
        let benchmark = SubstrateCoverageBenchmark::new(8, 10);
        let result = benchmark.run().unwrap();
        assert_eq!(result.name, "substrate_coverage");
        assert!((result.score - 80.0).abs() < 0.01);
    }
}
