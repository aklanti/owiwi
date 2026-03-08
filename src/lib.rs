//! `owiwi-tracing-opentelemetry` provides an opinionated abstraction for initializing tracing subscriber with OpenTelemetry.
//!
//! It allows sending traces to any of the collectors define in the [`trace::collector`] module.
//!
#![cfg_attr(test, deny(warnings))]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub(crate) mod env_vars;
pub mod error;
mod format;
mod guard;
#[cfg(feature = "metrics")]
mod metrics;
mod owiwi;
pub mod trace;

#[doc(inline)]
pub use error::{Error, Result};
#[doc(inline)]
pub use format::EventFormat;
#[doc(inline)]
pub use guard::OwiwiGuard;
#[cfg(feature = "metrics")]
#[doc(inline)]
pub use metrics::{MetricCollector, MetricOptions, MetricsConfig};
#[doc(inline)]
pub use owiwi::Owiwi;
#[doc(inline)]
pub use trace::{HoneycombConfig, JaegerConfig, TraceCollector, TraceCollectorConfig};

#[cfg(feature = "clap")]
/// Help heading for instrumentation options
pub const HELP_HEADING: &str = "Instrumentation options";
