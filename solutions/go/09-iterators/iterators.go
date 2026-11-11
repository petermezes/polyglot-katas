// Exercise 09: iterators — range-over-func (Go 1.23+), lazy sequences, stopping correctly via yield.
package iterators

import (
	"cmp"
	"iter"
	"maps"
	"slices"
)

// Range generates start, start+step, ... while the value is < end. Nothing for step <= 0.
func Range(start, end, step int) iter.Seq[int] {
	return func(yield func(int) bool) {
		if step <= 0 {
			return
		}
		for i := start; i < end; i += step {
			if !yield(i) { // the caller did break -> we must stop
				return
			}
		}
	}
}

// Fibonacci is the infinite sequence 0, 1, 1, 2, 3, 5, ...
func Fibonacci() iter.Seq[int] {
	return func(yield func(int) bool) {
		a, b := 0, 1
		for {
			if !yield(a) {
				return
			}
			a, b = b, a+b
		}
	}
}

// Take returns at most the first n elements of the sequence. It must not read more than needed.
func Take[T any](seq iter.Seq[T], n int) iter.Seq[T] {
	return func(yield func(T) bool) {
		if n <= 0 {
			return
		}
		i := 0
		for v := range seq {
			if !yield(v) {
				return
			}
			i++
			if i >= n {
				return
			}
		}
	}
}

// Filter lazily lets through only the elements for which pred returns true.
func Filter[T any](seq iter.Seq[T], pred func(T) bool) iter.Seq[T] {
	return func(yield func(T) bool) {
		for v := range seq {
			if pred(v) && !yield(v) {
				return
			}
		}
	}
}

// Window returns sliding windows of length size: [1 2 3 4], 2 -> [1 2] [2 3] [3 4].
// Each window is a separate copy. Nothing for size <= 0 or size > len(xs).
func Window[T any](xs []T, size int) iter.Seq[[]T] {
	return func(yield func([]T) bool) {
		if size <= 0 {
			return
		}
		for i := 0; i+size <= len(xs); i++ {
			if !yield(slices.Clone(xs[i : i+size])) {
				return
			}
		}
	}
}

// SortedPairs walks a map in key order (a plain range over a map has random order).
func SortedPairs[K cmp.Ordered, V any](m map[K]V) iter.Seq2[K, V] {
	return func(yield func(K, V) bool) {
		for _, k := range slices.Sorted(maps.Keys(m)) {
			if !yield(k, m[k]) {
				return
			}
		}
	}
}
