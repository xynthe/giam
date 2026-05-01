//! API types for GIAM
//!
//! Provides the main client API for interacting with GIAM systems

use serde::{Deserialize, Serialize};

use crate::content::StructuredContent;
use crate::core::GiamLevel;
use crate::error::Result;
use crate::intent::Intent;
use crate::routing::Router;
use crate::routing::RoutingDecision;

/// The main GIAM client for executing intents
pub struct GiamClient {
    level: GiamLevel,
    router: Box<dyn Router>,
}

impl std::fmt::Debug for GiamClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GiamClient")
            .field("level", &self.level)
            .finish()
    }
}

impl GiamClient {
    /// Creates a new GIAM client at the specified level
    pub fn new(level: GiamLevel) -> Self {
        Self {
            level,
            router: Box::new(DefaultRouter),
        }
    }

    /// Creates a new GIAM client with a custom router
    pub fn with_router(level: GiamLevel, router: Box<dyn Router>) -> Self {
        Self { level, router }
    }

    /// Returns the GIAM level of this client
    pub fn level(&self) -> GiamLevel {
        self.level
    }

    /// Executes an intent and returns the result
    pub fn execute(&self, intent: &Intent) -> Result<StructuredContent> {
        Ok(StructuredContent::text(format!(
            "Executed intent '{}' at level {}",
            intent.description, self.level
        )))
    }

    /// Routes an intent and returns the routing decision
    pub fn route(&self, intent: &Intent) -> RoutingDecision {
        self.router.route(intent)
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
