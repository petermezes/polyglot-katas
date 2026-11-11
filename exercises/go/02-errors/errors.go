// Exercise 02: errors as values — sentinel errors, custom error types, wrapping with %w.
package errs

import "errors"

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

// Error implements the error interface.
// Format e.g.: line 3: missing '=' in "no equals sign"
func (e *SyntaxError) Error() string {
	panic("TODO")
}

// ParseConfig parses text in the "key = value" format line by line (lines numbered from 1).
// Empty lines and lines starting with '#' are ignored. Both key and value are trimmed of whitespace.
//
//   - line without '='   -> &SyntaxError{Line, Text}  (Text = trimmed line)
//   - empty key          -> error wrapping ErrEmptyKey, exactly: "line 2: empty key"
//   - key already exists -> error wrapping ErrDuplicateKey
//
// Tip: strings.Split, strings.TrimSpace, strings.Cut, fmt.Errorf("...: %w", ..., ErrX).
func ParseConfig(text string) (map[string]string, error) {
	panic("TODO")
}

// GetInt returns the key's value as an int.
//   - missing key    -> error wrapping ErrMissingKey
//   - invalid number -> error wrapping the original error from strconv.Atoi
//     (so the caller can use errors.As with *strconv.NumError)
func GetInt(cfg map[string]string, key string) (int, error) {
	panic("TODO")
}
