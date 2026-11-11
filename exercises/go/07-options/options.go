// Exercise 07: functional options — the idiomatic Go replacement for builders and optional parameters.
package options

import (
	"errors"
	"time"
)

var (
	ErrInsecureURL   = errors.New("URL must start with https://")
	ErrInvalidOption = errors.New("invalid option")
)

// Client is an HTTP client configuration.
type Client struct {
	BaseURL   string
	Timeout   time.Duration
	Retries   int
	UserAgent string
	Headers   map[string]string
}

// Option modifies the client; it may return an error if the value is invalid.
type Option func(*Client) error

// WithTimeout sets the timeout; it must be positive, otherwise an error wrapping ErrInvalidOption.
//
// Tip: the function returns a closure: return func(c *Client) error { ... }
func WithTimeout(d time.Duration) Option {
	panic("TODO")
}

// WithRetries sets the number of retries; 0 to 10 allowed, otherwise an error wrapping ErrInvalidOption.
func WithRetries(n int) Option {
	panic("TODO")
}

// WithHeader adds a header.
func WithHeader(key, value string) Option {
	panic("TODO")
}

// WithUserAgent sets the User-Agent.
func WithUserAgent(ua string) Option {
	panic("TODO")
}

// NewClient checks that baseURL starts with "https://" (otherwise an error wrapping ErrInsecureURL),
// creates a client with default values (30 s, 3 retries, "exercises/1.0", an empty
// but non-nil header map) and applies the options in order. The first failing option aborts creation.
func NewClient(baseURL string, opts ...Option) (*Client, error) {
	panic("TODO")
}

// Clone returns a deep copy. A plain struct assignment copies only the reference
// to the map, so the copy and the original would share headers.
//
// Tip: a value receiver (c Client) is already a copy; just clone the map with maps.Clone.
func (c Client) Clone() Client {
	panic("TODO")
}
