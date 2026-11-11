use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration, Instant};

use under_test::ex10_concurrency::*;

fn ms(n: u64) -> Duration {
    Duration::from_millis(n)
}

#[test]
fn process_all_keeps_order_and_limits_parallelism() {
    let active = AtomicUsize::new(0);
    let max_active = AtomicUsize::new(0);
    let double = |_: &CancelToken, &n: &u64| {
        let current = active.fetch_add(1, Ordering::SeqCst) + 1;
        max_active.fetch_max(current, Ordering::SeqCst); // Go needs a CompareAndSwap loop for this
        thread::sleep(ms(10 - n)); // later inputs finish sooner
        active.fetch_sub(1, Ordering::SeqCst);
        Ok(n * 2)
    };

    let inputs: Vec<u64> = (0..10).collect();
    let got = process_all(&CancelToken::new(), &inputs, 3, double).expect("unexpected error");
    assert_eq!(
        got,
        [0, 2, 4, 6, 8, 10, 12, 14, 16, 18],
        "results must be in input order"
    );
    let max = max_active.load(Ordering::SeqCst);
    assert!(
        (2..=3).contains(&max),
        "max running at once = {max}, expected 2–3"
    );
}

#[test]
fn process_all_stops_on_first_error() {
    let started = AtomicUsize::new(0);
    let f = |token: &CancelToken, &n: &u64| {
        started.fetch_add(1, Ordering::SeqCst);
        if n == 2 {
            return Err(TaskError::Failed("sensor not responding".to_string()));
        }
        token.sleep(ms(50))?; // returns Err(Cancelled) early once the token is cancelled
        Ok(n)
    };

    let inputs: Vec<u64> = (0..100).collect();
    let start = Instant::now();
    let err = process_all(&CancelToken::new(), &inputs, 4, f).unwrap_err();
    assert_eq!(
        err,
        TaskError::Failed("sensor not responding".to_string()),
        "expected the original error"
    );
    let n = started.load(Ordering::SeqCst);
    assert!(
        n <= 10,
        "no further tasks should start after an error, started: {n}"
    );
    let elapsed = start.elapsed();
    assert!(
        elapsed < ms(500),
        "took {elapsed:?} — cancellation doesn't work"
    );
}

#[test]
fn process_all_honours_outside_cancellation() {
    let token = CancelToken::new();
    // A plain thread::spawn needs owned ('static) data: the clone is a new handle to the same flag.
    let canceller = token.clone();
    thread::spawn(move || {
        thread::sleep(ms(30));
        canceller.cancel();
    });
    let slow = |token: &CancelToken, &n: &u64| {
        token.sleep(Duration::from_secs(1))?;
        Ok(n)
    };
    let start = Instant::now();
    assert_eq!(
        process_all(&token, &[1, 2, 3, 4, 5], 2, slow),
        Err(TaskError::Cancelled)
    );
    assert!(start.elapsed() < ms(500), "cancellation doesn't work");
}

#[test]
fn first_success_returns_fastest_success_and_cancels_the_rest() {
    let cancelled = AtomicUsize::new(0);
    let slow = |token: &CancelToken| match token.sleep(Duration::from_secs(2)) {
        Ok(()) => Ok("slow"),
        Err(err) => {
            cancelled.fetch_add(1, Ordering::SeqCst);
            Err(err)
        }
    };
    let fast = |_: &CancelToken| {
        thread::sleep(ms(10));
        Ok("fast")
    };
    let failing = |_: &CancelToken| Err(TaskError::Failed("fails immediately".to_string()));

    assert_eq!(
        first_success(&CancelToken::new(), &[&slow, &failing, &fast, &slow]),
        Ok("fast")
    );
    // No waiting loop like in Go: the scoped threads have all finished by the time
    // first_success returns, so the cancelled ones have already counted themselves.
    assert_eq!(
        cancelled.load(Ordering::SeqCst),
        2,
        "slow functions should be cancelled"
    );
}

#[test]
fn first_success_collects_all_errors() {
    let first = |_: &CancelToken| Err::<i32, _>(TaskError::Failed("first".to_string()));
    let second = |_: &CancelToken| Err::<i32, _>(TaskError::Failed("second".to_string()));
    let Err(FirstSuccessError::AllFailed(errors)) =
        first_success(&CancelToken::new(), &[&first, &second])
    else {
        panic!("expected AllFailed");
    };
    // The order depends on which thread finished first.
    assert_eq!(errors.len(), 2);
    assert!(errors.contains(&TaskError::Failed("first".to_string())));
    assert!(errors.contains(&TaskError::Failed("second".to_string())));

    assert_eq!(
        first_success::<i32>(&CancelToken::new(), &[]),
        Err(FirstSuccessError::NoFunctions)
    );
}
