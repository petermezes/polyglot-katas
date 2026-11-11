// Exercise 10: concurrency — Task, async/await, SemaphoreSlim, CancellationToken, WhenAll/WhenAny.
namespace Exercises.Ex10Concurrency;

public static class Concurrency
{
    /// <summary>
    /// Processes all inputs, at most maxParallel at a time (treat maxParallel &lt; 1 as 1).
    /// Results are in input order. On the first exception it cancels the rest (via the token) and throws
    /// the ORIGINAL exception — not AggregateException or OperationCanceledException from the other tasks.
    ///
    /// Tip:
    ///   - using var cts = CancellationTokenSource.CreateLinkedTokenSource(ct);
    ///   - using var gate = new SemaphoreSlim(maxParallel); await gate.WaitAsync(cts.Token) ... finally gate.Release()
    ///   - results go into the array results[i]
    ///   - store the first exception (Interlocked.CompareExchange) and call cts.Cancel()
    ///   - await Task.WhenAll(...) throws only one of the exceptions — which one is not guaranteed;
    ///     rethrow the original with ExceptionDispatchInfo.Throw(first)
    /// </summary>
    public static Task<IReadOnlyList<TResult>> ProcessAllAsync<T, TResult>(
        IReadOnlyList<T> inputs,
        int maxParallel,
        Func<T, CancellationToken, Task<TResult>> f,
        CancellationToken ct = default) =>
        throw new NotImplementedException();

    /// <summary>
    /// Starts all functions at once, returns the first successful result and cancels the rest.
    /// If all fail, throws an AggregateException with all the errors.
    /// Empty list: ArgumentException.
    /// Tip: a loop with await Task.WhenAny(pending), pending.Remove(done), done.IsCompletedSuccessfully.
    /// </summary>
    public static Task<TResult> FirstSuccessAsync<TResult>(
        IReadOnlyList<Func<CancellationToken, Task<TResult>>> fns,
        CancellationToken ct = default) =>
        throw new NotImplementedException();
}
