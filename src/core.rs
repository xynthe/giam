//! Core types for the GIAM system
//!
//! Contains the fundamental types that define the GIAM model:
//! - GiamLevel: The seven-tier intelligence hierarchy
//! - ExecutionState: State machine states for task execution
//! - TemporalHorizon: Time scale expectations for each level
//! - AutonomyIndex: Quantified autonomy metrics

use serde::{Deserialize, Serialize};
use std::fmt;
use std::time::Duration;

/// The seven-tier General Intelligence Ascension Model
///
/// Each level represents a distinct capability threshold:
/// - AGI: Entry threshold, general cognition
/// - SI (L1): Super Intelligence, reliable task execution
/// - UI (L2): Ultra Intelligence, persistent goal autonomy
/// - HI (L3): Hyper Intelligence, recursive self-evolution
/// - SPI (L4): Supreme Intelligence, multi-agent orchestration
/// - ULI (L5): Ultimate Intelligence, global optimization
/// - TI (L6): Transcendent Intelligence, substrate independence
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GiamLevel {
    /// Entry threshold - general cognition
    Agi,
    /// Level 1 - Reliable task execution
    Si,
    /// Level 2 - Persistent goal autonomy
    Ui,
    /// Level 3 - Recursive self-evolution
    Hi,
    /// Level 4 - Multi-agent orchestration
    Spi,
    /// Level 5 - Global optimization
    Uli,
    /// Level 6 - Substrate-independent
    Ti,
}

impl GiamLevel {
    /// Returns the tier number (0-6)
    pub fn tier(&self) -> u8 {
        match self {
            GiamLevel::Agi => 0,
            GiamLevel::Si => 1,
            GiamLevel::Ui => 2,
            GiamLevel::Hi => 3,
            GiamLevel::Spi => 4,
            GiamLevel::Uli => 5,
            GiamLevel::Ti => 6,
        }
    }

    /// Returns the human-readable name
    pub fn name(&self) -> &'static str {
        match self {
            GiamLevel::Agi => "AGI",
            GiamLevel::Si => "SI",
            GiamLevel::Ui => "UI",
            GiamLevel::Hi => "HI",
            GiamLevel::Spi => "SPI",
            GiamLevel::Uli => "ULI",
            GiamLevel::Ti => "TI",
        }
    }

    /// Returns the successor level, if any
    pub fn successor(&self) -> Option<Self> {
        match self {
            GiamLevel::Agi => Some(GiamLevel::Si),
            GiamLevel::Si => Some(GiamLevel::Ui),
            GiamLevel::Ui => Some(GiamLevel::Hi),
            GiamLevel::Hi => Some(GiamLevel::Spi),
            GiamLevel::Spi => Some(GiamLevel::Uli),
            GiamLevel::Uli => Some(GiamLevel::Ti),
            GiamLevel::Ti => None,
        }
    }

    /// Returns the predecessor level, if any
    pub fn predecessor(&self) -> Option<Self> {
        match self {
            GiamLevel::Agi => None,
            GiamLevel::Si => Some(GiamLevel::Agi),
            GiamLevel::Ui => Some(GiamLevel::Si),
            GiamLevel::Hi => Some(GiamLevel::Ui),
            GiamLevel::Spi => Some(GiamLevel::Hi),
            GiamLevel::Uli => Some(GiamLevel::Spi),
            GiamLevel::Ti => Some(GiamLevel::Uli),
        }
    }
}

impl fmt::Display for GiamLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Execution states for the GIAM state machine
///
/// Valid state transitions:
/// - Initialized → Perceiving
/// - Perceiving → Reasoning
/// - Reasoning → Assigning
/// - Assigning → Executing
/// - Executing → Reflecting or Failed
/// - Reflecting → Completed
/// - Any → Paused (human intervention)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutionState {
    /// System has been initialized
    Initialized,
    /// Actively perceiving input
    Perceiving,
    /// Reasoning about the input
    Reasoning,
    /// Assigning resources to tasks
    Assigning,
    /// Executing tasks
    Executing,
    /// Reflecting on execution
    Reflecting,
    /// Execution completed successfully
    Completed,
    /// Execution failed
    Failed,
    /// Execution paused (awaiting human input)
    Paused,
}

impl ExecutionState {
    /// Returns whether this is a terminal state
    pub fn is_terminal(&self) -> bool {
        matches!(self, ExecutionState::Completed | ExecutionState::Failed)
    }

    /// Returns whether this state represents an active/running condition
    pub fn is_active(&self) -> bool {
        matches!(
            self,
            ExecutionState::Perceiving
                | ExecutionState::Reasoning
                | ExecutionState::Assigning
                | ExecutionState::Executing
                | ExecutionState::Reflecting
        )
    }
}

impl fmt::Display for ExecutionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ExecutionState::Initialized => "Initialized",
            ExecutionState::Perceiving => "Perceiving",
            ExecutionState::Reasoning => "Reasoning",
            ExecutionState::Assigning => "Assigning",
            ExecutionState::Executing => "Executing",
            ExecutionState::Reflecting => "Reflecting",
            ExecutionState::Completed => "Completed",
            ExecutionState::Failed => "Failed",
            ExecutionState::Paused => "Paused",
        };
        write!(f, "{s}")
    }
}

/// State transition validation trait
pub trait StateTransition: Send + Sync {
    /// Returns the source state
    fn from(&self) -> ExecutionState;
    /// Returns the target state
    fn to(&self) -> ExecutionState;
    /// Returns whether this is a valid transition
    fn is_valid(&self) -> bool;
}

