//! Exercise 02: errors as values — Result, a custom error enum, Display, Error::source, `?`.

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

/// Everything that can go wrong, as one enum. The variant says what happened (≈ Go's
/// errors.Is with a sentinel), its fields carry the details (≈ errors.As with a custom type).
///
/// In larger projects the `thiserror` crate derives Display and Error from attributes.
/// Writing them by hand once shows there is no magic behind it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigError {
    /// A line without '='. `text` is the trimmed line.
    Syntax {
        line: usize,
        text: String,
    },
    EmptyKey {
        line: usize,
    },
    DuplicateKey {
        line: usize,
        key: String,
    },
    MissingKey {
        key: String,
    },
    /// The value is not a number. The original error is in `source`.
    InvalidNumber {
        key: String,
        source: ParseIntError,
    },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // match on an enum must cover every variant: add a new one and this won't compile
        // until you write its message.
        match self {
            ConfigError::Syntax { line, text } => write!(f, "line {line}: missing '=' in {text:?}"),
            ConfigError::EmptyKey { line } => write!(f, "line {line}: empty key"),
            ConfigError::DuplicateKey { line, key } => {
                write!(f, "line {line}: key {key:?}: duplicate key")
            }
            ConfigError::MissingKey { key } => write!(f, "key {key:?}: missing key"),
            // Not "{source}" here: the cause is reachable through source(), and error reporters
            // (anyhow, eyre, a loop over source()) print the whole chain. Putting it in both
            // places would print it twice. This is the convention from the Rust API guidelines.
            ConfigError::InvalidNumber { key, .. } => write!(f, "key {key:?}: invalid number"),
        }
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ConfigError::InvalidNumber { source, .. } => Some(source),
            _ => None,
        }
    }
}

/// Parses text in the "key = value" format line by line (lines numbered from 1).
/// Empty lines and lines starting with '#' are ignored. Both key and value are trimmed.
pub fn parse_config(text: &str) -> Result<HashMap<String, String>, ConfigError> {
    let mut cfg = HashMap::new();
    for (i, raw) in text.lines().enumerate() {
        let line_no = i + 1;
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        // let-else: bind on success, otherwise the else block must leave (return/continue/...).
        let Some((key, value)) = line.split_once('=') else {
            return Err(ConfigError::Syntax {
                line: line_no,
                text: line.to_string(),
            });
        };
        let (key, value) = (key.trim(), value.trim());
        if key.is_empty() {
            return Err(ConfigError::EmptyKey { line: line_no });
        }
        if cfg.contains_key(key) {
            return Err(ConfigError::DuplicateKey {
                line: line_no,
                key: key.to_string(),
            });
        }
        cfg.insert(key.to_string(), value.to_string());
    }
    Ok(cfg)
}

/// The key's value as an i64. A missing key -> `MissingKey`, an invalid number -> `InvalidNumber`.
pub fn get_int(cfg: &HashMap<String, String>, key: &str) -> Result<i64, ConfigError> {
    // `?` returns the error to the caller right away (≈ Go's `if err != nil { return err }`).
    let value = cfg.get(key).ok_or_else(|| ConfigError::MissingKey {
        key: key.to_string(),
    })?;
    // `?` also converts the error if ConfigError implemented From<ParseIntError>. Here the
    // variant needs the key as well, so map_err builds it explicitly.
    // parse() rejects " 42" and "4_2" (like Go's Atoi, unlike Python's int()), but accepts "+42".
    value.parse().map_err(|source| ConfigError::InvalidNumber {
        key: key.to_string(),
        source,
    })
}

/// Like get_int, but without the reason: None on any error (≈ C#'s TryGetInt, Python's try_get_int).
pub fn try_get_int(cfg: &HashMap<String, String>, key: &str) -> Option<i64> {
    // In C# the Try pattern is a separate method with `out`. In Rust it's one call on the Result.
    get_int(cfg, key).ok()
}
