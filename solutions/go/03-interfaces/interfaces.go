// Exercise 03: interfaces — implicit implementation, Stringer, type switch.
package interfaces

import "fmt"

// Severity is the severity of a sensor's state. An enum in Go = a custom type + iota.
type Severity int

const (
	OK Severity = iota
	Warning
	Alarm
)

// String implements fmt.Stringer, so fmt.Println(Alarm) prints "ALARM".
func (s Severity) String() string {
	switch s {
	case OK:
		return "OK"
	case Warning:
		return "WARNING"
	case Alarm:
		return "ALARM"
	default:
		return fmt.Sprintf("Severity(%d)", int(s))
	}
}

// Sensor is implemented by every type that has these methods. No "implements".
type Sensor interface {
	ID() string
	Evaluate() Severity
}

// Compile-time check that the types really implement the interface.
var (
	_ Sensor = COSensor{}
	_ Sensor = VisibilitySensor{}
	_ Sensor = SmokeDetector{}
)

// COSensor measures CO concentration in ppm. >= 50 warning, >= 100 alarm.
type COSensor struct {
	SensorID string
	PPM      float64
}

func (s COSensor) ID() string { return s.SensorID }

func (s COSensor) Evaluate() Severity {
	switch {
	case s.PPM >= 100:
		return Alarm
	case s.PPM >= 50:
		return Warning
	default:
		return OK
	}
}

// VisibilitySensor measures visibility in meters. < 150 warning, < 50 alarm.
type VisibilitySensor struct {
	SensorID string
	Meters   float64
}

func (s VisibilitySensor) ID() string { return s.SensorID }

func (s VisibilitySensor) Evaluate() Severity {
	switch {
	case s.Meters < 50:
		return Alarm
	case s.Meters < 150:
		return Warning
	default:
		return OK
	}
}

// SmokeDetector reports smoke. Triggered = alarm.
type SmokeDetector struct {
	SensorID  string
	Triggered bool
}

func (s SmokeDetector) ID() string { return s.SensorID }

func (s SmokeDetector) Evaluate() Severity {
	if s.Triggered {
		return Alarm
	}
	return OK
}

// Worst returns the worst severity. OK for an empty list.
func Worst(sensors []Sensor) Severity {
	worst := OK
	for _, s := range sensors {
		worst = max(worst, s.Evaluate())
	}
	return worst
}

// Alarming returns the IDs of sensors with severity at least min, in original order.
func Alarming(sensors []Sensor, min Severity) []string {
	var ids []string
	for _, s := range sensors {
		if s.Evaluate() >= min {
			ids = append(ids, s.ID())
		}
	}
	return ids
}

// Describe returns a description based on the concrete sensor type (type switch).
func Describe(s Sensor) string {
	switch v := s.(type) {
	case COSensor:
		return fmt.Sprintf("CO %s: %.1f ppm", v.SensorID, v.PPM)
	case VisibilitySensor:
		return fmt.Sprintf("Visibility %s: %.0f m", v.SensorID, v.Meters)
	case SmokeDetector:
		state := "no"
		if v.Triggered {
			state = "yes"
		}
		return fmt.Sprintf("Smoke %s: %s", v.SensorID, state)
	default:
		return fmt.Sprintf("Unknown sensor %s", s.ID())
	}
}
