//! The code under test: `exercises/rust` by default, `solutions/rust` with `--features solutions`.
//!
//! Cargo can't switch a path dependency with a feature, so both crates are always compiled
//! and the feature only decides which one the tests see.

#[cfg(not(feature = "solutions"))]
pub use exercises::*;

#[cfg(feature = "solutions")]
pub use solutions::*;
