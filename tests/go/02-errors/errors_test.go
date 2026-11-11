package errs_test

import (
	"errors"
	"strconv"
	"testing"

	// Dot import: the tests use the exercise API unqualified, as if they were in the same package.
	// go.mod points it to exercises/go, solutions.mod to solutions/go.
	. "exercises/02-errors"
)

const validConfig = `
# server settings
host = tunel.example.no
port = 8080

timeout=30
`

func TestParseConfigValid(t *testing.T) {
	cfg, err := ParseConfig(validConfig)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	want := map[string]string{"host": "tunel.example.no", "port": "8080", "timeout": "30"}
	if len(cfg) != len(want) {
		t.Fatalf("expected %d keys, got %d: %v", len(want), len(cfg), cfg)
	}
	for k, v := range want {
		if cfg[k] != v {
			t.Errorf("cfg[%q] = %q, want %q", k, cfg[k], v)
		}
	}
}

func TestParseConfigSyntaxError(t *testing.T) {
	_, err := ParseConfig("a = 1\n\nno equals sign")
	var synErr *SyntaxError
	if !errors.As(err, &synErr) {
		t.Fatalf("expected *SyntaxError, got %v", err)
	}
	if synErr.Line != 3 || synErr.Text != "no equals sign" {
		t.Errorf("wrong data in the error: %+v", synErr)
	}
}

func TestParseConfigEmptyKey(t *testing.T) {
	_, err := ParseConfig("a = 1\n = 2")
	if !errors.Is(err, ErrEmptyKey) {
		t.Fatalf("expected ErrEmptyKey, got %v", err)
	}
	if err.Error() != "line 2: empty key" {
		t.Errorf("error message: %q", err.Error())
	}
}

func TestParseConfigDuplicateKey(t *testing.T) {
	_, err := ParseConfig("a = 1\na = 2")
	if !errors.Is(err, ErrDuplicateKey) {
		t.Fatalf("expected ErrDuplicateKey, got %v", err)
	}
}

func TestGetInt(t *testing.T) {
	cfg := map[string]string{"port": "8080", "host": "x"}

	if n, err := GetInt(cfg, "port"); err != nil || n != 8080 {
		t.Errorf("GetInt(port) = %d, %v", n, err)
	}

	if _, err := GetInt(cfg, "missing"); !errors.Is(err, ErrMissingKey) {
		t.Errorf("expected ErrMissingKey, got %v", err)
	}

	_, err := GetInt(cfg, "host")
	var numErr *strconv.NumError
	if !errors.As(err, &numErr) {
		t.Fatalf("expected a wrapped *strconv.NumError, got %v", err)
	}
	if numErr.Num != "x" {
		t.Errorf("NumError.Num = %q", numErr.Num)
	}
}
