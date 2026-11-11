using Exercises.Ex09Iterators;
using Xunit;

namespace Exercises.Tests.Ex09Iterators;

public class SequencesTests
{
    [Fact]
    public void Range_Basics()
    {
        Assert.Equal(new[] { 0, 3, 6, 9 }, Sequences.Range(0, 10, 3));
        Assert.Empty(Sequences.Range(5, 5, 1));
    }

    [Fact]
    public void Range_ValidatesEagerly()
    {
        // No foreach or ToList — the exception must come right at the call.
        Assert.Throws<ArgumentOutOfRangeException>(() => Sequences.Range(0, 10, 0));
    }

    [Fact]
    public void Fibonacci_IsInfiniteButLazy()
    {
        Assert.Equal(new long[] { 0, 1, 1, 2, 3, 5, 8, 13, 21, 34 }, Sequences.Fibonacci().MyTake(10));
        Assert.Empty(Sequences.Fibonacci().MyTake(0));
        Assert.Equal(new[] { 0, 1, 2 }, Sequences.Range(0, 3, 1).MyTake(10));
    }

    [Fact]
    public void Pipeline_IsLazy()
    {
        var calls = 0;
        bool IsEven(int n)
        {
            calls++;
            return n % 2 == 0;
        }

        var query = Sequences.Range(1, 1_000_000, 1).MyWhere(IsEven).MyTake(3);
        Assert.Equal(0, calls); // nothing has run yet

        Assert.Equal(new[] { 2, 4, 6 }, query.ToList());
        Assert.Equal(6, calls);
    }

    [Fact]
    public void Window_ReturnsIndependentCopies()
    {
        int[] data = [1, 2, 3, 4];
        var windows = data.Window(2).ToList();

        Assert.Equal(3, windows.Count);
        Assert.Equal(new[] { 1, 2 }, windows[0]);
        Assert.Equal(new[] { 3, 4 }, windows[2]);

        windows[0][1] = 99;
        Assert.Equal(2, windows[1][0]);

        Assert.Empty(new[] { 1, 2 }.Window(3));
        Assert.Throws<ArgumentOutOfRangeException>(() => data.Window(0));
    }

    [Fact]
    public void SortedPairs_OrdersByKey()
    {
        var dict = new Dictionary<string, int> { ["c"] = 3, ["a"] = 1, ["b"] = 2 };

        var pairs = dict.SortedPairs().ToList();

        Assert.Equal(new[] { "a", "b", "c" }, pairs.Select(p => p.Key));
        Assert.Equal(6, pairs.Sum(p => p.Value));
    }
}
