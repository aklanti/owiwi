//! Tracer provider configuration.

use std::time::Duration;

use bon::Builder;
use opentelemetry_otlp::SpanExporter;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::trace::Sampler;
use opentelemetry_sdk::trace::SdkTracerProvider;
use url::Url;

#[cfg(feature = "clap")]
use crate::HELP_HEADING;
use crate::OtlpConfig;
use crate::env_vars;
use crate::error::Error;
use crate::error::ErrorKind;

/// Tracer provider configuration.
#[must_use]
#[derive(Clone, Debug, Default, Builder)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "clap", derive(clap::Args))]
pub struct TracerProviderOptions {
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
            value_parser = humantime::parse_duration,
            env = env_vars::OTEL_EXPORTER_OTLP_TIMEOUT,
            help_heading = HELP_HEADING,
        )
    )]
    pub timeout: Option<Duration>,

    /// Exporter endpoint.
    #[cfg_attr(
        feature = "clap",
        arg(
            name = "otlp-endpoint",
            long,
            env = env_vars::OTEL_EXPORTER_OTLP_ENDPOINT,
            help_heading = HELP_HEADING,
        ),
    )]
    pub endpoint: Option<Url>,

    /// Additional gRPC metadata headers.
    #[cfg_attr(
        feature = "clap",
        arg(
            name = "otlp-headers",
            long,
            value_parser = env_vars::parse_key_values,
            env = env_vars::OTEL_EXPORTER_OTLP_HEADERS,
            help_heading = HELP_HEADING,
        )
    )]
    pub headers: Vec<(String, String)>,

    /// Span sampler. Defaults to the SDK default value
    #[cfg_attr(feature = "clap", arg(skip))]
    #[cfg_attr(feature = "serde", serde(skip))]
    pub sampler: Option<Sampler>,
}

impl TracerProviderOptions {
    /// Initializes the tracer provider.
    pub fn init_provider(
        &self,
        config: impl Into<OtlpConfig>,
        resource: Resource,
    ) -> Result<SdkTracerProvider, Error> {
        let mut provider_builder = SdkTracerProvider::builder().with_resource(resource);
        match &self.sampler {
            Some(sampler) => {
                provider_builder = provider_builder.with_sampler(sampler.clone());
            }
            None => {
                if let Ok(sampler) = std::env::var(env_vars::OTEL_TRACES_SAMPLER) {
                    let arg = std::env::var(env_vars::OTEL_TRACES_SAMPLER_ARG).ok();
                    let sampler = parse_sampler(&sampler, arg.as_deref())?;
                    provider_builder = provider_builder.with_sampler(sampler);
                }
            }
        }

        let mut config = config.into();
        if let Some(endpoint) = self.endpoint.clone() {
            config.endpoint = endpoint;
        }

        if let Some(timeout) = self.timeout {
            config.timeout = timeout;
        }

        #[cfg(feature = "clap")]
        let extra_headers = self.headers.clone();
        #[cfg(not(feature = "clap"))]
        let extra_headers = std::env::var(env_vars::OTEL_EXPORTER_OTLP_HEADERS)
            .ok()
            .and_then(|raw| env_vars::parse_key_values(&raw).ok())
            .unwrap_or_default();
        config.headers.extend(extra_headers);

        let exporter: SpanExporter = config.try_into()?;
        let tracer_provider = provider_builder.with_batch_exporter(exporter).build();
        Ok(tracer_provider)
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
    use googletest::matchers::ok;

    use super::*;

    #[tokio::test]
    #[gtest]
    async fn init_provider_with_defaults() {
        let opts = TracerProviderOptions::default();
        let config = OtlpConfig::builder()
            .endpoint("http://localhost:4317".parse().expect("to be valid"))
            .timeout(Duration::from_secs(7))
            .build();
        let resource = Resource::builder().with_service_name("test").build();
        let result = opts.init_provider(config, resource);
        expect_that!(result, ok(anything()));
    }
}