/// A concrete state transition with validation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StateTransitionImpl {
    from_state: ExecutionState,
    to_state: ExecutionState,
}

impl StateTransitionImpl {
    pub fn new(from: ExecutionState, to: ExecutionState) -> Self {
        Self {
            from_state: from,
            to_state: to,
        }
    }
}

impl StateTransition for StateTransitionImpl {
    fn from(&self) -> ExecutionState {
        self.from_state
    }

    fn to(&self) -> ExecutionState {
        self.to_state
    }

    fn is_valid(&self) -> bool {
        use ExecutionState::*;
        matches!(
            (self.from_state, self.to_state),
            (Initialized, Perceiving)
                | (Perceiving, Reasoning)
                | (Reasoning, Assigning)
                | (Assigning, Executing)
                | (Executing, Reflecting)
                | (Executing, Failed)
                | (Reflecting, Completed)
                | (_, Paused)
        )
    }
}

/// Autonomy index quantifying decision-making authority
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AutonomyIndex {
    /// Value from 0.0 (no autonomy) to 1.0 (full autonomy)
    pub value: f64,
    /// The associated GIAM level
    pub level: GiamLevel,
}

impl AutonomyIndex {
    /// Creates a new autonomy index
    pub fn new(value: f64, level: GiamLevel) -> Self {
        Self {
            value: value.clamp(0.0, 1.0),
            level,
        }
    }

    /// Returns the default autonomy index for a given level
    pub fn for_level(level: GiamLevel) -> Self {
        let value = match level {
            GiamLevel::Agi => 0.1,
            GiamLevel::Si => 0.3,
            GiamLevel::Ui => 0.5,
            GiamLevel::Hi => 0.7,
            GiamLevel::Spi => 0.8,
            GiamLevel::Uli => 0.9,
            GiamLevel::Ti => 1.0,
        };
        Self { value, level }
    }
}

/// Temporal horizon defining time scale expectations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalHorizon {
    /// Minimum expected duration
    pub min_duration: Duration,
    /// Maximum expected duration (None for unbounded)
    pub max_duration: Option<Duration>,
    /// Whether the horizon is continuous
    pub continuous: bool,
}

impl TemporalHorizon {
    /// Creates a new temporal horizon
    pub fn new(min_duration: Duration, max_duration: Option<Duration>, continuous: bool) -> Self {
        Self {
            min_duration,
            max_duration,
            continuous,
        }
    }

    /// Returns the default temporal horizon for a given level
    pub fn for_level(level: GiamLevel) -> Self {
        match level {
            GiamLevel::Agi => Self {
                min_duration: Duration::from_secs(60),
                max_duration: Some(Duration::from_secs(3600)),
                continuous: false,
            },
            GiamLevel::Si => Self {
                min_duration: Duration::from_secs(300),
                max_duration: Some(Duration::from_secs(14400)),
                continuous: false,
            },
            GiamLevel::Ui => Self {
                min_duration: Duration::from_secs(3600),
                max_duration: Some(Duration::from_secs(604800)),
                continuous: false,
            },
            GiamLevel::Hi => Self {
                min_duration: Duration::from_secs(1),
                max_duration: None,
                continuous: true,
            },
            GiamLevel::Spi => Self {
                min_duration: Duration::from_secs(1),
                max_duration: None,
                continuous: true,
            },
            GiamLevel::Uli => Self {
                min_duration: Duration::from_secs(1),
                max_duration: None,
                continuous: true,
            },
            GiamLevel::Ti => Self {
                min_duration: Duration::from_secs(0),
                max_duration: None,
                continuous: true,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_giam_level_tier() {
        assert_eq!(GiamLevel::Agi.tier(), 0);
        assert_eq!(GiamLevel::Ti.tier(), 6);
    }

    #[test]
    fn test_giam_level_successor() {
        assert_eq!(GiamLevel::Agi.successor(), Some(GiamLevel::Si));
        assert_eq!(GiamLevel::Ti.successor(), None);
    }

    #[test]
    fn test_execution_state_active() {
        assert!(ExecutionState::Perceiving.is_active());
        assert!(ExecutionState::Executing.is_active());
        assert!(!ExecutionState::Completed.is_active());
    }

    #[test]
    fn test_autonomy_index() {
        let idx = AutonomyIndex::for_level(GiamLevel::Ui);
        assert_eq!(idx.level, GiamLevel::Ui);
        assert!((idx.value - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_state_transition_valid() {
        let transition =
            StateTransitionImpl::new(ExecutionState::Initialized, ExecutionState::Perceiving);
        assert!(transition.is_valid());

        let transition =
            StateTransitionImpl::new(ExecutionState::Perceiving, ExecutionState::Reasoning);
        assert!(transition.is_valid());
    }

    #[test]
    fn test_state_transition_invalid() {
        let transition =
            StateTransitionImpl::new(ExecutionState::Initialized, ExecutionState::Completed);
        assert!(!transition.is_valid());

        let transition =
            StateTransitionImpl::new(ExecutionState::Completed, ExecutionState::Perceiving);
        assert!(!transition.is_valid());
    }

    #[test]
    fn test_state_transition_to_paused() {
        let transition =
            StateTransitionImpl::new(ExecutionState::Executing, ExecutionState::Paused);
        assert!(transition.is_valid());

        let transition =
            StateTransitionImpl::new(ExecutionState::Completed, ExecutionState::Paused);
        assert!(transition.is_valid());
    }
}
