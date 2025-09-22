//! # Owiwi
//!
//! This crate define opinionated OpenTelemetry tracer and metric initialization.
pub mod collector;
pub mod env_vars;
pub mod error;
pub mod format;
pub mod owiwi;
pub mod provider;

#[cfg(feature = "clap")]
pub const HELP_HEADING: &str = "Instrumentation options";
