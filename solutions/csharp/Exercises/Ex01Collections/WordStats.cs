// Exercise 01: collections — List, Dictionary, LINQ, sorting.
using System.Text.RegularExpressions;

namespace Exercises.Ex01Collections;

public record WordCount(string Word, int Count);

public static class WordStats
{
    /// <summary>Splits text into lowercase words. The separator is anything that is not a letter.</summary>
    public static IReadOnlyList<string> Words(string text) =>
        Regex.Split(text.ToLowerInvariant(), @"\P{L}+")
             .Where(w => w.Length > 0)
             .ToList();

    /// <summary>Counts the occurrences of each word.</summary>
    public static Dictionary<string, int> CountWords(IEnumerable<string> words) =>
        words.GroupBy(w => w)
             .ToDictionary(g => g.Key, g => g.Count());

    /// <summary>At most n most frequent words: by count descending, ties by word (ordinal).</summary>
    public static IReadOnlyList<WordCount> TopN(IReadOnlyDictionary<string, int> counts, int n) =>
        counts.OrderByDescending(kv => kv.Value)
              .ThenBy(kv => kv.Key, StringComparer.Ordinal)
              .Take(n)
              .Select(kv => new WordCount(kv.Key, kv.Value))
              .ToList();

    /// <summary>Groups unique words by length; words in a group are sorted ordinally.</summary>
    public static Dictionary<int, List<string>> GroupByLength(IEnumerable<string> words) =>
        words.Distinct()
             .GroupBy(w => w.Length)
             .ToDictionary(g => g.Key, g => g.Order(StringComparer.Ordinal).ToList());
}
