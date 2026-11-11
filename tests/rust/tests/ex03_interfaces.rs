use under_test::ex03_interfaces::*;

// A custom type in the test. Unlike Go it has to say `impl Sensor for` (like C#), but it
// overrides only id and evaluate, and gets describe() from the trait's default method.
struct FakeSensor {
    id: String,
}

impl Sensor for FakeSensor {
    fn id(&self) -> &str {
        &self.id
    }

    fn evaluate(&self) -> Severity {
        Severity::Warning
    }
}

fn sensors() -> Vec<Box<dyn Sensor>> {
    vec![
        Box::new(CoSensor {
            id: "co-1".into(),
            ppm: 12.0,
        }),
        Box::new(CoSensor {
            id: "co-2".into(),
            ppm: 75.0,
        }),
        Box::new(VisibilitySensor {
            id: "vis-1".into(),
            meters: 40.0,
        }),
        Box::new(SmokeDetector {
            id: "smoke-1".into(),
            triggered: false,
        }),
    ]
}

#[test]
fn evaluate_uses_thresholds() {
    let cases: Vec<(Box<dyn Sensor>, Severity)> = vec![
        (
            Box::new(CoSensor {
                ppm: 49.9,
                ..Default::default()
            }),
            Severity::Ok,
        ),
        (
            Box::new(CoSensor {
                ppm: 50.0,
                ..Default::default()
            }),
            Severity::Warning,
        ),
        (
            Box::new(CoSensor {
                ppm: 100.0,
                ..Default::default()
            }),
            Severity::Alarm,
        ),
        (
            Box::new(VisibilitySensor {
                meters: 150.0,
                ..Default::default()
            }),
            Severity::Ok,
        ),
        (
            Box::new(VisibilitySensor {
                meters: 149.0,
                ..Default::default()
            }),
            Severity::Warning,
        ),
        (
            Box::new(VisibilitySensor {
                meters: 49.0,
                ..Default::default()
            }),
            Severity::Alarm,
        ),
        (
            Box::new(SmokeDetector {
                triggered: true,
                ..Default::default()
            }),
            Severity::Alarm,
        ),
        (Box::new(SmokeDetector::default()), Severity::Ok),
    ];
    for (i, (sensor, want)) in cases.iter().enumerate() {
        assert_eq!(sensor.evaluate(), *want, "case {i}");
    }
}

#[test]
fn severity_display() {
    let got = format!("{} {} {}", Severity::Ok, Severity::Warning, Severity::Alarm);
    assert_eq!(got, "OK WARNING ALARM");
}

#[test]
fn worst_returns_max_severity() {
    assert_eq!(worst(&sensors()), Severity::Alarm);
    assert_eq!(worst(&[]), Severity::Ok);
}

#[test]
fn alarming_keeps_order_and_accepts_foreign_types() {
    let mut all = sensors();
    all.push(Box::new(FakeSensor { id: "fake".into() }));
    assert_eq!(alarming(&all, Severity::Warning), ["co-2", "vis-1", "fake"]);
}

#[test]
fn describe_per_type_with_default() {
    let cases: Vec<(Box<dyn Sensor>, &str)> = vec![
        (
            Box::new(CoSensor {
                id: "co-2".into(),
                ppm: 75.0,
            }),
            "CO co-2: 75.0 ppm",
        ),
        (
            Box::new(VisibilitySensor {
                id: "vis-1".into(),
                meters: 120.0,
            }),
            "Visibility vis-1: 120 m",
        ),
        (
            Box::new(SmokeDetector {
                id: "d-3".into(),
                triggered: true,
            }),
            "Smoke d-3: yes",
        ),
        (
            Box::new(SmokeDetector {
                id: "d-4".into(),
                triggered: false,
            }),
            "Smoke d-4: no",
        ),
        (Box::new(FakeSensor { id: "x".into() }), "Unknown sensor x"),
    ];
    for (sensor, want) in cases {
        assert_eq!(sensor.describe(), want);
    }
}
