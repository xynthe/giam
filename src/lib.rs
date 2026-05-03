//! GIAM - General Intelligence Ascension Model
//!
//! A framework for building tiered intelligence systems across seven levels:
//! - AGI (L0): Entry threshold - general cognition
//! - SI (L1): Super Intelligence - reliable task execution
//! - UI (L2): Ultra Intelligence - persistent goal autonomy
//! - HI (L3): Hyper Intelligence - recursive self-evolution
//! - SPI (L4): Supreme Intelligence - multi-agent orchestration
//! - ULI (L5): Ultimate Intelligence - global optimization
//! - TI (L6): Transcendent Intelligence - substrate independence

pub mod actor;
pub mod api;
pub mod autonomy;
pub mod benchmark;
pub mod capability;
pub mod capability_registry;
pub mod config;
pub mod constraint;
pub mod content;
pub mod events;
pub mod core;
pub mod error;
pub mod execution;
pub mod goals;
pub mod intent;
pub mod memory;
pub mod metrics;
pub mod multi_agent;
pub mod optimization;
pub mod planning;
pub mod provenance;
pub mod routing;
pub mod safety;
pub mod self_evolution;
pub mod session;
pub mod substrate;
pub mod temporal;
pub mod trace;
pub mod validation;
pub mod workflow;

// Re-exports for public API
pub use self::core::{AutonomyIndex, ExecutionState, GiamLevel, TemporalHorizon};
pub use self::error::{GiamError, Result};
pub use self::temporal::{TemporalInstant, TimeWindow, Timestamp};
pub use self::content::StructuredContent;
pub use self::provenance::{CausalLink, LinkType, ProvenanceChain};
pub use self::trace::{ExecutionTrace, ThoughtEvent, ThoughtEventType, TraceEvent};
pub use self::config::GiamConfig;
pub use self::intent::{GoalStatus, Intent};
pub use self::capability::{Capability, CapabilityContract};
pub use self::autonomy::AutonomyBoundary;
pub use self::metrics::LevelMetrics;
pub use self::benchmark::{Benchmark, BenchmarkResult};
pub use self::capability_registry::CapabilityRegistry;
pub use self::execution::{ExecutionConfig, ExecutionEngine};
pub use self::api::{GiamClient, ExecuteRequest, ExecuteResponse, RouteRequest, RouteResponse};
pub use self::events::{Event, EventBus, EventHandler, EventPriority};
pub use self::workflow::{Workflow, WorkflowEngine, WorkflowStep, WorkflowStatus};