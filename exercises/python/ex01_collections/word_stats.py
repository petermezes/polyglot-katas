"""Exercise 01: collections — list, dict, Counter, defaultdict, sorting."""

from collections.abc import Iterable, Mapping
from dataclasses import dataclass


@dataclass(frozen=True)
class WordCount:
    """A word and the number of its occurrences. frozen=True: immutable, comparable with ==, hashable."""

    word: str
    count: int


def words(text: str) -> list[str]:
    """Splits text into lowercase words. The separator is anything that is not a letter.

    Tip: str.lower(), str.isalpha(), str.split(). Or the re module: re.findall(r"[^\\W\\d_]+", ...)
    (\\w without digits and underscore = letters, diacritics included).
    """
    raise NotImplementedError("TODO")


def count_words(words: Iterable[str]) -> dict[str, int]:
    """Counts the occurrences of each word.

    Tip: collections.Counter. Or by hand: counts[w] = counts.get(w, 0) + 1.
    """
    raise NotImplementedError("TODO")


def top_n(counts: Mapping[str, int], n: int) -> list[WordCount]:
    """Returns at most n most frequent words: by count descending, ties by word ascending.

    Tip: sorted(counts.items(), key=lambda kv: ...). The key can be a tuple, compared element by element,
    so (-count, word) means "count descending, then word ascending".
    Careful: sorted() returns a new list, list.sort() sorts in place and returns None.
    Careful 2: Counter.most_common() keeps insertion order on ties, not alphabetical.
    """
    raise NotImplementedError("TODO")


def group_by_length(words: Iterable[str]) -> dict[int, list[str]]:
    """Groups unique words by number of characters. Words in a group are sorted.

    Tip: collections.defaultdict(set) and at the end a dict comprehension {k: sorted(v) for ...}.
    Return a plain dict: defaultdict would silently insert an empty group when reading a missing key.
    (len("kôň") is 3 in Python — str is characters, not bytes like in Go.)
    """
    raise NotImplementedError("TODO")
