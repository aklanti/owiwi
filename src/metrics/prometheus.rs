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

#[cfg(test)]
mod tests {
    use googletest::matchers::{anything, ok};
    use googletest::{expect_that, gtest};

    use super::*;

    #[tokio::test]
    #[gtest]
    async fn prometheus_config_http_endpoint() {
        let config = PrometheusConfig::builder()
            .endpoint("http://localhost:9090".parse().expect("to be valid"))
            .build();

        let result: Result<opentelemetry_otlp::MetricExporter> = config.try_into();
        expect_that!(result, ok(anything()));
    }

    #[tokio::test]
    #[gtest]
    async fn prometheus_config_with_timeout() {
        let config = PrometheusConfig::builder()
            .endpoint("http://localhost:9090".parse().expect("to be valid"))
            .timeout(Duration::ZERO)
            .build();

        let result: Result<opentelemetry_otlp::MetricExporter> = config.try_into();
        expect_that!(result, ok(anything()));
    }
}
