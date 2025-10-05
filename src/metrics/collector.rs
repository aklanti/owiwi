//! This module defines the metrics collector abstraction.

use std::fmt;
use std::str::FromStr;
use std::time::Duration;

use bon::Builder;
use opentelemetry::global;
use opentelemetry::metrics::Meter;
#[cfg(feature = "prometheus")]
use opentelemetry_otlp::{MetricExporter, WithExportConfig, WithTonicConfig};
use opentelemetry_sdk::Resource;
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
pub enum MetricCollector {
    /// Export metrics to `std::io::stdout`
    /// This variant is only suitable for development and debugging
    #[default]
    Console,
    /// Promethus metric exporter
    #[cfg(feature = "prometheus")]
    Prometheus,
}

impl MetricCollector {
    /// A slice of string of the enum variants
    pub const LITERALS: &[&str] = &["console", "prometheus"];
    /// Returns a `&str` value of `self`
    #[must_use]
    pub const fn as_str(&self) -> &str {
        Self::LITERALS[*self as usize]
    }
}

impl fmt::Display for MetricCollector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl FromStr for MetricCollector {
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

/// Configuration data for metrics initialization using the opentelementry `metrics` crate
/// Metric collector configuration options
#[must_use]
#[derive(Clone, Debug, Default, Builder)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "clap", derive(clap::Args))]
pub struct MetricOptions {
    /// Set the metric collector
    #[cfg_attr(
        feature = "clap",
        arg(
            name="metrics-collector",
            long,
            help_heading = HELP_HEADING,
        ),
    )]
    pub collector: MetricCollector,

    /// Metrics update time interval
    /// Set the metric collector
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

impl MetricOptions {
    /// Initializes metrics collector
    pub fn try_init(
        &self,
        service_name: &'static str,
        resource: Resource,
        exporter_config: MetricsConfig,
    ) -> Result<Meter, Error> {
        let meter_provider = match exporter_config {
            MetricsConfig::Console => {
                let exporter = opentelemetry_stdout::MetricExporter::default();
                SdkMeterProvider::builder()
                    .with_resource(resource)
                    .with_periodic_exporter(exporter)
                    .build()
            }
            #[cfg(feature = "prometheus")]
            MetricsConfig::Prometheus(config) => {
                use opentelemetry_sdk::metrics::PeriodicReader;
                let exporter = MetricExporter::try_from(config)?;
                let mut builder = PeriodicReader::builder(exporter);
                if let Some(interval) = self.interval {
                    builder = builder.with_interval(interval);
                }
                let reader = builder.build();
                SdkMeterProvider::builder()
                    .with_resource(resource)
                    .with_reader(reader)
                    .build()
            }
        };

        global::set_meter_provider(meter_provider);
        let meter = global::meter(service_name);
        Ok(meter)
    }
}

/// Metrics collector configuration
#[non_exhaustive]
#[derive(Debug, Default, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(rename_all(deserialize = "lowercase"))
)]
pub enum MetricsConfig {
    /// This is the default configuration representing `std::io::stdout`
    #[default]
    Console,
    #[cfg(feature = "prometheus")]
    /// This is Prometheus's configuration data
    Prometheus(PrometheusConfig),
}

/// This is the configuration data for Jaeger
#[derive(Debug, Clone, Builder)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct PrometheusConfig {
    /// Connection host,
    pub endpoint: Url,
    /// Set export timeout duration
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "humantime_serde::deserialize")
    )]
    /// Metrics update timeout
    pub timeout: Option<Duration>,
}

#[cfg(feature = "prometheus")]
impl TryFrom<PrometheusConfig> for opentelemetry_otlp::MetricExporter {
    type Error = Error;

    fn try_from(config: PrometheusConfig) -> Result<Self, Self::Error> {
        let mut builder = Self::builder()
            .with_tonic()
            .with_endpoint(config.endpoint.as_ref());
        if let Some(timeout) = config.timeout {
            builder = builder.with_timeout(timeout);
        }

        if config.endpoint.scheme() == "https" {
            builder = builder.with_tls_config(
                opentelemetry_otlp::tonic_types::transport::ClientTlsConfig::default()
                    .with_enabled_roots(),
            );
        }

        let exporter = builder.build()?;
        Ok(exporter)
    }
}

#[cfg(test)]
mod tests {
    use googletest::matchers::{anything, eq, err, ok};
    use googletest::{assert_that, gtest};
    use proptest::proptest;
    use proptest::strategy::Strategy;
    use rstest::rstest;

    use super::MetricCollector;

    #[gtest]
    #[rstest]
    #[case(MetricCollector::Console, "console")]
    #[case(MetricCollector::Prometheus, "prometheus")]
    fn display_correct_collector_value(#[case] collector: MetricCollector, #[case] display: &str) {
        assert_that!(collector.to_string(), eq(display));
    }

    proptest! {
        #[gtest]
        fn parse_valid_collector_from_string_successfully(value in "console|prometheus") {
            let result: Result<MetricCollector,_> = value.parse();
            assert_that!(result,ok(anything()));
        }

        #[gtest]
        fn parsing_invalid_collector_from_string_fails(
            value in "[a-zA-Z]*"
                .prop_filter("Value must be a valid variant",
                    |v| !["console", "prometheus"].contains(&v.as_str()))) {
            let result: Result<MetricCollector,_> = value.parse();
            assert_that!(result,err(anything()));
        }
    }
}
