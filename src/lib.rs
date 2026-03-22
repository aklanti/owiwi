//! Opinionated [`tracing`] subscriber with OpenTelemetry export.
//!
//! Sends traces to any of the exporters defined in the [`trace`] module.
//!
#![cfg_attr(test, deny(warnings))]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod env_vars;
mod error;
mod format;
mod guard;
#[cfg(feature = "metrics")]
mod metrics;
mod owiwi;
mod trace;

#[doc(inline)]
pub use error::{Error, Result};
#[doc(inline)]
pub use format::EventFormat;
#[doc(inline)]
pub use guard::OwiwiGuard;
#[cfg(feature = "prometheus")]
pub use metrics::PrometheusConfig;
#[doc(inline)]
pub use owiwi::Owiwi;
#[cfg(feature = "honeycomb")]
#[doc(inline)]
pub use trace::HoneycombConfig;
#[doc(inline)]
pub use trace::otlp::OtlpConfig;
#[cfg(feature = "clap")]
/// Help heading for instrumentation options.
pub const HELP_HEADING: &str = "Instrumentation options";
