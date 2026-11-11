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

impl fmt::Display for LaneState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            LaneState::Open => "open",
            LaneState::Restricted => "restricted",
            LaneState::Closed => "closed",
            LaneState::Maintenance => "maintenance",
        })
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

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Action::Restrict => "restrict",
            Action::Close => "close",
            Action::Reopen => "reopen",
            Action::StartMaintenance => "start maintenance",
            Action::EndMaintenance => "end maintenance",
        })
    }
}

/// An action that isn't allowed in the given state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidTransition {
    pub state: LaneState,
    pub action: Action,
}

impl fmt::Display for InvalidTransition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} + {}: invalid transition", self.state, self.action)
    }
}

impl Error for InvalidTransition {}

/// The new state after the action, or an error saying which state and action didn't match.
pub fn next_state(state: LaneState, action: Action) -> Result<LaneState, InvalidTransition> {
    use Action::*;
    use LaneState::*;

    // The whole transition table in one match. Go needs nested switches, C# has the same
    // tuple switch expression, Python matches on a tuple too.
    match (state, action) {
        (Open, Restrict) => Ok(Restricted),
        (Open | Restricted, Close) => Ok(Closed),
        (Restricted | Closed, Reopen) => Ok(Open),
        (Closed, StartMaintenance) => Ok(Maintenance),
        (Maintenance, EndMaintenance) => Ok(Closed),
        // Go returns the original state next to the error. A Result is either a value or an
        // error, so the state travels inside the error instead.
        _ => Err(InvalidTransition { state, action }),
    }
}

/// A lane with a state and a history of states.
#[derive(Debug, Clone)]
pub struct Lane {
    pub name: String,
    state: LaneState,
    history: Vec<LaneState>,
}

impl Lane {
    /// A new open lane. The history starts with Open.
    pub fn new(name: &str) -> Self {
        Lane {
            name: name.to_string(),
            state: LaneState::Open,
            history: vec![LaneState::Open],
        }
    }

    pub fn state(&self) -> LaneState {
        self.state // LaneState is Copy, so this returns a copy, not a borrow
    }

    /// Performs the actions in order. Stops at the first invalid one and returns its error;
    /// the state stays as after the last valid action.
    pub fn apply(&mut self, actions: &[Action]) -> Result<(), InvalidTransition> {
        for &action in actions {
            // `?` returns the error before the assignment, so the state stays unchanged.
            self.state = next_state(self.state, action)?;
            self.history.push(self.state);
        }
        Ok(())
    }

    /// The history, read-only.
    pub fn history(&self) -> &[LaneState] {
        &self.history // &Vec<T> coerces to &[T]
    }
}
