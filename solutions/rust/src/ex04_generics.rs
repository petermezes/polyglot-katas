//! Exercise 04: generics — type parameters, trait bounds, a generic type, closures.

use std::ops::Add;

/// A generic stack.
#[derive(Debug)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        self.items.push(value);
    }

    /// Removes and returns the top element. None for an empty stack.
    pub fn pop(&mut self) -> Option<T> {
        // The value moves out of the Vec to the caller. Nothing to clear for a GC
        // like in Go (`s.items[last] = zero`): the Vec no longer owns it.
        self.items.pop()
    }

    /// The top element without removing it.
    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        // #[derive(Default)] would generate `impl<T: Default> Default for Stack<T>`,
        // even though an empty Vec<T> needs nothing from T.
        Self::new()
    }
}

/// Applies f to every element.
pub fn map<T, U>(xs: &[T], f: impl Fn(&T) -> U) -> Vec<U> {
    let mut out = Vec::with_capacity(xs.len());
    for x in xs {
        out.push(f(x));
    }
    out
}

/// The elements for which pred returns true, as new values.
pub fn filter<T: Clone>(xs: &[T], pred: impl Fn(&T) -> bool) -> Vec<T> {
    let mut out = Vec::new();
    for x in xs {
        if pred(x) {
            out.push(x.clone());
        }
    }
    out
}

/// Folds the elements into one value: f(f(f(init, x0), x1), x2) ...
pub fn reduce<T, A>(xs: &[T], init: A, f: impl Fn(A, &T) -> A) -> A {
    let mut acc = init;
    for x in xs {
        // f takes acc by value and returns the new one: no copy, ownership just moves through.
        acc = f(acc, x);
    }
    acc
}

/// The sum of the numbers. 0 (`T::default()`) for an empty slice.
pub fn sum<T>(xs: &[T]) -> T
where
    T: Copy + Default + Add<Output = T>,
{
    let mut total = T::default();
    for &x in xs {
        // `&x` in the pattern copies the value out of the reference, which Copy allows.
        total = total + x;
    }
    total
}

/// The element with the largest key, the first one on ties. None for an empty slice.
pub fn max_by<T, K: PartialOrd>(xs: &[T], key: impl Fn(&T) -> K) -> Option<&T> {
    let (first, rest) = xs.split_first()?; // `?` on an Option: return None for an empty slice
    let mut best = first;
    let mut best_key = key(first);
    for x in rest {
        let k = key(x);
        if k > best_key {
            // strictly greater: the first one stays on ties
            best = x;
            best_key = k;
        }
    }
    Some(best)
}
