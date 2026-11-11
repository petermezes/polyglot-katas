use under_test::ex06_states::*;
// Importing the variants lets the tables read like the Go ones. Action and LaneState have no
// variant names in common, so both fit into one scope.
use under_test::ex06_states::{Action::*, LaneState::*};

#[test]
fn next_state_valid_transitions() {
    let cases = [
        (Open, Restrict, Restricted),
        (Open, Close, Closed),
        (Restricted, Close, Closed),
        (Restricted, Reopen, Open),
        (Closed, Reopen, Open),
        (Closed, StartMaintenance, Maintenance),
        (Maintenance, EndMaintenance, Closed),
    ];
    for (from, action, want) in cases {
        assert_eq!(next_state(from, action), Ok(want), "{from:?} + {action:?}");
    }
}

#[test]
fn next_state_invalid_transitions() {
    let cases = [
        (Open, Reopen),
        (Open, StartMaintenance),
        (Restricted, StartMaintenance),
        (Maintenance, Reopen),
        (Closed, Close),
    ];
    for (from, action) in cases {
        assert_eq!(
            next_state(from, action),
            Err(InvalidTransition {
                state: from,
                action
            }),
            "{from:?} + {action:?}"
        );
    }
    let err = next_state(Maintenance, Reopen).unwrap_err();
    assert_eq!(err.to_string(), "maintenance + reopen: invalid transition");
}

#[test]
fn display_labels() {
    assert_eq!(
        format!("{} {} {}", Open, Maintenance, Close),
        "open maintenance close"
    );
}

#[test]
fn lane_applies_actions_and_keeps_history() {
    let mut lane = Lane::new("Løvstakk lane 1");
    assert_eq!(lane.state(), Open, "a new lane should be open");

    assert_eq!(
        lane.apply(&[Restrict, Close, StartMaintenance, EndMaintenance]),
        Ok(())
    );
    assert_eq!(
        lane.history(),
        [Open, Restricted, Closed, Maintenance, Closed]
    );

    // Reopen, Restrict and Reopen are valid, StartMaintenance from Open is not.
    assert_eq!(
        lane.apply(&[Reopen, Restrict, Reopen, StartMaintenance]),
        Err(InvalidTransition {
            state: Open,
            action: StartMaintenance
        })
    );
    assert_eq!(
        lane.state(),
        Open,
        "after an error the last valid state should remain"
    );
    assert_eq!(lane.history().last(), Some(&Open));
    // No "History() must return a copy" check like in Go: a &[LaneState] can't be modified.
}
