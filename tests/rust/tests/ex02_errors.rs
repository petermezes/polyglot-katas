use std::collections::HashMap;
use std::error::Error;
use std::num::ParseIntError;

use under_test::ex02_errors::*;

const VALID_CONFIG: &str = "
# server settings
host = tunel.example.no
port = 8080

timeout=30
";

fn map(pairs: &[(&str, &str)]) -> HashMap<String, String> {
    pairs
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

#[test]
fn parse_config_valid() {
    let cfg = parse_config(VALID_CONFIG).expect("unexpected error");
    assert_eq!(
        cfg,
        map(&[
            ("host", "tunel.example.no"),
            ("port", "8080"),
            ("timeout", "30")
        ])
    );
}

#[test]
fn parse_config_syntax_error() {
    let err = parse_config("a = 1\n\nno equals sign").unwrap_err();
    // The variant and its data in one comparison (≈ errors.As + checking the fields).
    assert_eq!(
        err,
        ConfigError::Syntax {
            line: 3,
            text: "no equals sign".to_string()
        }
    );
    assert_eq!(err.to_string(), "line 3: missing '=' in \"no equals sign\"");
}

#[test]
fn parse_config_empty_key() {
    let err = parse_config("a = 1\n = 2").unwrap_err();
    assert_eq!(err, ConfigError::EmptyKey { line: 2 });
    assert_eq!(err.to_string(), "line 2: empty key");
}

#[test]
fn parse_config_duplicate_key() {
    let err = parse_config("a = 1\na = 2").unwrap_err();
    // matches! checks only the shape (≈ errors.Is), `..` ignores the other fields.
    assert!(
        matches!(err, ConfigError::DuplicateKey { line: 2, .. }),
        "expected DuplicateKey on line 2, got {err:?}"
    );
    assert_eq!(err.to_string(), "line 2: key \"a\": duplicate key");
}

#[test]
fn get_int_returns_number_or_error() {
    let cfg = map(&[("port", "8080"), ("host", "x")]);

    assert_eq!(get_int(&cfg, "port"), Ok(8080));

    let missing = get_int(&cfg, "missing").unwrap_err();
    assert_eq!(
        missing,
        ConfigError::MissingKey {
            key: "missing".to_string()
        }
    );
    assert_eq!(missing.to_string(), "key \"missing\": missing key");

    let invalid = get_int(&cfg, "host").unwrap_err();
    assert!(
        matches!(&invalid, ConfigError::InvalidNumber { key, .. } if key == "host"),
        "expected InvalidNumber for host, got {invalid:?}"
    );
    assert_eq!(invalid.to_string(), "key \"host\": invalid number");
    let source = invalid
        .source()
        .expect("the original ParseIntError should be in source()");
    assert!(source.is::<ParseIntError>(), "source() = {source:?}");
}

#[test]
fn try_get_int_returns_none_on_error() {
    let cfg = map(&[("port", "8080"), ("host", "x")]);
    assert_eq!(try_get_int(&cfg, "port"), Some(8080));
    assert_eq!(try_get_int(&cfg, "missing"), None);
    assert_eq!(try_get_int(&cfg, "host"), None);
}
