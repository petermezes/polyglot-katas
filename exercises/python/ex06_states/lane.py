"""Exercise 06: state machine — Enum, match on tuples, property, *args, encapsulation."""

from enum import Enum


class LaneState(Enum):
    """The enum value is its label directly — no names array needed like in Go."""

    OPEN = "open"
    RESTRICTED = "restricted"
    CLOSED = "closed"
    MAINTENANCE = "maintenance"

    def __str__(self) -> str:
        """str(LaneState.OPEN) == "open". Tip: self.value."""
        raise NotImplementedError("TODO")


class Action(Enum):
    RESTRICT = "restrict"
    CLOSE = "close"
    REOPEN = "reopen"
    START_MAINTENANCE = "start maintenance"
    END_MAINTENANCE = "end maintenance"

    def __str__(self) -> str:
        raise NotImplementedError("TODO")


class InvalidTransitionError(Exception):
    """Attributes state, action. Message e.g.: maintenance + reopen: invalid transition"""

    def __init__(self, state: LaneState, action: Action) -> None:
        raise NotImplementedError("TODO")


def next_state(state: LaneState, action: Action) -> LaneState:
    """The new state after the action, or InvalidTransitionError. (The name `next` would shadow a built-in.)

    OPEN        --RESTRICT--> RESTRICTED   OPEN       --CLOSE-->             CLOSED
    RESTRICTED  --CLOSE-->    CLOSED       RESTRICTED --REOPEN-->            OPEN
    CLOSED      --REOPEN-->   OPEN         CLOSED     --START_MAINTENANCE--> MAINTENANCE
    MAINTENANCE --END_MAINTENANCE--> CLOSED

    Tip: match (state, action): case (LaneState.OPEN, Action.RESTRICT): ...
    Several patterns in one branch with |: case (LaneState.OPEN | LaneState.RESTRICTED, Action.CLOSE).
    Everything else: case _: raise ...
    """
    raise NotImplementedError("TODO")


class Lane:
    """A lane with a state and history. Underscore (_state) = "private" by convention only."""

    def __init__(self, name: str) -> None:
        """A new lane is open, the history starts with OPEN.

        Careful: never `def __init__(self, name, history=[])` — the default list is shared
        by all instances.
        """
        raise NotImplementedError("TODO")

    @property
    def state(self) -> LaneState:
        """Getter only (no @state.setter), so `lane.state = ...` raises AttributeError."""
        raise NotImplementedError("TODO")

    @property
    def history(self) -> tuple[LaneState, ...]:
        """A copy of the history the caller can't change. Tip: tuple(...)."""
        raise NotImplementedError("TODO")

    def apply(self, *actions: Action) -> None:
        """Performs the actions in order (*actions = variadic parameter, like ...Action in Go).
        Raises on the first invalid one; the state stays as after the last valid action."""
        raise NotImplementedError("TODO")
