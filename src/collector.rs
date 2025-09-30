//! This module defines the abstractions for setting OpenTelemetry collectors.

use std::fmt;
use std::str::FromStr;
use std::time::Duration;

use bon::Builder;
use opentelemetry_otlp::tonic_types::metadata::MetadataMap;
use opentelemetry_otlp::{SpanExporter, WithExportConfig, WithTonicConfig};
use secrecy::{ExposeSecret, SecretString};
use url::Url;

use crate::Error;

/// This type enumerates the telemetry exporters
#[non_exhaustive]
#[derive(Clone, Debug, Default)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(rename_all(deserialize = "lowercase"))
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
pub enum Collector {
    /// Export traces to `std::io::stdout`
    /// This variant is only suitable for development and debugging
    #[default]
    Console,
    /// Send telemetry to honeycomb.io
    Honeycomb,
    /// Send telemetry to Jaeger,
    Jaeger,
}

impl fmt::Display for Collector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Console => "console".fmt(f),
            Self::Honeycomb => "honeycomb".fmt(f),
            Self::Jaeger => "jaeger".fmt(f),
        }
    }
}

impl FromStr for Collector {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let this = match value {
            "console" => Self::Console,
            "honeycomb" => Self::Honeycomb,
            "jaeger" => Self::Jaeger,
            _ => return Err(Error::UnsupportedCollectorKind),
        };
        Ok(this)
    }
}

/// Collector configuration data
#[non_exhaustive]
#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub enum CollectorConfig {
    /// This is the default configuration representing `std::io::stdout`
    #[default]
    Console,
    /// This is Jaeger's configuration data
    #[cfg_attr(feature = "serde", serde(rename(deserialize = "jaeger")))]
    Jaeger(JaegerConfig),
    /// This is the configuration data for honeycomb.io
    #[cfg_attr(feature = "serde", serde(rename(deserialize = "honeycomb")))]
    Honeycomb(HoneycombConfig),
}

impl CollectorConfig {
    /// Convert the `CollectorConfig` to an `Option<HoneycombConfig>`
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::time::Duration;
    /// # use owiwi_tracing_opentelemetry::collector::{CollectorConfig, HoneycombConfig};
    /// let honey_config = HoneycombConfig{endpoint: "https://honeycom.io".parse()?, api_key: "".into(), timeout: Duration::from_millis(0)};
    /// let exporter_config = CollectorConfig::Honeycomb(honey_config.clone());
    /// assert!(exporter_config.honeycomb().is_some_and(|config| config.endpoint == honey_config.endpoint));
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn honeycomb(self) -> Option<HoneycombConfig> {
        match self {
            Self::Honeycomb(config) => Some(config),
            _ => None,
        }
    }

    /// Convert the `CollectorConfig` to an `Option<JaegerConfig>`
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::time::Duration;
    /// # use owiwi_tracing_opentelemetry::collector::{CollectorConfig, JaegerConfig};
    /// let jaeger_config = JaegerConfig{endpoint: "http://localhost:4317".parse()?, timeout: Duration::from_millis(0)};
    /// let exporter_config = CollectorConfig::Jaeger(jaeger_config.clone());
    /// assert!(exporter_config.jaeger().is_some_and(|config| config.endpoint == jaeger_config.endpoint));
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn jaeger(self) -> Option<JaegerConfig> {
        match self {
            Self::Jaeger(config) => Some(config),
            _ => None,
        }
    }
}

/// This is the configuration data for honeycomb.io
#[derive(Debug, Clone, Builder)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct HoneycombConfig {
    /// Connection endpoint
    pub endpoint: Url,
    /// API Key
    pub api_key: SecretString,
    /// Set export timeout duration
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "humantime_serde::deserialize")
    )]
    pub timeout: Duration,
}

impl TryFrom<HoneycombConfig> for SpanExporter {
    type Error = Error;

    fn try_from(config: HoneycombConfig) -> crate::Result<Self> {
        let mut metadata = MetadataMap::with_capacity(1);
        metadata.insert(
            "x-honeycomb-team",
            config.api_key.expose_secret().try_into()?,
        );
        let exporter = SpanExporter::builder()
            .with_tonic()
            .with_endpoint(config.endpoint.as_ref())
            .with_timeout(config.timeout)
            .with_metadata(metadata)
            .with_tls_config(
                opentelemetry_otlp::tonic_types::transport::ClientTlsConfig::default()
                    .with_enabled_roots(),
            )
            .build()?;
        Ok(exporter)
    }
}

/// This is the configuration data for Jaeger
#[derive(Debug, Clone, Builder)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct JaegerConfig {
    /// Connection endpoint
    pub endpoint: Url,
    /// Set export timeout duration
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "humantime_serde::deserialize")
    )]
    pub timeout: Duration,
}

impl TryFrom<JaegerConfig> for SpanExporter {
    type Error = Error;

    fn try_from(config: JaegerConfig) -> crate::Result<Self> {
        let metadata = MetadataMap::default();
        let mut builder = SpanExporter::builder()
            .with_tonic()
            .with_endpoint(config.endpoint.as_ref())
            .with_timeout(config.timeout)
            .with_metadata(metadata);

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
    use std::time::Duration;

    use googletest::matchers::{anything, eq, err, ok};
    use googletest::{assert_that, gtest};
    use proptest::proptest;
    use proptest::strategy::Strategy;
    use rstest::rstest;

    use super::{Collector, CollectorConfig, HoneycombConfig, JaegerConfig};

    #[gtest]
    #[rstest]
    #[case(Collector::Console, "console")]
    #[case(Collector::Honeycomb, "honeycomb")]
    #[case(Collector::Jaeger, "jaeger")]
    fn display_correct_collector_value(#[case] collector: Collector, #[case] display: &str) {
        assert_that!(collector.to_string(), eq(display));
    }

    proptest! {
        #[gtest]
        fn parse_valid_collector_from_string_successfully(value in "console|honeycomb|jaeger") {
            let result: Result<Collector,_> = value.parse();
            assert_that!(result,ok(anything()));
        }

        #[gtest]
        fn parsing_invalid_collector_from_string_fails(
            value in "[a-zA-Z]*"
                .prop_filter("Value must be a valid variant",
                    |v| !["console", "honeycomb", "jaeger"].contains(&v.as_str()))) {
            let result: Result<Collector,_> = value.parse();
            assert_that!(result,err(anything()));
        }
    }

    #[test]
    fn get_honeycomb_config() {
        let honey_config = HoneycombConfig {
            endpoint: "https://api.honeycom.io/api/traces"
                .parse()
                .expect("it's a well formatter URL"),
            api_key: "".into(),
            timeout: Duration::from_millis(1),
        };
        let exporter_config = CollectorConfig::Honeycomb(honey_config.clone());
        assert!(
            exporter_config
                .honeycomb()
                .is_some_and(|config| config.endpoint == honey_config.endpoint)
        );
    }

    #[test]
    fn get_jaeger_config() {
        let jaeger_config = JaegerConfig::builder()
            .endpoint(
                "http://127.0.0.1:4317"
                    .parse()
                    .expect("it's a well formatter URL"),
            )
            .timeout(Duration::from_millis(1))
            .build();
        let exporter_config = CollectorConfig::Jaeger(jaeger_config.clone());
        assert!(
            exporter_config
                .jaeger()
                .is_some_and(|config| config.endpoint == jaeger_config.endpoint)
        );
    }

    #[test]
    fn console_does_not_have_config() {
        let exporter_config = CollectorConfig::Console;
        assert!(exporter_config.clone().honeycomb().is_none());
        assert!(exporter_config.jaeger().is_none());
    }
}
