using Exercises.Ex03Interfaces;
using Xunit;

namespace Exercises.Tests.Ex03Interfaces;

public class SensorTests
{
    // A custom implementation in the tests — in C# it must state the interface explicitly.
    private sealed class FakeSensor(string id) : ISensor
    {
        public string Id => id;
        public Severity Evaluate() => Severity.Warning;
    }

    private static List<ISensor> Sensors() =>
    [
        new CoSensor("co-1", 12),
        new CoSensor("co-2", 75),
        new VisibilitySensor("vis-1", 40),
        new SmokeDetector("smoke-1", false),
    ];

    // An array of tuples — a compact table of cases.
    private static readonly (ISensor Sensor, Severity Expected)[] EvaluateCases =
    [
        (new CoSensor("x", 49.9), Severity.Ok),
        (new CoSensor("x", 50), Severity.Warning),
        (new CoSensor("x", 100), Severity.Alarm),
        (new VisibilitySensor("x", 150), Severity.Ok),
        (new VisibilitySensor("x", 149), Severity.Warning),
        (new VisibilitySensor("x", 49), Severity.Alarm),
        (new SmokeDetector("x", true), Severity.Alarm),
        (new SmokeDetector("x", false), Severity.Ok),
    ];

    [Fact]
    public void Evaluate_UsesThresholds()
    {
        foreach (var (sensor, expected) in EvaluateCases)
        {
            Assert.Equal(expected, sensor.Evaluate());
        }
    }

    [Fact]
    public void Worst_ReturnsHighestSeverity()
    {
        Assert.Equal(Severity.Alarm, SensorTools.Worst(Sensors()));
        Assert.Equal(Severity.Ok, SensorTools.Worst([]));
    }

    [Fact]
    public void Alarming_KeepsInputOrder()
    {
        var all = Sensors().Append(new FakeSensor("fake"));

        Assert.Equal(new[] { "co-2", "vis-1", "fake" }, SensorTools.Alarming(all, Severity.Warning));
    }

    [Fact]
    public void Describe_DependsOnType()
    {
        Assert.Equal("CO co-2: 75.0 ppm", SensorTools.Describe(new CoSensor("co-2", 75)));
        Assert.Equal("Visibility vis-1: 120 m", SensorTools.Describe(new VisibilitySensor("vis-1", 120)));
        Assert.Equal("Smoke d-3: yes", SensorTools.Describe(new SmokeDetector("d-3", true)));
        Assert.Equal("Smoke d-4: no", SensorTools.Describe(new SmokeDetector("d-4", false)));
        Assert.Equal("Unknown sensor x", SensorTools.Describe(new FakeSensor("x")));
    }

    [Fact]
    public void Records_HaveValueEquality()
    {
        // Records compare values, not references — similar to a struct in Go.
        Assert.Equal(new CoSensor("a", 1), new CoSensor("a", 1));
        Assert.NotEqual(new CoSensor("a", 1), new CoSensor("a", 2));
    }
}
