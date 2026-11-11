//! Exercise 07: optional configuration — a consuming builder, validation in build(), Clone,
//! struct update syntax.

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::time::Duration;

/// An HTTP client configuration.
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
    pub fn slower(&self, factor: f64) -> ClientConfig {
        // The listed fields are new, the rest comes from the expression after `..`.
        // `..self.clone()`: the rest must be owned, `..*self` would try to move out of a borrow.
        ClientConfig {
            timeout: self.timeout.mul_f64(factor),
            ..self.clone()
        }
    }
}

/// What build() can reject.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigError {
    InsecureUrl { url: String },
    InvalidTimeout { timeout: Duration },
    InvalidRetries { retries: u32 },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::InsecureUrl { url } => write!(f, "URL must start with https://: {url}"),
            ConfigError::InvalidTimeout { timeout } => {
                write!(f, "timeout {timeout:?}: must be positive")
            }
            ConfigError::InvalidRetries { retries } => {
                write!(f, "retries {retries}: allowed 0 to 10")
            }
        }
    }
}

impl Error for ConfigError {}

/// A builder: `ClientBuilder::new(url).timeout(...).retries(0).build()?`.
#[derive(Debug, Clone)]
pub struct ClientBuilder {
    config: ClientConfig,
}

impl ClientBuilder {
    /// Defaults: timeout 30 s, 3 retries, user agent "exercises/1.0", no headers.
    pub fn new(base_url: &str) -> Self {
        ClientBuilder {
            config: ClientConfig {
                base_url: base_url.to_string(),
                timeout: Duration::from_secs(30),
                retries: 3,
                user_agent: "exercises/1.0".to_string(),
                headers: HashMap::new(),
            },
        }
    }

    // `mut self`: the builder is ours (taken by value), so we may change it before returning it.
    // No allocation per call: the same struct just moves through the chain.

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    pub fn retries(mut self, retries: u32) -> Self {
        self.config.retries = retries;
        self
    }

    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.config
            .headers
            .insert(key.to_string(), value.to_string());
        self
    }

    pub fn user_agent(mut self, user_agent: &str) -> Self {
        self.config.user_agent = user_agent.to_string();
        self
    }

    /// Validates and returns the configuration.
    pub fn build(self) -> Result<ClientConfig, ConfigError> {
        let c = self.config;
        if !c.base_url.starts_with("https://") {
            return Err(ConfigError::InsecureUrl { url: c.base_url });
        }
        if c.timeout.is_zero() {
            // Duration can't be negative, so "positive" means "not zero".
            return Err(ConfigError::InvalidTimeout { timeout: c.timeout });
        }
        if c.retries > 10 {
            return Err(ConfigError::InvalidRetries { retries: c.retries });
        }
        Ok(c)
    }
}
