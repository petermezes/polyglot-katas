// Exercise 01: collections — slices, maps, sorting.
package collections

import (
	"cmp"
	"slices"
	"strings"
	"unicode"
	"unicode/utf8"
)

// WordCount is a word and the number of its occurrences.
type WordCount struct {
	Word  string
	Count int
}

// Words splits text into lowercase words. The separator is anything that is not a letter.
func Words(text string) []string {
	return strings.FieldsFunc(strings.ToLower(text), func(r rune) bool {
		return !unicode.IsLetter(r)
	})
}

// CountWords counts the occurrences of each word.
func CountWords(words []string) map[string]int {
	counts := make(map[string]int, len(words))
	for _, w := range words {
		counts[w]++ // the zero value of int is 0, no need to check whether the key exists
	}
	return counts
}

// TopN returns at most n most frequent words: by count descending, ties by word ascending.
func TopN(counts map[string]int, n int) []WordCount {
	result := make([]WordCount, 0, len(counts))
	for w, c := range counts {
		result = append(result, WordCount{Word: w, Count: c})
	}
	slices.SortFunc(result, func(a, b WordCount) int {
		if c := cmp.Compare(b.Count, a.Count); c != 0 {
			return c
		}
		return strings.Compare(a.Word, b.Word)
	})
	if n < len(result) {
		result = result[:n]
	}
	return result
}

// GroupByLength groups unique words by number of characters (not bytes!). Words in a group are sorted.
func GroupByLength(words []string) map[int][]string {
	groups := make(map[int][]string)
	for _, w := range words {
		l := utf8.RuneCountInString(w)
		groups[l] = append(groups[l], w)
	}
	for l, g := range groups {
		slices.Sort(g)
		groups[l] = slices.Compact(g)
	}
	return groups
}
