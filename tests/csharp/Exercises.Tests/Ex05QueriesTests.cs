using Exercises.Ex05Queries;
using Xunit;

namespace Exercises.Tests.Ex05Queries;

public class EventQueriesTests
{
    private static DateTime At(int day, int hour, int minute) =>
        new(2026, 11, day, hour, minute, 0, DateTimeKind.Utc);

    private static List<TunnelEvent> Sample() =>
    [
        new("Løvstakk", 1, At(3, 8, 15)),
        new("Fløyfjell", 3, At(3, 7, 50)),
        new("Løvstakk", 2, At(3, 9, 0)),
        new("Damsgård", 1, At(2, 23, 59)),
        new("Fløyfjell", 1, At(3, 8, 5)),
        new("Løvstakk", 1, At(3, 6, 30)),
        new("Damsgård", 2, At(3, 8, 45)),
    ];

    [Fact]
    public void CountByTunnel()
    {
        var counts = EventQueries.CountByTunnel(Sample());

        Assert.Equal(3, counts.Count);
        Assert.Equal(3, counts["Løvstakk"]);
        Assert.Equal(2, counts["Fløyfjell"]);
        Assert.Equal(2, counts["Damsgård"]);
    }

    [Fact]
    public void LatestByTunnel()
    {
        var latest = EventQueries.LatestByTunnel(Sample());

        Assert.Equal(At(3, 9, 0), latest["Løvstakk"].At);
        Assert.Equal(1, latest["Fløyfjell"].Severity);
    }

    [Fact]
    public void TopTunnels()
    {
        var expected = new[]
        {
            new TunnelStat("Løvstakk", 3, 2),
            new TunnelStat("Damsgård", 2, 2),
        };

        Assert.Equal(expected, EventQueries.TopTunnels(Sample(), 2));
        Assert.Empty(EventQueries.TopTunnels([], 5));
    }

    [Fact]
    public void Between_IsHalfOpenAndSorted()
    {
        var events = Sample();
        var original = events.ToList();

        var times = EventQueries.Between(events, At(3, 7, 50), At(3, 9, 0)).Select(e => e.At);

        Assert.Equal(new[] { At(3, 7, 50), At(3, 8, 5), At(3, 8, 15), At(3, 8, 45) }, times);
        Assert.Equal(original, events);
    }

    [Fact]
    public void HourlyHistogram()
    {
        var hist = EventQueries.HourlyHistogram(Sample());

        Assert.Equal(24, hist.Length);
        var expected = new int[24];
        expected[6] = 1;
        expected[7] = 1;
        expected[8] = 3;
        expected[9] = 1;
        expected[23] = 1;
        Assert.Equal(expected, hist);
    }
}
