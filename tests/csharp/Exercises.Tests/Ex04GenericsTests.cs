using System.Globalization;
using Exercises.Ex04Generics;
using Xunit;

namespace Exercises.Tests.Ex04Generics;

public class GenericsTests
{
    [Fact]
    public void SimpleStack_PushPopPeek()
    {
        var s = new SimpleStack<string>();
        Assert.False(s.TryPop(out _));

        s.Push("a");
        s.Push("b");
        s.Push("c");

        Assert.True(s.TryPeek(out var top));
        Assert.Equal("c", top);
        Assert.Equal(3, s.Count);

        var popped = new List<string>();
        while (s.TryPop(out var v))
            popped.Add(v);

        Assert.Equal(new[] { "c", "b", "a" }, popped);
        Assert.False(s.TryPeek(out _));
    }

    [Fact]
    public void MapFilterReduce()
    {
        int[] nums = [1, 2, 3, 4, 5, 6];

        var strs = nums.MyMap(n => n.ToString(CultureInfo.InvariantCulture));
        Assert.Equal(new[] { "1", "2", "3", "4", "5", "6" }, strs);

        Assert.Equal(new[] { 2, 4, 6 }, nums.MyFilter(n => n % 2 == 0));

        Assert.Equal("123456", strs.MyReduce("", (acc, s) => acc + s));

        Assert.Empty(Array.Empty<int>().MyMap(n => n * 2));
    }

    [Fact]
    public void SumAll_WorksForAnyNumber()
    {
        Assert.Equal(6, new[] { 1, 2, 3 }.SumAll());
        Assert.Equal(4.0, new[] { 1.5, 2.5 }.SumAll());
        Assert.Equal(0.3m, new[] { 0.1m, 0.2m }.SumAll());
        Assert.Equal(0, Array.Empty<int>().SumAll());
    }

    [Fact]
    public void TryMaxBy()
    {
        string[] words = ["pes", "mačka", "kôň", "slimák"];

        Assert.True(words.TryMaxBy(w => w.Length, out var longest));
        Assert.Equal("slimák", longest);

        Assert.True(new[] { "ab", "cd" }.TryMaxBy(w => w.Length, out var tie));
        Assert.Equal("ab", tie);

        Assert.False(Array.Empty<int>().TryMaxBy(i => i, out _));
    }
}
