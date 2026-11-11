package concurrency_test

import (
	"context"
	"errors"
	"slices"
	"sync/atomic"
	"testing"
	"time"

	// Dot import: the tests use the exercise API unqualified, as if they were in the same package.
	// go.mod points it to exercises/go, solutions.mod to solutions/go.
	. "exercises/10-concurrency"
)

func TestProcessAllOrderAndLimit(t *testing.T) {
	var active, maxActive atomic.Int32
	double := func(ctx context.Context, n int) (int, error) {
		cur := active.Add(1)
		for {
			m := maxActive.Load()
			if cur <= m || maxActive.CompareAndSwap(m, cur) {
				break
			}
		}
		time.Sleep(time.Duration(10-n) * time.Millisecond) // later inputs finish sooner
		active.Add(-1)
		return n * 2, nil
	}

	inputs := []int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9}
	got, err := ProcessAll(context.Background(), inputs, 3, double)
	if err != nil {
		t.Fatal(err)
	}
	if !slices.Equal(got, []int{0, 2, 4, 6, 8, 10, 12, 14, 16, 18}) {
		t.Errorf("results must be in input order: %v", got)
	}
	if m := maxActive.Load(); m > 3 || m < 2 {
		t.Errorf("max running at once = %d, expected 2–3", m)
	}
}

func TestProcessAllStopsOnError(t *testing.T) {
	boom := errors.New("sensor not responding")
	var started atomic.Int32
	f := func(ctx context.Context, n int) (int, error) {
		started.Add(1)
		if n == 2 {
			return 0, boom
		}
		select {
		case <-time.After(50 * time.Millisecond):
			return n, nil
		case <-ctx.Done():
			return 0, ctx.Err()
		}
	}

	inputs := make([]int, 100)
	for i := range inputs {
		inputs[i] = i
	}
	start := time.Now()
	_, err := ProcessAll(context.Background(), inputs, 4, f)
	if !errors.Is(err, boom) {
		t.Fatalf("expected the original error, got %v", err)
	}
	if n := started.Load(); n > 10 {
		t.Errorf("no further tasks should start after an error, started: %d", n)
	}
	if d := time.Since(start); d > 500*time.Millisecond {
		t.Errorf("took %v — cancellation doesn't work", d)
	}
}

func TestProcessAllParentCancel(t *testing.T) {
	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Millisecond)
	defer cancel()
	slow := func(ctx context.Context, n int) (int, error) {
		select {
		case <-time.After(time.Second):
			return n, nil
		case <-ctx.Done():
			return 0, ctx.Err()
		}
	}
	_, err := ProcessAll(ctx, []int{1, 2, 3, 4, 5}, 2, slow)
	if !errors.Is(err, context.DeadlineExceeded) {
		t.Errorf("expected DeadlineExceeded, got %v", err)
	}
}

func TestFirstSuccess(t *testing.T) {
	var cancelled atomic.Int32
	slow := func(ctx context.Context) (string, error) {
		select {
		case <-time.After(2 * time.Second):
			return "slow", nil
		case <-ctx.Done():
			cancelled.Add(1)
			return "", ctx.Err()
		}
	}
	fast := func(ctx context.Context) (string, error) {
		time.Sleep(10 * time.Millisecond)
		return "fast", nil
	}
	failing := func(ctx context.Context) (string, error) {
		return "", errors.New("fails immediately")
	}

	got, err := FirstSuccess(context.Background(), slow, failing, fast, slow)
	if err != nil || got != "fast" {
		t.Fatalf("FirstSuccess = %q, %v", got, err)
	}

	deadline := time.Now().Add(time.Second)
	for cancelled.Load() < 2 && time.Now().Before(deadline) {
		time.Sleep(5 * time.Millisecond)
	}
	if cancelled.Load() != 2 {
		t.Errorf("slow functions should be cancelled, cancelled: %d", cancelled.Load())
	}
}

func TestFirstSuccessAllFail(t *testing.T) {
	e1, e2 := errors.New("first"), errors.New("second")
	_, err := FirstSuccess(context.Background(),
		func(context.Context) (int, error) { return 0, e1 },
		func(context.Context) (int, error) { return 0, e2 },
	)
	if !errors.Is(err, e1) || !errors.Is(err, e2) {
		t.Errorf("the error should contain both original errors: %v", err)
	}

	if _, err := FirstSuccess[int](context.Background()); !errors.Is(err, ErrNoFunctions) {
		t.Errorf("no functions: %v", err)
	}
}
