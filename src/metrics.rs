//! Metrics export.
#[cfg(feature = "prometheus")]
mod prometheus;

#[cfg(feature = "prometheus")]
#[doc(inline)]
pub use prometheus::PrometheusConfig;
