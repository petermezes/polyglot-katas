use std::collections::HashMap;

use under_test::ex01_collections::*;

const TEXT: &str = "Žltý kôň, žltý pes! Kôň beží.";

fn strings(xs: &[&str]) -> Vec<String> {
    xs.iter().map(|s| s.to_string()).collect()
}

#[test]
fn words_splits_and_lowercases() {
    // A Vec<String> compares with an array of &str: String implements PartialEq<&str>.
    assert_eq!(words(TEXT), ["žltý", "kôň", "žltý", "pes", "kôň", "beží"]);
    assert!(
        words("  ...  ").is_empty(),
        "text without letters should return an empty result"
    );
}

#[test]
fn count_words_counts_occurrences() {
    let want: HashMap<String, usize> = [("žltý", 2), ("kôň", 2), ("pes", 1), ("beží", 1)]
        .into_iter()
        .map(|(w, c)| (w.to_string(), c))
        .collect();
    assert_eq!(count_words(&words(TEXT)), want);
}

#[test]
fn top_n_sorts_by_count_then_word() {
    let counts = count_words(&words(TEXT));
    let wc = |word: &str, count| WordCount {
        word: word.to_string(),
        count,
    };
    assert_eq!(
        top_n(&counts, 3),
        [wc("kôň", 2), wc("žltý", 2), wc("beží", 1)]
    );
    assert_eq!(
        top_n(&counts, 100).len(),
        4,
        "top_n(100) should return all 4 words"
    );
}

#[test]
fn group_by_length_counts_characters_not_bytes() {
    let groups = group_by_length(&words(TEXT));
    for (length, want) in [(3, ["kôň", "pes"]), (4, ["beží", "žltý"])] {
        assert_eq!(
            groups.get(&length),
            Some(&strings(&want)),
            "length {length}"
        );
    }
    assert_eq!(groups.len(), 2, "expected 2 groups");
}
