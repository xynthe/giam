//! Event system for GIAM
//!
//! Provides an event system for agent communication and state changes

use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::content::StructuredContent;
use crate::temporal::Timestamp;

/// Event priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventPriority {
    Low,
    Normal,
    High,
    Critical,
}

/// A single event in the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub event_type: String,
    pub source: Uuid,
    pub target: Option<Uuid>,
    pub payload: StructuredContent,
    pub priority: EventPriority,
    pub timestamp: Timestamp,
}

impl Event {
    pub fn new(event_type: String, source: Uuid, payload: StructuredContent) -> Self {
        Self {
            id: Uuid::new_v4(),
            event_type,
            source,
            target: None,
            payload,
            priority: EventPriority::Normal,
            timestamp: Timestamp::now(),
        }
    }

    pub fn with_target(mut self, target: Uuid) -> Self {
        self.target = Some(target);
        self
    }

    pub fn with_priority(mut self, priority: EventPriority) -> Self {
        self.priority = priority;
        self
    }
}

/// Event handler trait
pub trait EventHandler: Send + Sync {
    fn handle(&self, event: &Event);
}

/// The event bus for distributing events
pub struct EventBus {
    handlers: HashMap<String, Vec<Arc<dyn EventHandler>>>,
    event_history: Vec<Event>,
    max_history: usize,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
            event_history: Vec::new(),
            max_history: 1000,
        }
    }

    pub fn subscribe(&mut self, event_type: String, handler: Arc<dyn EventHandler>) {
        self.handlers.entry(event_type).or_default().push(handler);
    }

    pub fn publish(&mut self, event: Event) {
        let event_type = event.event_type.clone();

        if let Some(handlers) = self.handlers.get(&event_type) {
            for handler in handlers {
                handler.handle(&event);
            }
        }

        if self.event_history.len() >= self.max_history {
            self.event_history.remove(0);
        }
        self.event_history.push(event);
    }

    pub fn history(&self) -> &[Event] {
        &self.event_history
    }

    pub fn recent_events(&self, count: usize) -> Vec<&Event> {
        self.event_history.iter().rev().take(count).collect()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

/// A simple event handler that prints events
pub struct LoggingEventHandler;

impl EventHandler for LoggingEventHandler {
    fn handle(&self, event: &Event) {
        println!(
            "[{:?}] {} from {:?} -> {:?}",
            event.priority, event.event_type, event.source, event.target
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_creation() {
        let event = Event::new(
            "test".to_string(),
            Uuid::new_v4(),
            StructuredContent::text("data".to_string()),
        );
        assert_eq!(event.priority, EventPriority::Normal);
    }

    #[test]
    fn test_event_with_target() {
        let target = Uuid::new_v4();
        let event = Event::new(
            "test".to_string(),
            Uuid::new_v4(),
            StructuredContent::text("data".to_string()),
        )
        .with_target(target);
        assert_eq!(event.target, Some(target));
    }

    #[test]
    fn test_event_bus() {
        let mut bus = EventBus::new();

        let handler = Arc::new(LoggingEventHandler);
        bus.subscribe("test".to_string(), handler);

        let event = Event::new(
            "test".to_string(),
            Uuid::new_v4(),
            StructuredContent::text("data".to_string()),
        );
        bus.publish(event);

        assert_eq!(bus.history().len(), 1);
    }
}
