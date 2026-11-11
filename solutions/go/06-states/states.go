// Exercise 06: state machine — enums, switch, methods with pointer receivers, encapsulation.
package states

import (
	"errors"
	"fmt"
	"slices"
)

// LaneState is the state of a traffic lane in a tunnel.
type LaneState int

const (
	Open LaneState = iota
	Restricted
	Closed
	Maintenance
)

var stateNames = [...]string{"open", "restricted", "closed", "maintenance"}

func (s LaneState) String() string {
	if s < 0 || int(s) >= len(stateNames) {
		return fmt.Sprintf("LaneState(%d)", int(s))
	}
	return stateNames[s]
}

// Action is an operator command.
type Action int

const (
	Restrict Action = iota
	Close
	Reopen
	StartMaintenance
	EndMaintenance
)

var actionNames = [...]string{"restrict", "close", "reopen", "start maintenance", "end maintenance"}

func (a Action) String() string {
	if a < 0 || int(a) >= len(actionNames) {
		return fmt.Sprintf("Action(%d)", int(a))
	}
	return actionNames[a]
}

var ErrInvalidTransition = errors.New("invalid transition")

// Next returns the new state after the action, or an error wrapping ErrInvalidTransition.
//
//	Open        --Restrict--> Restricted   Open       --Close-->            Closed
//	Restricted  --Close-->    Closed       Restricted --Reopen-->           Open
//	Closed      --Reopen-->   Open         Closed     --StartMaintenance--> Maintenance
//	Maintenance --EndMaintenance--> Closed
func Next(s LaneState, a Action) (LaneState, error) {
	switch s {
	case Open:
		switch a {
		case Restrict:
			return Restricted, nil
		case Close:
			return Closed, nil
		}
	case Restricted:
		switch a {
		case Close:
			return Closed, nil
		case Reopen:
			return Open, nil
		}
	case Closed:
		switch a {
		case Reopen:
			return Open, nil
		case StartMaintenance:
			return Maintenance, nil
		}
	case Maintenance:
		if a == EndMaintenance {
			return Closed, nil
		}
	}
	return s, fmt.Errorf("%v + %v: %w", s, a, ErrInvalidTransition)
}

// Lane is a lane with a state and a history of states. The fields are unexported (lowercase).
type Lane struct {
	Name    string
	state   LaneState
	history []LaneState
}

// NewLane creates an open lane; the history starts with the Open state.
func NewLane(name string) *Lane {
	return &Lane{Name: name, state: Open, history: []LaneState{Open}}
}

func (l *Lane) State() LaneState { return l.state }

// Apply performs the actions in order. It stops at the first invalid one and returns an error;
// the state stays as after the last valid action.
func (l *Lane) Apply(actions ...Action) error {
	for _, a := range actions {
		next, err := Next(l.state, a)
		if err != nil {
			return err
		}
		l.state = next
		l.history = append(l.history, next)
	}
	return nil
}

// History returns a copy of the history so the caller can't change it from outside.
func (l *Lane) History() []LaneState {
	return slices.Clone(l.history)
}
