from collections.abc import Iterator

import pytest

from ex09_iterators.sequences import fibonacci, filter_, sorted_items, step_range, take, window


def test_step_range():
    assert list(step_range(0, 10, 3)) == [0, 3, 6, 9]
    assert list(step_range(5, 5, 1)) == []


def test_step_range_validates_eagerly():
    with pytest.raises(ValueError):
        step_range(0, 10, 0)  # no list(): the error must be raised right at the call


def test_generator_can_be_consumed_only_once():
    numbers = step_range(0, 3, 1)
    assert list(numbers) == [0, 1, 2]
    assert list(numbers) == [], "an exhausted generator returns nothing more"


def test_break_on_huge_range():
    total = 0
    for v in step_range(0, 10**12, 1):
        if v == 3:
            break
        total += v
    assert total == 3


def test_fibonacci_take():
    assert list(take(fibonacci(), 10)) == [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
    assert list(take(fibonacci(), 0)) == []
    assert list(take(step_range(0, 3, 1), 10)) == [0, 1, 2]


def test_take_does_not_over_consume():
    pulled = []

    def source() -> Iterator[int]:
        for i in range(100):
            pulled.append(i)
            yield i

    assert list(take(source(), 3)) == [0, 1, 2]
    assert pulled == [0, 1, 2], "take must not pull more elements from the source than it returns"


def test_laziness():
    calls = 0

    def is_even(n: int) -> bool:
        nonlocal calls
        calls += 1
        return n % 2 == 0

    got = list(take(filter_(step_range(1, 1_000_000, 1), is_even), 3))
    assert got == [2, 4, 6]
    assert calls == 6, f"the predicate should be called 6× (lazy evaluation), was called {calls}×"


def test_window():
    assert list(window([1, 2, 3, 4], 2)) == [(1, 2), (2, 3), (3, 4)]
    assert list(window([1, 2], 3)) == []
    assert list(window([1, 2], 0)) == []


def test_window_on_infinite_source():
    assert list(take(window(fibonacci(), 3), 2)) == [(0, 1, 1), (1, 1, 2)]


def test_sorted_items():
    pairs = list(sorted_items({"c": 3, "a": 1, "b": 2}))
    assert pairs == [("a", 1), ("b", 2), ("c", 3)]
