use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::{Duration, Instant};

use under_test::ex08_cache::*;

/// A test implementation of Clock: time moves only when the test says so.
///
/// An Instant can't be built from a date, so the fake starts at "now" and adds an offset.
/// The Mutex is needed because Clock::now takes &self and the clock is shared (Send + Sync).
struct FakeClock {
    start: Instant,
    offset: Mutex<Duration>,
}

impl FakeClock {
    fn new() -> Arc<FakeClock> {
        Arc::new(FakeClock {
            start: Instant::now(),
            offset: Mutex::new(Duration::ZERO),
        })
    }

    fn advance(&self, d: Duration) {
        *self.offset.lock().unwrap() += d;
    }
}

impl Clock for FakeClock {
    fn now(&self) -> Instant {
        self.start + *self.offset.lock().unwrap()
    }
}

fn secs(n: u64) -> Duration {
    Duration::from_secs(n)
}

#[test]
fn set_get_expire() {
    let clock = FakeClock::new();
    // clock.clone() clones the Arc (a new pointer to the same clock), coerced to Arc<dyn Clock>.
    let cache = TtlCache::with_clock(secs(60), clock.clone());

    cache.set("a", 1);
    assert_eq!(cache.get(&"a"), Some(1));
    assert_eq!(cache.get(&"b"), None);

    clock.advance(secs(59));
    assert_eq!(
        cache.get(&"a"),
        Some(1),
        "after 59 s the entry should still be valid"
    );

    clock.advance(secs(1));
    assert_eq!(
        cache.get(&"a"),
        None,
        "exactly at TTL the entry is no longer valid"
    );
}

#[test]
fn set_refreshes_ttl() {
    let clock = FakeClock::new();
    let cache = TtlCache::with_clock(secs(60), clock.clone());
    cache.set("k", "v1");
    clock.advance(secs(50));
    cache.set("k", "v2");
    clock.advance(secs(50));
    assert_eq!(cache.get(&"k"), Some("v2"), "set should refresh the TTL");
}

#[test]
fn len_counts_only_valid_entries() {
    let clock = FakeClock::new();
    let cache = TtlCache::with_clock(secs(10), clock.clone());
    cache.set(1, "x");
    clock.advance(secs(5));
    cache.set(2, "y");
    assert_eq!(cache.len(), 2);
    clock.advance(secs(5));
    assert_eq!(cache.len(), 1, "len after expiration");
    assert!(!cache.is_empty());
}

#[test]
fn get_or_load_loads_once_and_skips_errors() {
    let clock = FakeClock::new();
    let cache = TtlCache::with_clock(secs(60), clock.clone());
    let mut calls = 0;
    // A closure that borrows `calls` mutably. FnOnce is enough: each call gets a new closure.
    let mut load = |k: &&str| -> Result<String, String> {
        calls += 1;
        Ok(format!("value-{k}"))
    };

    for _ in 0..3 {
        assert_eq!(cache.get_or_load("x", &mut load), Ok("value-x".to_string()));
    }

    clock.advance(secs(60));
    assert_eq!(cache.get_or_load("x", &mut load), Ok("value-x".to_string()));
    assert_eq!(
        calls, 2,
        "load once, then again after expiration (calls={calls})"
    );

    let failed = cache.get_or_load("y", |_| Err("database unavailable".to_string()));
    assert_eq!(failed, Err("database unavailable".to_string()));
    assert_eq!(cache.get(&"y"), None, "nothing should be stored on error");
}

#[test]
fn get_or_load_may_use_the_cache_inside_load() {
    // If get_or_load held the lock while calling load, this would deadlock (std's Mutex isn't
    // reentrant). It runs in its own thread so the test fails after a timeout instead of hanging.
    // thread::spawn needs 'static data, hence an Arc instead of a borrow.
    let cache = Arc::new(TtlCache::new(secs(60)));
    let (tx, rx) = mpsc::channel();
    let worker_cache = Arc::clone(&cache);
    thread::spawn(move || {
        let got = worker_cache.get_or_load("x", |k| {
            let other = worker_cache
                .get(&"other")
                .unwrap_or_else(|| "none".to_string());
            Ok::<_, ()>(format!("{other}-{k}"))
        });
        let _ = tx.send(got);
    });
    match rx.recv_timeout(Duration::from_secs(2)) {
        Ok(got) => assert_eq!(got, Ok("none-x".to_string())),
        Err(mpsc::RecvTimeoutError::Timeout) => panic!("deadlock: load is called under the lock"),
        Err(mpsc::RecvTimeoutError::Disconnected) => {
            panic!("get_or_load panicked in the worker thread, see its message above")
        }
    }
}

#[test]
fn new_uses_real_time() {
    let cache = TtlCache::new(Duration::from_secs(3600));
    cache.set("a", 1);
    assert_eq!(
        cache.get(&"a"),
        Some(1),
        "with the real clock the entry should be valid"
    );
}

#[test]
fn concurrent_access() {
    let cache = TtlCache::with_clock(secs(60), FakeClock::new());
    // Scoped threads may borrow `cache` from this function: the scope waits for all of them
    // before it returns, so `cache` outlives every thread. No Arc needed for the cache itself.
    thread::scope(|s| {
        for i in 0..50 {
            let cache = &cache;
            s.spawn(move || {
                let key = (i % 5).to_string();
                cache.set(key.clone(), i);
                cache.get(&key);
                cache.len();
            });
        }
    });
    assert_eq!(cache.len(), 5);
}
