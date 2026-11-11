package generics_test

import (
	"slices"
	"strconv"
	"strings"
	"testing"

	// Dot import: the tests use the exercise API unqualified, as if they were in the same package.
	// go.mod points it to exercises/go, solutions.mod to solutions/go.
	. "exercises/04-generics"
)

func TestStack(t *testing.T) {
	var s Stack[string] // the zero value must work
	if _, ok := s.Pop(); ok {
		t.Fatal("Pop on an empty stack should return false")
	}
	s.Push("a")
	s.Push("b")
	s.Push("c")
	if v, ok := s.Peek(); !ok || v != "c" {
		t.Errorf("Peek() = %q, %v", v, ok)
	}
	if s.Len() != 3 {
		t.Errorf("Len() = %d", s.Len())
	}
	var popped []string
	for s.Len() > 0 {
		v, _ := s.Pop()
		popped = append(popped, v)
	}
	if !slices.Equal(popped, []string{"c", "b", "a"}) {
		t.Errorf("Pop order: %q", popped)
	}
	if _, ok := s.Peek(); ok {
		t.Error("Peek on an empty stack should return false")
	}
}

func TestMapFilterReduce(t *testing.T) {
	nums := []int{1, 2, 3, 4, 5, 6}

	strs := Map(nums, strconv.Itoa)
	if !slices.Equal(strs, []string{"1", "2", "3", "4", "5", "6"}) {
		t.Errorf("Map = %q", strs)
	}

	even := Filter(nums, func(n int) bool { return n%2 == 0 })
	if !slices.Equal(even, []int{2, 4, 6}) {
		t.Errorf("Filter = %v", even)
	}

	joined := Reduce(strs, "", func(acc string, s string) string { return acc + s })
	if joined != "123456" {
		t.Errorf("Reduce = %q", joined)
	}

	if got := Map([]int{}, strconv.Itoa); len(got) != 0 {
		t.Errorf("Map of an empty input = %v", got)
	}
}

type Meters float64

func TestSum(t *testing.T) {
	if got := Sum([]int{1, 2, 3}); got != 6 {
		t.Errorf("Sum(int) = %d", got)
	}
	if got := Sum([]Meters{1.5, 2.5}); got != 4 {
		t.Errorf("Sum(Meters) = %v", got)
	}
	if got := Sum[float64](nil); got != 0 {
		t.Errorf("Sum(nil) = %v", got)
	}
}

func TestMaxBy(t *testing.T) {
	words := []string{"pes", "mačka", "kôň", "slimák"}
	got, ok := MaxBy(words, func(s string) int { return len([]rune(s)) })
	if !ok || got != "slimák" {
		t.Errorf("MaxBy(length) = %q, %v", got, ok)
	}
	got, _ = MaxBy(words, strings.ToUpper)
	if got != "slimák" {
		t.Errorf("MaxBy(alphabetical) = %q", got)
	}
	tie, _ := MaxBy([]string{"ab", "cd"}, func(s string) int { return len(s) })
	if tie != "ab" {
		t.Errorf("on a tie the first one should win, got %q", tie)
	}
	if _, ok := MaxBy([]int{}, func(i int) int { return i }); ok {
		t.Error("MaxBy of an empty input should return false")
	}
}
