//! Exercise 04: generics — type parameters, trait bounds, a generic type, closures.

use std::ops::Add;

/// A generic stack.
#[derive(Debug)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn push(&mut self, value: T) {
        todo!()
    }

    /// Removes and returns the top element. None for an empty stack.
    ///
    /// Option replaces Go's `(T, bool)` and C#'s `bool TryPop(out T)`: there's no zero value
    /// to return, the value exists only inside `Some`.
    pub fn pop(&mut self) -> Option<T> {
        todo!()
    }

    /// The top element without removing it. It is lent out (`&T`): the stack keeps owning it.
    pub fn peek(&self) -> Option<&T> {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    /// Clippy expects `is_empty` next to every public `len`.
    pub fn is_empty(&self) -> bool {
        todo!()
    }
}

/// `Stack::default()` = an empty stack (≈ Go's usable zero value).
///
/// Written by hand on purpose: `#[derive(Default)]` would add the bound `T: Default`,
/// so a stack of a type without a default value couldn't be created that way.
impl<T> Default for Stack<T> {
    fn default() -> Self {
        todo!()
    }
}

/// Applies f to every element. No `iter().map()`: the point is to write the loop yourself.
///
/// `impl Fn(&T) -> U` = any closure or function taking `&T` (a generic parameter in disguise).
pub fn map<T, U>(xs: &[T], f: impl Fn(&T) -> U) -> Vec<U> {
    todo!()
}

/// The elements for which pred returns true, as new values.
///
/// `T: Clone`: the input is only borrowed, so the result needs its own copies.
pub fn filter<T: Clone>(xs: &[T], pred: impl Fn(&T) -> bool) -> Vec<T> {
    todo!()
}

/// Folds the elements into one value: f(f(f(init, x0), x1), x2) ...
pub fn reduce<T, A>(xs: &[T], init: A, f: impl Fn(A, &T) -> A) -> A {
    todo!()
}

/// The sum of the numbers. 0 (`T::default()`) for an empty slice.
///
/// Go's `~int | ~float64` lists the allowed types. Rust lists the required abilities instead:
/// anything that is Copy, has a default and can be added, including your own types (the test's
/// `Meters` newtype implements Add itself).
pub fn sum<T>(xs: &[T]) -> T
where
    T: Copy + Default + Add<Output = T>,
{
    todo!()
}

/// The element with the largest key, the first one on ties. None for an empty slice.
///
/// `PartialOrd` (not `Ord`) so that f64 keys work too.
/// Careful: std's `Iterator::max_by_key` returns the *last* element on ties.
pub fn max_by<T, K: PartialOrd>(xs: &[T], key: impl Fn(&T) -> K) -> Option<&T> {
    todo!()
}
