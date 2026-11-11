// Exercise 03: interfaces — implicit implementation, Stringer, type switch.
package interfaces

// Severity is the severity of a sensor's state. An enum in Go = a custom type + iota.
type Severity int

const (
	OK Severity = iota
	Warning
	Alarm
)

// String implements fmt.Stringer: "OK", "WARNING", "ALARM".
// For an unknown value e.g. "Severity(7)".
func (s Severity) String() string {
	panic("TODO")
}

// Sensor is implemented by every type that has these methods. No "implements".
type Sensor interface {
	ID() string
	Evaluate() Severity
}

// Compile-time check that the types really implement the interface.
// Try deleting one of the methods below and look at the compiler error.
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
	panic("TODO")
}

// VisibilitySensor measures visibility in meters. < 150 warning, < 50 alarm.
type VisibilitySensor struct {
	SensorID string
	Meters   float64
}

func (s VisibilitySensor) ID() string { return s.SensorID }

func (s VisibilitySensor) Evaluate() Severity {
	panic("TODO")
}

// SmokeDetector reports smoke. Triggered = alarm.
type SmokeDetector struct {
	SensorID  string
	Triggered bool
}

func (s SmokeDetector) ID() string { return s.SensorID }

func (s SmokeDetector) Evaluate() Severity {
	panic("TODO")
}

// Worst returns the worst severity. OK for an empty list.
//
// Tip: the built-in max function also works for custom integer types (Go 1.21+).
func Worst(sensors []Sensor) Severity {
	panic("TODO")
}

// Alarming returns the IDs of sensors with severity at least min, in original order.
func Alarming(sensors []Sensor, min Severity) []string {
	panic("TODO")
}

// Describe returns a description based on the concrete sensor type:
//
//	COSensor         -> "CO co-2: 75.0 ppm"
//	VisibilitySensor -> "Visibility vis-1: 120 m"
//	SmokeDetector    -> "Smoke d-3: yes" / "Smoke d-4: no"
//	other type       -> "Unknown sensor x"
//
// Tip: switch v := s.(type) { case COSensor: ... }
func Describe(s Sensor) string {
	panic("TODO")
}
