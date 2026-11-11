// Exercise 04: generics — type parameters, constraints, a generic type.
package generics

import "cmp"

// Stack is a generic stack. The zero value (var s Stack[int]) is usable.
type Stack[T any] struct {
	items []T
}

func (s *Stack[T]) Push(v T) {
	s.items = append(s.items, v)
}

// Pop returns the top element. If the stack is empty, it returns the zero value and false.
func (s *Stack[T]) Pop() (T, bool) {
	var zero T
	if len(s.items) == 0 {
		return zero, false
	}
	last := len(s.items) - 1
	v := s.items[last]
	s.items[last] = zero // releases the reference for the GC
	s.items = s.items[:last]
	return v, true
}

// Peek returns the top element without removing it.
func (s *Stack[T]) Peek() (T, bool) {
	if len(s.items) == 0 {
		var zero T
		return zero, false
	}
	return s.items[len(s.items)-1], true
}

func (s *Stack[T]) Len() int { return len(s.items) }

// Map applies f to every element.
func Map[T, U any](xs []T, f func(T) U) []U {
	out := make([]U, 0, len(xs))
	for _, x := range xs {
		out = append(out, f(x))
	}
	return out
}

// Filter returns the elements for which pred returns true.
func Filter[T any](xs []T, pred func(T) bool) []T {
	var out []T
	for _, x := range xs {
		if pred(x) {
			out = append(out, x)
		}
	}
	return out
}

// Reduce folds the elements into a single value.
func Reduce[T, A any](xs []T, init A, f func(A, T) A) A {
	acc := init
	for _, x := range xs {
		acc = f(acc, x)
	}
	return acc
}

// Number also allows derived types thanks to ~ (e.g. type Meters float64).
type Number interface {
	~int | ~int64 | ~float64
}

// Sum adds up the numbers.
func Sum[T Number](xs []T) T {
	var total T
	for _, x := range xs {
		total += x
	}
	return total
}

// MaxBy returns the element with the largest key (the first one on ties). For empty input, false.
func MaxBy[T any, K cmp.Ordered](xs []T, key func(T) K) (T, bool) {
	var best T
	if len(xs) == 0 {
		return best, false
	}
	best = xs[0]
	bestKey := key(best)
	for _, x := range xs[1:] {
		if k := key(x); k > bestKey {
			best, bestKey = x, k
		}
	}
	return best, true
}
