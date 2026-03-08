//! This module defines the abstraction for exporting metrics
mod collector;
#[cfg(feature = "prometheus")]
mod prometheus;

#[doc(inline)]
pub use collector::{MetricExporter, MetricOptions, MetricsConfig};

#[cfg(feature = "prometheus")]
#[doc(inline)]
pub use prometheus::PrometheusConfig;
