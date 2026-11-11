//! Exercise 01: collections — Vec, HashMap, sorting, String vs &str.

use std::collections::HashMap;

/// A word and the number of its occurrences.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WordCount {
    pub word: String,
    pub count: usize,
}

/// Splits text into lowercase words. The separator is anything that is not a letter.
///
/// Tip: `str::to_lowercase`, `str::split(|c: char| !c.is_alphabetic())`. Unlike Go's
/// `strings.FieldsFunc`, `split` leaves empty pieces between neighbouring separators.
/// `String::from` turns a borrowed `&str` into an owned `String`.
pub fn words(text: &str) -> Vec<String> {
    todo!()
}

/// Counts the occurrences of each word.
///
/// Tip: the entry API does it in one lookup: `*counts.entry(key).or_insert(0) += 1`.
/// The map owns its keys, so it needs its own `String` (`clone()`), the caller keeps `words`.
pub fn count_words(words: &[String]) -> HashMap<String, usize> {
    todo!()
}

/// Returns at most n most frequent words: by count descending, ties by word ascending.
///
/// Tip: HashMap iteration order is random. Collect into a Vec and sort it with `sort_by`
/// and `Ordering::then_with`. `Vec::truncate(n)` does nothing when n is larger than the length.
pub fn top_n(counts: &HashMap<String, usize>, n: usize) -> Vec<WordCount> {
    todo!()
}

/// Groups unique words by number of characters (not bytes!). Words in a group are sorted.
///
/// Tip: `"kôň".len()` is 5 (bytes in UTF-8), `"kôň".chars().count()` is 3.
/// `entry(len).or_default().push(...)` creates the group on first use.
/// `sort` followed by `dedup` removes duplicates.
pub fn group_by_length(words: &[String]) -> HashMap<usize, Vec<String>> {
    todo!()
}
