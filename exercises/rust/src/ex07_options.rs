//! Exercise 07: optional configuration — a consuming builder, validation in build(), Clone,
//! struct update syntax.

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::time::Duration;

/// An HTTP client configuration.
///
/// `derive(Clone)` copies everything, the HashMap included. Go's struct assignment shares the
/// map, so Go needs a hand-written Clone. In Rust a copy that shares data isn't possible by
/// accident: sharing would need an explicit Rc/Arc.
#[derive(Debug, Clone, PartialEq)]
pub struct ClientConfig {
    pub base_url: String,
    pub timeout: Duration,
    pub retries: u32,
    pub user_agent: String,
    pub headers: HashMap<String, String>,
}

impl ClientConfig {
    /// A copy with the timeout multiplied by factor. The original is unchanged.
    ///
    /// Tip: struct update syntax `ClientConfig { timeout: ..., ..self.clone() }` (≈ C#'s `with`,
    /// Python's `dataclasses.replace`). `Duration::mul_f64`.
    pub fn slower(&self, factor: f64) -> ClientConfig {
        todo!()
    }
}

/// What build() can reject.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigError {
    /// "URL must start with https://: http://api.example.no"
    InsecureUrl { url: String },
    /// "timeout 0ns: must be positive" (`{:?}` of a Duration)
    InvalidTimeout { timeout: Duration },
    /// "retries 11: allowed 0 to 10"
    InvalidRetries { retries: u32 },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Error for ConfigError {}

/// A builder: `ClientBuilder::new(url).timeout(...).retries(0).build()?`.
///
/// Rust has no optional or named parameters. Go answers that with functional options, C# with
/// object initializers or a builder, Python with keyword arguments. In Rust the builder is the
/// common answer. Each setter takes `self` by value and returns it, so calls chain and the
/// builder is consumed by build(). Naming convention: `timeout(...)`, not `with_timeout(...)`.
#[derive(Debug, Clone)]
pub struct ClientBuilder {
    config: ClientConfig,
}

impl ClientBuilder {
    /// Defaults: timeout 30 s, 3 retries, user agent "exercises/1.0", no headers.
    pub fn new(base_url: &str) -> Self {
        todo!()
    }

    pub fn timeout(self, timeout: Duration) -> Self {
        todo!()
    }

    pub fn retries(self, retries: u32) -> Self {
        todo!()
    }

    pub fn header(self, key: &str, value: &str) -> Self {
        todo!()
    }

    pub fn user_agent(self, user_agent: &str) -> Self {
        todo!()
    }

    /// Validates and returns the configuration. Checks in this order: the URL starts with
    /// "https://", the timeout is positive, retries are 0 to 10.
    ///
    /// The setters can't return a Result without breaking the chain, so all validation is here
    /// (C#'s builder throws right in the setter, Go's options return an error each).
    /// Retries are u32, so a negative number doesn't even compile: one check fewer.
    pub fn build(self) -> Result<ClientConfig, ConfigError> {
        todo!()
    }
}
