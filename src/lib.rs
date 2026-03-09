//! `owiwi` provides an opinionated abstraction for initializing tracing subscriber with OpenTelemetry.
//!
//! It allows sending traces to any of the exporters define in the [`trace::exporter`] module.
//!
#![cfg_attr(test, deny(warnings))]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod env_vars;
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
#[cfg(feature = "metrics")]
#[doc(inline)]
pub use metrics::{MeterProviderOptions, MetricBackend};
#[doc(inline)]
pub use owiwi::Owiwi;
#[cfg(feature = "honeycomb")]
#[doc(inline)]
pub use trace::HoneycombConfig;
#[cfg(feature = "jaeger")]
#[doc(inline)]
pub use trace::JaegerConfig;
#[doc(inline)]
pub use trace::TraceBackend;

#[cfg(feature = "clap")]
/// Help heading for instrumentation options
pub const HELP_HEADING: &str = "Instrumentation options";
