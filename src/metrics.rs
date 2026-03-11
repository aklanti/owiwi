//! This module defines the abstraction for exporting metrics
mod exporter;
#[cfg(feature = "prometheus")]
mod prometheus;
mod provider;

#[doc(inline)]
pub use exporter::MetricBackend;
#[cfg(feature = "prometheus")]
#[doc(inline)]
pub use prometheus::PrometheusConfig;
#[doc(inline)]
pub use provider::MeterProviderOptions;
