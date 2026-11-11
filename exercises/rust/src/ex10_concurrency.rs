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
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

// ---------------------------------------------------------------------------------------------
// Given: a minimal cancellation token. std has nothing like Go's context.Context or C#'s
// CancellationToken, so here is a small one. Read it, you don't have to change it.
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
// Your part.
// ---------------------------------------------------------------------------------------------

/// Processes all inputs with `f`, at most `workers` at a time (0 counts as 1).
/// Results are in input order.
///
/// On the first error: cancel the other tasks (they get a cancelled token), start no new ones
/// and return that first error. If `token` is cancelled from outside, return
/// `Err(TaskError::Cancelled)`.
///
/// Tips:
/// - `thread::scope(|s| { s.spawn(...); })` waits for all its threads before it returns. That's
///   why the threads may borrow `inputs`, `f` and local variables, no Arc needed (≈ a WaitGroup
///   the compiler can't let you forget).
/// - Work distribution: a shared `AtomicUsize`, each worker takes the next index with
///   `fetch_add(1, Ordering::Relaxed)` until it runs past the end.
/// - Results: an `mpsc::channel()` of `(index, R)`. The Receiver can't be shared between workers,
///   but the Sender can be cloned, one per worker.
/// - The first error: a `Mutex<Option<TaskError>>`, set only if still None, then `token.cancel()`
///   on a `token.child()` (so you don't cancel the caller's token).
///
/// The bounds say what the compiler needs to share across threads: `T: Sync` (several threads
/// read `&T`), `R: Send` (results move to this thread), `F: Sync` (all workers call the same `f`).
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
    todo!()
}

/// Starts all functions at once and returns the first successful result; the others get
/// a cancelled token. If all fail -> `AllFailed` with all errors; an empty slice -> `NoFunctions`.
///
/// With `thread::scope` this function returns only after every thread has finished, so the
/// cancelled ones are guaranteed to have stopped by then (Go's version returns earlier and the
/// leftover goroutines finish in the background).
///
/// The functions are different closures with different types, so the slice holds trait objects
/// (see `Attempt`).
pub fn first_success<R: Send>(
    token: &CancelToken,
    fns: &[Attempt<'_, R>],
) -> Result<R, FirstSuccessError> {
    todo!()
}
