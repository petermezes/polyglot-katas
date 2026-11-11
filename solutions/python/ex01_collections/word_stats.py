"""Exercise 01: collections — list, dict, Counter, defaultdict, sorting."""

import re
from collections import Counter, defaultdict
from collections.abc import Iterable, Mapping
from dataclasses import dataclass


@dataclass(frozen=True)
class WordCount:
    """A word and the number of its occurrences. frozen=True: immutable, comparable with ==, hashable."""

    word: str
    count: int


# [^\W\d_] = "\w without digits and underscore" = letters (diacritics included, re is Unicode in Python 3).
_LETTERS = re.compile(r"[^\W\d_]+")


def words(text: str) -> list[str]:
    """Splits text into lowercase words. The separator is anything that is not a letter."""
    return _LETTERS.findall(text.lower())
    # Without regex, closer to the Go version:
    #   "".join(c if c.isalpha() else " " for c in text.lower()).split()


def count_words(words: Iterable[str]) -> dict[str, int]:
    """Counts the occurrences of each word."""
    return dict(Counter(words))  # Counter is a dict subclass, dict() is here just for a clean type


def top_n(counts: Mapping[str, int], n: int) -> list[WordCount]:
    """Returns at most n most frequent words: by count descending, ties by word ascending."""
    # The sort key as a tuple: tuples compare element by element, so (-count, word)
    # means "count descending, then word ascending". No comparator like in Go/C#.
    ranked = sorted(counts.items(), key=lambda kv: (-kv[1], kv[0]))
    # Slicing past the end of a list doesn't fail (Go would panic), so n > len needs no handling.
    return [WordCount(w, c) for w, c in ranked[:n]]
    # Trap: Counter(counts).most_common(n) keeps insertion order on ties, not alphabetical.


def group_by_length(words: Iterable[str]) -> dict[int, list[str]]:
    """Groups unique words by number of characters. Words in a group are sorted."""
    groups: defaultdict[int, set[str]] = defaultdict(set)
    for w in words:
        groups[len(w)].add(w)  # Python str is characters (code points), not bytes: len("kôň") == 3
    # Return a plain dict: defaultdict would silently insert an empty group when reading a missing key.
    return {length: sorted(group) for length, group in groups.items()}
