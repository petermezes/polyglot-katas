import pytest

from ex01_collections.word_stats import WordCount, count_words, group_by_length, top_n, words

TEXT = "Žltý kôň, žltý pes! Kôň beží."


def test_words():
    assert words(TEXT) == ["žltý", "kôň", "žltý", "pes", "kôň", "beží"]
    assert words("  ...  ") == []


def test_words_ignores_digits_and_underscore():
    assert words("a1b_c") == ["a", "b", "c"]


def test_count_words():
    assert count_words(words(TEXT)) == {"žltý": 2, "kôň": 2, "pes": 1, "beží": 1}


def test_top_n():
    counts = count_words(words(TEXT))
    assert top_n(counts, 3) == [WordCount("kôň", 2), WordCount("žltý", 2), WordCount("beží", 1)]
    assert len(top_n(counts, 100)) == 4
    assert top_n(counts, 0) == []


def test_top_n_ties_are_alphabetical_not_insertion_order():
    assert top_n({"b": 1, "a": 1, "c": 2}, 3) == [WordCount("c", 2), WordCount("a", 1), WordCount("b", 1)]


# Parametrized test — the equivalent of a Go table-driven test and [Theory] in xUnit.
@pytest.mark.parametrize(
    ("length", "expected"),
    [
        (3, ["kôň", "pes"]),
        (4, ["beží", "žltý"]),
    ],
)
def test_group_by_length(length, expected):
    assert group_by_length(words(TEXT))[length] == expected


def test_group_by_length_has_only_existing_groups():
    groups = group_by_length(words(TEXT))
    assert len(groups) == 2
    with pytest.raises(KeyError):
        groups[5]  # a plain dict, not a defaultdict
