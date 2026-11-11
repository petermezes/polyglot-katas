// Exercise 08: dependency injection and testability — TimeProvider, Lock (.NET 9+), async loading.
using System.Diagnostics.CodeAnalysis;

namespace Exercises.Ex08Cache;

/// <summary>
/// A cache with expiration. It must be safe for concurrent use.
/// Tip: a private Dictionary&lt;TKey, Entry&gt;, where Entry is e.g.
/// private readonly record struct Entry(TValue Value, DateTimeOffset ExpiresAt);
/// and a lock (private readonly Lock _gate = new(); — System.Threading.Lock, .NET 9+).
/// </summary>
public sealed class TtlCache<TKey, TValue> where TKey : notnull
{
    /// <summary>
    /// If time is not given, TimeProvider.System is used. Tests plug in
    /// their own TimeProvider (see ManualTimeProvider in Ex08CacheTests.cs).
    /// </summary>
    public TtlCache(TimeSpan ttl, TimeProvider? time = null) => throw new NotImplementedException();

    /// <summary>Stores a value; valid until now+ttl (at exactly now+ttl no longer valid). Time: time.GetUtcNow().</summary>
    public void Set(TKey key, TValue value) => throw new NotImplementedException();

    /// <summary>Returns a valid value; an expired entry is deleted right away.</summary>
    public bool TryGet(TKey key, [MaybeNullWhen(false)] out TValue value) => throw new NotImplementedException();

    /// <summary>Number of valid (unexpired) entries.</summary>
    public int Count => throw new NotImplementedException();

    /// <summary>
    /// Returns the value from the cache, or loads it with load and stores it. An exception from load propagates
    /// and nothing is stored. Careful: the compiler doesn't allow await inside lock { } — and that's good.
    /// </summary>
    public Task<TValue> GetOrLoadAsync(TKey key, Func<TKey, Task<TValue>> load) =>
        throw new NotImplementedException();
}
