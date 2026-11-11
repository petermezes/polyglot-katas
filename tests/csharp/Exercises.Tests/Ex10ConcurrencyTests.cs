using System.Diagnostics;
using Exercises.Ex10Concurrency;
using Xunit;

namespace Exercises.Tests.Ex10Concurrency;

public class ConcurrencyTests
{
    [Fact]
    public async Task ProcessAll_KeepsOrderAndLimit()
    {
        var active = 0;
        var maxActive = 0;

        async Task<int> Double(int n, CancellationToken ct)
        {
            var cur = Interlocked.Increment(ref active);
            int seen;
            while (cur > (seen = Volatile.Read(ref maxActive)) &&
                   Interlocked.CompareExchange(ref maxActive, cur, seen) != seen)
            {
            }
            await Task.Delay(20 - n, ct); // later inputs finish sooner
            Interlocked.Decrement(ref active);
            return n * 2;
        }

        var results = await Concurrency.ProcessAllAsync(Enumerable.Range(0, 10).ToList(), 3, Double);

        Assert.Equal(new[] { 0, 2, 4, 6, 8, 10, 12, 14, 16, 18 }, results);
        Assert.InRange(maxActive, 2, 3);
    }

    [Fact]
    public async Task ProcessAll_StopsOnFirstErrorAndRethrowsIt()
    {
        var started = 0;

        async Task<int> F(int n, CancellationToken ct)
        {
            Interlocked.Increment(ref started);
            if (n == 2)
                throw new InvalidOperationException("sensor not responding");
            await Task.Delay(50, ct);
            return n;
        }

        var sw = Stopwatch.StartNew();
        var ex = await Assert.ThrowsAsync<InvalidOperationException>(() =>
            Concurrency.ProcessAllAsync(Enumerable.Range(0, 100).ToList(), 4, F));

        Assert.Equal("sensor not responding", ex.Message);
        Assert.InRange(started, 3, 10);
        Assert.True(sw.ElapsedMilliseconds < 500, $"took {sw.ElapsedMilliseconds} ms — cancellation doesn't work");
    }

    [Fact]
    public async Task ProcessAll_RespectsExternalCancellation()
    {
        using var cts = new CancellationTokenSource(TimeSpan.FromMilliseconds(30));

        async Task<int> Slow(int n, CancellationToken ct)
        {
            await Task.Delay(1000, ct);
            return n;
        }

        await Assert.ThrowsAsync<TaskCanceledException>(() =>
            Concurrency.ProcessAllAsync([1, 2, 3, 4, 5], 2, Slow, cts.Token));
    }

    [Fact]
    public async Task FirstSuccess_ReturnsFastestAndCancelsOthers()
    {
        var cancelled = 0;

        async Task<string> Slow(CancellationToken ct)
        {
            try
            {
                await Task.Delay(2000, ct);
                return "slow";
            }
            catch (OperationCanceledException)
            {
                Interlocked.Increment(ref cancelled);
                throw;
            }
        }

        async Task<string> Fast(CancellationToken ct)
        {
            await Task.Delay(10, ct);
            return "fast";
        }

        Task<string> Failing(CancellationToken ct) => Task.FromException<string>(new Exception("fails immediately"));

        var result = await Concurrency.FirstSuccessAsync<string>([Slow, Failing, Fast, Slow]);
        Assert.Equal("fast", result);

        var deadline = DateTime.UtcNow.AddSeconds(1);
        while (Volatile.Read(ref cancelled) < 2 && DateTime.UtcNow < deadline)
            await Task.Delay(5);
        Assert.Equal(2, cancelled);
    }

    [Fact]
    public async Task FirstSuccess_AllFail_ThrowsAggregate()
    {
        var ex = await Assert.ThrowsAsync<AggregateException>(() =>
            Concurrency.FirstSuccessAsync<int>(
            [
                _ => Task.FromException<int>(new InvalidOperationException("first")),
                _ => Task.FromException<int>(new TimeoutException("second")),
            ]));

        Assert.Equal(2, ex.InnerExceptions.Count);
        Assert.Contains(ex.InnerExceptions, e => e is TimeoutException);
    }

    [Fact]
    public async Task FirstSuccess_Empty_Throws()
    {
        await Assert.ThrowsAsync<ArgumentException>(() =>
            Concurrency.FirstSuccessAsync<int>([]));
    }
}
