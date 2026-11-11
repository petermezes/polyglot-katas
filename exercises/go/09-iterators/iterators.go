// Exercise 09: iterators — range-over-func (Go 1.23+), lazy sequences, stopping correctly via yield.
//
// iter.Seq[T] is just func(yield func(T) bool). Your function calls yield for each element;
// when yield returns false (the caller did break), you MUST stop immediately, or the Go runtime panics.
package iterators

import (
	"cmp"
	"iter"
)

// Range generates start, start+step, ... while the value is < end. Nothing for step <= 0.
func Range(start, end, step int) iter.Seq[int] {
	panic("TODO")
}

// Fibonacci is the infinite sequence 0, 1, 1, 2, 3, 5, ...
func Fibonacci() iter.Seq[int] {
	panic("TODO")
}

// Take returns at most the first n elements of the sequence. It must not read more than needed
// (stop right after the n-th element, don't wait for the next one).
func Take[T any](seq iter.Seq[T], n int) iter.Seq[T] {
	panic("TODO")
}

// Filter lazily lets through only the elements for which pred returns true.
func Filter[T any](seq iter.Seq[T], pred func(T) bool) iter.Seq[T] {
	panic("TODO")
}

// Window returns sliding windows of length size: [1 2 3 4], 2 -> [1 2] [2 3] [3 4].
// Each window is a separate copy. Nothing for size <= 0 or size > len(xs).
//
// Tip: xs[i:i+size] is a view into the same array — without slices.Clone, changing
// one window would change its neighbors too.
func Window[T any](xs []T, size int) iter.Seq[[]T] {
	panic("TODO")
}

// SortedPairs walks a map in key order (a plain range over a map has random order).
//
// Tip: slices.Sorted(maps.Keys(m)). iter.Seq2 is func(yield func(K, V) bool).
func SortedPairs[K cmp.Ordered, V any](m map[K]V) iter.Seq2[K, V] {
	panic("TODO")
}
