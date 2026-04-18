use std::time::Duration;

use bon::Builder;
use opentelemetry_otlp::SpanExporter;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_otlp::WithTonicConfig;
use opentelemetry_otlp::tonic_types::metadata::MetadataMap;
use opentelemetry_otlp::tonic_types::transport::ClientTlsConfig;
use url::Url;

use crate::env_vars;
use crate::error::Error;
use crate::error::ErrorKind;

/// Default OTEL endpoint value
const DEFAULT_OTLP_ENDPOINT: &str = "http://localhost:4317";
/// Default timeout value.
const DEFAULT_OTLP_TIMEOUT: Duration = Duration::from_secs(10);

/// Configuration for an OTLP span exporter.
#[must_use]
#[derive(Clone, Debug, Builder)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct OtlpConfig {
    /// Exporter endpoint.
    pub endpoint: Url,

    /// Export timeout.
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "humantime_serde::deserialize")
    )]
    pub timeout: Duration,

    /// Additional gRPC metadata headers.
    #[builder(default)]
    pub headers: Vec<(String, String)>,

    /// Custom TLS configuration
    #[cfg_attr(feature = "serde", serde(skip))]
    pub tls_config: Option<ClientTlsConfig>,
}

impl OtlpConfig {
    /// Builds the OTLP span exporter from this configuration.
    pub fn build_exporter(self) -> Result<SpanExporter, Error> {
        let metadata = self.metadata()?;

        let mut builder = SpanExporter::builder()
            .with_tonic()
            .with_endpoint(self.endpoint.as_ref())
            .with_metadata(metadata);

        if self.endpoint.scheme() == "https" {
            let tls = self
                .tls_config
                .unwrap_or_else(|| ClientTlsConfig::default().with_enabled_roots());
            builder = builder.with_tls_config(tls);
        }

        Ok(builder.build()?)
    }

    /// Builds the gRPC metadata map from all header sources.
    fn metadata(&self) -> Result<MetadataMap, Error> {
        let mut map = MetadataMap::with_capacity(self.headers.len());
        for (key, val) in &self.headers {
            let val = val.try_into().map_err(|_err| ErrorKind::ExporterConfig {
                reason: format!("invalid metadata value for header `{key}`"),
            })?;
            map.entry(key.as_str())
                .map_err(|_err| ErrorKind::ExporterConfig {
                    reason: format!("invalid metadata key `{key}`"),
                })?
                .or_insert(val);
        }
        Ok(map)
    }
}

impl Default for OtlpConfig {
    fn default() -> Self {
        let endpoint = std::env::var(env_vars::OTEL_EXPORTER_OTLP_ENDPOINT)
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or_else(|| DEFAULT_OTLP_ENDPOINT.parse().expect("valid URL"));

        let timeout = std::env::var(env_vars::OTEL_EXPORTER_OTLP_TIMEOUT)
            .ok()
            .and_then(|s| humantime::parse_duration(&s).ok())
            .unwrap_or(DEFAULT_OTLP_TIMEOUT);

        let headers = std::env::var(env_vars::OTEL_EXPORTER_OTLP_HEADERS)
            .ok()
            .and_then(|s| env_vars::parse_key_values(&s).ok())
            .unwrap_or_default();

        Self::builder()
            .endpoint(endpoint)
            .timeout(timeout)
            .headers(headers)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use googletest::expect_that;
    use googletest::gtest;
    use googletest::matchers::anything;
    use googletest::matchers::eq;
    use googletest::matchers::err;
    use googletest::matchers::ok;
    use googletest::matchers::some;

    use super::*;

    #[tokio::test]
    #[gtest]
    async fn can_create_a_span_exporter() {
        let config = OtlpConfig::builder()
            .endpoint("http://test.example".parse().expect("to be valid"))
            .timeout(Duration::ZERO)
            .build();

        let result: Result<SpanExporter, _> = config.build_exporter();
        expect_that!(result, ok(anything()));
    }

    #[gtest]
    fn metadata_contains_headers() {
        let config = OtlpConfig::builder()
            .endpoint("http://test.example".parse().expect("to be valid"))
            .timeout(Duration::ZERO)
            .maybe_headers(Some(vec![("x-api-key".to_owned(), "test".to_owned())]))
            .build();
        let metadata = config.metadata().expect("valid metadata");
        expect_that!(metadata.get("x-api-key"), some(eq("test")));
    }

    #[gtest]
    fn metadata_rejects_invalid_header_value() {
        let config = OtlpConfig::builder()
            .endpoint("http://localhost:4317".parse().expect("to be valid"))
            .timeout(Duration::ZERO)
            .headers(vec![("valid-key".to_owned(), "\0baad-value".to_owned())])
            .build();
        let result = config.metadata();
        expect_that!(result, err(anything()));
    }

    #[gtest]
    fn default_config_has_spec_values() {
        let config = OtlpConfig::default();
        expect_that!(config.endpoint.as_str(), eq("http://localhost:4317/"));
        expect_that!(config.timeout, eq(Duration::from_secs(10)));
    }
}
