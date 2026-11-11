// Exercise 05: data queries — grouping, aggregation, sorting, working with time.
// (In C# you do the same with LINQ, in Go with loops and maps.)
package queries

import (
	"cmp"
	"slices"
	"time"
)

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
	counts := make(map[string]int)
	for _, e := range events {
		counts[e.Tunnel]++
	}
	return counts
}

// LatestByTunnel returns the latest event for each tunnel.
func LatestByTunnel(events []Event) map[string]Event {
	latest := make(map[string]Event)
	for _, e := range events {
		if cur, ok := latest[e.Tunnel]; !ok || e.At.After(cur.At) {
			latest[e.Tunnel] = e
		}
	}
	return latest
}

// TopTunnels returns at most n tunnels by event count descending, ties by name ascending.
func TopTunnels(events []Event, n int) []TunnelStat {
	byTunnel := make(map[string]*TunnelStat)
	for _, e := range events {
		st, ok := byTunnel[e.Tunnel]
		if !ok {
			st = &TunnelStat{Tunnel: e.Tunnel}
			byTunnel[e.Tunnel] = st
		}
		st.Count++
		st.MaxSeverity = max(st.MaxSeverity, e.Severity)
	}
	stats := make([]TunnelStat, 0, len(byTunnel))
	for _, st := range byTunnel {
		stats = append(stats, *st)
	}
	slices.SortFunc(stats, func(a, b TunnelStat) int {
		return cmp.Or(
			cmp.Compare(b.Count, a.Count),
			cmp.Compare(a.Tunnel, b.Tunnel),
		)
	})
	if n < len(stats) {
		stats = stats[:n]
	}
	return stats
}

// Between returns events in the interval [from, to) sorted by time. It does not modify the input.
func Between(events []Event, from, to time.Time) []Event {
	var out []Event
	for _, e := range events {
		if !e.At.Before(from) && e.At.Before(to) {
			out = append(out, e)
		}
	}
	slices.SortFunc(out, func(a, b Event) int { return a.At.Compare(b.At) })
	return out
}

// HourlyHistogram counts events by hour of day (in UTC).
func HourlyHistogram(events []Event) [24]int {
	var hist [24]int // an array (not a slice) has a fixed length and a zero value
	for _, e := range events {
		hist[e.At.UTC().Hour()]++
	}
	return hist
}
