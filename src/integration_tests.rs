//! Integration tests for GIAM
//!
//! These tests exercise the full GIAM stack

#[cfg(test)]
mod integration {
    use crate::api::GiamClient;
    use crate::autonomy::AutonomyBoundary;
    use crate::benchmark::{Benchmark, TaskCompletionBenchmark};
    use crate::capability::{Capability, CapabilityContract};
    use crate::capability_registry::{CapabilityRegistry, FunctionCapability};
    use crate::config::GiamConfig;
    use crate::content::StructuredContent;
    use crate::core::GiamLevel;
    use crate::execution::ExecutionEngine;
    use crate::goals::{Goal, Milestone};
    use crate::intent::{GoalStatus, Intent};
    use crate::metrics::LevelMetrics;
    use crate::planning::ExecutionPlan;
    use crate::self_evolution::{EvolutionLoop, Skill, SkillMetrics, SkillRepository, SkillSource};
    use crate::workflow::{Workflow, WorkflowEngine, WorkflowStep};

    #[test]
    fn test_full_giam_stack() {
        let config = GiamConfig::default();
        assert!(config.is_valid());

        let client = GiamClient::new(GiamLevel::Si);
        assert_eq!(client.level(), GiamLevel::Si);

        let boundary = client.autonomy_boundary();
        assert_eq!(boundary, AutonomyBoundary::BoundedWithApproval);

        let metrics = client.metrics();
        assert_eq!(metrics.autonomy_index, 0.3);
    }

    #[test]
    fn test_intent_execution_flow() {
        let intent = Intent::new("Analyze data".to_string(), 75);

        let mut plan = ExecutionPlan::new();
        plan.add_step("analyze".to_string(), vec![]);

        assert!(plan.validate().is_ok());
    }

    #[test]
    fn test_goal_hierarchy() {
        let mut parent = Goal::new("Parent goal".to_string());
        let child_id = uuid::Uuid::new_v4();

        parent.add_dependency(child_id);
        parent.set_progress(0.5);

        assert_eq!(parent.progress, 0.5);
        assert_eq!(parent.status, GoalStatus::Active);
    }

    #[test]
    fn test_capability_registry_integration() {
        let mut registry = CapabilityRegistry::new();

        let cap = std::sync::Arc::new(FunctionCapability::new(
            "test_cap".to_string(),
            CapabilityContract::new(),
            |input| Ok(input),
        ));

        registry.register(cap);

        let result = registry.invoke("test_cap", StructuredContent::text("test"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_execution_engine_integration() {
        let registry = CapabilityRegistry::new();
        let engine = ExecutionEngine::new(registry);

        let mut plan = ExecutionPlan::new();
        plan.add_step("echo".to_string(), vec![]);

        let result = engine.execute(&plan);
        assert!(result.is_ok());
    }

    #[test]
    fn test_workflow_integration() {
        let mut workflow = Workflow::new("Test workflow".to_string());
        workflow.add_step(WorkflowStep::new(
            "step1".to_string(),
            "First step".to_string(),
            "action1".to_string(),
        ));

        let result = WorkflowEngine::execute(&mut workflow);
        assert!(result.is_ok());
    }

    #[test]
    fn test_self_evolution_integration() {
        let mut repo = SkillRepository::new();

        let skill = Skill::new(
            "test_skill".to_string(),
            SkillSource::BuiltIn,
            SkillMetrics::default_metrics(),
        );
        repo.add(skill);

        let found = repo.find_by_name("test_skill");
        assert!(found.is_some());

        let all_skills = repo.all();
        assert!(!all_skills.is_empty());
    }

    #[test]
    fn test_evolution_loop() {
        let mut loop_state = EvolutionLoop::new();

        let generate = || -> Vec<Skill> {
            vec![
                Skill::new(
                    "skill1".to_string(),
                    SkillSource::Synthesized,
                    SkillMetrics::new(0.7, 0.8, 0.6),
                ),
                Skill::new(
                    "skill2".to_string(),
                    SkillSource::Synthesized,
                    SkillMetrics::new(0.9, 0.7, 0.8),
                ),
            ]
        };

        let select = |skills: &[Skill]| -> Skill {
            skills.first().cloned().unwrap_or_else(|| {
                Skill::new(
                    "default".to_string(),
                    SkillSource::BuiltIn,
                    SkillMetrics::default_metrics(),
                )
            })
        };

        let evolved = loop_state.evolve_until(generate, select, 0.85, 10);
        assert!(evolved || loop_state.iterations() > 0);
    }

    #[test]
    fn test_benchmark_integration() {
        let benchmark = TaskCompletionBenchmark::new(100, 85);
        let result = benchmark.run().expect("Benchmark should run");

        assert_eq!(result.score, 85.0);
    }

    #[test]
    fn test_metrics_integration() {
        let metrics = LevelMetrics::for_level(GiamLevel::Ui);
        assert_eq!(metrics.autonomy_index, 0.5);

        let score = metrics.overall_score();
        assert!(score > 0.0 && score <= 1.0);
    }

    #[test]
    fn test_autonomy_boundary_escalation() {
        let config = GiamConfig::new(GiamLevel::Agi, GiamLevel::Agi, GiamLevel::Ti)
            .with_auto_escalation(true);
        let mut client = crate::api::GiamClient::with_config(GiamLevel::Agi, config);

        let result = client.escalate();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), GiamLevel::Si);
    }

    #[test]
    fn test_state_transitions() {
        let mut client = GiamClient::new(GiamLevel::Si);

        let result = client.transition(crate::core::ExecutionState::Perceiving);
        assert!(result.is_ok());

        let result = client.transition(crate::core::ExecutionState::Reasoning);
        assert!(result.is_ok());

        let result = client.transition(crate::core::ExecutionState::Completed);
        assert!(result.is_err()); // Can't jump to Completed
    }

    #[test]
    fn test_content_types() {
        let text = StructuredContent::text("Hello");
        assert_eq!(text.as_text(), Some("Hello"));

        let json = StructuredContent::json(serde_json::json!({"key": "value"}));
        assert_eq!(json.as_json(), Some(&serde_json::json!({"key": "value"})));
    }
}
