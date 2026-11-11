import threading
from concurrent.futures import ThreadPoolExecutor

import pytest

from ex08_cache.ttl_cache import TtlCache


class FakeClock:
    """A test clock — time is advanced manually. The instance is callable thanks to __call__,
    so the cache uses it just like time.monotonic."""

    def __init__(self, start: float = 1_000.0) -> None:
        self.now = start

    def __call__(self) -> float:
        return self.now

    def advance(self, seconds: float) -> None:
        self.now += seconds


def test_set_get_expire():
    clock = FakeClock()
    cache = TtlCache[str, int](60, clock)

    cache.set("a", 1)
    assert cache.get("a") == 1
    assert cache.get("b") is None
    assert cache.get("b", -1) == -1

    clock.advance(59)
    assert "a" in cache, "after 59 s the entry should still be valid"

    clock.advance(1)
    assert "a" not in cache, "exactly at TTL the entry is no longer valid"
    assert cache.get("a") is None


def test_set_refreshes_ttl():
    clock = FakeClock()
    cache = TtlCache[str, str](60, clock)
    cache.set("k", "v1")
    clock.advance(50)
    cache.set("k", "v2")
    clock.advance(50)
    assert cache.get("k") == "v2"


def test_len_ignores_expired():
    clock = FakeClock()
    cache = TtlCache[int, str](10, clock)
    cache.set(1, "x")
    clock.advance(5)
    cache.set(2, "y")
    assert len(cache) == 2
    clock.advance(5)
    assert len(cache) == 1


def test_get_or_load():
    clock = FakeClock()
    cache = TtlCache[str, str](60, clock)
    calls = 0

    def load(key: str) -> str:
        nonlocal calls  # without nonlocal, `calls += 1` would create a new local variable
        calls += 1
        return f"value-{key}"

    for _ in range(3):
        assert cache.get_or_load("x", load) == "value-x"
    assert calls == 1

    clock.advance(60)
    cache.get_or_load("x", load)
    assert calls == 2, "after expiration load should be called again"


def test_get_or_load_does_not_store_on_failure():
    cache = TtlCache[str, str](60, FakeClock())

    def failing(key: str) -> str:
        raise ConnectionError("database unavailable")

    with pytest.raises(ConnectionError):
        cache.get_or_load("y", failing)
    assert "y" not in cache


def test_get_or_load_caches_none():
    cache = TtlCache[str, None](60, FakeClock())
    calls = []
    for _ in range(3):
        cache.get_or_load("n", lambda key: calls.append(key))  # append returns None
    assert len(calls) == 1, "a stored None is a valid value too"


def test_get_or_load_does_not_hold_lock_while_loading():
    cache = TtlCache[str, str](60, FakeClock())
    result = []

    def load(key: str) -> str:
        return f"{cache.get('other', 'none')}-{key}"  # load itself uses the cache

    worker = threading.Thread(target=lambda: result.append(cache.get_or_load("x", load)), daemon=True)
    worker.start()
    worker.join(timeout=2)
    assert not worker.is_alive(), "deadlock: load is called under the lock"
    assert result == ["none-x"]


def test_default_clock_uses_real_time():
    cache = TtlCache[str, int](3600)
    cache.set("a", 1)
    assert cache.get("a") == 1


def test_concurrent_access():
    cache = TtlCache[str, int](60, FakeClock())

    def work(i: int) -> None:
        key = str(i % 5)
        cache.set(key, i)
        cache.get(key)
        len(cache)

    with ThreadPoolExecutor(max_workers=8) as pool:
        list(pool.map(work, range(200)))  # list() forces completion and propagates any exceptions
    assert len(cache) == 5
