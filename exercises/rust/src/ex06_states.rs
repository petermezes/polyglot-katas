//! Exercise 06: state machine — enums, match on a tuple, a custom error, encapsulation.

use std::error::Error;
use std::fmt;

/// The state of a traffic lane in a tunnel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LaneState {
    Open,
    Restricted,
    Closed,
    Maintenance,
}

/// "open", "restricted", "closed", "maintenance".
///
/// Go also needs a fallback like "LaneState(42)". In Rust an enum can only hold its variants.
impl fmt::Display for LaneState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

/// An operator command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    Restrict,
    Close,
    Reopen,
    StartMaintenance,
    EndMaintenance,
}

/// "restrict", "close", "reopen", "start maintenance", "end maintenance".
impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

/// An action that isn't allowed in the given state.
/// Message exactly: "maintenance + reopen: invalid transition" (uses the Display of both enums).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidTransition {
    pub state: LaneState,
    pub action: Action,
}

impl fmt::Display for InvalidTransition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

/// An empty impl is enough: the default `source()` returns None.
impl Error for InvalidTransition {}

/// The new state after the action, or an error saying which state and action didn't match.
///
/// ```text
/// Open        --Restrict--> Restricted   Open       --Close-->            Closed
/// Restricted  --Close-->    Closed       Restricted --Reopen-->           Open
/// Closed      --Reopen-->   Open         Closed     --StartMaintenance--> Maintenance
/// Maintenance --EndMaintenance--> Closed
/// ```
///
/// Tip: `match (state, action) { (Open, Restrict) => ..., _ => Err(...) }`. Patterns can be
/// combined with `|`: `(Open | Restricted, Close)`. `use LaneState::*;` inside the function lets
/// you write the variants without the prefix.
pub fn next_state(state: LaneState, action: Action) -> Result<LaneState, InvalidTransition> {
    todo!()
}

/// A lane with a state and a history of states.
///
/// The fields without `pub` are private to this module: from outside they're reachable only
/// through the methods, so nobody can set the state without going through `apply`.
#[derive(Debug, Clone)]
pub struct Lane {
    pub name: String,
    state: LaneState,
    history: Vec<LaneState>,
}

impl Lane {
    /// A new open lane. The history starts with Open.
    pub fn new(name: &str) -> Self {
        todo!()
    }

    pub fn state(&self) -> LaneState {
        todo!()
    }

    /// Performs the actions in order. Stops at the first invalid one and returns its error;
    /// the state stays as after the last valid action.
    ///
    /// Rust has no variadic parameters (Go's `...Action`, Python's `*actions`), a slice does
    /// the job: `lane.apply(&[Close, Reopen])`. Tip: `?` inside the loop.
    pub fn apply(&mut self, actions: &[Action]) -> Result<(), InvalidTransition> {
        todo!()
    }

    /// The history, read-only.
    ///
    /// Go returns a copy, C# an IReadOnlyList wrapper, Python a tuple. Here a borrowed `&[LaneState]`
    /// is enough: the caller can't modify through a shared reference, and the borrow checker
    /// won't let `apply` run while the caller still holds it.
    pub fn history(&self) -> &[LaneState] {
        todo!()
    }
}
