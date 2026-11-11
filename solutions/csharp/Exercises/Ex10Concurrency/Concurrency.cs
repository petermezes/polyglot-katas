// Exercise 10: concurrency — Task, async/await, SemaphoreSlim, CancellationToken, WhenAll/WhenAny.
using System.Runtime.ExceptionServices;

namespace Exercises.Ex10Concurrency;

public static class Concurrency
{
    /// <summary>
    /// Processes all inputs, at most maxParallel at a time. Results are in input order.
    /// On the first exception it cancels the rest (via the token) and throws the ORIGINAL exception, not AggregateException
    /// or OperationCanceledException from the other tasks.
    /// </summary>
    public static async Task<IReadOnlyList<TResult>> ProcessAllAsync<T, TResult>(
        IReadOnlyList<T> inputs,
        int maxParallel,
        Func<T, CancellationToken, Task<TResult>> f,
        CancellationToken ct = default)
    {
        if (maxParallel < 1)
            maxParallel = 1;

        using var cts = CancellationTokenSource.CreateLinkedTokenSource(ct);
        using var gate = new SemaphoreSlim(maxParallel);
        var results = new TResult[inputs.Count];
        Exception? first = null;

        async Task RunOne(int i)
        {
            await gate.WaitAsync(cts.Token); // waits for a free slot; throws OCE on cancellation
            try
            {
                results[i] = await f(inputs[i], cts.Token);
            }
            catch (Exception ex)
            {
                Interlocked.CompareExchange(ref first, ex, null); // remembers only the first one
                cts.Cancel();
                throw;
            }
            finally
            {
                gate.Release();
            }
        }

        var tasks = Enumerable.Range(0, inputs.Count).Select(RunOne).ToList();
        try
        {
            await Task.WhenAll(tasks);
        }
        catch when (first is not null)
        {
            ExceptionDispatchInfo.Throw(first); // preserves the original stack trace
        }
        return results;
    }

    /// <summary>
    /// Starts all functions at once, returns the first successful result and cancels the rest.
    /// If all fail, throws an AggregateException with all the errors.
    /// Empty list: ArgumentException.
    /// </summary>
    public static async Task<TResult> FirstSuccessAsync<TResult>(
        IReadOnlyList<Func<CancellationToken, Task<TResult>>> fns,
        CancellationToken ct = default)
    {
        if (fns.Count == 0)
            throw new ArgumentException("no functions to run", nameof(fns));

        using var cts = CancellationTokenSource.CreateLinkedTokenSource(ct);
        var pending = fns.Select(fn => fn(cts.Token)).ToList();
        var errors = new List<Exception>();

        while (pending.Count > 0)
        {
            var done = await Task.WhenAny(pending);
            pending.Remove(done);

            if (done.IsCompletedSuccessfully)
            {
                cts.Cancel();
                return done.Result;
            }
            errors.Add(done.Exception?.InnerException ?? new OperationCanceledException());
        }
        throw new AggregateException(errors);
    }
}
