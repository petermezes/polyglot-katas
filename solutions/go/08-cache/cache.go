// Exercise 08: dependency injection and testability — a TTL cache with an injected clock.
package cache

import (
	"sync"
	"time"
)

// Clock is an abstraction of time so tests can advance it.
type Clock interface {
	Now() time.Time
}

type realClock struct{}

func (realClock) Now() time.Time { return time.Now() }

type entry[V any] struct {
	value     V
	expiresAt time.Time
}

// Cache is a generic cache with entry expiration. Safe for concurrent use.
type Cache[K comparable, V any] struct {
	mu    sync.Mutex
	ttl   time.Duration
	clock Clock
	items map[K]entry[V]
}

// NewCache creates a cache. If clock is nil, it uses real time.
func NewCache[K comparable, V any](ttl time.Duration, clock Clock) *Cache[K, V] {
	if clock == nil {
		clock = realClock{}
	}
	return &Cache[K, V]{ttl: ttl, clock: clock, items: make(map[K]entry[V])}
}

// Set stores a value; it is valid until now+ttl (at now+ttl it is no longer valid).
func (c *Cache[K, V]) Set(key K, value V) {
	c.mu.Lock()
	defer c.mu.Unlock()
	c.items[key] = entry[V]{value: value, expiresAt: c.clock.Now().Add(c.ttl)}
}

// Get returns a valid value. An expired entry is deleted right away.
func (c *Cache[K, V]) Get(key K) (V, bool) {
	c.mu.Lock()
	defer c.mu.Unlock()
	return c.getLocked(key)
}

// getLocked assumes the caller holds the lock.
func (c *Cache[K, V]) getLocked(key K) (V, bool) {
	e, ok := c.items[key]
	if !ok {
		var zero V
		return zero, false
	}
	if !c.clock.Now().Before(e.expiresAt) {
		delete(c.items, key)
		var zero V
		return zero, false
	}
	return e.value, true
}

// Len returns the number of valid entries.
func (c *Cache[K, V]) Len() int {
	c.mu.Lock()
	defer c.mu.Unlock()
	now := c.clock.Now()
	n := 0
	for _, e := range c.items {
		if now.Before(e.expiresAt) {
			n++
		}
	}
	return n
}

// GetOrLoad returns the value from the cache, or loads it with load and stores it.
// If load fails, nothing is stored and the error is returned.
func (c *Cache[K, V]) GetOrLoad(key K, load func(K) (V, error)) (V, error) {
	if v, ok := c.Get(key); ok {
		return v, nil
	}
	// we call load without the lock — it may be slow and we don't want to block others.
	v, err := load(key)
	if err != nil {
		var zero V
		return zero, err
	}
	c.Set(key, v)
	return v, nil
}
