// Exercise 03: interfaces — explicit implementation, records, pattern matching.
using System.Globalization;

namespace Exercises.Ex03Interfaces;

public enum Severity { Ok, Warning, Alarm }

public interface ISensor
{
    string Id { get; }
    Severity Evaluate();
}

/// <summary>CO in ppm: &gt;= 50 warning, &gt;= 100 alarm.</summary>
public record CoSensor(string Id, double Ppm) : ISensor
{
    public Severity Evaluate() => Ppm switch
    {
        >= 100 => Severity.Alarm,
        >= 50 => Severity.Warning,
        _ => Severity.Ok,
    };
}

/// <summary>Visibility in meters: &lt; 150 warning, &lt; 50 alarm.</summary>
public record VisibilitySensor(string Id, double Meters) : ISensor
{
    public Severity Evaluate() => Meters switch
    {
        < 50 => Severity.Alarm,
        < 150 => Severity.Warning,
        _ => Severity.Ok,
    };
}

public record SmokeDetector(string Id, bool Triggered) : ISensor
{
    public Severity Evaluate() => Triggered ? Severity.Alarm : Severity.Ok;
}

public static class SensorTools
{
    /// <summary>The worst severity; Ok for empty input.</summary>
    public static Severity Worst(IEnumerable<ISensor> sensors) =>
        sensors.Select(s => s.Evaluate()).DefaultIfEmpty(Severity.Ok).Max();

    /// <summary>IDs of sensors with severity at least min, in original order.</summary>
    public static IReadOnlyList<string> Alarming(IEnumerable<ISensor> sensors, Severity min) =>
        sensors.Where(s => s.Evaluate() >= min).Select(s => s.Id).ToList();

    /// <summary>
    /// Description based on the concrete type (switch expression + type/property patterns).
    /// Format numbers culture-independently (SK/NO culture would use a decimal comma).
    /// </summary>
    public static string Describe(ISensor sensor) => sensor switch
    {
        CoSensor co => string.Create(CultureInfo.InvariantCulture, $"CO {co.Id}: {co.Ppm:F1} ppm"),
        VisibilitySensor v => string.Create(CultureInfo.InvariantCulture, $"Visibility {v.Id}: {v.Meters:F0} m"),
        SmokeDetector { Triggered: true } d => $"Smoke {d.Id}: yes",
        SmokeDetector d => $"Smoke {d.Id}: no",
        _ => $"Unknown sensor {sensor.Id}",
    };
}
