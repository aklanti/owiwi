//! Prometheus configuration.

use std::time::Duration;

use bon::Builder;
use opentelemetry_otlp::{WithExportConfig, WithTonicConfig};
use url::Url;

use crate::error::{Error, Result};

/// Configuration for a Prometheus OTLP metrics exporter.
#[derive(Debug, Clone, Builder)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct PrometheusConfig {
    /// Exporter endpoint.
    pub endpoint: Url,
    /// Export timeout.
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "humantime_serde::deserialize")
    )]
    pub timeout: Option<Duration>,
}

impl TryFrom<PrometheusConfig> for opentelemetry_otlp::MetricExporter {
    type Error = Error;

    fn try_from(config: PrometheusConfig) -> Result<Self> {
        let mut builder = Self::builder()
            .with_tonic()
            .with_endpoint(config.endpoint.as_ref());
        if let Some(timeout) = config.timeout {
            builder = builder.with_timeout(timeout);
        }

        if config.endpoint.scheme() == "https" {
            builder = builder.with_tls_config(
                opentelemetry_otlp::tonic_types::transport::ClientTlsConfig::default()
                    .with_enabled_roots(),
            );
        }

        let exporter = builder.build()?;
        Ok(exporter)
    }
}
