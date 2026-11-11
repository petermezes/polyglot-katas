// Exercise 07: functional options — the idiomatic Go replacement for builders and optional parameters.
package options

import (
	"errors"
	"fmt"
	"maps"
	"strings"
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

// WithTimeout sets the timeout; it must be positive.
func WithTimeout(d time.Duration) Option {
	return func(c *Client) error {
		if d <= 0 {
			return fmt.Errorf("timeout %v: %w", d, ErrInvalidOption)
		}
		c.Timeout = d
		return nil
	}
}

// WithRetries sets the number of retries; 0 to 10 allowed.
func WithRetries(n int) Option {
	return func(c *Client) error {
		if n < 0 || n > 10 {
			return fmt.Errorf("retries %d: %w", n, ErrInvalidOption)
		}
		c.Retries = n
		return nil
	}
}

// WithHeader adds a header.
func WithHeader(key, value string) Option {
	return func(c *Client) error {
		c.Headers[key] = value
		return nil
	}
}

// WithUserAgent sets the User-Agent.
func WithUserAgent(ua string) Option {
	return func(c *Client) error {
		c.UserAgent = ua
		return nil
	}
}

// NewClient creates a client with default values (30 s, 3 retries, "exercises/1.0",
// empty headers) and applies the options in order. The first failing option aborts creation.
func NewClient(baseURL string, opts ...Option) (*Client, error) {
	if !strings.HasPrefix(baseURL, "https://") {
		return nil, fmt.Errorf("%q: %w", baseURL, ErrInsecureURL)
	}
	c := &Client{
		BaseURL:   baseURL,
		Timeout:   30 * time.Second,
		Retries:   3,
		UserAgent: "exercises/1.0",
		Headers:   map[string]string{},
	}
	for _, opt := range opts {
		if err := opt(c); err != nil {
			return nil, err
		}
	}
	return c, nil
}

// Clone returns a deep copy. Careful: a plain struct assignment copies only the
// reference to the map, so the copy and the original would share headers.
func (c Client) Clone() Client {
	c.Headers = maps.Clone(c.Headers) // c is already a copy here (value receiver)
	return c
}
