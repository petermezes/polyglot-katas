"""Exercise 04: generics — type parameters (PEP 695), Protocol as a constraint, dunder methods.

Rule: no built-in map/filter/sum/max or functools.reduce. The goal is to write it by hand.
Type parameters ([T], [N: Addable]) are checked only by a type checker (mypy, pyright, Pylance),
the interpreter ignores them at runtime.
"""

from collections.abc import Callable, Iterable, Iterator
from typing import Protocol, Self


class Stack[T]:
    """A generic stack. `Stack[int]()` or `Stack()` — the type parameter is only for the type checker."""

    def __init__(self) -> None:
        """Tip: self._items: list[T] = []. Mutable state always in __init__, never as a class attribute
        (that one would be shared by all instances)."""
        raise NotImplementedError("TODO")

    def push(self, item: T) -> None:
        raise NotImplementedError("TODO")

    def pop(self) -> T:
        """Removes the top element. Empty stack -> IndexError (same as list.pop)."""
        raise NotImplementedError("TODO")

    def peek(self) -> T:
        """The top element without removing it. Empty stack -> IndexError."""
        raise NotImplementedError("TODO")

    def __len__(self) -> int:
        """Thanks to __len__, len(s) works and so does `if s:` (an empty stack is falsy)."""
        raise NotImplementedError("TODO")

    def __iter__(self) -> Iterator[T]:
        """Iterates from top to bottom without removing. Tip: reversed()."""
        raise NotImplementedError("TODO")


# Trailing underscore = convention when the name would shadow a built-in (map, filter, ...).
def map_[T, U](xs: Iterable[T], f: Callable[[T], U]) -> list[U]:
    """Applies f to every element."""
    raise NotImplementedError("TODO")


def filter_[T](xs: Iterable[T], pred: Callable[[T], bool]) -> list[T]:
    """The elements for which pred returns True."""
    raise NotImplementedError("TODO")


def reduce_[T, A](xs: Iterable[T], initial: A, f: Callable[[A, T], A]) -> A:
    """Folds the elements into one value: f(f(f(initial, x0), x1), x2) ..."""
    raise NotImplementedError("TODO")


class Addable(Protocol):
    """A "can be added" constraint — like Go's `Number` constraint, but structural."""

    def __add__(self, other: Self, /) -> Self: ...


def total[N: Addable](xs: Iterable[N], start: N) -> N:
    """Adds up the elements, starting from start. Works for anything with +: int, float, Fraction, even str."""
    raise NotImplementedError("TODO")


class Comparable(Protocol):
    """Like cmp.Ordered: it's enough that the type supports <."""

    def __lt__(self, other: Self, /) -> bool: ...


def max_by[T, K: Comparable](xs: Iterable[T], key: Callable[[T], K]) -> T:
    """The element with the largest key, the first one on ties. Empty input -> ValueError (like the built-in max).

    Tip: xs may be a generator (can't be indexed or iterated twice). Use it = iter(xs),
    get the first element with next(it) and turn StopIteration into ValueError.
    """
    raise NotImplementedError("TODO")
