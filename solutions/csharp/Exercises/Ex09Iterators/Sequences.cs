// Exercise 09: iterators — yield return, lazy evaluation, deferred exceptions.
namespace Exercises.Ex09Iterators;

public static class Sequences
{
    /// <summary>
    /// start, start+step, ... while the value is &lt; end. For step &lt;= 0 throws
    /// ArgumentOutOfRangeException RIGHT AT the call, not only on the first iteration.
    /// </summary>
    public static IEnumerable<int> Range(int start, int end, int step)
    {
        ArgumentOutOfRangeException.ThrowIfNegativeOrZero(step);
        return Iterate();

        // A local function with yield: the validation above runs eagerly, the iteration lazily.
        IEnumerable<int> Iterate()
        {
            for (var i = start; i < end; i += step)
                yield return i;
        }
    }

    /// <summary>The infinite sequence 0, 1, 1, 2, 3, 5, ...</summary>
    public static IEnumerable<long> Fibonacci()
    {
        long a = 0, b = 1;
        while (true)
        {
            yield return a;
            (a, b) = (b, a + b);
        }
    }

    /// <summary>At most the first n elements. Must not read more from the source than needed.</summary>
    public static IEnumerable<T> MyTake<T>(this IEnumerable<T> source, int n)
    {
        if (n <= 0)
            yield break;
        var i = 0;
        foreach (var x in source)
        {
            yield return x;
            if (++i >= n)
                yield break;
        }
    }

    /// <summary>A lazy filter.</summary>
    public static IEnumerable<T> MyWhere<T>(this IEnumerable<T> source, Func<T, bool> predicate)
    {
        foreach (var x in source)
            if (predicate(x))
                yield return x;
    }

    /// <summary>Sliding windows: [1,2,3,4], 2 -> [1,2] [2,3] [3,4]. Each window is a new array.</summary>
    public static IEnumerable<T[]> Window<T>(this IReadOnlyList<T> source, int size)
    {
        ArgumentOutOfRangeException.ThrowIfNegativeOrZero(size);
        return Iterate();

        IEnumerable<T[]> Iterate()
        {
            for (var i = 0; i + size <= source.Count; i++)
            {
                var window = new T[size];
                for (var j = 0; j < size; j++)
                    window[j] = source[i + j];
                yield return window;
            }
        }
    }

    /// <summary>Key–value pairs sorted by key.</summary>
    public static IEnumerable<(TKey Key, TValue Value)> SortedPairs<TKey, TValue>(this IReadOnlyDictionary<TKey, TValue> dict)
    {
        foreach (var key in dict.Keys.Order())
            yield return (key, dict[key]);
    }
}
