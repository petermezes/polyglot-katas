use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use under_test::ex05_queries::*;

/// 2026-11-01T00:00:00Z as Unix time. The standard library has no calendar to compute it.
const NOV_1_2026: u64 = 1_793_491_200;

fn at(day: u64, hour: u64, minute: u64) -> SystemTime {
    UNIX_EPOCH + Duration::from_secs(NOV_1_2026 + (day - 1) * 86_400 + hour * 3600 + minute * 60)
}

fn event(tunnel: &str, severity: u8, at: SystemTime) -> Event {
    Event {
        tunnel: tunnel.to_string(),
        severity,
        at,
    }
}

fn sample_events() -> Vec<Event> {
    vec![
        event("Løvstakk", 1, at(3, 8, 15)),
        event("Fløyfjell", 3, at(3, 7, 50)),
        event("Løvstakk", 2, at(3, 9, 0)),
        event("Damsgård", 1, at(2, 23, 59)),
        event("Fløyfjell", 1, at(3, 8, 5)),
        event("Løvstakk", 1, at(3, 6, 30)),
        event("Damsgård", 2, at(3, 8, 45)),
    ]
}

#[test]
fn count_by_tunnel_counts() {
    let events = sample_events();
    let want = HashMap::from([("Løvstakk", 3), ("Fløyfjell", 2), ("Damsgård", 2)]);
    assert_eq!(count_by_tunnel(&events), want);
}

#[test]
fn latest_by_tunnel_picks_latest() {
    let events = sample_events();
    let latest = latest_by_tunnel(&events);
    assert_eq!(latest["Løvstakk"].at, at(3, 9, 0));
    assert_eq!(
        latest["Fløyfjell"].severity, 1,
        "Fløyfjell: the latest event should have severity 1"
    );
    assert_eq!(latest.len(), 3);
}

#[test]
fn top_tunnels_by_count_then_name() {
    let stat = |tunnel: &str, count, max_severity| TunnelStat {
        tunnel: tunnel.to_string(),
        count,
        max_severity,
    };
    assert_eq!(
        top_tunnels(&sample_events(), 2),
        [
            stat("Løvstakk", 3, 2),
            stat("Damsgård", 2, 2), // Damsgård < Fløyfjell alphabetically
        ]
    );
    assert!(top_tunnels(&[], 5).is_empty());
}

#[test]
fn between_is_half_open_and_sorted() {
    // No "must not modify the input" check like in Go: `&[Event]` makes it impossible.
    let events = sample_events();
    let got = between(&events, at(3, 7, 50), at(3, 9, 0));
    let times: Vec<SystemTime> = got.iter().map(|e| e.at).collect();
    assert_eq!(
        times,
        [at(3, 7, 50), at(3, 8, 5), at(3, 8, 15), at(3, 8, 45)]
    );
}

#[test]
fn hourly_histogram_in_utc() {
    let hist = hourly_histogram(&sample_events());
    let want = HashMap::from([(6, 1), (7, 1), (8, 3), (9, 1), (23, 1)]);
    for (hour, &count) in hist.iter().enumerate() {
        assert_eq!(count, want.get(&hour).copied().unwrap_or(0), "hour {hour}");
    }
}
