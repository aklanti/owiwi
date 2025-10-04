//! This module defines the metrics collector abstraction.

use std::fmt;
use std::str::FromStr;
use std::time::Duration;

use bon::Builder;
#[cfg(feature = "metrics")]
use metrics_exporter_prometheus::PrometheusBuilder;
use metrics_util::MetricKindMask;
#[cfg(feature = "otel-metrics")]
use opentelemetry_otlp::{MetricExporter, WithExportConfig, WithTonicConfig};
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

/// Metrics collector configuration
#[non_exhaustive]
#[derive(Debug, Default, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(rename_all(deserialize = "lowercase"))
)]
pub enum MetricCollectorConfig {
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
    pub host: String,
    /// Connection port
    pub port: u16,
    /// Set export timeout duration
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "humantime_serde::deserialize")
    )]
    pub timeout: Option<Duration>,
}

impl PrometheusConfig {
    /// Returns the Prometheus instance URL
    pub fn endpoint(&self) -> Result<Url, Error> {
        format!("{}:{}", self.host, self.port)
            .parse()
            .map_err(Error::ParseUrlError)
    }

    #[cfg(feature = "metrics")]
    /// Initializes Prometheus metrics exporter
    pub fn init_metrics(&self, mask: MetricKindMask) -> Result<(), Error> {
        PrometheusBuilder::new()
            .idle_timeout(mask, self.timeout)
            .install()?;
        Ok(())
    }
}

#[cfg(feature = "otel-metrics")]
impl TryFrom<PrometheusConfig> for MetricExporter {
    type Error = Error;

    fn try_from(config: PrometheusConfig) -> Result<Self, Self::Error> {
        let endpoint = config.endpoint()?;
        let mut builder = Self::builder()
            .with_tonic()
            .with_endpoint(endpoint.as_ref());
        if let Some(timeout) = config.timeout {
            builder = builder.with_timeout(timeout);
        }

        if endpoint.scheme() == "https" {
            builder = builder.with_tls_config(
                opentelemetry_otlp::tonic_types::transport::ClientTlsConfig::default()
                    .with_enabled_roots(),
            );
        }

        let exporter = builder.build()?;
        Ok(exporter)
    }
}

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

    /// Set the kind of metrics
    #[cfg(feature = "metrics")]
    #[cfg_attr(
        feature = "clap",
        arg(
            name="metrics-kind",
            long,
            value_delimiter = ',',
            num_args = 0..3,
            help_heading = HELP_HEADING,
        )
    )]
    pub kinds: Vec<MetricKind>,
}

impl MetricOptions {
    #[cfg(feature = "metrics")]
    /// Returns the IDLE timeout metric kind mask
    #[must_use]
    fn metric_kind_mask(&self) -> MetricKindMask {
        self.kinds
            .iter()
            .map(MetricKindMask::from)
            .reduce(|acc, kind| acc | kind)
            .unwrap_or(MetricKindMask::ALL)
    }

    #[cfg(any(feature = "metrics", feature = "otel-metrics"))]
    /// Initializes metrics collector
    pub(crate) fn try_init(&self, config: MetricCollectorConfig) -> Result<(), Error> {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "metrics", feature = "prometheus"))]
            {
                let mask = self.metric_kind_mask();
                let MetricCollectorConfig::Prometheus(config) = config else {
                    unreachable!()
                };
                config.init_metrics(mask)?;
            } else if
            #[cfg(all(feature = "otel-metrics", feature = "prometheus"))]
            {
                todo!()
            }
        }
        Ok(())
    }
}

#[cfg(feature = "metrics")]
/// Enumerate the metric count.
///
/// This is in parity with [`metrics_util::MetricKind`]
#[must_use]
#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(rename_all(deserialize = "lowercase"))
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
pub enum MetricKind {
    /// All metric kinds
    #[default]
    All,
    /// The counter kind
    Counter,
    /// The gauge kind
    Guage,
    /// The histogram kind
    Histogram,
    /// No metric kinds
    None,
}

#[cfg(feature = "metrics")]
impl From<MetricKind> for MetricKindMask {
    fn from(kind: MetricKind) -> Self {
        Self::from(&kind)
    }
}

#[cfg(feature = "metrics")]
impl From<&MetricKind> for MetricKindMask {
    fn from(kind: &MetricKind) -> Self {
        match kind {
            MetricKind::All => Self::ALL,
            MetricKind::Counter => Self::COUNTER,
            MetricKind::Guage => Self::GAUGE,
            MetricKind::Histogram => Self::HISTOGRAM,
            MetricKind::None => Self::NONE,
        }
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
