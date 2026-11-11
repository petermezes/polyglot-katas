"""Exercise 06: state machine — Enum, match on tuples, property, *args, encapsulation."""

from enum import Enum


class LaneState(Enum):
    """The enum value is its label directly — no names array needed like in Go."""

    OPEN = "open"
    RESTRICTED = "restricted"
    CLOSED = "closed"
    MAINTENANCE = "maintenance"

    def __str__(self) -> str:
        return self.value


class Action(Enum):
    RESTRICT = "restrict"
    CLOSE = "close"
    REOPEN = "reopen"
    START_MAINTENANCE = "start maintenance"
    END_MAINTENANCE = "end maintenance"

    def __str__(self) -> str:
        return self.value


class InvalidTransitionError(Exception):
    def __init__(self, state: LaneState, action: Action) -> None:
        super().__init__(f"{state} + {action}: invalid transition")
        self.state = state
        self.action = action


def next_state(state: LaneState, action: Action) -> LaneState:
    """The new state after the action, or InvalidTransitionError. (The name `next` would shadow a built-in.)

    OPEN        --RESTRICT--> RESTRICTED   OPEN       --CLOSE-->             CLOSED
    RESTRICTED  --CLOSE-->    CLOSED       RESTRICTED --REOPEN-->            OPEN
    CLOSED      --REOPEN-->   OPEN         CLOSED     --START_MAINTENANCE--> MAINTENANCE
    MAINTENANCE --END_MAINTENANCE--> CLOSED
    """
    # match on a tuple, like C# `(state, action) switch`. `|` = several patterns in one branch.
    match (state, action):
        case (LaneState.OPEN, Action.RESTRICT):
            return LaneState.RESTRICTED
        case (LaneState.OPEN | LaneState.RESTRICTED, Action.CLOSE):
            return LaneState.CLOSED
        case (LaneState.RESTRICTED | LaneState.CLOSED, Action.REOPEN):
            return LaneState.OPEN
        case (LaneState.CLOSED, Action.START_MAINTENANCE):
            return LaneState.MAINTENANCE
        case (LaneState.MAINTENANCE, Action.END_MAINTENANCE):
            return LaneState.CLOSED
        case _:
            raise InvalidTransitionError(state, action)
    # A data-driven alternative: _TRANSITIONS = {(LaneState.OPEN, Action.RESTRICT): LaneState.RESTRICTED, ...}
    # and then _TRANSITIONS.get((state, action)). Reads better for large state machines.


class Lane:
    """A lane with a state and history. Underscore = "private" by convention only, Python doesn't enforce it."""

    def __init__(self, name: str) -> None:
        self.name = name
        self._state = LaneState.OPEN
        # Never `def __init__(self, name, history=[])`: the default list is shared by all instances.
        self._history: list[LaneState] = [LaneState.OPEN]

    @property
    def state(self) -> LaneState:
        """Getter only: `lane.state = ...` raises AttributeError."""
        return self._state

    @property
    def history(self) -> tuple[LaneState, ...]:
        """A tuple is an immutable copy: the caller can't change our internal list."""
        return tuple(self._history)

    def apply(self, *actions: Action) -> None:
        """Performs the actions in order. Raises on the first invalid one; the state stays as after the last valid one."""
        for action in actions:
            # If next_state raises, the assignment doesn't happen, so the state stays valid.
            self._state = next_state(self._state, action)
            self._history.append(self._state)
