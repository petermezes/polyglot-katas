"""Exercise 05: data queries — Counter, comprehensions, sorted with a key, datetime."""

from collections import Counter
from collections.abc import Iterable
from dataclasses import dataclass
from datetime import UTC, datetime
from operator import attrgetter


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
    return dict(Counter(e.tunnel for e in events))


def latest_by_tunnel(events: Iterable[Event]) -> dict[str, Event]:
    latest: dict[str, Event] = {}
    for e in events:
        current = latest.get(e.tunnel)  # .get returns None instead of KeyError
        if current is None or e.at > current.at:  # aware datetimes compare with > just fine
            latest[e.tunnel] = e
    return latest


def top_tunnels(events: Iterable[Event], n: int) -> list[TunnelStat]:
    """At most n tunnels by event count descending, ties by name ascending."""
    counts: Counter[str] = Counter()
    worst: dict[str, int] = {}
    for e in events:
        counts[e.tunnel] += 1
        worst[e.tunnel] = max(worst.get(e.tunnel, 0), e.severity)
    stats = [TunnelStat(t, c, worst[t]) for t, c in counts.items()]
    stats.sort(key=lambda s: (-s.count, s.tunnel))  # stats is our own new list, an in-place sort is fine here
    return stats[:n]


def between(events: Iterable[Event], start: datetime, end: datetime) -> list[Event]:
    """Events in the interval [start, end) sorted by time. Does not modify the input."""
    # sorted() returns a new list. events.sort() would sort the caller's data (and return None).
    return sorted((e for e in events if start <= e.at < end), key=attrgetter("at"))


def hourly_histogram(events: Iterable[Event]) -> list[int]:
    """Event count by hour of day in UTC. The result always has 24 elements."""
    hist = [0] * 24  # [0] * 24 is OK (int is immutable). [[]] * 24 would be the same list 24 times!
    for e in events:
        hist[e.at.astimezone(UTC).hour] += 1
    return hist
