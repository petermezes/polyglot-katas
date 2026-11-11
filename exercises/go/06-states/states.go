// Exercise 06: state machine — enums, switch, methods with pointer receivers, encapsulation.
package states

import "errors"

// LaneState is the state of a traffic lane in a tunnel.
type LaneState int

const (
	Open LaneState = iota
	Restricted
	Closed
	Maintenance
)

// String: "open", "restricted", "closed", "maintenance"; unknown state e.g. "LaneState(42)".
//
// Tip: var names = [...]string{...} and array indexing is shorter than a switch.
func (s LaneState) String() string {
	panic("TODO")
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

// String: "restrict", "close", "reopen", "start maintenance", "end maintenance".
func (a Action) String() string {
	panic("TODO")
}

var ErrInvalidTransition = errors.New("invalid transition")

// Next returns the new state after performing the action:
//
//	Open        --Restrict--> Restricted   Open       --Close-->            Closed
//	Restricted  --Close-->    Closed       Restricted --Reopen-->           Open
//	Closed      --Reopen-->   Open         Closed     --StartMaintenance--> Maintenance
//	Maintenance --EndMaintenance--> Closed
//
// Any other combination: returns the original state and an error wrapping ErrInvalidTransition,
// formatted exactly as "maintenance + reopen: invalid transition" (use %v and String()).
func Next(s LaneState, a Action) (LaneState, error) {
	panic("TODO")
}

// Lane is a lane with a state and a history of states.
// Keep the state and history in unexported fields (lowercase) so they can't be changed from outside.
type Lane struct {
	Name string
	// TODO: more fields
}

// NewLane creates an open lane; the history starts with the Open state.
func NewLane(name string) *Lane {
	panic("TODO")
}

func (l *Lane) State() LaneState {
	panic("TODO")
}

// Apply performs the actions in order (variadic function). It stops at the first invalid one and returns an error;
// the state stays as after the last valid action. Each new state is appended to the history.
func (l *Lane) Apply(actions ...Action) error {
	panic("TODO")
}

// History returns a COPY of the history so the caller can't change it from outside.
//
// Tip: slices.Clone.
func (l *Lane) History() []LaneState {
	panic("TODO")
}
