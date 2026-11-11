from fractions import Fraction
from typing import NewType

import pytest

from ex04_generics.generics import Stack, filter_, map_, max_by, reduce_, total


def test_stack():
    s = Stack[str]()
    assert len(s) == 0
    assert not s, "an empty stack should be falsy (__len__ == 0)"
    with pytest.raises(IndexError):
        s.pop()

    for item in "abc":
        s.push(item)
    assert s.peek() == "c"
    assert len(s) == 3
    assert list(s) == ["c", "b", "a"], "iteration goes from the top and removes nothing"
    assert len(s) == 3

    popped = []
    while s:
        popped.append(s.pop())
    assert popped == ["c", "b", "a"]
    with pytest.raises(IndexError):
        s.peek()


def test_stacks_do_not_share_items():
    a, b = Stack[int](), Stack[int]()
    a.push(1)
    assert len(b) == 0


def test_map_filter_reduce():
    nums = [1, 2, 3, 4, 5, 6]

    strs = map_(nums, str)
    assert strs == ["1", "2", "3", "4", "5", "6"]

    assert filter_(nums, lambda n: n % 2 == 0) == [2, 4, 6]

    assert reduce_(strs, "", lambda acc, s: acc + s) == "123456"

    assert map_([], str) == []


def test_map_accepts_any_iterable():
    assert map_(range(3), lambda n: n * 10) == [0, 10, 20]
    assert filter_((c for c in "a1b2"), str.isdigit) == ["1", "2"]


Meters = NewType("Meters", float)


def test_total():
    assert total([1, 2, 3], 0) == 6
    assert total([Meters(1.5), Meters(2.5)], Meters(0)) == 4
    assert total([], 0.0) == 0.0
    assert total([Fraction(1, 2), Fraction(1, 3)], Fraction(0)) == Fraction(5, 6)
    assert total(["a", "b"], "") == "ab", "duck typing: it's enough that the type supports +"


def test_max_by():
    words = ["pes", "mačka", "kôň", "slimák"]
    assert max_by(words, len) == "slimák"
    assert max_by(words, str.upper) == "slimák"
    assert max_by(["ab", "cd"], len) == "ab", "on a tie the first one should win"
    assert max_by((w for w in words), len) == "slimák", "should also work with a generator"
    with pytest.raises(ValueError):
        max_by([], lambda x: x)
