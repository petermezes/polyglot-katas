// Exercise 03: interfaces — explicit implementation, records, pattern matching.
namespace Exercises.Ex03Interfaces;

public enum Severity { Ok, Warning, Alarm }

public interface ISensor
{
    string Id { get; }
    Severity Evaluate();
}

/// <summary>
/// CO in ppm: &gt;= 50 warning, &gt;= 100 alarm.
/// Tip: a switch expression with relational patterns: Ppm switch { &gt;= 100 =&gt; ..., _ =&gt; ... }
/// </summary>
public record CoSensor(string Id, double Ppm) : ISensor
{
    public Severity Evaluate() => throw new NotImplementedException();
}

/// <summary>Visibility in meters: &lt; 150 warning, &lt; 50 alarm.</summary>
public record VisibilitySensor(string Id, double Meters) : ISensor
{
    public Severity Evaluate() => throw new NotImplementedException();
}

/// <summary>Triggered detector = alarm.</summary>
public record SmokeDetector(string Id, bool Triggered) : ISensor
{
    public Severity Evaluate() => throw new NotImplementedException();
}

public static class SensorTools
{
    /// <summary>
    /// The worst severity; Ok for empty input.
    /// Tip: Max() on an empty sequence throws — DefaultIfEmpty.
    /// </summary>
    public static Severity Worst(IEnumerable<ISensor> sensors) =>
        throw new NotImplementedException();

    /// <summary>IDs of sensors with severity at least min, in original order.</summary>
    public static IReadOnlyList<string> Alarming(IEnumerable<ISensor> sensors, Severity min) =>
        throw new NotImplementedException();

    /// <summary>
    /// Description based on the concrete type:
    ///   CoSensor         -> "CO co-2: 75.0 ppm"
    ///   VisibilitySensor -> "Visibility vis-1: 120 m"
    ///   SmokeDetector    -> "Smoke d-3: yes" / "Smoke d-4: no"
    ///   other type       -> "Unknown sensor x"
    /// Tip: sensor switch { CoSensor co =&gt; ..., SmokeDetector { Triggered: true } d =&gt; ..., _ =&gt; ... }
    /// Careful: in SK/NO culture {x:F1} would print "75,0" — use
    /// string.Create(CultureInfo.InvariantCulture, $"...").
    /// </summary>
    public static string Describe(ISensor sensor) =>
        throw new NotImplementedException();
}
