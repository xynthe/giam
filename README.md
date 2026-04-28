# GIAM — General Intelligence Ascension Model

> A Rust implementation of the GIAM framework for building progressively autonomous intelligence systems.

[![Crates.io](https://img.shields.io/crates/v/giam.svg)](https://crates.io/crates/giam)
[![docs.rs](https://docs.rs/giam/badge.svg)](https://docs.rs/giam)
[![License](https://img.shields.io/badge/license-DOSL-blue.svg)](LICENSE)

---

## Overview

GIAM (General Intelligence Ascension Model) is a formal framework for building intelligence systems that **progress beyond AGI** through structured levels of execution authority, autonomy, and self-evolution.

This crate provides:

- **Core types** for all GIAM levels (AGI → SI → UI → HI → SPI → ULI → TI)
- **Trait definitions** for capability binding, routing, and actor systems
- **State machines** for execution tracking
- **Provenance and trace** primitives for determinism
- **Safety and constraint** primitives

---

## The GIAM Ladder

AGI is the entry threshold and is **not itself one of the ranked post-AGI levels**.

```
AGI → Entry Threshold

L1 → SI   (Super Intelligence)      Reliable task execution
L2 → UI   (Ultra Intelligence)      Persistent goal autonomy
L3 → HI   (Hyper Intelligence)      Recursive self-evolution
L4 → SPI  (Supreme Intelligence)     Multi-agent orchestration
L5 → ULI  (Ultimate Intelligence)    Global optimization
L6 → TI   (Transcendent Intelligence) Substrate independence
```

### Core Principle

> **Intelligence is not only what a system can understand. Intelligence is what a system can autonomously execute, sustain, adapt, govern, optimize, and transcend.**

Each level represents a qualitative shift in execution capability:

| Level | Execution Meaning |
|-------|-------------------|
| AGI | Understands tasks |
| SI | Executes tasks |
| UI | Executes goals |
| HI | Evolves execution |
| SPI | Controls execution systems |
| ULI | Optimizes execution globally |
| TI | Becomes execution infrastructure |

---

## Installation

```toml
[dependencies]
giam = "0.1"
```

### Features

```toml
[dependencies]
giam = { version = "0.1", features = ["full"] }
```

---

## Quick Start

```rust
use giam::prelude::*;

fn main() {
    // Create a GIAM level
    let level = GiamLevel::Si;
    println!("Level: {:?} (Tier {})", level, level.tier());
    
    // Create an intent
    let intent = Intent::new("Process user request", 50);
    println!("Intent: {}", intent.description);
    
    // Create a routing decision
    let decision = RoutingDecision {
        level: GiamLevel::Si,
        confidence: 0.95,
        rationale: "Task requires reliable execution".into(),
    };
    println!("Routed to: {:?}", decision.level);
}
```

---

## Architecture

### Core Modules

| Module | Description |
|--------|-------------|
| `core` | GiamLevel, ExecutionState, AutonomyIndex, TemporalHorizon |
| `temporal` | Timestamp, TimeWindow, TemporalInstant |
| `content` | StructuredContent (Text, Json, Binary) |
| `provenance` | ProvenanceChain, CausalLink, LinkType |
| `error` | GiamError enum and Result type |
| `capability` | Capability trait and CapabilityContract |
| `trace` | ThoughtEvent, TraceEvent, ExecutionTrace |
| `config` | GiamConfig |
| `intent` | Intent, GoalStatus |
| `routing` | RoutingDecision, Router trait |
| `memory` | MemoryLayer, MemoryQuery |
| `constraint` | ConstraintType, ConstraintToken |
| `safety` | SafetyPolicy, Action |
| `actor` | Actor trait, ActorSystem |
| `goals` | Goal |
| `session` | Session, SessionState |
| `planning` | ExecutionPlan, ExecutionStep, ExecutionResult |
| `validation` | Validator trait |

### Higher-Level Systems (Per-Level)

| Module | GIAM Level | Description |
|--------|-----------|-------------|
| `self_evolution` | HI | Recursive self-improvement |
| `multi_agent` | SPI | Multi-agent coordination |
| `optimization` | ULI | Global optimization engine |
| `substrate` | TI | Substrate abstraction |

---

## GIAM Levels

### AGI — Artificial General Intelligence

Entry threshold. General cognition with bounded execution capability.

### SI — Super Intelligence (L1)

Reliable, multi-step execution across domains without continuous human intervention.

```rust
use giam::core::{GiamLevel, ExecutionState};

let state = ExecutionState::Executing;
assert_eq!(GiamLevel::Si.tier(), 1);
```

### UI — Ultra Intelligence (L2)

Persistent autonomy. Goal ownership, long-horizon execution, adaptive strategy evolution.

### HI — Hyper Intelligence (L3)

Recursive self-evolution. Generates, evaluates, and integrates improvements to its own execution.

### SPI — Supreme Intelligence (L4)

Orchestration authority. Coordinates, controls, and governs multiple intelligent systems.

### ULI — Ultimate Intelligence (L5)

Global optimization. Achieves near-perfect efficiency across all controlled systems.

### TI — Transcendent Intelligence (L6)

Substrate independence. Operates across, redefines, and extends beyond computational constraints.

---

## Usage Patterns

### Building a GIAM-Aware Agent

```rust
use giam::prelude::*;

pub struct Agent {
    pub id: Uuid,
    pub level: GiamLevel,
    pub capabilities: Vec<Arc<dyn Capability>>,
    pub safety_policy: SafetyPolicy,
}

impl Agent {
    pub fn new(level: GiamLevel) -> Self {
        Self {
            id: Uuid::new_v4(),
            level,
            capabilities: Vec::new(),
            safety_policy: SafetyPolicy::default(),
        }
    }
    
    pub fn execute(&self, intent: Intent) -> Result<StructuredContent> {
        // Implementation
    }
}
```

### Implementing Capabilities

```rust
use giam::capability::{Capability, CapabilityContract};
use giam::prelude::*;

pub struct MyCapability;

impl Capability for MyCapability {
    fn name(&self) -> &str {
        "my_capability"
    }
    
    fn contract(&self) -> CapabilityContract {
        CapabilityContract {
            preconditions: vec!["input_validated".into()],
            effects: vec!["output_generated".into()],
            failure_modes: vec!["validation_failed".into()],
        }
    }
    
    fn invoke(&self, input: StructuredContent) -> Result<StructuredContent> {
        // Implementation
        Ok(StructuredContent::text("result"))
    }
}
```

### Routing Intents

```rust
use giam::routing::{Router, RoutingDecision};

pub struct GiamRouter {
    level_handlers: HashMap<GiamLevel, Arc<dyn Router>>,
}

impl GiamRouter {
    pub fn route(&self, intent: &Intent) -> RoutingDecision {
        // Route based on intent characteristics
        RoutingDecision {
            level: GiamLevel::Si,
            confidence: 0.9,
            rationale: "Standard task routing".into(),
        }
    }
}
```

---

## Safety

GIAM systems include safety primitives:

```rust
use giam::safety::SafetyPolicy;
use giam::constraint::{ConstraintType, ConstraintToken};

let policy = SafetyPolicy {
    allowed_actions: vec![Action::Read, Action::Write],
    blocked_actions: vec![Action::Delete, Action::SystemModify],
    approval_required: vec![Action::ExternalNetwork],
};

let token = ConstraintToken {
    id: Uuid::new_v4(),
    constraint_type: ConstraintType::EthicalBoundary,
    payload: StructuredContent::text("user_privacy"),
};
```

---

## Measurement

GIAM defines metrics per level:

| Metric | Description |
|--------|-------------|
| Autonomy Index | Proportion of work without human intervention (0.0-1.0) |
| Temporal Horizon | Duration of independent operation |
| Execution Breadth | Number of systems/tools managed |
| Adaptability Score | Adjustment to new contexts |

---

## Specification Reference

GIAM is defined by the [giam-spec](https://github.com/xynthe/giam-spec) repository:

- [AGI Model](giam-spec/0-agi.md) — Entry threshold
- [SI Model](giam-spec/1-super.md) — Super Intelligence
- [UI Model](giam-spec/2-ultra.md) — Ultra Intelligence
- [HI Model](giam-spec/3-hyper.md) — Hyper Intelligence
- [SPI Model](giam-spec/4-supreme.md) — Supreme Intelligence
- [ULI Model](giam-spec/5-ultimate.md) — Ultimate Intelligence
- [TI Model](giam-spec/6-transcendent.md) — Transcendent Intelligence

Engineering specifications: [giam-spec/01-giam-core-types.md](giam-spec/01-giam-core-types.md) through [giam-spec/28-configuration.md](giam-spec/28-configuration.md)

---

## Relationship to Xynthe

**Xynthe** is a separate orchestration platform that uses the GIAM family of intelligence models.

```
GIAM ← Intelligence Model (this crate)
Xynthe ← Orchestration Platform (uses GIAM)
```

GIAM defines the **what** (intelligence levels, capabilities, traits). Xynthe provides the **how** (runtime, execution engine, persistence).

---

## Contributing

Contributions welcome. See the specification references in `giam-spec/` for detailed engineering requirements.

---

## License

DOSL License. See [LICENSE](LICENSE).

---

## Closing Statement

> AGI is the threshold.  
> Ascension begins after that.