//! This module defines the metrics collector abstraction.

use std::fmt;
use std::str::FromStr;
use std::time::Duration;

use bon::Builder;
use opentelemetry::global;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::metrics::PeriodicReader;
use opentelemetry_sdk::metrics::SdkMeterProvider;
use url::Url;

#[cfg(feature = "clap")]
use crate::HELP_HEADING;
use crate::error::Error;

/// This type enumerates the metric collectors
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(rename_all(deserialize = "lowercase"))
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
pub enum MetricExporter {
    /// Export metrics to `std::io::stdout`
    /// This variant is only suitable for development and debugging
    #[default]
    Console,
    /// Promethus metric exporter
    #[cfg(feature = "prometheus")]
    Prometheus,
}

impl MetricExporter {
    /// Returns a `&str` value of `self`
    #[must_use]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Console => "console",
            #[cfg(feature = "prometheus")]
            Self::Prometheus => "prometheus",
        }
    }
}

impl fmt::Display for MetricExporter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl FromStr for MetricExporter {
    type Err = Error;
    fn from_str(value: &str) -> Result<Self, Error> {
        let this = match value {
            "console" => Self::Console,
            #[cfg(feature = "prometheus")]
            "prometheus" => Self::Prometheus,
            _ => return Err(Error::UnsupportedMetricsCollector(value.to_owned())),
        };

        Ok(this)
    }
}

/// Configuration data for metrics initialization using the opentelemetry `metrics` crate
#[must_use]
#[derive(Clone, Debug, Default, Builder)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "clap", derive(clap::Args))]
pub struct MeterProviderOptions {
    /// Set the metric collector
    #[cfg_attr(
        feature = "clap",
        arg(
            name="metrics-exporter",
            long,
            help_heading = HELP_HEADING,
        ),
    )]
    pub collector: MetricExporter,

    /// Metrics update time interval
    #[cfg_attr(
        feature = "clap",
        arg(
            name="metrics-interval",
            long,
            value_parser = humantime::parse_duration,
            help_heading = HELP_HEADING,
        ),
    )]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "humantime_serde::deserialize")
    )]
    pub interval: Option<Duration>,
}

impl MeterProviderOptions {
    /// Initializes meter provider
    pub fn init_provider(
        &self,
        resource: Resource,
        config: impl MetricExporterConfig,
    ) -> Result<SdkMeterProvider, Error> {
        let exporter = config.try_into()?;
        let mut builder = PeriodicReader::builder(exporter);
        if let Some(interval) = self.interval {
            builder = builder.with_interval(interval);
        }
        let reader = builder.build();
        let meter_provider = SdkMeterProvider::builder()
            .with_resource(resource)
            .with_reader(reader)
            .build();

        global::set_meter_provider(meter_provider.clone());
        Ok(meter_provider)
    }
}

/// Metric exporter configuration trait
pub trait MetricExporterConfig: TryInto<opentelemetry_otlp::MetricExporter, Error = Error> {
    /// Set metric exporter API URL
    fn set_endpoint(&mut self, endpoint: Url);
    /// Set the metric exporter timeout
    fn set_timeout(&mut self, timeout: Duration);
}

#[cfg(test)]
mod tests {
    use googletest::matchers::{anything, eq, ok};
    use googletest::{assert_that, gtest};

    use super::MetricExporter;

    #[gtest]
    fn display_console_collector_value() {
        assert_that!(MetricExporter::Console.as_str(), eq("console"));
    }

    #[cfg(feature = "prometheus")]
    #[gtest]
    fn display_prometheus_collector_value() {
        assert_that!(MetricExporter::Prometheus.as_str(), eq("prometheus"));
    }

    #[cfg(feature = "prometheus")]
    #[gtest]
    fn parse_valid_prometheus_collector_from_string() {
        let result: Result<MetricExporter, _> = "prometheus".parse();
        assert_that!(result, ok(anything()));
    }

    #[gtest]
    fn parse_valid_console_collector_from_string() {
        let result: Result<MetricExporter, _> = "console".parse();
        assert_that!(result, ok(anything()));
    }
}
