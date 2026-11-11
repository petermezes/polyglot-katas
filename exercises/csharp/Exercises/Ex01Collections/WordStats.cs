// Exercise 01: collections — List, Dictionary, LINQ, sorting.
using System.Net.Security;
using System.Text.RegularExpressions;

namespace Exercises.Ex01Collections;

public record WordCount(string Word, int Count);

public static class WordStats
{
    /// <summary>
    /// Splits text into lowercase words. The separator is anything that is not a letter.
    /// Tip: ToLowerInvariant, Regex.Split with @"\P{L}+" (\P{L} = "not a letter"), Where.
    /// </summary>
    public static IReadOnlyList<string> Words(string text)
    {
      return Regex.Matches(text.ToLowerInvariant(), @"\p{L}+")
        .Select(m => m.Value)
        .ToList();
    }


    /// <summary>
    /// Counts the occurrences of each word.
    /// Tip: GroupBy + ToDictionary.
    /// </summary>
    public static Dictionary<string, int> CountWords(IEnumerable<string> words)
    {
        return words.GroupBy(w => w).ToDictionary(g => g.Key, g => g.Count());
    }
    /// <summary>
    /// At most n most frequent words: by count descending, ties by word (ordinal).
    /// Tip: OrderByDescending, ThenBy(..., StringComparer.Ordinal), Take, Select.
    /// </summary>
    public static IReadOnlyList<WordCount> TopN(IReadOnlyDictionary<string, int> counts, int n) =>
        counts.OrderByDescending(kv => kv.Value)
        .ThenBy(kv => kv.Key, StringComparer.Ordinal)
        .Take(n)
        .Select(kv => new WordCount(kv.Key, kv.Value))
        .ToList();

    /// <summary>
    /// Groups unique words by length; words in a group are sorted ordinally.
    /// Tip: Distinct, GroupBy, ToDictionary, Order(StringComparer.Ordinal).
    /// </summary>
    public static Dictionary<int, List<string>> GroupByLength(IEnumerable<string> words) =>
        throw new NotImplementedException();
}
