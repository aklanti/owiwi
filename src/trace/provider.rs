//! This module defines the telemetry provider abstractions.

use std::time::Duration;

use bon::Builder;
use opentelemetry::Value;
use opentelemetry_otlp::SpanExporter;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::trace::SdkTracerProvider;
use url::Url;

use super::exporter::TraceBackend;
use super::otlp;
#[cfg(feature = "clap")]
use crate::HELP_HEADING;
use crate::OtlpConfig;
use crate::env_vars;
use crate::error::Error;

/// Tracer provider configuration options
#[must_use]
#[derive(Clone, Debug, Default, Builder)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "clap", derive(clap::Args))]
pub struct TracerProviderOptions {
    /// Set the traces exporter
    #[cfg_attr(
        feature = "clap",
        arg(
             name = "trace-exporter",
             long,
             value_enum,
             default_value_t = Default::default(),
             env = env_vars::OTEL_TRACES_EXPORTER,
             help_heading = HELP_HEADING,
         )
    )]
    pub trace_backend: TraceBackend,

    /// Set export timeout duration
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

    /// Set exporter endpoint
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

    #[cfg_attr(
        feature = "clap",
        arg(
            name = "otlp-headers",
            long,
            value_parser = otlp::parse_headers,
            env = env_vars::OTEL_EXPORTER_OTLP_HEADERS,
            help_heading = HELP_HEADING,
        )
    )]
    pub headers: Vec<(String, String)>,
}

impl TracerProviderOptions {
    /// Initializes the tracer provider
    pub fn init_provider(
        &self,
        config: impl Into<OtlpConfig>,
        resource: Resource,
    ) -> Result<SdkTracerProvider, Error> {
        let mut config = config.into();

        let provider_builder = SdkTracerProvider::builder().with_resource(resource);
        if let Some(endpoint) = self.endpoint.clone() {
            config.endpoint = endpoint;
        }

        if let Some(timeout) = self.timeout {
            config.timeout = timeout;
        }

        cfg_if::cfg_if! {
            if #[cfg(feature="clap")] {
                config.headers.extend(self.headers.clone());
            } else {
                if let Ok(raw) = std::env::var(env_vars::OTEL_EXPORTER_OTLP_HEADERS)
                    && let Ok(headers) = otlp::parse_headers(&raw) {
                        config.headers.extend(headers);
                }
            }
        }
        let exporter: SpanExporter = config.try_into()?;
        let tracer_provider = provider_builder.with_batch_exporter(exporter).build();
        Ok(tracer_provider)
    }
}

/// Inititalizes the resource
pub(crate) fn init_resource(service_name: impl Into<Value>) -> Resource {
    let mut builder = Resource::builder().with_service_name(service_name);
    if let Ok(attrs) = std::env::var(env_vars::OTEL_RESOURCE_ATTRIBUTES) {
        for pair in attrs.split(',') {
            if let Some((key, value)) = pair.split_once('=') {
                builder = builder.with_attribute(opentelemetry::KeyValue::new(
                    key.trim().to_owned(),
                    value.trim().to_owned(),
                ));
            }
        }
    }
    builder.build()
}
