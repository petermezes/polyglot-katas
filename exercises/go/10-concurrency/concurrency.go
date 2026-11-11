// Exercise 10: concurrency — goroutines, channels, WaitGroup, context and cancellation.
package concurrency

import (
	"context"
	"errors"
)

var ErrNoFunctions = errors.New("no functions to run")

// ProcessAll processes all inputs with f, at most `workers` at a time (treat workers < 1 as 1).
// Results are in input order. On the first error it cancels the context for the other tasks,
// starts no new ones and returns that first error. If the caller cancels the context, it returns ctx.Err().
//
// Tip — a classic worker pool:
//   - ctx, cancel := context.WithCancel(ctx); defer cancel()
//   - a jobs channel with indices; `workers` goroutines read `for i := range jobs`
//   - write results to results[i] — each index is written by only one goroutine, no lock needed
//   - store the first error via sync.Once and call cancel()
//   - feeding jobs: select { case jobs <- i: case <-ctx.Done(): }
//     (select picks randomly when two cases are ready — also check ctx.Err() explicitly)
//   - close(jobs), wg.Wait()
func ProcessAll[T, R any](ctx context.Context, inputs []T, workers int,
	f func(context.Context, T) (R, error)) ([]R, error) {
	panic("TODO")
}

// FirstSuccess starts all functions at once and returns the first successful result;
// it cancels the others via the context. If all fail, it returns errors.Join of all errors.
// With no functions, it returns ErrNoFunctions.
//
// Tip: collect results through a channel with buffer len(fns) — otherwise goroutines whose result
// nobody reads anymore will hang forever (goroutine leak).
func FirstSuccess[R any](ctx context.Context, fns ...func(context.Context) (R, error)) (R, error) {
	panic("TODO")
}
