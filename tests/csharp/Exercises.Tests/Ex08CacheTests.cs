using Exercises.Ex08Cache;
using Xunit;

namespace Exercises.Tests.Ex08Cache;

/// <summary>A test TimeProvider — time is advanced manually. (In practice there is also the NuGet
/// Microsoft.Extensions.TimeProvider.Testing with FakeTimeProvider.)</summary>
public sealed class ManualTimeProvider : TimeProvider
{
    private DateTimeOffset _now = new(2026, 11, 3, 10, 0, 0, TimeSpan.Zero);

    public override DateTimeOffset GetUtcNow() => _now;

    public void Advance(TimeSpan by) => _now += by;
}

public class TtlCacheTests
{
    [Fact]
    public void SetGetExpire()
    {
        var time = new ManualTimeProvider();
        var cache = new TtlCache<string, int>(TimeSpan.FromMinutes(1), time);

        cache.Set("a", 1);
        Assert.True(cache.TryGet("a", out var v));
        Assert.Equal(1, v);
        Assert.False(cache.TryGet("b", out _));

        time.Advance(TimeSpan.FromSeconds(59));
        Assert.True(cache.TryGet("a", out _));

        time.Advance(TimeSpan.FromSeconds(1));
        Assert.False(cache.TryGet("a", out _));
    }

    [Fact]
    public void Set_RefreshesTtl()
    {
        var time = new ManualTimeProvider();
        var cache = new TtlCache<string, string>(TimeSpan.FromMinutes(1), time);

        cache.Set("k", "v1");
        time.Advance(TimeSpan.FromSeconds(50));
        cache.Set("k", "v2");
        time.Advance(TimeSpan.FromSeconds(50));

        Assert.True(cache.TryGet("k", out var v));
        Assert.Equal("v2", v);
    }

    [Fact]
    public void Count_IgnoresExpired()
    {
        var time = new ManualTimeProvider();
        var cache = new TtlCache<int, string>(TimeSpan.FromSeconds(10), time);

        cache.Set(1, "x");
        time.Advance(TimeSpan.FromSeconds(5));
        cache.Set(2, "y");
        Assert.Equal(2, cache.Count);

        time.Advance(TimeSpan.FromSeconds(5));
        Assert.Equal(1, cache.Count);
    }

    [Fact]
    public async Task GetOrLoadAsync_LoadsOnceUntilExpiry()
    {
        var time = new ManualTimeProvider();
        var cache = new TtlCache<string, string>(TimeSpan.FromMinutes(1), time);
        var calls = 0;

        async Task<string> Load(string key)
        {
            calls++;
            await Task.Yield();
            return $"value-{key}";
        }

        for (var i = 0; i < 3; i++)
            Assert.Equal("value-x", await cache.GetOrLoadAsync("x", Load));
        Assert.Equal(1, calls);

        time.Advance(TimeSpan.FromMinutes(1));
        await cache.GetOrLoadAsync("x", Load);
        Assert.Equal(2, calls);
    }

    [Fact]
    public async Task GetOrLoadAsync_DoesNotStoreOnFailure()
    {
        var cache = new TtlCache<string, string>(TimeSpan.FromMinutes(1), new ManualTimeProvider());

        await Assert.ThrowsAsync<InvalidOperationException>(() =>
            cache.GetOrLoadAsync("y", _ => Task.FromException<string>(new InvalidOperationException("database unavailable"))));

        Assert.False(cache.TryGet("y", out _));
    }

    [Fact]
    public void DefaultTimeProvider_UsesRealTime()
    {
        var cache = new TtlCache<string, int>(TimeSpan.FromHours(1));
        cache.Set("a", 1);
        Assert.True(cache.TryGet("a", out _));
    }

    [Fact]
    public async Task ConcurrentAccess_IsSafe()
    {
        var cache = new TtlCache<string, int>(TimeSpan.FromMinutes(1), new ManualTimeProvider());

        var tasks = Enumerable.Range(0, 200).Select(i => Task.Run(() =>
        {
            var key = (i % 5).ToString();
            cache.Set(key, i);
            cache.TryGet(key, out _);
            _ = cache.Count;
        }));
        await Task.WhenAll(tasks);

        Assert.Equal(5, cache.Count);
    }
}
