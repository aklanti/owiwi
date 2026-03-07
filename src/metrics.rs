//! This module defines the abstraction for exporting metrics
pub mod collector;

#[doc(inline)]
pub use collector::{MetricCollector, MetricOptions, MetricsConfig};

#[cfg(feature = "prometheus")]
#[doc(inline)]
pub use collector::PrometheusConfig;
