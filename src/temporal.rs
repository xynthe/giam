//! Temporal types for GIAM
//!
//! Provides time-related types for tracking and managing execution time

use chrono::{DateTime, Duration as ChronoDuration, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::time::Duration;

/// A timestamp representing a point in time
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
pub struct Timestamp(pub DateTime<Utc>);

impl Timestamp {
    /// Creates a new timestamp from a DateTime
    pub fn new(dt: DateTime<Utc>) -> Self {
        Self(dt)
    }

    /// Returns the current timestamp
    pub fn now() -> Self {
        Self(Utc::now())
    }

    /// Calculates the duration since another timestamp
    pub fn duration_since(&self, other: Timestamp) -> Duration {
        let diff = self.0.signed_duration_since(other.0);
        ChronoDuration::to_std(&diff).unwrap_or_default()
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::now()
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A time window representing a span between two timestamps
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeWindow {
    /// The start timestamp
    pub start: Timestamp,
    /// The end timestamp
    pub end: Timestamp,
}

impl TimeWindow {
    /// Creates a new time window with the given start and end
    pub fn new(start: Timestamp, end: Timestamp) -> Self {
        Self { start, end }
    }

    /// Creates a time window starting from now for the given duration
    pub fn from_now(duration: Duration) -> Self {
        let start = Timestamp::now();
        let end = Timestamp(start.0 + ChronoDuration::from_std(duration).unwrap_or_default());
        Self { start, end }
    }

    /// Checks if this window contains the given timestamp
    pub fn contains(&self, ts: Timestamp) -> bool {
        ts >= self.start && ts <= self.end
    }

    /// Returns the duration of this window
    pub fn duration(&self) -> Duration {
        self.start.duration_since(self.end)
    }
}

/// A temporal instant with a timestamp and sequence number
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalInstant {
    /// The timestamp
    pub timestamp: Timestamp,
    /// The sequence number for ordering
    pub sequence: u64,
}

impl TemporalInstant {
    /// Creates a new temporal instant
    pub fn new(timestamp: Timestamp, sequence: u64) -> Self {
        Self {
            timestamp,
            sequence,
        }
    }

    /// Creates a new instant with the current time
    pub fn now(sequence: u64) -> Self {
        Self {
            timestamp: Timestamp::now(),
            sequence,
        }
    }
}
