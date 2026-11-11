"""Exercise 03: interfaces — Protocol (structural typing), dataclass, IntEnum, match."""

from collections.abc import Iterable
from dataclasses import dataclass
from enum import IntEnum
from typing import Protocol, runtime_checkable


class Severity(IntEnum):
    """Severity. IntEnum behaves like an int, so <, >= and max() work out of the box."""

    OK = 0
    WARNING = 1
    ALARM = 2

    def __str__(self) -> str:  # like fmt.Stringer / ToString()
        return _LABELS[self]


_LABELS = {Severity.OK: "OK", Severity.WARNING: "WARNING", Severity.ALARM: "ALARM"}


@runtime_checkable
class Sensor(Protocol):
    """A sensor is anything that has sensor_id and evaluate(). Like in Go: no inheritance, no "implements".
    @runtime_checkable allows isinstance(x, Sensor), but only checks that the members exist."""

    sensor_id: str

    def evaluate(self) -> Severity: ...


@dataclass(frozen=True)
class CoSensor:
    """CO concentration in ppm. >= 50 warning, >= 100 alarm."""

    sensor_id: str
    ppm: float

    def evaluate(self) -> Severity:
        if self.ppm >= 100:
            return Severity.ALARM
        if self.ppm >= 50:
            return Severity.WARNING
        return Severity.OK


@dataclass(frozen=True)
class VisibilitySensor:
    """Visibility in meters. < 150 warning, < 50 alarm."""

    sensor_id: str
    meters: float

    def evaluate(self) -> Severity:
        if self.meters < 50:
            return Severity.ALARM
        if self.meters < 150:
            return Severity.WARNING
        return Severity.OK


@dataclass(frozen=True)
class SmokeDetector:
    """Triggered detector = alarm."""

    sensor_id: str
    triggered: bool = False

    def evaluate(self) -> Severity:
        return Severity.ALARM if self.triggered else Severity.OK


def worst(sensors: Iterable[Sensor]) -> Severity:
    """The worst severity. OK for empty input."""
    return max((s.evaluate() for s in sensors), default=Severity.OK)


def alarming(sensors: Iterable[Sensor], minimum: Severity) -> list[str]:
    """IDs of sensors with severity at least minimum, in original order."""
    return [s.sensor_id for s in sensors if s.evaluate() >= minimum]


def describe(sensor: Sensor) -> str:
    """Description based on the concrete type (like a type switch / C# switch with property patterns)."""
    match sensor:
        # Dataclass generates __match_args__, which is why the positional pattern CoSensor(sid, ppm) works.
        case CoSensor(sid, ppm):
            return f"CO {sid}: {ppm:.1f} ppm"
        case VisibilitySensor(sensor_id=sid, meters=m):  # or by keyword
            return f"Visibility {sid}: {m:.0f} m"
        case SmokeDetector(sid, triggered=True):
            return f"Smoke {sid}: yes"
        case SmokeDetector(sid):
            return f"Smoke {sid}: no"
        case _:
            return f"Unknown sensor {sensor.sensor_id}"
    # Trap: `case OK:` (a bare name) is not a comparison but a capture into variable OK — it matches everything.
    # Constants must be dotted: `case Severity.OK:`.
