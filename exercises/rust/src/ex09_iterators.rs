//! Exercise 09: iterators — implementing Iterator, generic adapters, laziness, `impl Iterator`.
//!
//! A Rust iterator is a struct with a `next(&mut self) -> Option<Item>` method. It's pull-based:
//! the caller asks for each element (like C# IEnumerator.MoveNext, Python's __next__), whereas Go's
//! range-over-func pushes elements into a yield callback. That's why `break` in Rust needs no
//! cooperation from the iterator: the caller just stops calling next().
//!
//! Write the iterators below by hand. std has them all (`(a..b).step_by`, `take`, `filter`,
//! `windows`) — the point is to see what they do inside.

use std::collections::HashMap;

/// start, start+step, ... while the value is < end. Nothing for step <= 0.
///
/// (std's `step_by(0)` panics instead.)
pub struct StepRange {
    next: i64,
    end: i64,
    step: i64,
}

pub fn step_range(start: i64, end: i64, step: i64) -> StepRange {
    todo!()
}

impl Iterator for StepRange {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        todo!()
    }
}

/// The infinite sequence 0, 1, 1, 2, 3, 5, ... (next() never returns None).
///
/// Never call `collect()` on it. u64 overflows after 93 numbers (a panic in a debug build),
/// the tests stop long before that.
pub struct Fibonacci {
    a: u64,
    b: u64,
}

pub fn fibonacci() -> Fibonacci {
    todo!()
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        todo!()
    }
}

/// At most the first n elements of `inner`. Must not pull more from `inner` than it returns.
///
/// Generic over any iterator `I`. The struct owns `inner`: take(iter, 3) moves `iter` in.
pub struct Take<I> {
    inner: I,
    remaining: usize,
}

pub fn take<I: Iterator>(inner: I, n: usize) -> Take<I> {
    todo!()
}

impl<I: Iterator> Iterator for Take<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        todo!()
    }
}

/// Lazily lets through only the elements for which `pred` returns true.
///
/// `P: FnMut`: the predicate may change captured state (the test counts its calls).
pub struct Filter<I, P> {
    inner: I,
    pred: P,
}

pub fn filter<I, P>(inner: I, pred: P) -> Filter<I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    todo!()
}

impl<I, P> Iterator for Filter<I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;

    /// Tip: `for x in self.inner.by_ref()` (by_ref: loop without consuming self.inner, so the
    /// rest stays there for the next call). Once it works, compare with `Iterator::find`. Calling a closure stored in a field needs
    /// parentheses: `(self.pred)(&x)`, otherwise Rust looks for a method named pred.
    fn next(&mut self) -> Option<I::Item> {
        todo!()
    }
}

/// Sliding windows of length size: [1, 2, 3, 4], 2 -> [1, 2] [2, 3] [3, 4].
/// Nothing for size 0 or size > xs.len() (std's `slice::windows(0)` panics).
///
/// Go has to copy each window so the caller can't change the original through it. Here the
/// windows are borrowed sub-slices `&[T]`: no copies, and still read-only.
///
/// No struct this time: `impl Iterator<Item = &[T]>` means "some iterator, the caller doesn't
/// need its name". A range mapped through a closure is enough: `(0..count).map(move |i| ...)`.
/// The returned iterator borrows `xs`, the compiler ties their lifetimes together.
pub fn windows<T>(xs: &[T], size: usize) -> impl Iterator<Item = &[T]> {
    // Not todo!(): `impl Iterator` needs a concrete type to stand for, and todo!() has none.
    // This placeholder yields nothing, so the tests fail until you replace it.
    std::iter::empty()
}

/// (key, value) pairs in key order. A HashMap iterates in random order.
///
/// Tip: collect the pairs into a Vec, sort it, return `vec.into_iter()`.
/// (A BTreeMap is always sorted by key, often the better choice when you need the order.)
pub fn sorted_pairs<K: Ord, V>(map: &HashMap<K, V>) -> impl Iterator<Item = (&K, &V)> {
    std::iter::empty() // placeholder, see windows()
}
