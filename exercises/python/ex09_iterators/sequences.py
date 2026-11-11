"""Exercise 09: iterators — generators (yield), lazy evaluation, infinite sequences.

Rule: no itertools module. Once the tests pass, compare with itertools.islice,
itertools.count and itertools.pairwise.
"""

from collections.abc import Callable, Iterable, Iterator, Mapping
from typing import Protocol, Self


class _Comparable(Protocol):
    def __lt__(self, other: Self, /) -> bool: ...


def step_range(start: int, end: int, step: int) -> Iterator[int]:
    """start, start+step, ... while the value is < end. step <= 0 -> ValueError right at the call.

    Trap: a function with `yield` doesn't run when called, only on the first next(). A check inside
    the generator would therefore raise only during iteration. Solution: this function (without yield)
    validates and returns the generator from a helper function _step_range (with yield).
    """
    raise NotImplementedError("TODO")


def fibonacci() -> Iterator[int]:
    """The infinite sequence 0, 1, 1, 2, 3, 5, ... Tip: while True: yield ..."""
    raise NotImplementedError("TODO")


def take[T](items: Iterable[T], n: int) -> Iterator[T]:
    """At most the first n elements. Must not read more from the source than needed.

    Tip: enumerate(items, start=1). Check for the end only AFTER yield, otherwise you needlessly
    pull the (n+1)-th element from the source.
    """
    raise NotImplementedError("TODO")


def filter_[T](items: Iterable[T], pred: Callable[[T], bool]) -> Iterator[T]:
    """Lazily lets through the elements for which pred returns True."""
    raise NotImplementedError("TODO")


def window[T](items: Iterable[T], size: int) -> Iterator[tuple[T, ...]]:
    """Sliding windows: [1, 2, 3, 4], 2 -> (1, 2), (2, 3), (3, 4). Must work on an infinite
    source too (must not copy the whole input into a list). Nothing for size <= 0 or too few elements.

    Tip: collections.deque(maxlen=size) drops the oldest element by itself when full.
    Return each window as tuple(...) — a tuple is immutable, so windows are independent.
    """
    raise NotImplementedError("TODO")


def sorted_items[K: _Comparable, V](mapping: Mapping[K, V]) -> Iterator[tuple[K, V]]:
    """(key, value) pairs in key order.
    (A dict remembers insertion order — not random like in Go, but not sorted either.)"""
    raise NotImplementedError("TODO")
