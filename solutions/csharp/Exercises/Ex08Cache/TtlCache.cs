// Exercise 08: dependency injection and testability — TimeProvider, Lock (.NET 9+), async loading.
using System.Diagnostics.CodeAnalysis;

namespace Exercises.Ex08Cache;

/// <summary>A cache with expiration. Safe for concurrent use.</summary>
public sealed class TtlCache<TKey, TValue> where TKey : notnull
{
    private readonly record struct Entry(TValue Value, DateTimeOffset ExpiresAt);

    private readonly Lock _gate = new(); // System.Threading.Lock (.NET 9+); before that: object _gate = new()
    private readonly Dictionary<TKey, Entry> _items = new();
    private readonly TimeSpan _ttl;
    private readonly TimeProvider _time;

    /// <summary>If time is not given, TimeProvider.System is used.</summary>
    public TtlCache(TimeSpan ttl, TimeProvider? time = null)
    {
        _ttl = ttl;
        _time = time ?? TimeProvider.System;
    }

    /// <summary>Stores a value; valid until now+ttl (at now+ttl no longer valid).</summary>
    public void Set(TKey key, TValue value)
    {
        lock (_gate)
        {
            _items[key] = new Entry(value, _time.GetUtcNow() + _ttl);
        }
    }

    /// <summary>Returns a valid value; an expired entry is deleted right away.</summary>
    public bool TryGet(TKey key, [MaybeNullWhen(false)] out TValue value)
    {
        lock (_gate)
        {
            if (_items.TryGetValue(key, out var e))
            {
                if (_time.GetUtcNow() < e.ExpiresAt)
                {
                    value = e.Value;
                    return true;
                }
                _items.Remove(key);
            }
            value = default;
            return false;
        }
    }

    /// <summary>Number of valid entries.</summary>
    public int Count
    {
        get
        {
            lock (_gate)
            {
                var now = _time.GetUtcNow();
                return _items.Values.Count(e => now < e.ExpiresAt);
            }
        }
    }

    /// <summary>
    /// Returns the value from the cache, or loads it with load and stores it. An exception from load propagates
    /// and nothing is stored. Careful: await inside a lock is not allowed — load runs outside the lock.
    /// </summary>
    public async Task<TValue> GetOrLoadAsync(TKey key, Func<TKey, Task<TValue>> load)
    {
        if (TryGet(key, out var cached))
            return cached;

        var value = await load(key);
        Set(key, value);
        return value;
    }
}
