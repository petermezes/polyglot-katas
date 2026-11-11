"""Exercise 05: data queries — Counter, comprehensions, sorted with a key, datetime.
(In C# you do the same with LINQ, in Go with loops and maps.)"""

from collections.abc import Iterable
from dataclasses import dataclass
from datetime import datetime


@dataclass(frozen=True)
class Event:
    """An event from a tunnel. at is always a datetime with a time zone (aware)."""

    tunnel: str
    severity: int  # 1 = info, 2 = warning, 3 = alarm
    at: datetime


@dataclass(frozen=True)
class TunnelStat:
    tunnel: str
    count: int
    max_severity: int


def count_by_tunnel(events: Iterable[Event]) -> dict[str, int]:
    """The number of events for each tunnel.

    Tip: Counter(generator expression).
    """
    raise NotImplementedError("TODO")


def latest_by_tunnel(events: Iterable[Event]) -> dict[str, Event]:
    """The latest event for each tunnel.

    Tip: dict.get(key) returns None instead of KeyError.
    """
    raise NotImplementedError("TODO")


def top_tunnels(events: Iterable[Event], n: int) -> list[TunnelStat]:
    """At most n tunnels by event count descending, ties by name ascending."""
    raise NotImplementedError("TODO")


def between(events: Iterable[Event], start: datetime, end: datetime) -> list[Event]:
    """Events in the interval [start, end) sorted by time. Does not modify the input.

    Tip: chained comparison start <= e.at < end, sorted(..., key=operator.attrgetter("at")).
    Careful: events.sort() would sort the caller's data.
    """
    raise NotImplementedError("TODO")


def hourly_histogram(events: Iterable[Event]) -> list[int]:
    """Event count by hour of day in UTC. The result always has 24 elements.

    Tip: [0] * 24, e.at.astimezone(datetime.UTC).hour.
    Careful: [0] * 24 is OK, but [[]] * 24 would be the same list 24 times.
    """
    raise NotImplementedError("TODO")
