//! Execution trace types for GIAM
//!
//! Provides types for recording and replaying execution history

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::GiamLevel;
use crate::temporal::Timestamp;

/// The type of thought event
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThoughtEventType {
    /// An observation or perception
    Observation,
    /// A hypothesis or prediction
    Hypothesis,
    /// An intention or goal
    Intention,
    /// A reflection on past actions
    Reflection,
    /// An action taken
    Action,
    /// A warning message
    Warning,
    /// A success notification
    Success,
    /// A failure notification
    Failure,
}

/// An event representing a thought or cognitive event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThoughtEvent {
    /// Unique identifier for this event
    pub id: Uuid,
    /// The type of thought event
    pub event_type: ThoughtEventType,
    /// The content of the thought
    pub content: crate::content::StructuredContent,
    /// Confidence level from 0.0 to 1.0
    pub confidence: f64,
}

impl ThoughtEvent {
    /// Creates a new thought event
    pub fn new(
        event_type: ThoughtEventType,
        content: crate::content::StructuredContent,
        confidence: f64,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            event_type,
            content,
            confidence: confidence.clamp(0.0, 1.0),
        }
    }

    /// Returns whether this is a high confidence event (> 0.8)
    pub fn is_high_confidence(&self) -> bool {
        self.confidence > 0.8
    }

    /// Returns whether this is a low confidence event (< 0.5)
    pub fn is_low_confidence(&self) -> bool {
        self.confidence < 0.5
    }
}

/// Trace events recorded during execution
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TraceEvent {
    /// A thought event
    Thought(ThoughtEvent),
    /// A state change event
    StateChange {
        /// The previous state
        from: String,
        /// The new state
        to: String,
    },
    /// A user intervention event
    UserIntervention {
        /// The intervention action
        action: String,
    },
}

/// A complete execution trace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionTrace {
    /// Unique identifier for this trace
    pub id: Uuid,
    /// The recorded events
    pub events: Vec<TraceEvent>,
    /// The GIAM level this trace was created at
    pub level: GiamLevel,
    /// When this trace was started
    pub started_at: Timestamp,
}

impl ExecutionTrace {
    /// Creates a new execution trace
    pub fn new(level: GiamLevel) -> Self {
        Self {
            id: Uuid::new_v4(),
            events: Vec::new(),
            level,
            started_at: Timestamp::now(),
        }
    }

    /// Adds a trace event
    pub fn add_event(&mut self, event: TraceEvent) {
        self.events.push(event);
    }

    /// Adds a thought event
    pub fn add_thought(&mut self, thought: ThoughtEvent) {
        self.events.push(TraceEvent::Thought(thought));
    }

    /// Adds a state change event
    pub fn add_state_change(&mut self, from: String, to: String) {
        self.events.push(TraceEvent::StateChange { from, to });
    }

    /// Returns the number of events in this trace
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Returns whether this trace is empty
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}
