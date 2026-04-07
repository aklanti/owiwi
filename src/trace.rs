//! OpenTelemetry trace setup.

#[cfg(feature = "honeycomb")]
mod honeycomb;
mod otlp;
#[doc(hidden)]
pub use otlp::OtlpConfig;

#[cfg(feature = "honeycomb")]
#[doc(inline)]
pub use honeycomb::HoneycombConfig;

use opentelemetry_otlp::SpanExporter;

use crate::Error;

/// Trait for types that can build a span exporter.
///
/// Implement this for custom backend configurations. The built-in
/// implementations are [`OtlpConfig`] and [`HoneycombConfig`].
pub trait SpanExporterConfig {
    /// Builds the span exporter.
    fn build_exporter(self) -> Result<SpanExporter, Error>;
}
