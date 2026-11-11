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

    def __str__(self) -> str:
        """Like fmt.Stringer / ToString(): "OK", "WARNING", "ALARM"."""
        raise NotImplementedError("TODO")


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
        raise NotImplementedError("TODO")


@dataclass(frozen=True)
class VisibilitySensor:
    """Visibility in meters. < 150 warning, < 50 alarm."""

    sensor_id: str
    meters: float

    def evaluate(self) -> Severity:
        raise NotImplementedError("TODO")


@dataclass(frozen=True)
class SmokeDetector:
    """Triggered detector = alarm."""

    sensor_id: str
    triggered: bool = False

    def evaluate(self) -> Severity:
        raise NotImplementedError("TODO")


def worst(sensors: Iterable[Sensor]) -> Severity:
    """The worst severity. OK for empty input.

    Tip: the built-in max() has a default= parameter for empty input.
    """
    raise NotImplementedError("TODO")


def alarming(sensors: Iterable[Sensor], minimum: Severity) -> list[str]:
    """IDs of sensors with severity at least minimum, in original order.

    Tip: list comprehension [... for s in sensors if ...].
    """
    raise NotImplementedError("TODO")


def describe(sensor: Sensor) -> str:
    """Description based on the concrete sensor type:

      CoSensor         -> "CO co-2: 75.0 ppm"
      VisibilitySensor -> "Visibility vis-1: 120 m"
      SmokeDetector    -> "Smoke d-3: yes" / "Smoke d-4: no"
      other type       -> "Unknown sensor x"

    Tip: match sensor: case CoSensor(sid, ppm): ... (dataclass generates __match_args__,
    so the positional pattern works too). By keyword: case SmokeDetector(sensor_id=sid, triggered=True).
    Number format: f"{x:.1f}". Careful: `case OK:` (a bare name) compares nothing, it captures everything
    into variable OK. Write constants dotted: `case Severity.OK:`.
    """
    raise NotImplementedError("TODO")
