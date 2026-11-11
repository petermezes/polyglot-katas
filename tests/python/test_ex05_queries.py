from datetime import UTC, datetime, timedelta, timezone

from ex05_queries.event_queries import (
    Event,
    TunnelStat,
    between,
    count_by_tunnel,
    hourly_histogram,
    latest_by_tunnel,
    top_tunnels,
)


def at(day: int, hour: int, minute: int) -> datetime:
    return datetime(2026, 11, day, hour, minute, tzinfo=UTC)


def sample_events() -> list[Event]:
    return [
        Event("Løvstakk", 1, at(3, 8, 15)),
        Event("Fløyfjell", 3, at(3, 7, 50)),
        Event("Løvstakk", 2, at(3, 9, 0)),
        Event("Damsgård", 1, at(2, 23, 59)),
        Event("Fløyfjell", 1, at(3, 8, 5)),
        Event("Løvstakk", 1, at(3, 6, 30)),
        Event("Damsgård", 2, at(3, 8, 45)),
    ]


def test_count_by_tunnel():
    assert count_by_tunnel(sample_events()) == {"Løvstakk": 3, "Fløyfjell": 2, "Damsgård": 2}


def test_latest_by_tunnel():
    latest = latest_by_tunnel(sample_events())
    assert latest["Løvstakk"].at == at(3, 9, 0)
    assert latest["Fløyfjell"].severity == 1
    assert len(latest) == 3


def test_top_tunnels():
    assert top_tunnels(sample_events(), 2) == [
        TunnelStat("Løvstakk", 3, 2),
        TunnelStat("Damsgård", 2, 2),  # Damsgård < Fløyfjell alphabetically
    ]
    assert top_tunnels([], 5) == []


def test_between():
    events = sample_events()
    original = list(events)
    got = between(events, at(3, 7, 50), at(3, 9, 0))
    assert [e.at for e in got] == [at(3, 7, 50), at(3, 8, 5), at(3, 8, 15), at(3, 8, 45)]
    assert events == original, "between must not modify the input list"


def test_between_accepts_generator():
    got = between((e for e in sample_events()), at(3, 0, 0), at(4, 0, 0))
    assert len(got) == 6


def test_hourly_histogram():
    hist = hourly_histogram(sample_events())
    assert len(hist) == 24
    expected = {6: 1, 7: 1, 8: 3, 9: 1, 23: 1}
    assert hist == [expected.get(hour, 0) for hour in range(24)]


def test_hourly_histogram_converts_to_utc():
    oslo_winter = timezone(timedelta(hours=1))
    event = Event("Løvstakk", 1, datetime(2026, 11, 3, 10, 0, tzinfo=oslo_winter))  # = 09:00 UTC
    hist = hourly_histogram([event])
    assert hist[9] == 1
    assert hist[10] == 0
