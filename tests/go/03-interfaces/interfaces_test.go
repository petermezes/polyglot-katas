package interfaces_test

import (
	"fmt"
	"slices"
	"testing"

	// Dot import: the tests use the exercise API unqualified, as if they were in the same package.
	// go.mod points it to exercises/go, solutions.mod to solutions/go.
	. "exercises/03-interfaces"
)

// A custom type in the test — implements Sensor without the package knowing about it.
type fakeSensor struct{ id string }

func (f fakeSensor) ID() string         { return f.id }
func (f fakeSensor) Evaluate() Severity { return Warning }

func sensors() []Sensor {
	return []Sensor{
		COSensor{SensorID: "co-1", PPM: 12},
		COSensor{SensorID: "co-2", PPM: 75},
		VisibilitySensor{SensorID: "vis-1", Meters: 40},
		SmokeDetector{SensorID: "smoke-1", Triggered: false},
	}
}

func TestEvaluate(t *testing.T) {
	tests := []struct {
		sensor Sensor
		want   Severity
	}{
		{COSensor{PPM: 49.9}, OK},
		{COSensor{PPM: 50}, Warning},
		{COSensor{PPM: 100}, Alarm},
		{VisibilitySensor{Meters: 150}, OK},
		{VisibilitySensor{Meters: 149}, Warning},
		{VisibilitySensor{Meters: 49}, Alarm},
		{SmokeDetector{Triggered: true}, Alarm},
		{SmokeDetector{}, OK},
	}
	for _, tt := range tests {
		if got := tt.sensor.Evaluate(); got != tt.want {
			t.Errorf("%#v.Evaluate() = %v, want %v", tt.sensor, got, tt.want)
		}
	}
}

func TestSeverityString(t *testing.T) {
	if got := fmt.Sprint(OK, Warning, Alarm); got != "OK WARNING ALARM" {
		t.Errorf("String() = %q", got)
	}
}

func TestWorst(t *testing.T) {
	if got := Worst(sensors()); got != Alarm {
		t.Errorf("Worst() = %v", got)
	}
	if got := Worst(nil); got != OK {
		t.Errorf("Worst(nil) = %v", got)
	}
}

func TestAlarming(t *testing.T) {
	all := append(sensors(), fakeSensor{id: "fake"})
	got := Alarming(all, Warning)
	want := []string{"co-2", "vis-1", "fake"}
	if !slices.Equal(got, want) {
		t.Errorf("Alarming(Warning) = %q, want %q", got, want)
	}
}

func TestDescribe(t *testing.T) {
	tests := []struct {
		sensor Sensor
		want   string
	}{
		{COSensor{SensorID: "co-2", PPM: 75}, "CO co-2: 75.0 ppm"},
		{VisibilitySensor{SensorID: "vis-1", Meters: 120}, "Visibility vis-1: 120 m"},
		{SmokeDetector{SensorID: "d-3", Triggered: true}, "Smoke d-3: yes"},
		{SmokeDetector{SensorID: "d-4"}, "Smoke d-4: no"},
		{fakeSensor{id: "x"}, "Unknown sensor x"},
	}
	for _, tt := range tests {
		if got := Describe(tt.sensor); got != tt.want {
			t.Errorf("Describe() = %q, want %q", got, tt.want)
		}
	}
}
