//! Session types for UI level
//!
//! Provides types for managing user sessions

use serde::{Deserialize, Serialize};

use crate::core::GiamLevel;
use crate::temporal::Timestamp;
use uuid::Uuid;

/// Session states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionState {
    /// Session is active
    Active,
    /// Session is paused
    Paused,
    /// Session has completed
    Completed,
}

/// A user session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Unique identifier
    pub id: Uuid,
    /// The agent ID for this session
    pub agent_id: Uuid,
    /// When the session started
    pub started_at: Timestamp,
    /// Current session state
    pub state: SessionState,
    /// The GIAM level for this session
    pub level: GiamLevel,
}

impl Session {
    /// Creates a new session
    pub fn new(agent_id: Uuid, level: GiamLevel) -> Self {
        Self {
            id: Uuid::new_v4(),
            agent_id,
            started_at: Timestamp::now(),
            state: SessionState::Active,
            level,
        }
    }

    /// Pauses the session
    pub fn pause(&mut self) {
        self.state = SessionState::Paused;
    }

    /// Resumes the session
    pub fn resume(&mut self) {
        self.state = SessionState::Active;
    }

    /// Completes the session
    pub fn complete(&mut self) {
        self.state = SessionState::Completed;
    }
}
