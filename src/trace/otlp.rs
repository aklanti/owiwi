use std::time::Duration;

use bon::Builder;
use opentelemetry_otlp::SpanExporter;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_otlp::WithTonicConfig;
use opentelemetry_otlp::tonic_types::metadata::MetadataMap;
use opentelemetry_otlp::tonic_types::transport::ClientTlsConfig;
use url::Url;

use crate::error::Error;
use crate::error::ErrorKind;

/// Configuration for an OTLP span exporter.
#[derive(Debug, Clone, Builder)]
pub struct OtlpConfig {
    /// Exporter endpoint.
    pub endpoint: Url,
    /// Export timeout.
    pub timeout: Duration,
    /// Additional gRPC metadata headers.
    #[builder(default)]
    pub headers: Vec<(String, String)>,
    /// Custom TLS configuration.
    pub tls_config: Option<ClientTlsConfig>,
}

impl OtlpConfig {
    /// Builds the gRPC metadata map from all header sources.
    fn metadata(&self) -> Result<MetadataMap, Error> {
        let mut map = MetadataMap::with_capacity(self.headers.len());
        for (key, val) in &self.headers {
            let val = val.try_into().map_err(|_| ErrorKind::ExporterConfig {
                reason: format!("invalid metadata value for header `{key}`"),
            })?;
            map.entry(key.as_str())
                .map_err(|_| ErrorKind::ExporterConfig {
                    reason: format!("invalid metadata key `{key}`"),
                })?
                .or_insert(val);
        }
        Ok(map)
    }
}

impl TryFrom<OtlpConfig> for SpanExporter {
    type Error = Error;
    fn try_from(config: OtlpConfig) -> Result<Self, Self::Error> {
        let metadata = config.metadata()?;
        let mut builder = SpanExporter::builder()
            .with_tonic()
            .with_endpoint(config.endpoint.as_ref())
            .with_metadata(metadata);

        if config.endpoint.scheme() == "https" {
            let tls = config
                .tls_config
                .unwrap_or_else(|| ClientTlsConfig::default().with_enabled_roots());
            builder = builder.with_tls_config(tls);
        }

        Ok(builder.build()?)
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

    #[gtest]
    fn otlp_config_builder_defaults() {
        let config = OtlpConfig::builder()
            .endpoint("http://test.example".parse().expect("to be valid"))
            .timeout(Duration::ZERO)
            .build();

        expect_that!(config.endpoint.as_str(), eq("http://test.example/"));
        expect_that!(config.timeout, eq(Duration::ZERO));
    }

    #[tokio::test]
    #[gtest]
    async fn can_create_a_span_exporter() {
        let config = OtlpConfig::builder()
            .endpoint("http://test.example".parse().expect("to be valid"))
            .timeout(Duration::ZERO)
            .build();

        let result: Result<SpanExporter, _> = config.try_into();
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
}
