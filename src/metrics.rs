//! Metrics export.
#[cfg(feature = "prometheus")]
mod prometheus;
mod provider;

#[cfg(feature = "prometheus")]
#[doc(inline)]
pub use prometheus::PrometheusConfig;
pub(crate) use provider::MeterProviderOptions;
