// Exercise 02: errors as values — sentinel errors, custom error types, wrapping with %w.
package errs

import (
	"errors"
	"fmt"
	"strconv"
	"strings"
)

// Sentinel errors: compared with errors.Is.
var (
	ErrEmptyKey     = errors.New("empty key")
	ErrDuplicateKey = errors.New("duplicate key")
	ErrMissingKey   = errors.New("missing key")
)

// SyntaxError is a custom error type with extra data: extracted with errors.As.
type SyntaxError struct {
	Line int
	Text string
}

func (e *SyntaxError) Error() string {
	return fmt.Sprintf("line %d: missing '=' in %q", e.Line, e.Text)
}

// ParseConfig parses text in the "key = value" format line by line.
// Empty lines and lines starting with '#' are ignored.
func ParseConfig(text string) (map[string]string, error) {
	cfg := make(map[string]string)
	for i, raw := range strings.Split(text, "\n") {
		n := i + 1
		line := strings.TrimSpace(raw)
		if line == "" || strings.HasPrefix(line, "#") {
			continue
		}
		key, value, ok := strings.Cut(line, "=")
		if !ok {
			return nil, &SyntaxError{Line: n, Text: line}
		}
		key, value = strings.TrimSpace(key), strings.TrimSpace(value)
		if key == "" {
			return nil, fmt.Errorf("line %d: %w", n, ErrEmptyKey)
		}
		if _, exists := cfg[key]; exists {
			return nil, fmt.Errorf("line %d: key %q: %w", n, key, ErrDuplicateKey)
		}
		cfg[key] = value
	}
	return cfg, nil
}

// GetInt returns the key's value as an int.
// A missing key wraps ErrMissingKey, an invalid number wraps the original error from strconv.
func GetInt(cfg map[string]string, key string) (int, error) {
	s, ok := cfg[key]
	if !ok {
		return 0, fmt.Errorf("key %q: %w", key, ErrMissingKey)
	}
	n, err := strconv.Atoi(s)
	if err != nil {
		return 0, fmt.Errorf("key %q: %w", key, err)
	}
	return n, nil
}
