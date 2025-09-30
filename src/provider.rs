//! This module defines the telemetry provider abstractions.

use std::time::Duration;

use bon::Builder;
use opentelemetry::Value;
use opentelemetry_otlp::SpanExporter;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::trace::SdkTracerProvider;
use url::Url;

use super::collector::{Collector, CollectorConfig};
#[cfg(feature = "clap")]
use crate::HELP_HEADING;
#[cfg(feature = "clap")]
use crate::env_vars::EnvVars;
use crate::error::Error;

/// Tracer provider configuration options
#[must_use]
#[derive(Clone, Debug, Builder)]
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
            help_heading = HELP_HEADING,
        )
    )]
    pub exporter_timeout: Option<Duration>,

    /// Set exporter endpoint
    #[cfg_attr(
        feature = "clap",
        arg(
            name = "otel-exporter-endpoint",
            long,
            env = EnvVars::OTEL_EXPORTER_OTLP_ENDPOINT,
            help_heading = HELP_HEADING,
        ),
    )]
    pub exporter_endpoint: Option<Url>,
}

impl TracerProviderOptions {
    /// Initializes the tracer
    pub fn init_provider(
        &self,
        collector_config: CollectorConfig,
        resource: Resource,
    ) -> Result<SdkTracerProvider, Error> {
        let provider_builder = SdkTracerProvider::builder().with_resource(resource);
        let tracer_provider = match self.collector {
            Collector::Console => provider_builder
                .with_simple_exporter(opentelemetry_stdout::SpanExporter::default())
                .build(),
            Collector::Honeycomb => {
                let mut config = collector_config
                    .honeycomb()
                    .ok_or(Error::CollectorConfigError)?;

                if let Some(endpoint) = self.exporter_endpoint.clone() {
                    config.endpoint = endpoint;
                }

                if let Some(timeout) = self.exporter_timeout {
                    config.timeout = timeout;
                }

                let exporter: SpanExporter = config.try_into()?;
                provider_builder.with_batch_exporter(exporter).build()
            }
            Collector::Jaeger => {
                let mut config = collector_config
                    .jaeger()
                    .ok_or(Error::CollectorConfigError)?;
                if let Some(endpoint) = self.exporter_endpoint.clone() {
                    config.endpoint = endpoint;
                }

                if let Some(timeout) = self.exporter_timeout {
                    config.timeout = timeout;
                }

                let exporter: SpanExporter = config.try_into()?;
                provider_builder.with_batch_exporter(exporter).build()
            }
        };

        Ok(tracer_provider)
    }
}

/// Inititalizes the resource
pub fn init_resource(service_name: impl Into<Value>) -> Resource {
    Resource::builder().with_service_name(service_name).build()
}
