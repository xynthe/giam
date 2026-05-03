//! Multi-agent types for SPI level
//!
//! Provides types for multi-agent orchestration

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::GiamLevel;
use crate::planning::{ExecutionPlan, ExecutionResult};

/// The role of an agent in a network
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentRole {
    /// Coordinates other agents
    Coordinator,
    /// Specializes in specific capabilities
    Specialist,
    /// Executes tasks
    Worker,
}

/// An individual agent in the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    /// Unique identifier
    pub id: Uuid,
    /// The role of this agent
    pub role: AgentRole,
    /// The GIAM level of this agent
    pub level: GiamLevel,
    /// Capabilities this agent can provide
    pub capabilities: Vec<String>,
    /// Whether the agent is currently available
    pub available: bool,
}

impl Agent {
    /// Creates a new agent
    pub fn new(role: AgentRole, level: GiamLevel) -> Self {
        Self {
            id: Uuid::new_v4(),
            role,
            level,
            capabilities: Vec::new(),
            available: true,
        }
    }

    /// Creates an agent with capabilities
    pub fn with_capabilities(role: AgentRole, level: GiamLevel, capabilities: Vec<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            role,
            level,
            capabilities,
            available: true,
        }
    }

    /// Checks if agent has a specific capability
    pub fn has_capability(&self, capability: &str) -> bool {
        self.capabilities.iter().any(|c| c == capability)
    }
}

/// Message types for inter-agent communication
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AgentMessage {
    /// Request for task execution
    TaskRequest { task_id: Uuid, plan: ExecutionPlan },
    /// Response with task result
    TaskResponse {
        task_id: Uuid,
        result: ExecutionResult,
    },
    /// Request for coordination
    CoordinationRequest {
        requester_id: Uuid,
        description: String,
    },
    /// Status update
    StatusUpdate { agent_id: Uuid, available: bool },
}

/// A network of agents working together
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentNetwork {
    /// All agents in the network
    pub agents: HashMap<Uuid, Agent>,
    /// Connection pairs (agent_id, connected_agent_id)
    pub connections: Vec<(Uuid, Uuid)>,
}

impl AgentNetwork {
    /// Creates a new empty agent network
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
            connections: Vec::new(),
        }
    }

    /// Adds an agent to the network
    pub fn add_agent(&mut self, agent: Agent) {
        self.agents.insert(agent.id, agent);
    }

    /// Adds a connection between two agents
    pub fn add_connection(&mut self, from: Uuid, to: Uuid) {
        if self.agents.contains_key(&from) && self.agents.contains_key(&to) {
            self.connections.push((from, to));
        }
    }

    /// Returns the number of agents in the network
    pub fn len(&self) -> usize {
        self.agents.len()
    }

    /// Returns whether the network is empty
    pub fn is_empty(&self) -> bool {
        self.agents.is_empty()
    }

    /// Finds available agents with a specific capability
    pub fn find_agents_with_capability(&self, capability: &str) -> Vec<Uuid> {
        self.agents
            .iter()
            .filter(|(_, agent)| agent.available && agent.has_capability(capability))
            .map(|(id, _)| *id)
            .collect()
    }

    /// Gets the coordinator agent (if any)
    pub fn coordinator(&self) -> Option<&Agent> {
        self.agents
            .values()
            .find(|a| a.role == AgentRole::Coordinator)
    }

    /// Gets available workers
    pub fn available_workers(&self) -> Vec<&Agent> {
        self.agents
            .values()
            .filter(|a| a.role == AgentRole::Worker && a.available)
            .collect()
    }

    /// Distributes a plan across available agents
    pub fn distribute_plan(&mut self, plan: &ExecutionPlan) -> HashMap<Uuid, Vec<usize>> {
        let mut assignment: HashMap<Uuid, Vec<usize>> = HashMap::new();
        let workers = self.available_workers();

        for (idx, _step) in plan.steps.iter().enumerate() {
            if let Some(agent_id) = workers.get(idx % workers.len()).map(|a| a.id) {
                assignment.entry(agent_id).or_default().push(idx);
            }
        }
        assignment
    }

    /// Gets all agents of a specific role
    pub fn agents_with_role(&self, role: AgentRole) -> Vec<&Agent> {
        self.agents.values().filter(|a| a.role == role).collect()
    }
}

impl Default for AgentNetwork {
    fn default() -> Self {
        Self::new()
    }
}
