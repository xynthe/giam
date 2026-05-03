//! Memory layer types for GIAM
//!
//! Provides types for hierarchical memory management

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::content::StructuredContent;
use crate::temporal::{TimeWindow, Timestamp};

/// Memory layers representing different time scales of retention
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MemoryLayer {
    /// Immediate memory (seconds)
    Immediate,
    /// Working memory (hours)
    Working,
    /// Episodic memory (days)
    Episodic,
    /// Semantic memory (persistent)
    Semantic,
}

impl MemoryLayer {
    /// Returns the TTL for this memory layer
    pub fn ttl_seconds(&self) -> u64 {
        match self {
            MemoryLayer::Immediate => 60,
            MemoryLayer::Working => 3600,
            MemoryLayer::Episodic => 86400 * 7,
            MemoryLayer::Semantic => u64::MAX,
        }
    }
}

/// A memory entry stored in a memory layer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub id: Uuid,
    pub content: StructuredContent,
    pub layer: MemoryLayer,
    pub created_at: Timestamp,
    pub accessed_at: Timestamp,
    pub access_count: u32,
    pub importance: f64,
}

impl MemoryEntry {
    pub fn new(content: StructuredContent, layer: MemoryLayer) -> Self {
        let now = Timestamp::now();
        Self {
            id: Uuid::new_v4(),
            content,
            layer,
            created_at: now,
            accessed_at: now,
            access_count: 0,
            importance: 0.5,
        }
    }

    pub fn with_importance(mut self, importance: f64) -> Self {
        self.importance = importance.clamp(0.0, 1.0);
        self
    }

    pub fn access(&mut self) {
        self.access_count += 1;
        self.accessed_at = Timestamp::now();
    }
}

/// A query for searching memory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryQuery {
    /// The search pattern
    pub pattern: String,
    /// The memory layer to search
    pub layer: MemoryLayer,
    /// The time window to search within
    pub window: TimeWindow,
}

impl MemoryQuery {
    /// Creates a new memory query
    pub fn new(pattern: String, layer: MemoryLayer, window: TimeWindow) -> Self {
        Self {
            pattern,
            layer,
            window,
        }
    }
}

/// A memory store for storing and retrieving memories
pub struct MemoryStore {
    entries: HashMap<Uuid, MemoryEntry>,
    by_layer: HashMap<MemoryLayer, Vec<Uuid>>,
}

impl MemoryStore {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            by_layer: HashMap::new(),
        }
    }

    pub fn store(&mut self, entry: MemoryEntry) {
        let id = entry.id;
        let layer = entry.layer;
        self.entries.insert(id, entry);
        self.by_layer.entry(layer).or_default().push(id);
    }

    pub fn retrieve(&mut self, id: Uuid) -> Option<MemoryEntry> {
        self.entries.get_mut(&id).map(|e| {
            e.access();
            e.clone()
        })
    }

    pub fn search(&self, query: &MemoryQuery) -> Vec<&MemoryEntry> {
        let layer_entries = match self.by_layer.get(&query.layer) {
            Some(ids) => ids,
            None => return Vec::new(),
        };

        layer_entries
            .iter()
            .filter_map(|id| self.entries.get(id))
            .filter(|e| {
                if let Some(text) = e.content.as_text() {
                    text.contains(&query.pattern)
                } else {
                    false
                }
            })
            .collect()
    }

    pub fn all_at_layer(&self, layer: MemoryLayer) -> Vec<&MemoryEntry> {
        self.by_layer
            .get(&layer)
            .map(|ids| ids.iter().filter_map(|id| self.entries.get(id)).collect())
            .unwrap_or_default()
    }

    pub fn evict_expired(&mut self) -> Vec<Uuid> {
        let now = Timestamp::now();
        let mut evicted = Vec::new();

        for (layer, ids) in self.by_layer.iter_mut() {
            let ttl = layer.ttl_seconds();
            if ttl == u64::MAX {
                continue;
            }

            ids.retain(|id| {
                if let Some(entry) = self.entries.get(id) {
                    let age = now.duration_since(entry.created_at).as_secs();
                    if age > ttl {
                        evicted.push(*id);
                        return false;
                    }
                }
                true
            });
        }

        for id in &evicted {
            self.entries.remove(id);
        }

        evicted
    }
}

impl Default for MemoryStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_entry() {
        let entry = MemoryEntry::new(StructuredContent::text("test"), MemoryLayer::Working);
        assert_eq!(entry.access_count, 0);

        let mut entry = entry;
        entry.access();
        assert_eq!(entry.access_count, 1);
    }

    #[test]
    fn test_memory_store() {
        let mut store = MemoryStore::new();

        let entry = MemoryEntry::new(StructuredContent::text("hello"), MemoryLayer::Working);
        let id = entry.id;
        store.store(entry);

        let retrieved = store.retrieve(id);
        assert!(retrieved.is_some());
    }

    #[test]
    fn test_search() {
        let mut store = MemoryStore::new();
        store.store(MemoryEntry::new(
            StructuredContent::text("hello world"),
            MemoryLayer::Semantic,
        ));
        store.store(MemoryEntry::new(
            StructuredContent::text("goodbye"),
            MemoryLayer::Semantic,
        ));

        let query = MemoryQuery::new(
            "hello".to_string(),
            MemoryLayer::Semantic,
            TimeWindow::from_now(std::time::Duration::from_secs(3600)),
        );
        let results = store.search(&query);
        assert_eq!(results.len(), 1);
    }
}
