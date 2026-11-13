// Exercise 01: collections — slices, maps, sorting.
package collections

// WordCount is a word and the number of its occurrences.
type WordCount struct {
	Word  string
	Count int
}

// Words splits text into lowercase words. The separator is anything that is not a letter.
//
// Tip: strings.ToLower, strings.FieldsFunc, unicode.IsLetter.
func Words(text string) []string {
	panic("TODO")
}

// CountWords counts the occurrences of each word.
//
// Tip: the zero value of int in a map is 0, so counts[w]++ works for a new key too.
func CountWords(words []string) map[string]int {
	panic("TODO")
}

// TopN returns at most n most frequent words: by count descending, ties by word ascending.
//
// Tip: map iteration order is random. Copy it into a slice and sort with slices.SortFunc
// and cmp.Compare. Watch out for n larger than the number of words.
func TopN(counts map[string]int, n int) []WordCount {
	panic("TODO")
}

// GroupByLength groups unique words by number of characters (not bytes!). Words in a group are sorted.
//
// Tip: len("kôň") is 5 (bytes in UTF-8). utf8.RuneCountInString counts characters.
// To remove duplicates from a sorted slice, use slices.Compact.
func GroupByLength(words []string) map[int][]string {
	panic("TODO")
}
