//! This module defines the telemetry provider abstractions.

use std::time::Duration;

use opentelemetry_otlp::tonic_types::metadata::MetadataMap;
use opentelemetry_otlp::{SpanExporter, WithExportConfig, WithTonicConfig};
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::trace::SdkTracerProvider;
use secrecy::{ExposeSecret, SecretString};
use url::Url;

use super::collector::Collector;
#[cfg(feature = "clap")]
use crate::HELP_HEADING;
#[cfg(feature = "clap")]
use crate::env_vars::EnvVars;
use crate::error::Error;

/// Tracer provider configuration options
#[must_use]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "clap", derive(clap::Args))]
pub struct TracerProviderOptions {
    /// Set the traces collector
    #[cfg_attr(
        feature = "clap",
        arg(
             name = "otel-collector",
             long,
             env = EnvVars::OTEL_TRACES_EXPORTER,
             default_value_t = Default::default(),
             help_heading = HELP_HEADING,
         )
    )]
    pub collector: Collector,

    /// Set export timeout duration
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "humantime_serde::deserialize")
    )]
    #[cfg_attr(
        feature = "clap",
        arg(
            name = "otel-exporter-timeout",
            long,
            value_parser = humantime::parse_duration,
            default_value = "10s",
            help_heading = HELP_HEADING,
        )
    )]
    pub exporter_timeout: Duration,

    /// Set exporter endpoint
    #[cfg_attr(
        feature = "clap",
        arg(
            name = "otel-exporter-timeout",
            long,
            env = EnvVars::OTEL_EXPORTER_OTLP_ENDPOINT,
            help_heading = HELP_HEADING,
        ),
    )]
    pub exporter_endpoint: Url,
}

impl TracerProviderOptions {
    /// Initializes the tracer
    pub fn provider(
        &self,
        api_key: SecretString,
        resource: Resource,
    ) -> Result<SdkTracerProvider, Error> {
        let provider_builder = SdkTracerProvider::builder().with_resource(resource);
        let tracer_provider = match self.collector {
            Collector::Console => provider_builder
                .with_simple_exporter(opentelemetry_stdout::SpanExporter::default())
                .build(),
            Collector::Honeycomb => {
                let mut metadata = MetadataMap::with_capacity(1);
                metadata.insert("x-honeycomb-team", api_key.expose_secret().try_into()?);
                let exporter = self.init_exporter(metadata)?;
                provider_builder.with_batch_exporter(exporter).build()
            }
            Collector::Jaeger => {
                let metadata = MetadataMap::with_capacity(1);
                let exporter = self.init_exporter(metadata)?;
                provider_builder.with_batch_exporter(exporter).build()
            }
        };

        Ok(tracer_provider)
    }

    pub fn init_exporter(&self, metadata: MetadataMap) -> Result<SpanExporter, Error> {
        let exporter = SpanExporter::builder()
            .with_tonic()
            .with_timeout(self.exporter_timeout)
            .with_endpoint(self.exporter_endpoint.as_ref())
            .with_metadata(metadata)
            .build()?;
        Ok(exporter)
    }
}
