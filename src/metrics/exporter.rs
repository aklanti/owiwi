//! This module defines the metrics collector abstraction.

use std::fmt;
use std::str::FromStr;
use std::time::Duration;

use bon::Builder;
use opentelemetry::global;
use opentelemetry_otlp::MetricExporter;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::metrics::PeriodicReader;
use opentelemetry_sdk::metrics::SdkMeterProvider;

#[cfg(feature = "clap")]
use crate::HELP_HEADING;
use crate::error::Error;

/// Supported metric export backends
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(rename_all(deserialize = "lowercase"))
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
pub enum MetricBackend {
    /// Writes metrics to `std::io::stdout`
    #[default]
    Console,
    /// Exports metrics to [Promethus](https://prometheus.io)
    #[cfg(feature = "prometheus")]
    Prometheus,
}

impl MetricBackend {
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

impl fmt::Display for MetricBackend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl FromStr for MetricBackend {
    type Err = Error;
    fn from_str(value: &str) -> Result<Self, Error> {
        let this = match value {
            "console" => Self::Console,
            #[cfg(feature = "prometheus")]
            "prometheus" => Self::Prometheus,
            _ => return Err(Error::MetricBackend(value.to_owned())),
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
            name="metric-exporter",
            long,
            help_heading = HELP_HEADING,
        ),
    )]
    pub metric_exporter: MetricBackend,

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
        config: impl TryInto<MetricExporter, Error = Error>,
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

#[cfg(test)]
mod tests {
    use googletest::matchers::{anything, eq, ok};
    use googletest::{assert_that, gtest};

    use super::MetricBackend;

    #[gtest]
    fn display_console_collector_value() {
        assert_that!(MetricBackend::Console.as_str(), eq("console"));
    }

    #[cfg(feature = "prometheus")]
    #[gtest]
    fn display_prometheus_collector_value() {
        assert_that!(MetricBackend::Prometheus.as_str(), eq("prometheus"));
    }

    #[cfg(feature = "prometheus")]
    #[gtest]
    fn parse_valid_prometheus_collector_from_string() {
        let result: Result<MetricBackend, _> = "prometheus".parse();
        assert_that!(result, ok(anything()));
    }

    #[gtest]
    fn parse_valid_console_collector_from_string() {
        let result: Result<MetricBackend, _> = "console".parse();
        assert_that!(result, ok(anything()));
    }
}
