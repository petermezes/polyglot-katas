use std::ops::Add;

use under_test::ex04_generics::*;

#[test]
fn stack_push_pop_peek() {
    let mut s: Stack<&str> = Stack::default();
    assert_eq!(s.pop(), None, "pop on an empty stack should return None");
    s.push("a");
    s.push("b");
    s.push("c");
    assert_eq!(s.peek(), Some(&"c"));
    assert_eq!(s.len(), 3);

    let mut popped = Vec::new();
    while let Some(v) = s.pop() {
        popped.push(v);
    }
    assert_eq!(popped, ["c", "b", "a"], "pop order");
    assert_eq!(s.peek(), None, "peek on an empty stack should return None");
    assert!(s.is_empty());
}

#[test]
fn stack_of_a_type_without_default() {
    struct NoDefault;
    let mut s = Stack::new();
    s.push(NoDefault);
    assert_eq!(s.len(), 1);
    // Compiles only if Default for Stack<T> doesn't require T: Default.
    let empty: Stack<NoDefault> = Stack::default();
    assert!(empty.is_empty());
}

#[test]
fn map_filter_reduce() {
    let nums = [1, 2, 3, 4, 5, 6];

    let strs = map(&nums, i32::to_string); // a function instead of a closure works too
    assert_eq!(strs, ["1", "2", "3", "4", "5", "6"]);

    assert_eq!(filter(&nums, |n| n % 2 == 0), [2, 4, 6]);

    let joined = reduce(&strs, String::new(), |acc, s| acc + s);
    assert_eq!(joined, "123456");

    assert!(
        map(&[] as &[i32], i32::to_string).is_empty(),
        "map of an empty input"
    );
}

// A newtype: Rust has no `type Meters float64` that inherits +. You implement Add yourself.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct Meters(f64);

impl Add for Meters {
    type Output = Meters;

    fn add(self, other: Meters) -> Meters {
        Meters(self.0 + other.0)
    }
}

#[test]
fn sum_of_any_addable_type() {
    assert_eq!(sum(&[1, 2, 3]), 6);
    assert_eq!(sum(&[Meters(1.5), Meters(2.5)]), Meters(4.0));
    assert_eq!(sum::<f64>(&[]), 0.0);
}

#[test]
fn max_by_key_first_on_ties() {
    let words = ["pes", "mačka", "kôň", "slimák"];
    assert_eq!(
        max_by(&words, |s| s.chars().count()),
        Some(&"slimák"),
        "max_by(length)"
    );
    assert_eq!(
        max_by(&words, |s| s.to_uppercase()),
        Some(&"slimák"),
        "max_by(alphabetical)"
    );
    assert_eq!(
        max_by(&["ab", "cd"], |s| s.len()),
        Some(&"ab"),
        "on a tie the first one should win"
    );
    assert_eq!(
        max_by(&[] as &[i32], |i| *i),
        None,
        "max_by of an empty input"
    );
}
