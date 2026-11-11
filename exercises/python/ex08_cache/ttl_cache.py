"""Exercise 08: testability — a TTL cache, time injected as a function, threading.Lock.

In Python functions are values, so the "Clock interface" is simply Callable[[], float].
The default is time.monotonic (unaffected by system clock changes), in tests a fake object
with a __call__ method (see FakeClock in test_cache.py).
"""

import threading
import time
from collections.abc import Callable

# Sentinel: a unique object to tell "nothing there" apart from a stored value None.
_MISSING = object()


class TtlCache[K, V]:
    """A cache with entry expiration (ttl in seconds). Safe for concurrent use from threads."""

    def __init__(self, ttl: float, clock: Callable[[], float] = time.monotonic) -> None:
        """Tip: self._lock = threading.Lock(), self._items: dict[K, tuple[V, float]] = {}
        (key -> (value, expiration time))."""
        raise NotImplementedError("TODO")

    def set(self, key: K, value: V) -> None:
        """Stores a value; valid until now+ttl (at exactly now+ttl no longer valid).

        Tip: `with self._lock:` always releases the lock, even on exception (like defer Unlock / lock {}).
        """
        raise NotImplementedError("TODO")

    def get(self, key: K, default: object = None) -> object:
        """A valid value or default (like dict.get). An expired entry is deleted right away."""
        raise NotImplementedError("TODO")

    def __contains__(self, key: K) -> bool:
        """Enables `key in cache`: True only for a valid entry. Watch out for a stored value None."""
        raise NotImplementedError("TODO")

    def __len__(self) -> int:
        """Number of valid (unexpired) entries."""
        raise NotImplementedError("TODO")

    def get_or_load(self, key: K, load: Callable[[K], V]) -> V:
        """The value from the cache, or loads it with load and stores it.
        An exception from load reaches the caller and nothing is stored.

        Tip: don't call load while holding the lock — it may be slow and may call the cache itself.
        threading.Lock is not reentrant (RLock is), so get() under the lock = deadlock.
        Careful: `if value is None` isn't enough to tell whether the value is in the cache. That's what _MISSING is for.
        """
        raise NotImplementedError("TODO")
