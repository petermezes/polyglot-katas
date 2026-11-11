use std::collections::HashMap;

use under_test::ex09_iterators::*;

#[test]
fn step_range_yields_until_end() {
    assert_eq!(step_range(0, 10, 3).collect::<Vec<_>>(), [0, 3, 6, 9]);
    assert_eq!(step_range(5, 5, 1).count(), 0, "empty range");
    assert_eq!(step_range(0, 10, 0).count(), 0, "step 0");
    assert_eq!(step_range(0, 10, -1).count(), 0, "negative step");
}

#[test]
fn break_stops_iteration() {
    // In Go this tests that the iterator respects yield's return value. Rust iterators are
    // pull-based, so breaking out works no matter how next() is written.
    let mut sum = 0;
    for v in step_range(0, 1_000_000, 1) {
        if v == 3 {
            break;
        }
        sum += v;
    }
    assert_eq!(sum, 3);

    for v in fibonacci() {
        if v > 100 {
            break;
        }
    }
}

#[test]
fn fibonacci_take() {
    assert_eq!(
        take(fibonacci(), 10).collect::<Vec<_>>(),
        [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
    );
    assert_eq!(take(fibonacci(), 0).count(), 0, "take(0)");
    assert_eq!(
        take(step_range(0, 3, 1), 10).collect::<Vec<_>>(),
        [0, 1, 2],
        "take more than there are elements"
    );
}

#[test]
fn take_does_not_pull_more_than_needed() {
    let mut pulled = Vec::new();
    // inspect (from std) records every element take pulls out of the source.
    let got: Vec<i64> = take(step_range(0, 10, 1).inspect(|&v| pulled.push(v)), 3).collect();
    assert_eq!(got, [0, 1, 2]);
    assert_eq!(
        pulled,
        [0, 1, 2],
        "take must not pull more elements from the source than it returns"
    );
}

#[test]
fn laziness() {
    let mut calls = 0;
    let is_even = |n: &i64| {
        calls += 1;
        n % 2 == 0
    };
    let got: Vec<i64> = take(filter(step_range(1, 1_000_000, 1), is_even), 3).collect();
    assert_eq!(got, [2, 4, 6]);
    assert_eq!(
        calls, 6,
        "the predicate should be called 6x (lazy evaluation), was called {calls}x"
    );
}

#[test]
fn windows_are_borrowed_slices() {
    let xs = [1, 2, 3, 4];
    let got: Vec<&[i32]> = windows(&xs, 2).collect();
    assert_eq!(got, [[1, 2], [2, 3], [3, 4]]);
    // No "windows must be independent copies" check like in Go: a &[i32] can't be written to.
    assert_eq!(windows(&[1, 2], 3).count(), 0, "window too large");
    assert_eq!(windows(&[1, 2], 0).count(), 0, "size 0");
}

#[test]
fn sorted_pairs_in_key_order() {
    let map = HashMap::from([("c", 3), ("a", 1), ("b", 2)]);
    let mut keys = Vec::new();
    let mut sum = 0;
    for (k, v) in sorted_pairs(&map) {
        keys.push(*k);
        sum += v;
    }
    assert_eq!(keys, ["a", "b", "c"]);
    assert_eq!(sum, 6);
}
