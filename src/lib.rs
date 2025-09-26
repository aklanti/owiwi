//! # Owiwi
//!
//! This crates defines an opinionated abstraction to initialize Opentelemetry tracer and metric.
pub mod collector;
pub mod env_vars;
pub mod error;
pub mod format;
pub mod owiwi;
pub mod provider;

#[doc(inline)]
pub use owiwi::Owiwi;

#[doc(inline)]
pub use error::Error;

#[cfg(feature = "clap")]
/// Help heading for instrumentation options
pub const HELP_HEADING: &str = "Instrumentation options";
