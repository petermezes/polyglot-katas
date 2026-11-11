// Exercise 05: data queries — LINQ: GroupBy, ToDictionary, OrderBy, MaxBy, aggregations.
// Try writing each method as a single LINQ expression.
namespace Exercises.Ex05Queries;

/// <summary>An event from a tunnel. Severity: 1 = info, 2 = warning, 3 = alarm. At is in UTC.</summary>
public record TunnelEvent(string Tunnel, int Severity, DateTime At);

public record TunnelStat(string Tunnel, int Count, int MaxSeverity);

public static class EventQueries
{
    public static Dictionary<string, int> CountByTunnel(IEnumerable<TunnelEvent> events) =>
        throw new NotImplementedException();

    /// <summary>The latest event for each tunnel. Tip: MaxBy (.NET 6+).</summary>
    public static Dictionary<string, TunnelEvent> LatestByTunnel(IEnumerable<TunnelEvent> events) =>
        throw new NotImplementedException();

    /// <summary>At most n tunnels by count descending, ties by name (ordinal).</summary>
    public static IReadOnlyList<TunnelStat> TopTunnels(IEnumerable<TunnelEvent> events, int n) =>
        throw new NotImplementedException();

    /// <summary>Events in the interval [from, to) sorted by time. Does not modify the input.</summary>
    public static IReadOnlyList<TunnelEvent> Between(IEnumerable<TunnelEvent> events, DateTime from, DateTime to) =>
        throw new NotImplementedException();

    /// <summary>Event count by hour of day — an array of length 24.</summary>
    public static int[] HourlyHistogram(IEnumerable<TunnelEvent> events) =>
        throw new NotImplementedException();
}
