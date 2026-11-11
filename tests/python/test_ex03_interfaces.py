import pytest

from ex03_interfaces.sensors import (
    CoSensor,
    Sensor,
    Severity,
    SmokeDetector,
    VisibilitySensor,
    alarming,
    describe,
    worst,
)


def enum_name(value: object) -> str | None:
    """Test id from .name, not from str(): __str__ is part of the exercise and while it is TODO it would break test collection."""
    return getattr(value, "name", None)


class FakeSensor:
    """A custom type in the test — doesn't inherit from Sensor, yet it is one (structural typing)."""

    def __init__(self, sensor_id: str) -> None:
        self.sensor_id = sensor_id

    def evaluate(self) -> Severity:
        return Severity.WARNING


def sensors() -> list[Sensor]:
    return [
        CoSensor("co-1", ppm=12),
        CoSensor("co-2", ppm=75),
        VisibilitySensor("vis-1", meters=40),
        SmokeDetector("smoke-1", triggered=False),
    ]


@pytest.mark.parametrize(
    ("sensor", "expected"),
    [
        (CoSensor("x", ppm=49.9), Severity.OK),
        (CoSensor("x", ppm=50), Severity.WARNING),
        (CoSensor("x", ppm=100), Severity.ALARM),
        (VisibilitySensor("x", meters=150), Severity.OK),
        (VisibilitySensor("x", meters=149), Severity.WARNING),
        (VisibilitySensor("x", meters=49), Severity.ALARM),
        (SmokeDetector("x", triggered=True), Severity.ALARM),
        (SmokeDetector("x"), Severity.OK),
    ],
    ids=enum_name,
)
def test_evaluate(sensor, expected):
    assert sensor.evaluate() == expected


def test_severity_str_and_order():
    assert " ".join(str(s) for s in Severity) == "OK WARNING ALARM"
    assert f"state: {Severity.ALARM}" == "state: ALARM"
    assert Severity.OK < Severity.WARNING < Severity.ALARM


def test_fake_sensor_is_a_sensor():
    assert isinstance(FakeSensor("f"), Sensor)


def test_worst():
    assert worst(sensors()) == Severity.ALARM
    assert worst([]) == Severity.OK


def test_alarming():
    all_sensors = [*sensors(), FakeSensor("fake")]
    assert alarming(all_sensors, Severity.WARNING) == ["co-2", "vis-1", "fake"]


@pytest.mark.parametrize(
    ("sensor", "expected"),
    [
        (CoSensor("co-2", ppm=75), "CO co-2: 75.0 ppm"),
        (VisibilitySensor("vis-1", meters=120), "Visibility vis-1: 120 m"),
        (SmokeDetector("d-3", triggered=True), "Smoke d-3: yes"),
        (SmokeDetector("d-4"), "Smoke d-4: no"),
        (FakeSensor("x"), "Unknown sensor x"),
    ],
)
def test_describe(sensor, expected):
    assert describe(sensor) == expected


def test_sensors_are_immutable():
    s = CoSensor("co-1", ppm=12)
    with pytest.raises(AttributeError):  # dataclasses.FrozenInstanceError is a subclass of AttributeError
        s.ppm = 99  # type: ignore[misc]
