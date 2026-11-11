// Exercise 10: concurrency — goroutines, channels, WaitGroup, context and cancellation.
package concurrency

import (
	"context"
	"errors"
	"sync"
)

var ErrNoFunctions = errors.New("no functions to run")

// ProcessAll processes all inputs with f, at most `workers` at a time.
// Results are in input order. On the first error it cancels the context for the other tasks,
// starts no new ones and returns that first error.
func ProcessAll[T, R any](ctx context.Context, inputs []T, workers int,
	f func(context.Context, T) (R, error)) ([]R, error) {

	if workers < 1 {
		workers = 1
	}
	ctx, cancel := context.WithCancel(ctx)
	defer cancel()

	results := make([]R, len(inputs))
	jobs := make(chan int)

	var (
		wg       sync.WaitGroup
		once     sync.Once
		firstErr error
	)

	for range workers {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for i := range jobs {
				if ctx.Err() != nil {
					continue // just drain the channel, don't start new jobs
				}
				r, err := f(ctx, inputs[i])
				if err != nil {
					once.Do(func() {
						firstErr = err
						cancel()
					})
					continue
				}
				results[i] = r // each index is written by exactly one goroutine -> no lock
			}
		}()
	}

feed:
	for i := range inputs {
		// select picks randomly when several cases are ready,
		// so we also check for cancellation explicitly.
		if ctx.Err() != nil {
			break
		}
		select {
		case jobs <- i:
		case <-ctx.Done():
			break feed
		}
	}
	close(jobs)
	wg.Wait()

	if firstErr != nil {
		return nil, firstErr
	}
	if err := ctx.Err(); err != nil { // cancelled from outside
		return nil, err
	}
	return results, nil
}

// FirstSuccess starts all functions at once and returns the first successful result;
// it cancels the others via the context. If all fail, it returns errors.Join of all errors.
func FirstSuccess[R any](ctx context.Context, fns ...func(context.Context) (R, error)) (R, error) {
	var zero R
	if len(fns) == 0 {
		return zero, ErrNoFunctions
	}
	ctx, cancel := context.WithCancel(ctx)
	defer cancel()

	type result struct {
		val R
		err error
	}
	// Buffer = number of functions, so no goroutine is left blocked on send (goroutine leak).
	ch := make(chan result, len(fns))
	for _, fn := range fns {
		go func() {
			v, err := fn(ctx)
			ch <- result{v, err}
		}()
	}

	var errs []error
	for range fns {
		r := <-ch
		if r.err == nil {
			return r.val, nil // defer cancel() cancels the rest
		}
		errs = append(errs, r.err)
	}
	return zero, errors.Join(errs...)
}
