//! API types for GIAM
//!
//! Provides the main client API for interacting with GIAM systems

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::autonomy::AutonomyBoundary;
use crate::config::GiamConfig;
use crate::content::StructuredContent;
use crate::core::{ExecutionState, GiamLevel};
use crate::error::{GiamError, Result};
use crate::intent::Intent;
use crate::metrics::LevelMetrics;
use crate::routing::Router;
use crate::routing::RoutingDecision;

/// The main GIAM client for executing intents
pub struct GiamClient {
    level: GiamLevel,
    config: GiamConfig,
    router: Box<dyn Router>,
    current_state: ExecutionState,
}

impl std::fmt::Debug for GiamClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GiamClient")
            .field("level", &self.level)
            .field("state", &self.current_state)
            .finish()
    }
}

impl GiamClient {
    /// Creates a new GIAM client at the specified level
    pub fn new(level: GiamLevel) -> Self {
        Self {
            level,
            config: GiamConfig::default(),
            router: Box::new(DefaultRouter),
            current_state: ExecutionState::Initialized,
        }
    }

    /// Creates a new GIAM client with a custom router
    pub fn with_router(level: GiamLevel, router: Box<dyn Router>) -> Self {
        Self {
            level,
            config: GiamConfig::default(),
            router,
            current_state: ExecutionState::Initialized,
        }
    }

    /// Creates a new GIAM client with full configuration
    pub fn with_config(level: GiamLevel, config: GiamConfig) -> Self {
        Self {
            level,
            config,
            router: Box::new(DefaultRouter),
            current_state: ExecutionState::Initialized,
        }
    }

    /// Returns the GIAM level of this client
    pub fn level(&self) -> GiamLevel {
        self.level
    }

    /// Returns the current execution state
    pub fn state(&self) -> ExecutionState {
        self.current_state
    }

    /// Returns the autonomy boundary for the current level
    pub fn autonomy_boundary(&self) -> AutonomyBoundary {
        AutonomyBoundary::for_level(self.level.tier())
    }

    /// Returns the metrics for the current level
    pub fn metrics(&self) -> LevelMetrics {
        LevelMetrics::for_level(self.level)
    }

    /// Escalates the GIAM level if allowed by config
    pub fn escalate(&mut self) -> Result<GiamLevel> {
        if !self.config.auto_escalation {
            return Err(GiamError::LevelViolation(
                "Auto-escalation is disabled".to_string(),
            ));
        }

        if let Some(next_level) = self.level.successor() {
            if next_level.tier() <= self.config.max_level.tier() {
                self.level = next_level;
                Ok(self.level)
            } else {
                Err(GiamError::LevelViolation(
                    "Cannot exceed max level".to_string(),
                ))
            }
        } else {
            Err(GiamError::LevelViolation(
                "Already at maximum level".to_string(),
            ))
        }
    }

    /// Transitions to a new state if valid
    pub fn transition(&mut self, new_state: ExecutionState) -> Result<()> {
        if !self.is_valid_transition(self.current_state, new_state) {
            return Err(GiamError::InvalidStateTransition(format!(
                "Cannot transition from {:?} to {:?}",
                self.current_state, new_state
            )));
        }
        self.current_state = new_state;
        Ok(())
    }

    /// Checks if a state transition is valid
    #[allow(clippy::match_like_matches_macro)]
    fn is_valid_transition(&self, from: ExecutionState, to: ExecutionState) -> bool {
        match (from, to) {
            (ExecutionState::Initialized, ExecutionState::Perceiving) => true,
            (ExecutionState::Perceiving, ExecutionState::Reasoning) => true,
            (ExecutionState::Reasoning, ExecutionState::Assigning) => true,
            (ExecutionState::Assigning, ExecutionState::Executing) => true,
            (ExecutionState::Executing, ExecutionState::Reflecting) => true,
            (ExecutionState::Executing, ExecutionState::Failed) => true,
            (ExecutionState::Reflecting, ExecutionState::Completed) => true,
            (_, ExecutionState::Paused) => true,
            _ => false,
        }
    }

    /// Executes an intent and returns the result
    pub fn execute(&self, intent: &Intent) -> Result<StructuredContent> {
        let boundary = self.autonomy_boundary();
        if boundary == AutonomyBoundary::ReadOnly {
            return Err(GiamError::AutonomyBoundaryViolated(
                "Client is in read-only mode".to_string(),
            ));
        }

        Ok(StructuredContent::text(format!(
            "Executed intent '{}' at level {}",
            intent.description, self.level
        )))
    }

    /// Routes an intent and returns the routing decision
    pub fn route(&self, intent: &Intent) -> RoutingDecision {
        self.router.route(intent)
    }

    /// Creates a new session with this client
    pub fn create_session(&self) -> crate::session::Session {
        crate::session::Session::new(Uuid::new_v4(), self.level)
    }
}

/// Default router implementation
struct DefaultRouter;

impl Router for DefaultRouter {
    fn route(&self, _intent: &Intent) -> RoutingDecision {
        RoutingDecision::new(
            GiamLevel::Agi,
            0.5,
            "Default routing - no specific routing logic".to_string(),
        )
    }
}

/// Request for execute gRPC call
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteRequest {
    pub intent: Intent,
}

/// Response for execute gRPC call
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteResponse {
    pub result: StructuredContent,
}

/// Request for route gRPC call
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteRequest {
    pub intent: Intent,
}

/// Response for route gRPC call
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteResponse {
    pub decision: RoutingDecision,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_giam_client_new() {
        let client = GiamClient::new(GiamLevel::Si);
        assert_eq!(client.level(), GiamLevel::Si);
    }

    #[test]
    fn test_giam_client_execute() {
        let client = GiamClient::new(GiamLevel::Ui);
        let intent = Intent::new("test intent".to_string(), 1);
        let result = client.execute(&intent);
        assert!(result.is_ok());
    }

    #[test]
    fn test_giam_client_route() {
        let client = GiamClient::new(GiamLevel::Hi);
        let intent = Intent::new("test intent".to_string(), 1);
        let decision = client.route(&intent);
        assert!(decision.confidence >= 0.0 && decision.confidence <= 1.0);
    }
}
