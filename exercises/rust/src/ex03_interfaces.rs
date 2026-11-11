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

/// `format!("{}", Severity::Alarm)` gives "ALARM". Labels: "OK", "WARNING", "ALARM".
///
/// Tip: `f.write_str(...)`. Unlike Go's iota, a Rust enum can't hold any other value,
/// so a match over the three variants needs no default branch.
impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

/// A sensor. A type becomes one only with an explicit `impl Sensor for T` (like C#, unlike Go).
pub trait Sensor {
    fn id(&self) -> &str;
    fn evaluate(&self) -> Severity;

    /// A description of the sensor. The default for types that don't override it:
    /// "Unknown sensor x" (x = id).
    ///
    /// Each sensor below overrides it:
    ///
    /// - `CoSensor`         -> "CO co-2: 75.0 ppm"        (one decimal place)
    /// - `VisibilitySensor` -> "Visibility vis-1: 120 m"  (no decimal places)
    /// - `SmokeDetector`    -> "Smoke d-3: yes" / "Smoke d-4: no"
    ///
    /// Go does this with a type switch, C# with a switch on types, Python with match on classes.
    /// A `dyn Sensor` doesn't reveal its concrete type, so in Rust the behavior belongs to the type.
    /// Tip: `format!("{:.1}", x)`.
    fn describe(&self) -> String {
        todo!()
    }
}

/// CO concentration in ppm. >= 50 warning, >= 100 alarm.
///
/// `Default` enables `CoSensor { ppm: 50.0, ..Default::default() }` (the remaining fields get defaults).
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

// Each impl also needs its own `fn describe(&self) -> String`, overriding the default.

impl Sensor for CoSensor {
    fn id(&self) -> &str {
        todo!()
    }

    fn evaluate(&self) -> Severity {
        todo!()
    }
}

impl Sensor for VisibilitySensor {
    fn id(&self) -> &str {
        todo!()
    }

    fn evaluate(&self) -> Severity {
        todo!()
    }
}

impl Sensor for SmokeDetector {
    fn id(&self) -> &str {
        todo!()
    }

    fn evaluate(&self) -> Severity {
        todo!()
    }
}

/// The worst severity. Ok for an empty slice.
///
/// `Box<dyn Sensor>` = a pointer to any type implementing Sensor (≈ a Go interface value,
/// a C# ISensor reference). A Vec of different sensor types needs it: a `Vec<T>` holds one T.
/// Tip: `iter().map(...).max()` returns an Option, `unwrap_or` supplies the value for empty input.
pub fn worst(sensors: &[Box<dyn Sensor>]) -> Severity {
    todo!()
}

/// IDs of sensors with severity at least `min`, in original order.
///
/// The result borrows the IDs from `sensors` (no copies). Rust infers that the returned `&str`s
/// live as long as `sensors`: there is only one reference among the inputs (lifetime elision).
pub fn alarming(sensors: &[Box<dyn Sensor>], min: Severity) -> Vec<&str> {
    todo!()
}
