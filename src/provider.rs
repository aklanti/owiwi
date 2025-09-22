//! This module defines the telemetry provider abstractions.

use std::time::Duration;

use url::Url;

use super::collector::Collector;
#[cfg(feature = "clap")]
use crate::HELP_HEADING;
use crate::env_vars::EnvVars;

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
