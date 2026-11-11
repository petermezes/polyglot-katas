//! Exercise 08: dependency injection and testability — a TTL cache with an injected clock,
//! Mutex, Arc, trait objects.

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// An abstraction of time so tests can advance it.
///
/// `Send + Sync`: a clock may be used from several threads at once (the cache is shared).
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

/// One stored value. Tip: `struct Entry<V> { value: V, expires_at: Instant }`.
struct Entry<V> {
    value: V,
    expires_at: Instant,
}

/// A cache with entry expiration. Safe for concurrent use.
///
/// Every method takes `&self`, even `set`: the data sits behind a Mutex, which hands out a
/// mutable view only to whoever holds the lock (interior mutability). So a cache shared by
/// several threads needs no `&mut`, and the compiler won't let you touch `items` without locking.
pub struct TtlCache<K, V> {
    ttl: Duration,
    clock: Arc<dyn Clock>,
    items: Mutex<HashMap<K, Entry<V>>>,
}

impl<K: Eq + Hash, V: Clone> TtlCache<K, V> {
    /// A cache with the real clock.
    pub fn new(ttl: Duration) -> Self {
        todo!()
    }

    /// A cache with the given clock. `Arc<dyn Clock>` = shared ownership: the test keeps its own
    /// Arc to the fake clock to advance it, the cache holds another one.
    pub fn with_clock(ttl: Duration, clock: Arc<dyn Clock>) -> Self {
        todo!()
    }

    /// Stores a value; valid until now + ttl (at now + ttl no longer valid).
    ///
    /// Tip: `self.items.lock().unwrap()` returns a guard that unlocks when it goes out of scope.
    /// `unwrap()` panics only if another thread panicked while holding the lock ("poisoning").
    pub fn set(&self, key: K, value: V) {
        todo!()
    }

    /// A clone of the valid value. An expired entry is deleted right away.
    ///
    /// Why a clone and not `Option<&V>`: the reference would point into the map, but the map is
    /// accessible only while the lock is held, and the lock ends with this method.
    pub fn get(&self, key: &K) -> Option<V> {
        todo!()
    }

    /// The number of valid (unexpired) entries.
    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    /// The value from the cache, or loads it with `load` and stores it. An error from `load`
    /// is returned and nothing is stored.
    ///
    /// Careful: don't hold the lock while `load` runs. It may be slow (blocking everyone else),
    /// and std's Mutex isn't reentrant: if load itself used the cache, it would deadlock.
    /// Reusing get() and set() solves both: each takes and releases the lock on its own.
    pub fn get_or_load<E>(&self, key: K, load: impl FnOnce(&K) -> Result<V, E>) -> Result<V, E> {
        todo!()
    }
}
