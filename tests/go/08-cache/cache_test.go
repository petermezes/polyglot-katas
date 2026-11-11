package cache_test

import (
	"errors"
	"fmt"
	"sync"
	"testing"
	"time"

	// Dot import: the tests use the exercise API unqualified, as if they were in the same package.
	// go.mod points it to exercises/go, solutions.mod to solutions/go.
	. "exercises/08-cache"
)

// fakeClock is a test implementation of Clock — time is advanced manually.
type fakeClock struct {
	mu  sync.Mutex
	now time.Time
}

func (f *fakeClock) Now() time.Time {
	f.mu.Lock()
	defer f.mu.Unlock()
	return f.now
}

func (f *fakeClock) Advance(d time.Duration) {
	f.mu.Lock()
	defer f.mu.Unlock()
	f.now = f.now.Add(d)
}

func newFake() *fakeClock {
	return &fakeClock{now: time.Date(2026, 11, 3, 10, 0, 0, 0, time.UTC)}
}

func TestSetGetExpire(t *testing.T) {
	clock := newFake()
	c := NewCache[string, int](time.Minute, clock)

	c.Set("a", 1)
	if v, ok := c.Get("a"); !ok || v != 1 {
		t.Fatalf("Get(a) = %d, %v", v, ok)
	}
	if _, ok := c.Get("b"); ok {
		t.Error("Get(b) should return false")
	}

	clock.Advance(59 * time.Second)
	if _, ok := c.Get("a"); !ok {
		t.Error("after 59 s the entry should still be valid")
	}

	clock.Advance(time.Second)
	if _, ok := c.Get("a"); ok {
		t.Error("exactly at TTL the entry is no longer valid")
	}
}

func TestSetRefreshesTTL(t *testing.T) {
	clock := newFake()
	c := NewCache[string, string](time.Minute, clock)
	c.Set("k", "v1")
	clock.Advance(50 * time.Second)
	c.Set("k", "v2")
	clock.Advance(50 * time.Second)
	if v, ok := c.Get("k"); !ok || v != "v2" {
		t.Errorf("Get = %q, %v (Set should refresh the TTL)", v, ok)
	}
}

func TestLen(t *testing.T) {
	clock := newFake()
	c := NewCache[int, string](10*time.Second, clock)
	c.Set(1, "x")
	clock.Advance(5 * time.Second)
	c.Set(2, "y")
	if c.Len() != 2 {
		t.Errorf("Len = %d", c.Len())
	}
	clock.Advance(5 * time.Second)
	if c.Len() != 1 {
		t.Errorf("Len after expiration = %d", c.Len())
	}
}

func TestGetOrLoad(t *testing.T) {
	clock := newFake()
	c := NewCache[string, string](time.Minute, clock)
	calls := 0
	load := func(k string) (string, error) {
		calls++
		return "value-" + k, nil
	}

	for range 3 {
		v, err := c.GetOrLoad("x", load)
		if err != nil || v != "value-x" {
			t.Fatalf("GetOrLoad = %q, %v", v, err)
		}
	}
	if calls != 1 {
		t.Errorf("load should be called once, was called %dx", calls)
	}

	clock.Advance(time.Minute)
	_, _ = c.GetOrLoad("x", load)
	if calls != 2 {
		t.Errorf("after expiration load should be called again (calls=%d)", calls)
	}

	boom := errors.New("database unavailable")
	if _, err := c.GetOrLoad("y", func(string) (string, error) { return "", boom }); !errors.Is(err, boom) {
		t.Errorf("the error from load should be returned, got %v", err)
	}
	if _, ok := c.Get("y"); ok {
		t.Error("nothing should be stored on error")
	}
}

func TestNilClockUsesRealTime(t *testing.T) {
	c := NewCache[string, int](time.Hour, nil)
	c.Set("a", 1)
	if _, ok := c.Get("a"); !ok {
		t.Error("with the real clock the entry should be valid")
	}
}

func TestConcurrentAccess(t *testing.T) {
	c := NewCache[string, int](time.Minute, newFake())
	var wg sync.WaitGroup
	for i := range 50 {
		wg.Add(1)
		go func() {
			defer wg.Done()
			key := fmt.Sprint(i % 5)
			c.Set(key, i)
			c.Get(key)
			c.Len()
		}()
	}
	wg.Wait()
	if c.Len() != 5 {
		t.Errorf("Len = %d", c.Len())
	}
}
