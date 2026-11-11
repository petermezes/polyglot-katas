//! Exercise 01: collections — Vec, HashMap, sorting, String vs &str.

use std::collections::HashMap;

/// A word and the number of its occurrences.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WordCount {
    pub word: String,
    pub count: usize,
}

/// Splits text into lowercase words. The separator is anything that is not a letter.
pub fn words(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split(|c: char| !c.is_alphabetic())
        // "kôň, žltý" splits into "kôň", "", "žltý": split keeps the empty piece between
        // ',' and ' ' (Go's FieldsFunc and Python's str.split() drop them).
        .filter(|w| !w.is_empty())
        // The pieces borrow from the temporary lowercase String, which dies at the end of
        // this statement. String::from copies each piece into a String the caller owns.
        .map(String::from)
        .collect()
}

/// Counts the occurrences of each word.
pub fn count_words(words: &[String]) -> HashMap<String, usize> {
    let mut counts = HashMap::with_capacity(words.len());
    for w in words {
        // entry = a single lookup for "insert 0 if missing, then add 1".
        // clone(): the map owns its keys. Returning HashMap<&str, usize> would avoid the copies,
        // but the result would then borrow `words` and couldn't outlive it (a lifetime parameter).
        *counts.entry(w.clone()).or_insert(0) += 1;
    }
    counts
}

/// Returns at most n most frequent words: by count descending, ties by word ascending.
pub fn top_n(counts: &HashMap<String, usize>, n: usize) -> Vec<WordCount> {
    let mut result: Vec<WordCount> = counts
        .iter()
        .map(|(word, &count)| WordCount {
            word: word.clone(),
            count,
        })
        .collect();
    // b before a = descending. String comparison is byte-wise (ordinal), like Go,
    // not culture-sensitive like the default in C#.
    result.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.word.cmp(&b.word)));
    result.truncate(n);
    result
}

/// Groups unique words by number of characters (not bytes!). Words in a group are sorted.
pub fn group_by_length(words: &[String]) -> HashMap<usize, Vec<String>> {
    let mut groups: HashMap<usize, Vec<String>> = HashMap::new();
    for w in words {
        // len() counts bytes: "kôň".len() == 5. chars() iterates Unicode scalar values.
        groups.entry(w.chars().count()).or_default().push(w.clone());
    }
    for group in groups.values_mut() {
        group.sort();
        group.dedup(); // removes only neighbouring duplicates, hence sort first
    }
    groups
}
