// Exercise 08: dependency injection and testability — a TTL cache with an injected clock.
package cache

import "time"

// Clock is an abstraction of time so tests can advance it (see fakeClock in cache_test.go).
type Clock interface {
	Now() time.Time
}

// Cache is a generic cache with entry expiration. It must be safe for concurrent use.
//
// Tip: fields e.g. mu sync.Mutex, ttl, clock, items map[K]entry[V], where entry holds
// the value and expiration time. A struct can be generic too: type entry[V any] struct{...}
type Cache[K comparable, V any] struct {
	// TODO: fields
}

// NewCache creates a cache. If clock is nil, it uses real time (time.Now).
//
// Tip: make a small realClock struct{} type with a Now() method for that.
func NewCache[K comparable, V any](ttl time.Duration, clock Clock) *Cache[K, V] {
	panic("TODO")
}

// Set stores a value; it is valid until now+ttl (at exactly now+ttl it is no longer valid).
func (c *Cache[K, V]) Set(key K, value V) {
	panic("TODO")
}

// Get returns a valid value. An expired entry is deleted right away.
//
// Tip: mu.Lock(); defer mu.Unlock()
func (c *Cache[K, V]) Get(key K) (V, bool) {
	panic("TODO")
}

// Len returns the number of valid (unexpired) entries.
func (c *Cache[K, V]) Len() int {
	panic("TODO")
}

// GetOrLoad returns the value from the cache, or loads it with load and stores it.
// If load fails, nothing is stored and the error is returned.
//
// Tip: don't call load while holding the lock — it may be slow. And watch out: sync.Mutex is not
// reentrant, so if you hold the lock and call c.Get, you get stuck (deadlock).
func (c *Cache[K, V]) GetOrLoad(key K, load func(K) (V, error)) (V, error) {
	panic("TODO")
}
