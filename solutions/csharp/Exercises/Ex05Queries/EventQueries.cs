// Exercise 05: data queries — LINQ: GroupBy, ToDictionary, OrderBy, MaxBy, aggregations.
namespace Exercises.Ex05Queries;

/// <summary>An event from a tunnel. Severity: 1 = info, 2 = warning, 3 = alarm. At is in UTC.</summary>
public record TunnelEvent(string Tunnel, int Severity, DateTime At);

public record TunnelStat(string Tunnel, int Count, int MaxSeverity);

public static class EventQueries
{
    public static Dictionary<string, int> CountByTunnel(IEnumerable<TunnelEvent> events) =>
        events.GroupBy(e => e.Tunnel)
              .ToDictionary(g => g.Key, g => g.Count());

    public static Dictionary<string, TunnelEvent> LatestByTunnel(IEnumerable<TunnelEvent> events) =>
        events.GroupBy(e => e.Tunnel)
              .ToDictionary(g => g.Key, g => g.MaxBy(e => e.At)!);

    /// <summary>At most n tunnels by count descending, ties by name (ordinal).</summary>
    public static IReadOnlyList<TunnelStat> TopTunnels(IEnumerable<TunnelEvent> events, int n) =>
        events.GroupBy(e => e.Tunnel)
              .Select(g => new TunnelStat(g.Key, g.Count(), g.Max(e => e.Severity)))
              .OrderByDescending(s => s.Count)
              .ThenBy(s => s.Tunnel, StringComparer.Ordinal)
              .Take(n)
              .ToList();

    /// <summary>Events in the interval [from, to) sorted by time.</summary>
    public static IReadOnlyList<TunnelEvent> Between(IEnumerable<TunnelEvent> events, DateTime from, DateTime to) =>
        events.Where(e => e.At >= from && e.At < to)
              .OrderBy(e => e.At)
              .ToList();

    /// <summary>Event count by hour of day — an array of length 24.</summary>
    public static int[] HourlyHistogram(IEnumerable<TunnelEvent> events)
    {
        var hist = new int[24];
        foreach (var e in events)
            hist[e.At.Hour]++;
        return hist;
    }
}
