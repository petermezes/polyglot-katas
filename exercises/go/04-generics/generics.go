// Exercise 04: generics — type parameters, constraints, a generic type.
package generics

import "cmp"

// Stack is a generic stack. The zero value (var s Stack[int]) must be usable.
type Stack[T any] struct {
	// TODO: design the fields (one slice is enough)
}

func (s *Stack[T]) Push(v T) {
	panic("TODO")
}

// Pop returns the top element. If the stack is empty, it returns the zero value and false.
//
// Tip: the zero value of any T is `var zero T`.
func (s *Stack[T]) Pop() (T, bool) {
	panic("TODO")
}

// Peek returns the top element without removing it.
func (s *Stack[T]) Peek() (T, bool) {
	panic("TODO")
}

func (s *Stack[T]) Len() int {
	panic("TODO")
}

// Map applies f to every element.
func Map[T, U any](xs []T, f func(T) U) []U {
	panic("TODO")
}

// Filter returns the elements for which pred returns true.
func Filter[T any](xs []T, pred func(T) bool) []T {
	panic("TODO")
}

// Reduce folds the elements into a single value, starting from init.
func Reduce[T, A any](xs []T, init A, f func(A, T) A) A {
	panic("TODO")
}

// Number also allows derived types thanks to ~ (e.g. type Meters float64).
// Try deleting ~ and see which test stops compiling.
type Number interface {
	~int | ~int64 | ~float64
}

// Sum adds up the numbers.
func Sum[T Number](xs []T) T {
	panic("TODO")
}

// MaxBy returns the element with the largest key (the first one on ties). For empty input, false.
func MaxBy[T any, K cmp.Ordered](xs []T, key func(T) K) (T, bool) {
	panic("TODO")
}
