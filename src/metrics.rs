//! Metrics export.
mod exporter;
#[cfg(feature = "prometheus")]
mod prometheus;
mod provider;

#[doc(inline)]
pub use exporter::MetricBackend;
#[cfg(feature = "prometheus")]
#[doc(inline)]
pub use prometheus::PrometheusConfig;
pub(crate) use provider::MeterProviderOptions;
