//! # Owiwi
//!
//! This crates defines an opinionated abstraction to initialize OpenTelemetry tracer and metric.
pub mod collector;
pub mod env_vars;
pub mod error;
pub mod format;
pub mod owiwi;
pub mod provider;

#[doc(inline)]
pub use error::{Error, Result};
#[doc(inline)]
pub use owiwi::Owiwi;

#[cfg(feature = "clap")]
/// Help heading for instrumentation options
pub const HELP_HEADING: &str = "Instrumentation options";
