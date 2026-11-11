//! Exercise 10: concurrency — scoped threads, channels, atomics, cancellation.
//!
//! Standard library only: OS threads instead of async. For lots of I/O-bound tasks Rust projects
//! use async/await with a runtime crate (tokio). The ideas (cancellation, limited parallelism,
//! first result wins) are the same.
//!
//! The big difference from Go, C# and Python: data races don't compile. A closure passed to a
//! thread may only share data that is `Sync` and move data that is `Send`. Forget a Mutex and you
//! get a compile error, not a flaky test.

use std::error::Error;
use std::fmt;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::{Duration, Instant};

// ---------------------------------------------------------------------------------------------
// Given in the exercise: a minimal cancellation token. std has nothing like Go's
// context.Context or C#'s CancellationToken, so here is a small one.
// ---------------------------------------------------------------------------------------------

/// A cancellation flag shared between threads. Cloning gives another handle to the same flag.
#[derive(Debug, Clone, Default)]
pub struct CancelToken {
    cancelled: Arc<AtomicBool>,
    parent: Option<Box<CancelToken>>,
}

impl CancelToken {
    pub fn new() -> Self {
        Self::default()
    }

    /// A token that is cancelled when this one is, but can also be cancelled on its own
    /// without affecting this one (≈ Go's context.WithCancel(parent)).
    pub fn child(&self) -> CancelToken {
        CancelToken {
            cancelled: Arc::new(AtomicBool::new(false)),
            parent: Some(Box::new(self.clone())),
        }
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
            || self.parent.as_ref().is_some_and(|p| p.is_cancelled())
    }

    /// Sleeps for `d`, or returns `Err(TaskError::Cancelled)` as soon as the token is cancelled.
    /// (It polls every millisecond. A production version would wait on a Condvar instead.)
    pub fn sleep(&self, d: Duration) -> Result<(), TaskError> {
        let deadline = Instant::now() + d;
        loop {
            if self.is_cancelled() {
                return Err(TaskError::Cancelled);
            }
            let now = Instant::now();
            if now >= deadline {
                return Ok(());
            }
            thread::sleep((deadline - now).min(Duration::from_millis(1)));
        }
    }
}

/// Why a task didn't produce a result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskError {
    /// The token was cancelled (≈ Go's context.Canceled / DeadlineExceeded).
    Cancelled,
    /// The task itself failed.
    Failed(String),
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskError::Cancelled => f.write_str("cancelled"),
            TaskError::Failed(msg) => f.write_str(msg),
        }
    }
}

impl Error for TaskError {}

/// One function for first_success: `&(dyn Fn ...)` is a reference to some closure, whatever its
/// concrete type, that may be called from several threads (`Sync`). A type alias only names it.
pub type Attempt<'a, R> = &'a (dyn Fn(&CancelToken) -> Result<R, TaskError> + Sync);

/// Why first_success has no result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FirstSuccessError {
    /// Called with an empty slice.
    NoFunctions,
    /// Every function failed. All their errors, in the order they finished
    /// (≈ Go's errors.Join, C#'s AggregateException, Python's ExceptionGroup).
    AllFailed(Vec<TaskError>),
}

impl fmt::Display for FirstSuccessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FirstSuccessError::NoFunctions => f.write_str("no functions to run"),
            FirstSuccessError::AllFailed(errors) => {
                write!(f, "all attempts failed: {}", errors.len())
            }
        }
    }
}

impl Error for FirstSuccessError {}

// ---------------------------------------------------------------------------------------------
// The exercise.
// ---------------------------------------------------------------------------------------------

/// Processes all inputs with `f`, at most `workers` at a time (0 counts as 1).
/// Results are in input order. Stops on the first error, see the exercise for details.
pub fn process_all<T, R, F>(
    token: &CancelToken,
    inputs: &[T],
    workers: usize,
    f: F,
) -> Result<Vec<R>, TaskError>
where
    T: Sync,
    R: Send,
    F: Fn(&CancelToken, &T) -> Result<R, TaskError> + Sync,
{
    // Our own child token: cancelling it stops our workers, not the caller's other work.
    let token = token.child();
    let next_index = AtomicUsize::new(0);
    let first_error: Mutex<Option<TaskError>> = Mutex::new(None);
    let (tx, rx) = mpsc::channel();

    thread::scope(|s| {
        for _ in 0..workers.max(1) {
            // Each worker gets its own Sender (moved in) and shared references to the rest.
            // Plain references work because the scope outlives every thread spawned in it.
            let tx = tx.clone();
            let (token, next_index, first_error, f) = (&token, &next_index, &first_error, &f);
            s.spawn(move || {
                while !token.is_cancelled() {
                    // Relaxed is enough: we only need each index handed out once, not ordering
                    // with other memory. (The Go solution uses an unbuffered channel instead.)
                    let i = next_index.fetch_add(1, Ordering::Relaxed);
                    let Some(input) = inputs.get(i) else { break };
                    match f(token, input) {
                        // send fails only if the receiver is gone, which can't happen before
                        // the scope ends, hence unwrap.
                        Ok(result) => tx.send((i, result)).unwrap(),
                        Err(err) => {
                            // Keep only the first error. Later ones are usually Cancelled,
                            // caused by our own cancel() below.
                            first_error.lock().unwrap().get_or_insert(err);
                            token.cancel();
                            break;
                        }
                    }
                }
            });
        }
    }); // all workers have finished here

    // Drop our Sender so the channel closes; otherwise iterating rx would wait forever.
    drop(tx);

    if let Some(err) = first_error.into_inner().unwrap() {
        return Err(err);
    }
    if token.is_cancelled() {
        // Cancelled from outside before every input was processed.
        return Err(TaskError::Cancelled);
    }

    // Results arrive in completion order. Put each into its slot, then unwrap the slots.
    let mut slots: Vec<Option<R>> = (0..inputs.len()).map(|_| None).collect();
    for (i, result) in rx {
        slots[i] = Some(result);
    }
    // collect() into Option<Vec<R>> gives None if any slot is empty, which can't happen
    // when no worker failed and nobody cancelled.
    Ok(slots
        .into_iter()
        .collect::<Option<Vec<R>>>()
        .expect("every input produced a result"))
}

/// Starts all functions at once and returns the first successful result; the others get
/// a cancelled token. If all fail -> `AllFailed` with all errors; an empty slice -> `NoFunctions`.
pub fn first_success<R: Send>(
    token: &CancelToken,
    fns: &[Attempt<'_, R>],
) -> Result<R, FirstSuccessError> {
    if fns.is_empty() {
        return Err(FirstSuccessError::NoFunctions);
    }
    let token = token.child();
    let (tx, rx) = mpsc::channel();

    thread::scope(|s| {
        for &f in fns {
            let tx = tx.clone();
            let token = &token;
            s.spawn(move || {
                // After the first success the receiver may be gone: ignore the send error.
                let _ = tx.send(f(token));
            });
        }
        drop(tx); // only the workers' Senders remain, so rx ends once they all finished

        let mut errors = Vec::new();
        for result in rx {
            match result {
                Ok(value) => {
                    token.cancel();
                    // Returns from the scope's closure. thread::scope still waits for the
                    // cancelled threads before first_success returns.
                    return Ok(value);
                }
                Err(err) => errors.push(err),
            }
        }
        Err(FirstSuccessError::AllFailed(errors))
    })
}
