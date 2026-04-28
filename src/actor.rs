//! Actor model types for GIAM
//!
//! Provides types for the actor model pattern

use std::collections::HashMap;
use std::sync::Arc;

use uuid::Uuid;

use crate::content::StructuredContent;
use crate::error::Result;

/// The actor trait for actor model implementation
pub trait Actor: Send + Sync {
    /// Performs an action with the given context
    fn act(&self, context: &StructuredContent) -> Result<StructuredContent>;
}

/// The actor system managing actors and message passing
pub struct ActorSystem {
    /// All actors in the system
    #[allow(dead_code)]
    actors: HashMap<Uuid, Arc<dyn Actor>>,
}

impl ActorSystem {
    /// Creates a new actor system
    pub fn new() -> Self {
        Self {
            actors: HashMap::new(),
        }
    }

    /// Spawns an actor
    pub fn spawn(&mut self, actor: Arc<dyn Actor>) -> Uuid {
        let id = Uuid::new_v4();
        self.actors.insert(id, actor);
        id
    }

    /// Returns the number of actors
    pub fn len(&self) -> usize {
        self.actors.len()
    }
}

impl Default for ActorSystem {
    fn default() -> Self {
        Self::new()
    }
}
