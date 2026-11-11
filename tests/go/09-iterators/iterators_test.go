package iterators_test

import (
	"slices"
	"testing"

	// Dot import: the tests use the exercise API unqualified, as if they were in the same package.
	// go.mod points it to exercises/go, solutions.mod to solutions/go.
	. "exercises/09-iterators"
)

func TestRange(t *testing.T) {
	if got := slices.Collect(Range(0, 10, 3)); !slices.Equal(got, []int{0, 3, 6, 9}) {
		t.Errorf("Range(0,10,3) = %v", got)
	}
	if got := slices.Collect(Range(5, 5, 1)); len(got) != 0 {
		t.Errorf("empty range = %v", got)
	}
	if got := slices.Collect(Range(0, 10, 0)); len(got) != 0 {
		t.Errorf("step 0 = %v", got)
	}
}

func TestBreakStopsIteration(t *testing.T) {
	// If the iterator ignores yield's return value, the Go runtime panics on break.
	sum := 0
	for v := range Range(0, 1_000_000, 1) {
		if v == 3 {
			break
		}
		sum += v
	}
	if sum != 3 {
		t.Errorf("sum = %d", sum)
	}

	for v := range Fibonacci() {
		if v > 100 {
			break
		}
	}
}

func TestFibonacciTake(t *testing.T) {
	got := slices.Collect(Take(Fibonacci(), 10))
	want := []int{0, 1, 1, 2, 3, 5, 8, 13, 21, 34}
	if !slices.Equal(got, want) {
		t.Errorf("Take(Fibonacci, 10) = %v", got)
	}
	if got := slices.Collect(Take(Fibonacci(), 0)); len(got) != 0 {
		t.Errorf("Take(0) = %v", got)
	}
	if got := slices.Collect(Take(Range(0, 3, 1), 10)); !slices.Equal(got, []int{0, 1, 2}) {
		t.Errorf("Take more than there are elements = %v", got)
	}
}

func TestLaziness(t *testing.T) {
	calls := 0
	isEven := func(n int) bool {
		calls++
		return n%2 == 0
	}
	got := slices.Collect(Take(Filter(Range(1, 1_000_000, 1), isEven), 3))
	if !slices.Equal(got, []int{2, 4, 6}) {
		t.Errorf("result = %v", got)
	}
	if calls != 6 {
		t.Errorf("the predicate should be called 6x (lazy evaluation), was called %dx", calls)
	}
}

func TestWindow(t *testing.T) {
	var got [][]int
	for w := range Window([]int{1, 2, 3, 4}, 2) {
		got = append(got, w)
	}
	want := [][]int{{1, 2}, {2, 3}, {3, 4}}
	if !slices.EqualFunc(got, want, slices.Equal[[]int]) {
		t.Errorf("Window = %v", got)
	}
	got[0][1] = 99
	if got[1][0] != 2 {
		t.Error("windows must be independent copies")
	}
	if n := len(slices.Collect(Window([]int{1, 2}, 3))); n != 0 {
		t.Errorf("window too large: %d windows", n)
	}
}

func TestSortedPairs(t *testing.T) {
	m := map[string]int{"c": 3, "a": 1, "b": 2}
	var keys []string
	sum := 0
	for k, v := range SortedPairs(m) {
		keys = append(keys, k)
		sum += v
	}
	if !slices.Equal(keys, []string{"a", "b", "c"}) || sum != 6 {
		t.Errorf("keys = %v, sum = %d", keys, sum)
	}
}
