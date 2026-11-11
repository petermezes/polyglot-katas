"""Tests are plain functions that run asyncio.run(...). No pytest-asyncio plugin needed."""

import asyncio
import time

import pytest

from ex10_concurrency.concurrency import first_success, process_all


def test_process_all_keeps_order_and_limit():
    # The event loop runs in one thread, so plain int counters are enough (no atomics/Interlocked).
    active = 0
    max_active = 0

    async def double(n: int) -> int:
        nonlocal active, max_active
        active += 1
        max_active = max(max_active, active)
        await asyncio.sleep((10 - n) / 1000)  # later inputs finish sooner
        active -= 1
        return n * 2

    results = asyncio.run(process_all(list(range(10)), 3, double))

    assert results == [0, 2, 4, 6, 8, 10, 12, 14, 16, 18], "results must be in input order"
    assert 2 <= max_active <= 3, f"max running at once = {max_active}, expected 2–3"


def test_process_all_stops_on_first_error_and_raises_it():
    started = 0

    async def f(n: int) -> int:
        nonlocal started
        started += 1
        if n == 2:
            raise RuntimeError("sensor not responding")
        await asyncio.sleep(0.05)
        return n

    start = time.perf_counter()
    with pytest.raises(RuntimeError, match="sensor not responding"):
        asyncio.run(process_all(list(range(100)), 4, f))
    elapsed = time.perf_counter() - start

    assert 3 <= started <= 10, f"no further tasks should start after an error, started: {started}"
    assert elapsed < 0.5, f"took {elapsed:.2f} s — cancellation doesn't work"


def test_process_all_respects_external_cancellation():
    async def slow(n: int) -> int:
        await asyncio.sleep(1)
        return n

    async def main() -> None:
        async with asyncio.timeout(0.03):  # like context.WithTimeout / CancellationTokenSource(30 ms)
            await process_all([1, 2, 3, 4, 5], 2, slow)

    start = time.perf_counter()
    with pytest.raises(TimeoutError):
        asyncio.run(main())
    assert time.perf_counter() - start < 0.5


def test_first_success_returns_fastest_and_cancels_others():
    cancelled = 0

    async def slow() -> str:
        nonlocal cancelled
        try:
            await asyncio.sleep(2)
            return "slow"
        except asyncio.CancelledError:
            cancelled += 1
            raise  # CancelledError must be re-raised, otherwise the task "can't be cancelled"

    async def fast() -> str:
        await asyncio.sleep(0.01)
        return "fast"

    async def failing() -> str:
        raise RuntimeError("fails immediately")

    async def main() -> str:
        result = await first_success(slow, failing, fast, slow)
        deadline = time.perf_counter() + 1
        while cancelled < 2 and time.perf_counter() < deadline:
            await asyncio.sleep(0.005)
        return result

    assert asyncio.run(main()) == "fast"
    assert cancelled == 2, f"slow functions should be cancelled, cancelled: {cancelled}"


def test_first_success_all_fail_raises_exception_group():
    async def first() -> int:
        raise RuntimeError("first")

    async def second() -> int:
        raise TimeoutError("second")

    with pytest.raises(ExceptionGroup) as exc_info:
        asyncio.run(first_success(first, second))
    assert len(exc_info.value.exceptions) == 2

    # except* (3.11+) picks only errors of the given type out of an ExceptionGroup.
    caught = []
    try:
        asyncio.run(first_success(first, second))
    except* TimeoutError as group:
        caught.extend(group.exceptions)
    except* RuntimeError:
        pass
    assert [str(e) for e in caught] == ["second"]


def test_first_success_without_functions():
    with pytest.raises(ValueError):
        asyncio.run(first_success())
