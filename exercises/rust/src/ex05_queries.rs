//! Exercise 05: data queries — iterators, grouping, sorting, borrowing from the input, SystemTime.

use std::collections::HashMap;
use std::time::SystemTime;

/// An event from a tunnel (e.g. a sensor alarm).
///
/// The standard library has no calendar (no dates, no time zones): only `SystemTime`, a point
/// in time you can compare and subtract. Real projects use the `chrono` or `jiff` crate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub tunnel: String,
    pub severity: u8, // 1 = info, 2 = warning, 3 = alarm
    pub at: SystemTime,
}

/// A summary of one tunnel's events.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TunnelStat {
    pub tunnel: String,
    pub count: usize,
    pub max_severity: u8,
}

/// The event count for each tunnel.
///
/// Unlike exercise 01, the keys are `&str` borrowed from the events: no copies. The map then
/// can't outlive `events`, and the compiler checks that for you.
pub fn count_by_tunnel(events: &[Event]) -> HashMap<&str, usize> {
    todo!()
}

/// The latest event for each tunnel (borrowed from the input).
pub fn latest_by_tunnel(events: &[Event]) -> HashMap<&str, &Event> {
    todo!()
}

/// At most n tunnels by event count descending, ties by name ascending.
///
/// Tip: aggregate in a HashMap with `entry(...).or_insert_with(...)`, then sort like in 01.
pub fn top_tunnels(events: &[Event], n: usize) -> Vec<TunnelStat> {
    todo!()
}

/// Events in the interval [from, to) sorted by time.
///
/// Go's test checks that the input slice isn't modified. Here `&[Event]` already guarantees it:
/// you can't sort through a shared reference, so you sort a new Vec of references.
/// Tip: `sort_by_key(|e| e.at)`, SystemTime is Ord and Copy.
pub fn between(events: &[Event], from: SystemTime, to: SystemTime) -> Vec<&Event> {
    todo!()
}

/// The event count by hour of day in UTC. A fixed-size array: always 24 elements.
///
/// Tip: `at.duration_since(UNIX_EPOCH)` gives the seconds since 1970-01-01 UTC (a Result, because
/// the time could be before 1970). Days have 86 400 seconds in Unix time, so the hour is
/// `secs / 3600 % 24`.
pub fn hourly_histogram(events: &[Event]) -> [usize; 24] {
    todo!()
}
