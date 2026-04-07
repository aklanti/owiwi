use std::time::Duration;

use bon::Builder;
use opentelemetry_otlp::SpanExporter;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_otlp::WithTonicConfig;
use opentelemetry_otlp::tonic_types::metadata::MetadataMap;
use opentelemetry_otlp::tonic_types::transport::ClientTlsConfig;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::trace::Sampler;
use opentelemetry_sdk::trace::SdkTracerProvider;
use url::Url;

#[cfg(feature = "clap")]
use crate::HELP_HEADING;
use crate::env_vars;
use crate::error::Error;
use crate::error::ErrorKind;

/// Configuration for an OTLP span exporter.
#[must_use]
#[derive(Clone, Debug, Builder)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "clap", derive(clap::Args))]
pub struct OtlpConfig {
    /// Exporter endpoint.
    #[cfg_attr(
        feature = "clap",
        arg(
            name = "otlp-endpoint",
            long,
            help = "Exporter endpoint (e.g. http://localhost:4317)",
            env = env_vars::OTEL_EXPORTER_OTLP_ENDPOINT,
            help_heading = HELP_HEADING,
        ),
    )]
    pub endpoint: Url,

    /// Export timeout.
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "humantime_serde::deserialize")
    )]
    #[cfg_attr(
        feature = "clap",
        arg(
            name = "otlp-timeout",
            long,
            help = "Export timeout (e.g. 10s, 5m)",
            value_parser = humantime::parse_duration,
            env = env_vars::OTEL_EXPORTER_OTLP_TIMEOUT,
            help_heading = HELP_HEADING,
        )
    )]
    pub timeout: Duration,

    /// Additional gRPC metadata headers.
    #[cfg_attr(
        feature = "clap",
        arg(
            name = "otlp-headers",
            long,
            help = "gRPC metadata headers (key=value,key=value)",
            value_parser = env_vars::parse_key_values,
            env = env_vars::OTEL_EXPORTER_OTLP_HEADERS,
            help_heading = HELP_HEADING,
        )
    )]
    #[builder(default)]
    pub headers: Vec<(String, String)>,

    /// Span sampler. Defaults to the SDK default value
    #[cfg_attr(feature = "clap", arg(skip))]
    #[cfg_attr(feature = "serde", serde(skip))]
    pub sampler: Option<Sampler>,

    /// Custom TLS configuration
    #[cfg_attr(feature = "clap", arg(skip))]
    #[cfg_attr(feature = "serde", serde(skip))]
    pub tls_config: Option<ClientTlsConfig>,
}

impl OtlpConfig {
    /// Creates new configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Initializes the tracer provider.
    pub fn init_provider(mut self, resource: Resource) -> Result<SdkTracerProvider, Error> {
        let mut provider_builder = SdkTracerProvider::builder().with_resource(resource);
        match self.sampler.take() {
            Some(sampler) => {
                provider_builder = provider_builder.with_sampler(sampler);
            }
            None => {
                if let Ok(sampler) = std::env::var(env_vars::OTEL_TRACES_SAMPLER) {
                    let arg = std::env::var(env_vars::OTEL_TRACES_SAMPLER_ARG).ok();
                    let sampler = parse_sampler(&sampler, arg.as_deref())?;
                    provider_builder = provider_builder.with_sampler(sampler);
                }
            }
        }

        let exporter: SpanExporter = self.try_into()?;
        let tracer_provider = provider_builder.with_batch_exporter(exporter).build();
        Ok(tracer_provider)
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
        Self::builder()
            .endpoint("http://localhost:4317".parse().expect("valid URL"))
            .timeout(Duration::from_secs(10))
            .build()
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

/// Parses trace sampler
fn parse_sampler(name: &str, arg: Option<&str>) -> Result<Sampler, Error> {
    match name {
        "always_on" => Ok(Sampler::AlwaysOn),
        "always_off" => Ok(Sampler::AlwaysOff),
        "traceidratio" => {
            let ratio: f64 = arg
                .ok_or_else(|| ErrorKind::ExporterConfig {
                    reason: String::from("missing trace id ratio"),
                })?
                .parse()
                .map_err(|err| ErrorKind::ExporterConfig {
                    reason: format!("unable to parse trace id argument `{err}`"),
                })?;
            Ok(Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(
                ratio,
            ))))
        }
        other => Err(ErrorKind::ExporterConfig {
            reason: format!("invalid sampler `{other}`"),
        }
        .into()),
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

    #[tokio::test]
    #[gtest]
    async fn init_provider_with_defaults() {
        let config = OtlpConfig::builder()
            .endpoint("http://localhost:4317".parse().expect("to be valid"))
            .timeout(Duration::from_secs(7))
            .build();
        let resource = Resource::builder().with_service_name("test").build();
        let result = config.init_provider(resource);
        expect_that!(result, ok(anything()));
    }
}
