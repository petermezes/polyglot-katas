"""Exercise 09: iterators — generators (yield), lazy evaluation, infinite sequences.

Rule: no itertools module. In practice you'd reach for itertools.islice / count / pairwise.
"""

from collections import deque
from collections.abc import Callable, Iterable, Iterator, Mapping
from typing import Protocol, Self


class _Comparable(Protocol):
    def __lt__(self, other: Self, /) -> bool: ...


def step_range(start: int, end: int, step: int) -> Iterator[int]:
    """start, start+step, ... while the value is < end. step <= 0 -> ValueError right at the call."""
    # A function with `yield` doesn't run when called, only on the first next(). If the check were
    # inside the generator, the error would pop up somewhere during iteration. So: the wrapper validates, the inner part generates.
    if step <= 0:
        raise ValueError(f"step must be positive, got {step}")
    return _step_range(start, end, step)


def _step_range(start: int, end: int, step: int) -> Iterator[int]:
    i = start
    while i < end:
        yield i
        i += step


def fibonacci() -> Iterator[int]:
    """The infinite sequence 0, 1, 1, 2, 3, 5, ... Never call list() on it."""
    a, b = 0, 1
    while True:
        yield a
        a, b = b, a + b


def take[T](items: Iterable[T], n: int) -> Iterator[T]:
    """At most the first n elements. Doesn't read more from the source than needed. (In practice: itertools.islice.)"""
    if n <= 0:
        return
    for i, item in enumerate(items, start=1):
        yield item
        if i >= n:  # check AFTER yield, otherwise we'd pull the (n+1)-th element from the source for nothing
            return


def filter_[T](items: Iterable[T], pred: Callable[[T], bool]) -> Iterator[T]:
    """Lazily lets through the elements for which pred returns True."""
    for item in items:
        if pred(item):
            yield item
    # Shorter: return (x for x in items if pred(x)) — a generator expression is lazy too.


def window[T](items: Iterable[T], size: int) -> Iterator[tuple[T, ...]]:
    """Sliding windows: [1, 2, 3, 4], 2 -> (1, 2), (2, 3), (3, 4). Works on an infinite source too.
    Nothing for size <= 0 or too few elements. (In practice: itertools.pairwise for size=2.)"""
    if size <= 0:
        return
    buf: deque[T] = deque(maxlen=size)  # when the deque is full, append drops the oldest element on the left
    for item in items:
        buf.append(item)
        if len(buf) == size:
            yield tuple(buf)  # a tuple is an immutable copy, windows are independent "for free"


def sorted_items[K: _Comparable, V](mapping: Mapping[K, V]) -> Iterator[tuple[K, V]]:
    """(key, value) pairs in key order. A dict remembers insertion order, not sorted order."""
    for key in sorted(mapping):
        yield key, mapping[key]
