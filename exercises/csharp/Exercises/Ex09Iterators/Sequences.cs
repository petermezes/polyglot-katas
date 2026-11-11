// Exercise 09: iterators — yield return, lazy evaluation, deferred exceptions.
namespace Exercises.Ex09Iterators;

public static class Sequences
{
    /// <summary>
    /// start, start+step, ... while the value is &lt; end. For step &lt;= 0 throws
    /// ArgumentOutOfRangeException RIGHT AT the call, not only on the first iteration.
    /// Careful: in a method with yield the WHOLE body (validation too) runs only on the first MoveNext.
    /// Tip: validation in a method without yield + a local function with yield that you return.
    /// </summary>
    public static IEnumerable<int> Range(int start, int end, int step) =>
        throw new NotImplementedException();

    /// <summary>The infinite sequence 0, 1, 1, 2, 3, 5, ... Tip: while (true) { yield return ...; }</summary>
    public static IEnumerable<long> Fibonacci() =>
        throw new NotImplementedException();

    /// <summary>At most the first n elements. Stop right after the n-th (yield break), don't read the next one.</summary>
    public static IEnumerable<T> MyTake<T>(this IEnumerable<T> source, int n) =>
        throw new NotImplementedException();

    /// <summary>A lazy filter.</summary>
    public static IEnumerable<T> MyWhere<T>(this IEnumerable<T> source, Func<T, bool> predicate) =>
        throw new NotImplementedException();

    /// <summary>
    /// Sliding windows: [1,2,3,4], 2 -> [1,2] [2,3] [3,4]. Each window is a new array.
    /// size &lt;= 0 -> ArgumentOutOfRangeException right at the call.
    /// </summary>
    public static IEnumerable<T[]> Window<T>(this IReadOnlyList<T> source, int size) =>
        throw new NotImplementedException();

    /// <summary>Key–value pairs sorted by key (a tuple with named elements).</summary>
    public static IEnumerable<(TKey Key, TValue Value)> SortedPairs<TKey, TValue>(this IReadOnlyDictionary<TKey, TValue> dict) =>
        throw new NotImplementedException();
}
