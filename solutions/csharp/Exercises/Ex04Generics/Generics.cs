// Exercise 04: generics — a generic class, extension methods, constraints, generic math (INumber).
using System.Diagnostics.CodeAnalysis;
using System.Numerics;

namespace Exercises.Ex04Generics;

/// <summary>A simple stack. (Stack&lt;T&gt; already exists in .NET, hence a different name.)</summary>
public class SimpleStack<T>
{
    private readonly List<T> _items = [];

    public int Count => _items.Count;

    public void Push(T value) => _items.Add(value);

    /// <summary>Try pattern: the equivalent of Go's (T, bool).</summary>
    public bool TryPop([MaybeNullWhen(false)] out T value)
    {
        if (_items.Count == 0)
        {
            value = default;
            return false;
        }
        value = _items[^1];
        _items.RemoveAt(_items.Count - 1);
        return true;
    }

    public bool TryPeek([MaybeNullWhen(false)] out T value)
    {
        if (_items.Count == 0)
        {
            value = default;
            return false;
        }
        value = _items[^1];
        return true;
    }
}

/// <summary>Your own versions of Select/Where/Aggregate — without LINQ, so you practice generics.</summary>
public static class SeqExtensions
{
    public static List<TResult> MyMap<T, TResult>(this IEnumerable<T> source, Func<T, TResult> f)
    {
        var result = new List<TResult>();
        foreach (var x in source)
            result.Add(f(x));
        return result;
    }

    public static List<T> MyFilter<T>(this IEnumerable<T> source, Func<T, bool> predicate)
    {
        var result = new List<T>();
        foreach (var x in source)
            if (predicate(x))
                result.Add(x);
        return result;
    }

    public static TAcc MyReduce<T, TAcc>(this IEnumerable<T> source, TAcc seed, Func<TAcc, T, TAcc> f)
    {
        var acc = seed;
        foreach (var x in source)
            acc = f(acc, x);
        return acc;
    }

    /// <summary>Generic math: works for int, double, decimal and custom numeric types.</summary>
    public static T SumAll<T>(this IEnumerable<T> source) where T : INumber<T>
    {
        var total = T.Zero;
        foreach (var x in source)
            total += x;
        return total;
    }

    /// <summary>The element with the largest key (the first one on ties). False for empty input.</summary>
    public static bool TryMaxBy<T, TKey>(this IEnumerable<T> source, Func<T, TKey> key, [MaybeNullWhen(false)] out T result)
        where TKey : IComparable<TKey>
    {
        using var e = source.GetEnumerator();
        if (!e.MoveNext())
        {
            result = default;
            return false;
        }
        result = e.Current;
        var bestKey = key(result);
        while (e.MoveNext())
        {
            var k = key(e.Current);
            if (k.CompareTo(bestKey) > 0)
            {
                result = e.Current;
                bestKey = k;
            }
        }
        return true;
    }
}
