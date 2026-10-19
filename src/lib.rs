//! Opinionated [`tracing`] subscriber with OpenTelemetry export.
//!
//! Sends traces to any [`TraceExporter`] backend.
//!
//! ## Decisions
//! - **Transport:** gRPC is the only transport. `OTEL_EXPORTER_OTLP_PROTOCOL` is not read.
//! - **Export strategy:** OTLP backends export in batches, and the console exporter writes each
//!   span as it ends.
//! - **Subscriber layers**, bottom to top: OpenTelemetry with the export filter, `ErrorLayer`,
//!   and fmt with the env filter.
//! - **Backend selection:** the builder's `traces` and `metrics` exporters pick the backends.
//!   `OTEL_TRACES_EXPORTER` and `OTEL_METRICS_EXPORTER` are not read.
//! - **TLS:** an HTTPS endpoint uses the system roots, unless the exporter's config sets its own.

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
pub use error::Error;
#[doc(inline)]
pub use error::Result;
#[doc(inline)]
pub use format::EventFormat;
#[doc(inline)]
pub use guard::FilterHandle;
#[doc(inline)]
pub use guard::OwiwiGuard;
#[doc(inline)]
#[cfg(feature = "metrics")]
pub use metrics::MetricExporter;
#[cfg(feature = "prometheus")]
pub use metrics::PrometheusConfig;
#[doc(inline)]
pub use owiwi::Owiwi;
#[cfg(feature = "honeycomb")]
#[doc(inline)]
pub use trace::HoneycombConfig;
#[doc(inline)]
pub use trace::OtlpConfig;
#[doc(inline)]
pub use trace::TraceExporter;
/// Help heading for telemetry options.
pub const HELP_HEADING: &str = "Telemetry";
