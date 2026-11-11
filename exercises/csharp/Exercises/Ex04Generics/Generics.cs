// Exercise 04: generics — a generic class, extension methods, constraints, generic math (INumber).
using System.Diagnostics.CodeAnalysis;
using System.Numerics;

namespace Exercises.Ex04Generics;

/// <summary>A simple stack. (Stack&lt;T&gt; already exists in .NET, hence a different name.)</summary>
public class SimpleStack<T>
{
    // TODO: a private field (e.g. List<T>)

    public int Count => throw new NotImplementedException();

    public void Push(T value) => throw new NotImplementedException();

    /// <summary>
    /// Try pattern: the equivalent of Go's (T, bool). On an empty stack value = default and false.
    /// [MaybeNullWhen(false)] tells the compiler that value may be null when false.
    /// </summary>
    public bool TryPop([MaybeNullWhen(false)] out T value) => throw new NotImplementedException();

    public bool TryPeek([MaybeNullWhen(false)] out T value) => throw new NotImplementedException();
}

/// <summary>Your own versions of Select/Where/Aggregate — WITHOUT LINQ, so you practice generics.</summary>
public static class SeqExtensions
{
    public static List<TResult> MyMap<T, TResult>(this IEnumerable<T> source, Func<T, TResult> f) =>
        throw new NotImplementedException();

    public static List<T> MyFilter<T>(this IEnumerable<T> source, Func<T, bool> predicate) =>
        throw new NotImplementedException();

    public static TAcc MyReduce<T, TAcc>(this IEnumerable<T> source, TAcc seed, Func<TAcc, T, TAcc> f) =>
        throw new NotImplementedException();

    /// <summary>
    /// Generic math (.NET 7+): works for int, double, decimal...
    /// Tip: T.Zero and the + operator are available thanks to the INumber&lt;T&gt; constraint.
    /// </summary>
    public static T SumAll<T>(this IEnumerable<T> source) where T : INumber<T> =>
        throw new NotImplementedException();

    /// <summary>
    /// The element with the largest key (the first one on ties). False for empty input.
    /// Tip: CompareTo; try doing it with GetEnumerator/MoveNext so you go through the input only once.
    /// </summary>
    public static bool TryMaxBy<T, TKey>(this IEnumerable<T> source, Func<T, TKey> key, [MaybeNullWhen(false)] out T result)
        where TKey : IComparable<TKey> =>
        throw new NotImplementedException();
}
