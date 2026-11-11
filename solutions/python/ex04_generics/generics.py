"""Exercise 04: generics — type parameters (PEP 695), Protocol as a constraint, dunder methods.

Rule: no built-in map/filter/sum/max or functools.reduce. The goal is to write it by hand.
"""

from collections.abc import Callable, Iterable, Iterator
from typing import Protocol, Self


class Stack[T]:
    """A generic stack. `Stack[int]()` or `Stack()` — the type parameter is only for the type checker."""

    def __init__(self) -> None:
        self._items: list[T] = []  # mutable state always in __init__, never as a class attribute

    def push(self, item: T) -> None:
        self._items.append(item)

    def pop(self) -> T:
        """Removes the top element. Empty stack -> IndexError (same as list.pop)."""
        if not self._items:
            raise IndexError("pop from an empty stack")
        return self._items.pop()

    def peek(self) -> T:
        if not self._items:
            raise IndexError("peek at an empty stack")
        return self._items[-1]

    def __len__(self) -> int:
        # Thanks to __len__, len(s) works and so does `if s:` (an empty stack is falsy).
        return len(self._items)

    def __iter__(self) -> Iterator[T]:
        """Iterates from top to bottom without removing."""
        return reversed(self._items)

    def __repr__(self) -> str:
        return f"Stack({self._items!r})"


# Trailing underscore = convention when the name would shadow a built-in (map, filter, ...).
def map_[T, U](xs: Iterable[T], f: Callable[[T], U]) -> list[U]:
    return [f(x) for x in xs]


def filter_[T](xs: Iterable[T], pred: Callable[[T], bool]) -> list[T]:
    return [x for x in xs if pred(x)]


def reduce_[T, A](xs: Iterable[T], initial: A, f: Callable[[A, T], A]) -> A:
    acc = initial
    for x in xs:
        acc = f(acc, x)
    return acc


class Addable(Protocol):
    """A "can be added" constraint — like Go's `Number` constraint, but structural."""

    def __add__(self, other: Self, /) -> Self: ...


def total[N: Addable](xs: Iterable[N], start: N) -> N:
    """Adds up the elements, starting from start. Works for anything with +: int, float, Fraction, even str."""
    acc = start
    for x in xs:
        acc = acc + x
    return acc


class Comparable(Protocol):
    """Like cmp.Ordered: it's enough that the type supports <."""

    def __lt__(self, other: Self, /) -> bool: ...


def max_by[T, K: Comparable](xs: Iterable[T], key: Callable[[T], K]) -> T:
    """The element with the largest key, the first one on ties. Empty input -> ValueError (like the built-in max)."""
    it = iter(xs)  # works for a list and a generator; a generator can be iterated only once
    try:
        best = next(it)
    except StopIteration:
        raise ValueError("max_by() of an empty input") from None
    best_key = key(best)
    for x in it:
        k = key(x)
        if best_key < k:  # strict <, so the first one wins on ties
            best, best_key = x, k
    return best
