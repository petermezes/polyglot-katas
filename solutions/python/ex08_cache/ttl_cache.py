"""Exercise 08: testability — a TTL cache, time injected as a function, threading.Lock.

In Python functions are values, so the "Clock interface" is simply Callable[[], float].
The default is time.monotonic (unaffected by system clock changes), in tests a fake object with __call__.
"""

import threading
import time
from collections.abc import Callable

# Sentinel: a unique object to tell "nothing there" apart from a stored value None.
_MISSING = object()


class TtlCache[K, V]:
    """A cache with entry expiration (ttl in seconds). Safe for concurrent use from threads."""

    def __init__(self, ttl: float, clock: Callable[[], float] = time.monotonic) -> None:
        self._ttl = ttl
        self._clock = clock
        self._lock = threading.Lock()
        self._items: dict[K, tuple[V, float]] = {}  # key -> (value, expires_at)

    def set(self, key: K, value: V) -> None:
        """Stores a value; valid until now+ttl (at now+ttl no longer valid)."""
        with self._lock:  # `with` always releases the lock, even on exception (like defer Unlock / lock {})
            self._items[key] = (value, self._clock() + self._ttl)

    def get(self, key: K, default: object = None) -> object:
        """A valid value or default (like dict.get). An expired entry is deleted right away."""
        with self._lock:
            return self._get_locked(key, default)

    def _get_locked(self, key: K, default: object) -> object:
        """Assumes the caller holds the lock."""
        entry = self._items.get(key)
        if entry is None:
            return default
        value, expires_at = entry
        if self._clock() >= expires_at:
            del self._items[key]
            return default
        return value

    def __contains__(self, key: K) -> bool:
        """Enables `key in cache`."""
        with self._lock:
            return self._get_locked(key, _MISSING) is not _MISSING

    def __len__(self) -> int:
        """Number of valid entries."""
        with self._lock:
            now = self._clock()
            return sum(1 for _, expires_at in self._items.values() if now < expires_at)

    def get_or_load(self, key: K, load: Callable[[K], V]) -> V:
        """The value from the cache, or loads it with load and stores it. An exception from load propagates and nothing is stored."""
        cached = self.get(key, _MISSING)
        # Trap: `if cached is not None` would call load every time for a stored value None.
        if cached is not _MISSING:
            return cached  # type: ignore[return-value]
        # load runs outside the lock: it may be slow and may call the cache itself. threading.Lock is not
        # reentrant (RLock is), so get() under the lock would get stuck (deadlock).
        value = load(key)
        self.set(key, value)
        return value
