//! Honeycomb configuration.

use std::time::Duration;

use bon::Builder;
use opentelemetry_otlp::SpanExporter;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_otlp::WithTonicConfig;
use opentelemetry_otlp::tonic_types::metadata::MetadataMap;
use opentelemetry_otlp::tonic_types::transport::ClientTlsConfig;
use secrecy::ExposeSecret;
use secrecy::SecretString;
use url::Url;

use super::SpanExporterConfig;
use crate::error::ErrorKind;
use crate::error::Result;

/// Configuration for [Honeycomb](https://honeycomb.io) trace export.
#[derive(Debug, Clone, Builder)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct HoneycombConfig {
    /// Exporter endpoint.
    pub endpoint: Url,
    /// API key.
    pub api_key: SecretString,
    /// Export timeout.
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "humantime_serde::deserialize")
    )]
    pub timeout: Duration,
}

impl SpanExporterConfig for HoneycombConfig {
    fn build_exporter(self) -> Result<SpanExporter> {
        let metadata = {
            let mut map = MetadataMap::new();
            let val = self.api_key.expose_secret().try_into().map_err(|_err| {
                ErrorKind::ExporterConfig {
                    reason: "invalid honeycomb API key".into(),
                }
            })?;
            map.insert("x-honeycomb-team", val);
            map
        };

        let mut builder = SpanExporter::builder()
            .with_tonic()
            .with_endpoint(self.endpoint.as_ref())
            .with_timeout(self.timeout)
            .with_metadata(metadata);

        if self.endpoint.scheme() == "https" {
            builder = builder.with_tls_config(ClientTlsConfig::default().with_enabled_roots());
        }

        Ok(builder.build()?)
    }
}

#[cfg(test)]
mod tests {
    use googletest::expect_that;
    use googletest::gtest;
    use googletest::matchers::anything;
    use googletest::matchers::err;
    use googletest::matchers::ok;

    use super::*;

    #[tokio::test]
    #[gtest]
    async fn build_exporter_succeeds() {
        let config = HoneycombConfig::builder()
            .endpoint("http://localhost:4317".parse().expect("to be valid"))
            .api_key("test-key".into())
            .timeout(Duration::from_secs(5))
            .build();

        expect_that!(config.build_exporter(), ok(anything()));
    }

    #[gtest]
    fn build_exporter_rejects_invalid_api_key() {
        let config = HoneycombConfig::builder()
            .endpoint("http://localhost:4317".parse().expect("to be valid"))
            .api_key("\0bad-key".into())
            .timeout(Duration::from_secs(5))
            .build();

        expect_that!(config.build_exporter(), err(anything()));
    }
}
