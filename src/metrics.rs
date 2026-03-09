//! This module defines the abstraction for exporting metrics
mod exporter;
#[cfg(feature = "prometheus")]
mod prometheus;

#[doc(inline)]
pub use exporter::{MeterProviderOptions, MetricExporter, MetricExporterConfig};
#[cfg(feature = "prometheus")]
#[doc(inline)]
pub use prometheus::PrometheusConfig;
