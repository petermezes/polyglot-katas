//! Exercise 05: data queries — iterators, grouping, sorting, borrowing from the input, SystemTime.

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

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
pub fn count_by_tunnel(events: &[Event]) -> HashMap<&str, usize> {
    let mut counts = HashMap::new();
    for e in events {
        // e.tunnel.as_str() borrows from the event. Exercise 01 cloned the keys instead.
        *counts.entry(e.tunnel.as_str()).or_insert(0) += 1;
    }
    counts
}

/// The latest event for each tunnel (borrowed from the input).
pub fn latest_by_tunnel(events: &[Event]) -> HashMap<&str, &Event> {
    let mut latest: HashMap<&str, &Event> = HashMap::new();
    for e in events {
        latest
            .entry(&e.tunnel)
            .and_modify(|cur| {
                if e.at > cur.at {
                    *cur = e;
                }
            })
            .or_insert(e);
    }
    latest
}

/// At most n tunnels by event count descending, ties by name ascending.
pub fn top_tunnels(events: &[Event], n: usize) -> Vec<TunnelStat> {
    let mut by_tunnel: HashMap<&str, TunnelStat> = HashMap::new();
    for e in events {
        // or_insert_with runs the closure only for a new key (or_insert would build the
        // TunnelStat, including the String allocation, on every call).
        let st = by_tunnel.entry(&e.tunnel).or_insert_with(|| TunnelStat {
            tunnel: e.tunnel.clone(),
            count: 0,
            max_severity: 0,
        });
        st.count += 1;
        st.max_severity = st.max_severity.max(e.severity);
    }
    let mut stats: Vec<TunnelStat> = by_tunnel.into_values().collect();
    stats.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.tunnel.cmp(&b.tunnel)));
    stats.truncate(n);
    stats
}

/// Events in the interval [from, to) sorted by time.
pub fn between(events: &[Event], from: SystemTime, to: SystemTime) -> Vec<&Event> {
    let mut out: Vec<&Event> = events
        .iter()
        .filter(|e| from <= e.at && e.at < to)
        .collect();
    // sort_by_key is stable: events with the same time keep their input order.
    out.sort_by_key(|e| e.at);
    out
}

/// The event count by hour of day in UTC. A fixed-size array: always 24 elements.
pub fn hourly_histogram(events: &[Event]) -> [usize; 24] {
    let mut hist = [0; 24];
    for e in events {
        let secs =
            e.at.duration_since(UNIX_EPOCH)
                .expect("event before 1970")
                .as_secs();
        // Unix time ignores leap seconds, so every day has exactly 86 400 s and this is exact.
        hist[(secs / 3600 % 24) as usize] += 1;
    }
    hist
}
