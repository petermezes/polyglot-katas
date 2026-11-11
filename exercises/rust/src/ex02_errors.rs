//! Exercise 02: errors as values — Result, a custom error enum, Display, Error::source, `?`.

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

/// Everything that can go wrong, as one enum. The variant says what happened (≈ Go's
/// errors.Is with a sentinel), its fields carry the details (≈ errors.As with a custom type).
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

/// The message. Exactly:
///
/// - `Syntax`        -> `line 3: missing '=' in "no equals sign"`
/// - `EmptyKey`      -> `line 2: empty key`
/// - `DuplicateKey`  -> `line 2: key "a": duplicate key`
/// - `MissingKey`    -> `key "port": missing key`
/// - `InvalidNumber` -> `key "host": invalid number` (without the source's message, see source())
///
/// Tip: `write!(f, "line {line}: ...")`. `{text:?}` prints a string in quotes, like Go's %q.
impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

/// `source()` returns the underlying error (≈ Go's errors.Unwrap, C#'s InnerException,
/// Python's __cause__): the ParseIntError for `InvalidNumber`, nothing for the rest.
impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        todo!()
    }
}

/// Parses text in the "key = value" format line by line (lines numbered from 1).
/// Empty lines and lines starting with '#' are ignored. Both key and value are trimmed.
///
/// Tip: `str::lines`, `enumerate`, `trim`, `split_once('=')`. `let Some((key, value)) = ... else { ... };`
/// returns early on a line without '='.
pub fn parse_config(text: &str) -> Result<HashMap<String, String>, ConfigError> {
    todo!()
}

/// The key's value as an i64. A missing key -> `MissingKey`, an invalid number -> `InvalidNumber`.
///
/// Tip: `cfg.get(key).ok_or_else(|| ...)?` turns an Option into a Result and returns early on None.
/// `str::parse::<i64>()` returns `Result<i64, ParseIntError>`, `map_err` wraps the error into yours.
pub fn get_int(cfg: &HashMap<String, String>, key: &str) -> Result<i64, ConfigError> {
    todo!()
}

/// Like get_int, but without the reason: None on any error (≈ C#'s TryGetInt, Python's try_get_int).
///
/// Tip: `Result::ok()`.
pub fn try_get_int(cfg: &HashMap<String, String>, key: &str) -> Option<i64> {
    todo!()
}
