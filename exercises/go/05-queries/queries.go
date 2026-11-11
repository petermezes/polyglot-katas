// Exercise 05: data queries — grouping, aggregation, sorting, working with time.
// (In C# you do the same with LINQ, in Go with loops and maps.)
package queries

import "time"

// Event is an event from a tunnel (e.g. a sensor alarm).
type Event struct {
	Tunnel   string
	Severity int // 1 = info, 2 = warning, 3 = alarm
	At       time.Time
}

// TunnelStat is a summary of one tunnel's events.
type TunnelStat struct {
	Tunnel      string
	Count       int
	MaxSeverity int
}

// CountByTunnel counts the events for each tunnel.
func CountByTunnel(events []Event) map[string]int {
	panic("TODO")
}

// LatestByTunnel returns the latest event for each tunnel.
//
// Tip: compare times with e.At.After(...), not with ==.
func LatestByTunnel(events []Event) map[string]Event {
	panic("TODO")
}

// TopTunnels returns at most n tunnels by event count descending, ties by name ascending.
//
// Tip: map[string]*TunnelStat lets you update the stats directly through a pointer.
// cmp.Or(cmp.Compare(...), cmp.Compare(...)) is a neat way to sort by multiple keys.
func TopTunnels(events []Event, n int) []TunnelStat {
	panic("TODO")
}

// Between returns events in the interval [from, to) sorted by time. It must not modify the input slice.
//
// Tip: careful — slices.SortFunc(events, ...) would also sort the caller's slice.
func Between(events []Event, from, to time.Time) []Event {
	panic("TODO")
}

// HourlyHistogram counts events by hour of day (in UTC).
//
// Tip: [24]int is an array (a fixed-length value type), not a slice.
func HourlyHistogram(events []Event) [24]int {
	panic("TODO")
}
