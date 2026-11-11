package collections_test

import (
	"reflect"
	"slices"
	"testing"

	// Dot import: the tests use the exercise API unqualified, as if they were in the same package.
	// go.mod points it to exercises/go, solutions.mod to solutions/go.
	. "exercises/01-collections"
)

const text = "Žltý kôň, žltý pes! Kôň beží."

func TestWords(t *testing.T) {
	got := Words(text)
	want := []string{"žltý", "kôň", "žltý", "pes", "kôň", "beží"}
	if !slices.Equal(got, want) {
		t.Errorf("Words() = %q, want %q", got, want)
	}
	if len(Words("  ...  ")) != 0 {
		t.Errorf("text without letters should return an empty result")
	}
}

func TestCountWords(t *testing.T) {
	got := CountWords(Words(text))
	want := map[string]int{"žltý": 2, "kôň": 2, "pes": 1, "beží": 1}
	if !reflect.DeepEqual(got, want) {
		t.Errorf("CountWords() = %v, want %v", got, want)
	}
}

func TestTopN(t *testing.T) {
	counts := CountWords(Words(text))
	got := TopN(counts, 3)
	want := []WordCount{{Word: "kôň", Count: 2}, {Word: "žltý", Count: 2}, {Word: "beží", Count: 1}}
	if !slices.Equal(got, want) {
		t.Errorf("TopN(3) = %v, want %v", got, want)
	}
	if got := TopN(counts, 100); len(got) != 4 {
		t.Errorf("TopN(100) should return all 4 words, got %d", len(got))
	}
}

// Table-driven test — a typical Go idiom.
func TestGroupByLength(t *testing.T) {
	groups := GroupByLength(Words(text))
	tests := []struct {
		length int
		want   []string
	}{
		{3, []string{"kôň", "pes"}},
		{4, []string{"beží", "žltý"}},
	}
	for _, tt := range tests {
		if got := groups[tt.length]; !slices.Equal(got, tt.want) {
			t.Errorf("length %d: %q, want %q", tt.length, got, tt.want)
		}
	}
	if len(groups) != 2 {
		t.Errorf("expected 2 groups, got %d", len(groups))
	}
}
