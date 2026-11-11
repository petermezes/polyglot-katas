package options_test

import (
	"errors"
	"testing"
	"time"

	// Dot import: the tests use the exercise API unqualified, as if they were in the same package.
	// go.mod points it to exercises/go, solutions.mod to solutions/go.
	. "exercises/07-options"
)

func TestDefaults(t *testing.T) {
	c, err := NewClient("https://api.example.no")
	if err != nil {
		t.Fatal(err)
	}
	if c.Timeout != 30*time.Second || c.Retries != 3 || c.UserAgent != "exercises/1.0" {
		t.Errorf("wrong defaults: %+v", c)
	}
	if c.Headers == nil {
		t.Error("Headers must not be nil (writing to a nil map panics)")
	}
}

func TestOptions(t *testing.T) {
	c, err := NewClient("https://api.example.no",
		WithTimeout(5*time.Second),
		WithRetries(0),
		WithHeader("Accept", "application/json"),
		WithHeader("X-Tunnel", "Løvstakk"),
		WithUserAgent("vegvokter-test"),
	)
	if err != nil {
		t.Fatal(err)
	}
	if c.Timeout != 5*time.Second || c.Retries != 0 || c.UserAgent != "vegvokter-test" {
		t.Errorf("options were not applied: %+v", c)
	}
	if len(c.Headers) != 2 || c.Headers["X-Tunnel"] != "Løvstakk" {
		t.Errorf("Headers = %v", c.Headers)
	}
}

func TestInvalid(t *testing.T) {
	if _, err := NewClient("http://api.example.no"); !errors.Is(err, ErrInsecureURL) {
		t.Errorf("http:// should fail with ErrInsecureURL, got %v", err)
	}
	if _, err := NewClient("https://x", WithTimeout(0)); !errors.Is(err, ErrInvalidOption) {
		t.Errorf("timeout 0: %v", err)
	}
	if _, err := NewClient("https://x", WithRetries(11)); !errors.Is(err, ErrInvalidOption) {
		t.Errorf("retries 11: %v", err)
	}
	if _, err := NewClient("https://x", WithRetries(-1)); !errors.Is(err, ErrInvalidOption) {
		t.Errorf("retries -1: %v", err)
	}
}

// A custom option defined outside the package — the pattern's advantage: extensible without changing NewClient.
func withDoubleRetries() Option {
	return func(c *Client) error {
		c.Retries *= 2
		return nil
	}
}

func TestCustomOptionAndOrder(t *testing.T) {
	c, err := NewClient("https://x", WithRetries(2), withDoubleRetries())
	if err != nil || c.Retries != 4 {
		t.Errorf("Retries = %d, %v (options should be applied in order)", c.Retries, err)
	}
}

func TestClone(t *testing.T) {
	orig, _ := NewClient("https://x", WithHeader("A", "1"))
	clone := orig.Clone()
	clone.Headers["B"] = "2"
	clone.Retries = 9
	if len(orig.Headers) != 1 {
		t.Errorf("Clone shares the map with the original: %v", orig.Headers)
	}
	if orig.Retries != 3 {
		t.Errorf("changing the copy affected the original")
	}
}
