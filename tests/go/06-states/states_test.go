package states_test

import (
	"errors"
	"fmt"
	"slices"
	"testing"

	// Dot import: the tests use the exercise API unqualified, as if they were in the same package.
	// go.mod points it to exercises/go, solutions.mod to solutions/go.
	. "exercises/06-states"
)

func TestNextValid(t *testing.T) {
	tests := []struct {
		from LaneState
		a    Action
		want LaneState
	}{
		{Open, Restrict, Restricted},
		{Open, Close, Closed},
		{Restricted, Close, Closed},
		{Restricted, Reopen, Open},
		{Closed, Reopen, Open},
		{Closed, StartMaintenance, Maintenance},
		{Maintenance, EndMaintenance, Closed},
	}
	for _, tt := range tests {
		t.Run(fmt.Sprintf("%v_%v", tt.from, tt.a), func(t *testing.T) {
			got, err := Next(tt.from, tt.a)
			if err != nil || got != tt.want {
				t.Errorf("Next(%v, %v) = %v, %v; want %v", tt.from, tt.a, got, err, tt.want)
			}
		})
	}
}

func TestNextInvalid(t *testing.T) {
	invalid := []struct {
		from LaneState
		a    Action
	}{
		{Open, Reopen},
		{Open, StartMaintenance},
		{Restricted, StartMaintenance},
		{Maintenance, Reopen},
		{Closed, Close},
	}
	for _, tt := range invalid {
		got, err := Next(tt.from, tt.a)
		if !errors.Is(err, ErrInvalidTransition) {
			t.Errorf("Next(%v, %v): expected ErrInvalidTransition, got %v", tt.from, tt.a, err)
		}
		if got != tt.from {
			t.Errorf("on error Next should return the original state, got %v", got)
		}
	}
	_, err := Next(Maintenance, Reopen)
	if err.Error() != "maintenance + reopen: invalid transition" {
		t.Errorf("error message: %q", err)
	}
}

func TestStrings(t *testing.T) {
	if s := fmt.Sprint(Open, Maintenance, Close); s != "open maintenance close" {
		t.Errorf("String() = %q", s)
	}
	if s := LaneState(42).String(); s != "LaneState(42)" {
		t.Errorf("unknown state: %q", s)
	}
}

func TestLane(t *testing.T) {
	l := NewLane("Løvstakk lane 1")
	if l.State() != Open {
		t.Fatalf("a new lane should be open, is %v", l.State())
	}

	if err := l.Apply(Restrict, Close, StartMaintenance, EndMaintenance); err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	want := []LaneState{Open, Restricted, Closed, Maintenance, Closed}
	if !slices.Equal(l.History(), want) {
		t.Errorf("History() = %v, want %v", l.History(), want)
	}

	err := l.Apply(Reopen, Restrict, Reopen, StartMaintenance)
	if !errors.Is(err, ErrInvalidTransition) {
		t.Fatalf("expected ErrInvalidTransition, got %v", err)
	}
	if l.State() != Open {
		t.Errorf("after an error the last valid state (open) should remain, is %v", l.State())
	}

	h := l.History()
	h[0] = Maintenance
	if l.History()[0] != Open {
		t.Error("History() must return a copy")
	}
}
