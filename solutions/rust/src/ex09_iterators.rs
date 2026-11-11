//! Exercise 09: iterators — implementing Iterator, generic adapters, laziness, `impl Iterator`.
//!
//! A Rust iterator is a struct with a `next(&mut self) -> Option<Item>` method. It's pull-based:
//! the caller asks for each element (like C# IEnumerator.MoveNext, Python's __next__), whereas Go's
//! range-over-func pushes elements into a yield callback. That's why `break` in Rust needs no
//! cooperation from the iterator: the caller just stops calling next().
//!
//! Implementing only next() is enough: map, filter, sum, collect and the other ~70 methods
//! of Iterator come for free as default methods.

use std::collections::HashMap;

/// start, start+step, ... while the value is < end. Nothing for step <= 0.
pub struct StepRange {
    next: i64,
    end: i64,
    step: i64,
}

pub fn step_range(start: i64, end: i64, step: i64) -> StepRange {
    StepRange {
        next: start,
        end,
        step,
    }
}

impl Iterator for StepRange {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        if self.step <= 0 || self.next >= self.end {
            return None;
        }
        let current = self.next;
        self.next += self.step;
        Some(current)
    }
}

/// The infinite sequence 0, 1, 1, 2, 3, 5, ... (next() never returns None).
pub struct Fibonacci {
    a: u64,
    b: u64,
}

pub fn fibonacci() -> Fibonacci {
    Fibonacci { a: 0, b: 1 }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let current = self.a;
        // The state lives in the struct between calls. A Python generator or a C# `yield return`
        // method keeps it in local variables, and the compiler builds such a struct for you.
        (self.a, self.b) = (self.b, self.a + self.b);
        Some(current)
    }
}

/// At most the first n elements of `inner`. Must not pull more from `inner` than it returns.
pub struct Take<I> {
    inner: I,
    remaining: usize,
}

pub fn take<I: Iterator>(inner: I, n: usize) -> Take<I> {
    Take {
        inner,
        remaining: n,
    }
}

impl<I: Iterator> Iterator for Take<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        // Check before calling inner.next(): asking for the (n+1)-th element would already
        // compute it, which for an expensive or side-effecting source is wasted work.
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        self.inner.next()
    }
}

/// Lazily lets through only the elements for which `pred` returns true.
pub struct Filter<I, P> {
    inner: I,
    pred: P,
}

pub fn filter<I, P>(inner: I, pred: P) -> Filter<I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    Filter { inner, pred }
}

impl<I, P> Iterator for Filter<I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        // Pulls from inner only until the first match: that's the laziness. find() does exactly
        // this loop:
        //
        //     for x in self.inner.by_ref() {      // by_ref: don't consume self.inner
        //         if (self.pred)(&x) { return Some(x); }
        //     }
        //     None
        //
        // The closure borrows only self.pred while find borrows self.inner mutably. Closures
        // capture individual fields, so the two borrows don't clash.
        self.inner.find(|x| (self.pred)(x))
    }
}

/// Sliding windows of length size. Nothing for size 0 or size > xs.len().
pub fn windows<T>(xs: &[T], size: usize) -> impl Iterator<Item = &[T]> {
    let count = if size == 0 {
        0
    } else {
        (xs.len() + 1).saturating_sub(size) // saturating: no underflow when size > len + 1
    };
    // `move` copies xs (a reference) and size into the closure, so the iterator doesn't
    // borrow this function's local variables, which are gone once it returns.
    (0..count).map(move |i| &xs[i..i + size])
}

/// (key, value) pairs in key order. A HashMap iterates in random order.
pub fn sorted_pairs<K: Ord, V>(map: &HashMap<K, V>) -> impl Iterator<Item = (&K, &V)> {
    let mut pairs: Vec<(&K, &V)> = map.iter().collect();
    // sort_unstable: faster and allocation-free; stability doesn't matter, keys are unique.
    pairs.sort_unstable_by(|a, b| a.0.cmp(b.0));
    pairs.into_iter()
}
