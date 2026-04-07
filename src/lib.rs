//! Opinionated [`tracing`] subscriber with OpenTelemetry export.
//!
//! Sends traces to any of the exporters defined in the [`trace`] module.
//!
//! ## Decisions
//! - **Transport:** gRPC is the only supported transport protocol.
//!   `OTEL_EXPORTER_OTLP_PROTOCOL` is not read.
//! - **Export strategy:** Only batch export is supported. Spans are buffered and flushed
//!   periodically. There is no simple/synchronous exporter option.
//! - **Subscriber layers** bottom to top: Opentelemetry with export filter, `ErrorLayer`,
//!   `EnvFilter` with env filter, and fmt.
//! - **Backend selection** This is determined by which initialization method you call, not
//!   by the `OTEL_TRACES_EXPORTER`.
//! - **TLS:** It's auto-enabled for HTTPS endpoints using system roots but can be configured.

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
#[cfg(feature = "prometheus")]
pub use metrics::PrometheusConfig;
#[doc(inline)]
pub use owiwi::Owiwi;
#[cfg(feature = "honeycomb")]
#[doc(inline)]
pub use trace::HoneycombConfig;
#[doc(inline)]
pub use trace::OtlpConfig;
/// Help heading for instrumentation options.
pub const HELP_HEADING: &str = "Instrumentation options";
