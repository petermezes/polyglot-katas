//! Exercise 08: dependency injection and testability — a TTL cache with an injected clock,
//! Mutex, Arc, trait objects.

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// An abstraction of time so tests can advance it.
pub trait Clock: Send + Sync {
    fn now(&self) -> Instant;
}

/// The real clock. `Instant` is monotonic: it never jumps back when the system time changes.
pub struct RealClock;

impl Clock for RealClock {
    fn now(&self) -> Instant {
        Instant::now()
    }
}

struct Entry<V> {
    value: V,
    expires_at: Instant,
}

/// A cache with entry expiration. Safe for concurrent use.
pub struct TtlCache<K, V> {
    ttl: Duration,
    clock: Arc<dyn Clock>,
    items: Mutex<HashMap<K, Entry<V>>>,
}

impl<K: Eq + Hash, V: Clone> TtlCache<K, V> {
    /// A cache with the real clock.
    pub fn new(ttl: Duration) -> Self {
        Self::with_clock(ttl, Arc::new(RealClock))
    }

    pub fn with_clock(ttl: Duration, clock: Arc<dyn Clock>) -> Self {
        TtlCache {
            ttl,
            clock,
            items: Mutex::new(HashMap::new()),
        }
    }

    /// Stores a value; valid until now + ttl (at now + ttl no longer valid).
    pub fn set(&self, key: K, value: V) {
        let expires_at = self.clock.now() + self.ttl;
        // The guard is a temporary here: the lock is released at the end of this statement.
        self.items
            .lock()
            .unwrap()
            .insert(key, Entry { value, expires_at });
    }

    /// A clone of the valid value. An expired entry is deleted right away.
    pub fn get(&self, key: &K) -> Option<V> {
        let mut items = self.items.lock().unwrap(); // held until `items` goes out of scope
        let entry = items.get(key)?;
        if self.clock.now() < entry.expires_at {
            return Some(entry.value.clone());
        }
        // `entry` borrows from `items`. Its last use is above, so the borrow is over by now and
        // remove (which needs `&mut items`) is allowed. The borrow checker tracks this per use.
        items.remove(key);
        None
    }

    /// The number of valid (unexpired) entries.
    pub fn len(&self) -> usize {
        let now = self.clock.now();
        let items = self.items.lock().unwrap();
        items.values().filter(|e| now < e.expires_at).count()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The value from the cache, or loads it with `load` and stores it.
    pub fn get_or_load<E>(&self, key: K, load: impl FnOnce(&K) -> Result<V, E>) -> Result<V, E> {
        if let Some(value) = self.get(&key) {
            return Ok(value);
        }
        // No lock here: get() released it. Two threads may both miss and both call load. That
        // is accepted here (as in the Go and C# solutions); avoiding it needs a per-key in-flight
        // marker (the "singleflight" pattern).
        let value = load(&key)?;
        self.set(key, value.clone());
        Ok(value)
    }
}
