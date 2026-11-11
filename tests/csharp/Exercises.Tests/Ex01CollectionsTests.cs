using Exercises.Ex01Collections;
using Xunit;

namespace Exercises.Tests.Ex01Collections;

public class WordStatsTests
{
    private const string Text = "Žltý kôň, žltý pes! Kôň beží.";

    [Fact]
    public void Words_SplitsAndLowercases()
    {
        Assert.Equal(new[] { "žltý", "kôň", "žltý", "pes", "kôň", "beží" }, WordStats.Words(Text));
        Assert.Empty(WordStats.Words("  ...  "));
    }

    [Fact]
    public void CountWords_CountsOccurrences()
    {
        var counts = WordStats.CountWords(WordStats.Words(Text));

        Assert.Equal(4, counts.Count);
        Assert.Equal(2, counts["žltý"]);
        Assert.Equal(2, counts["kôň"]);
        Assert.Equal(1, counts["pes"]);
        Assert.Equal(1, counts["beží"]);
    }

    [Fact]
    public void TopN_SortsByCountThenWord()
    {
        var counts = WordStats.CountWords(WordStats.Words(Text));

        var expected = new[] { new WordCount("kôň", 2), new WordCount("žltý", 2), new WordCount("beží", 1) };
        Assert.Equal(expected, WordStats.TopN(counts, 3));
        Assert.Equal(4, WordStats.TopN(counts, 100).Count);
    }

    // Theory + InlineData = the equivalent of table-driven tests in Go.
    [Theory]
    [InlineData(3, new[] { "kôň", "pes" })]
    [InlineData(4, new[] { "beží", "žltý" })]
    public void GroupByLength_GroupsUniqueSortedWords(int length, string[] expected)
    {
        var groups = WordStats.GroupByLength(WordStats.Words(Text));

        Assert.Equal(expected, groups[length]);
        Assert.Equal(2, groups.Count);
    }
}
