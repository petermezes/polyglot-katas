package queries_test

import (
	"slices"
	"testing"
	"time"

	// Dot import: the tests use the exercise API unqualified, as if they were in the same package.
	// go.mod points it to exercises/go, solutions.mod to solutions/go.
	. "exercises/05-queries"
)

func at(day, hour, minute int) time.Time {
	return time.Date(2026, time.November, day, hour, minute, 0, 0, time.UTC)
}

func sampleEvents() []Event {
	return []Event{
		{Tunnel: "Løvstakk", Severity: 1, At: at(3, 8, 15)},
		{Tunnel: "Fløyfjell", Severity: 3, At: at(3, 7, 50)},
		{Tunnel: "Løvstakk", Severity: 2, At: at(3, 9, 0)},
		{Tunnel: "Damsgård", Severity: 1, At: at(2, 23, 59)},
		{Tunnel: "Fløyfjell", Severity: 1, At: at(3, 8, 5)},
		{Tunnel: "Løvstakk", Severity: 1, At: at(3, 6, 30)},
		{Tunnel: "Damsgård", Severity: 2, At: at(3, 8, 45)},
	}
}

func TestCountByTunnel(t *testing.T) {
	got := CountByTunnel(sampleEvents())
	want := map[string]int{"Løvstakk": 3, "Fløyfjell": 2, "Damsgård": 2}
	for k, v := range want {
		if got[k] != v {
			t.Errorf("%s: %d, want %d", k, got[k], v)
		}
	}
	if len(got) != 3 {
		t.Errorf("expected 3 tunnels, got %d", len(got))
	}
}

func TestLatestByTunnel(t *testing.T) {
	got := LatestByTunnel(sampleEvents())
	if e := got["Løvstakk"]; !e.At.Equal(at(3, 9, 0)) {
		t.Errorf("Løvstakk: %v", e.At)
	}
	if e := got["Fløyfjell"]; e.Severity != 1 {
		t.Errorf("Fløyfjell: the latest event should have severity 1, got %d", e.Severity)
	}
}

func TestTopTunnels(t *testing.T) {
	got := TopTunnels(sampleEvents(), 2)
	want := []TunnelStat{
		{Tunnel: "Løvstakk", Count: 3, MaxSeverity: 2},
		{Tunnel: "Damsgård", Count: 2, MaxSeverity: 2}, // Damsgård < Fløyfjell alphabetically
	}
	if !slices.Equal(got, want) {
		t.Errorf("TopTunnels = %+v, want %+v", got, want)
	}
	if got := TopTunnels(nil, 5); len(got) != 0 {
		t.Errorf("TopTunnels(nil) = %v", got)
	}
}

func TestBetween(t *testing.T) {
	events := sampleEvents()
	original := slices.Clone(events)
	got := Between(events, at(3, 7, 50), at(3, 9, 0))
	var times []time.Time
	for _, e := range got {
		times = append(times, e.At)
	}
	want := []time.Time{at(3, 7, 50), at(3, 8, 5), at(3, 8, 15), at(3, 8, 45)}
	if !slices.EqualFunc(times, want, time.Time.Equal) {
		t.Errorf("Between = %v, want %v", times, want)
	}
	if !slices.Equal(events, original) {
		t.Error("Between must not modify the input slice")
	}
}

func TestHourlyHistogram(t *testing.T) {
	h := HourlyHistogram(sampleEvents())
	want := map[int]int{6: 1, 7: 1, 8: 3, 9: 1, 23: 1}
	for hour := range 24 {
		if h[hour] != want[hour] {
			t.Errorf("hour %d: %d, want %d", hour, h[hour], want[hour])
		}
	}
}
