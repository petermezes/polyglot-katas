//! Exercise 03: interfaces — traits, trait objects (`dyn`), default methods, an enum with Display.

use std::fmt;

/// The severity of a sensor's state.
///
/// `derive(PartialOrd, Ord)` orders the variants by declaration: Ok < Warning < Alarm,
/// so `>=`, `max()` and sorting work without writing anything.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Severity {
    Ok,
    Warning,
    Alarm,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // No default branch like Go's "Severity(%d)": there is no fourth value.
        f.write_str(match self {
            Severity::Ok => "OK",
            Severity::Warning => "WARNING",
            Severity::Alarm => "ALARM",
        })
    }
}

/// A sensor. A type becomes one only with an explicit `impl Sensor for T` (like C#, unlike Go).
pub trait Sensor {
    fn id(&self) -> &str;
    fn evaluate(&self) -> Severity;

    /// A description of the sensor. Types that don't override it get "Unknown sensor x".
    ///
    /// Go uses a type switch here. A `dyn Sensor` doesn't reveal its concrete type (short of
    /// downcasting through std::any::Any, which is rarely the right tool). Two idiomatic options:
    /// - a trait method, like here: an open set, anyone can add a sensor (the test's FakeSensor
    ///   does, and gets this default),
    /// - `enum Sensor { Co(CoSensor), Visibility(..), Smoke(..) }` with `match`: a closed set,
    ///   and the compiler checks that every match handles every variant.
    fn describe(&self) -> String {
        format!("Unknown sensor {}", self.id())
    }
}

/// CO concentration in ppm. >= 50 warning, >= 100 alarm.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CoSensor {
    pub id: String,
    pub ppm: f64,
}

/// Visibility in meters. < 150 warning, < 50 alarm.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct VisibilitySensor {
    pub id: String,
    pub meters: f64,
}

/// A triggered detector = alarm.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SmokeDetector {
    pub id: String,
    pub triggered: bool,
}

impl Sensor for CoSensor {
    fn id(&self) -> &str {
        &self.id // a field and a method may share a name, they live in different namespaces
    }

    fn evaluate(&self) -> Severity {
        if self.ppm >= 100.0 {
            Severity::Alarm
        } else if self.ppm >= 50.0 {
            Severity::Warning
        } else {
            Severity::Ok
        }
    }

    fn describe(&self) -> String {
        // Rust formatting ignores the OS locale: always a decimal point (C# needs InvariantCulture).
        format!("CO {}: {:.1} ppm", self.id, self.ppm)
    }
}

impl Sensor for VisibilitySensor {
    fn id(&self) -> &str {
        &self.id
    }

    fn evaluate(&self) -> Severity {
        if self.meters < 50.0 {
            Severity::Alarm
        } else if self.meters < 150.0 {
            Severity::Warning
        } else {
            Severity::Ok
        }
    }

    fn describe(&self) -> String {
        format!("Visibility {}: {:.0} m", self.id, self.meters)
    }
}

impl Sensor for SmokeDetector {
    fn id(&self) -> &str {
        &self.id
    }

    fn evaluate(&self) -> Severity {
        if self.triggered {
            Severity::Alarm
        } else {
            Severity::Ok
        }
    }

    fn describe(&self) -> String {
        let state = if self.triggered { "yes" } else { "no" }; // if is an expression
        format!("Smoke {}: {state}", self.id)
    }
}

/// The worst severity. Ok for an empty slice.
pub fn worst(sensors: &[Box<dyn Sensor>]) -> Severity {
    sensors
        .iter()
        .map(|s| s.evaluate())
        .max()
        .unwrap_or(Severity::Ok)
}

/// IDs of sensors with severity at least `min`, in original order.
pub fn alarming(sensors: &[Box<dyn Sensor>], min: Severity) -> Vec<&str> {
    // The &str values point into the sensors' Strings: no copies. The full signature is
    // `fn alarming<'a>(sensors: &'a [Box<dyn Sensor>], min: Severity) -> Vec<&'a str>`;
    // with a single input reference the compiler fills in 'a by itself (lifetime elision).
    sensors
        .iter()
        .filter(|s| s.evaluate() >= min)
        .map(|s| s.id())
        .collect()
}
