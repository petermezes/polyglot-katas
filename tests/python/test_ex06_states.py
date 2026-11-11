import pytest

from ex06_states.lane import Action, InvalidTransitionError, Lane, LaneState, next_state


def enum_name(value: object) -> str | None:
    """Test id from .name, not from str(): __str__ is part of the exercise and while it is TODO it would break test collection."""
    return getattr(value, "name", None)


@pytest.mark.parametrize(
    ("state", "action", "expected"),
    [
        (LaneState.OPEN, Action.RESTRICT, LaneState.RESTRICTED),
        (LaneState.OPEN, Action.CLOSE, LaneState.CLOSED),
        (LaneState.RESTRICTED, Action.CLOSE, LaneState.CLOSED),
        (LaneState.RESTRICTED, Action.REOPEN, LaneState.OPEN),
        (LaneState.CLOSED, Action.REOPEN, LaneState.OPEN),
        (LaneState.CLOSED, Action.START_MAINTENANCE, LaneState.MAINTENANCE),
        (LaneState.MAINTENANCE, Action.END_MAINTENANCE, LaneState.CLOSED),
    ],
    ids=enum_name,
)
def test_next_state_valid(state, action, expected):
    assert next_state(state, action) is expected  # enum members are singletons, `is` is fine


@pytest.mark.parametrize(
    ("state", "action"),
    [
        (LaneState.OPEN, Action.REOPEN),
        (LaneState.OPEN, Action.START_MAINTENANCE),
        (LaneState.RESTRICTED, Action.START_MAINTENANCE),
        (LaneState.MAINTENANCE, Action.REOPEN),
        (LaneState.CLOSED, Action.CLOSE),
    ],
    ids=enum_name,
)
def test_next_state_invalid(state, action):
    with pytest.raises(InvalidTransitionError) as exc_info:
        next_state(state, action)
    assert (exc_info.value.state, exc_info.value.action) == (state, action)


def test_invalid_transition_message():
    with pytest.raises(InvalidTransitionError, match=r"^maintenance \+ reopen: invalid transition$"):
        next_state(LaneState.MAINTENANCE, Action.REOPEN)


def test_strings():
    assert " ".join(map(str, [LaneState.OPEN, LaneState.MAINTENANCE, Action.CLOSE])) == "open maintenance close"
    assert LaneState("maintenance") is LaneState.MAINTENANCE  # lookup by value
    with pytest.raises(ValueError):
        LaneState("nonexistent")


def test_lane():
    lane = Lane("Løvstakk lane 1")
    assert lane.state is LaneState.OPEN

    lane.apply(Action.RESTRICT, Action.CLOSE, Action.START_MAINTENANCE, Action.END_MAINTENANCE)
    assert lane.history == (
        LaneState.OPEN,
        LaneState.RESTRICTED,
        LaneState.CLOSED,
        LaneState.MAINTENANCE,
        LaneState.CLOSED,
    )

    with pytest.raises(InvalidTransitionError):
        lane.apply(Action.REOPEN, Action.RESTRICT, Action.REOPEN, Action.START_MAINTENANCE)
    assert lane.state is LaneState.OPEN, "after an error the last valid state should remain"
    assert len(lane.history) == 8


def test_lane_is_encapsulated():
    lane = Lane("lane 2")
    with pytest.raises(AttributeError):
        lane.state = LaneState.CLOSED  # type: ignore[misc]

    history = lane.history
    with pytest.raises((TypeError, AttributeError)):
        history[0] = LaneState.MAINTENANCE  # type: ignore[index]
    assert lane.history[0] is LaneState.OPEN


def test_lanes_do_not_share_history():
    a, b = Lane("a"), Lane("b")
    a.apply(Action.CLOSE)
    assert b.history == (LaneState.OPEN,)
