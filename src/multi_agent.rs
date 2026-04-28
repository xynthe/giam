//! Multi-agent types for SPI level
//!
//! Provides types for multi-agent orchestration

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::GiamLevel;

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
}

impl Agent {
    /// Creates a new agent
    pub fn new(role: AgentRole, level: GiamLevel) -> Self {
        Self {
            id: Uuid::new_v4(),
            role,
            level,
        }
    }
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
}

impl Default for AgentNetwork {
    fn default() -> Self {
        Self::new()
    }
}
