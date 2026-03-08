//! This module defines the abstractions for setting OpenTelemetry collectors.

use std::fmt;
use std::str::FromStr;

use super::HoneycombConfig;
use super::JaegerConfig;
use crate::Error;

/// This type enumerates the telemetry exporters
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(rename_all(deserialize = "lowercase"))
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
pub enum TraceExporter {
    /// Export traces to `std::io::stdout`
    /// This variant is only suitable for development and debugging
    #[default]
    Console,
    /// Send telemetry to honeycomb.io
    Honeycomb,
    /// Send telemetry to Jaeger,
    Jaeger,
}

impl fmt::Display for TraceExporter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl TraceExporter {
    /// Returns a `&str` value of `self`
    #[must_use]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Console => "console",
            Self::Honeycomb => "honeycomb",
            Self::Jaeger => "jaeger",
        }
    }
}

impl FromStr for TraceExporter {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let this = match value {
            "console" => Self::Console,
            "honeycomb" => Self::Honeycomb,
            "jaeger" => Self::Jaeger,
            _ => return Err(Error::UnsupportedTracesCollector(value.to_owned())),
        };
        Ok(this)
    }
}

/// Traces collector configuration data
#[non_exhaustive]
#[derive(Debug, Default, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(rename_all(deserialize = "lowercase"))
)]
pub enum TraceExporterConfig {
    /// This is the default configuration representing `std::io::stdout`
    #[default]
    Console,
    /// This is Jaeger's configuration data
    Jaeger(JaegerConfig),
    /// This is the configuration data for honeycomb.io
    Honeycomb(HoneycombConfig),
}

impl TraceExporterConfig {
    /// Convert the `TraceExporterConfig` to an `Option<HoneycombConfig>`
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::time::Duration;
    /// # use owiwi_tracing_opentelemetry::trace::TraceExporterConfig;
    /// # use owiwi_tracing_opentelemetry::trace::HoneycombConfig;
    /// let honey_config = HoneycombConfig{
    ///     endpoint: "https://honeycom.io".parse()?,
    ///     api_key: "".into(),
    ///     timeout: Duration::from_millis(0)
    /// };
    /// let exporter_config = TraceExporterConfig::Honeycomb(honey_config.clone());
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

    /// Convert the `TraceExporterConfig` to an `Option<JaegerConfig>`
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::time::Duration;
    /// # use owiwi_tracing_opentelemetry::trace::TraceExporterConfig;
    /// # use owiwi_tracing_opentelemetry::trace::JaegerConfig;
    /// let jaeger_config = JaegerConfig{
    ///     endpoint: "http://localhost:4317".parse()?,
    ///     timeout: Duration::from_millis(0)
    /// };
    /// let exporter_config = TraceExporterConfig::Jaeger(jaeger_config.clone());
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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use googletest::matchers::{anything, eq, err, ok};
    use googletest::{assert_that, gtest};
    use proptest::proptest;
    use proptest::strategy::Strategy;
    use rstest::rstest;

    use super::*;

    #[gtest]
    #[rstest]
    #[case(TraceExporter::Console, "console")]
    #[case(TraceExporter::Honeycomb, "honeycomb")]
    #[case(TraceExporter::Jaeger, "jaeger")]
    fn display_correct_collector_value(#[case] collector: TraceExporter, #[case] display: &str) {
        assert_that!(collector.to_string(), eq(display));
    }

    proptest! {
        #[gtest]
        fn parse_valid_collector_from_string_successfully(value in "console|honeycomb|jaeger") {
            let result: Result<TraceExporter,_> = value.parse();
            assert_that!(result,ok(anything()));
        }

        #[gtest]
        fn parsing_invalid_collector_from_string_fails(
            value in "[a-zA-Z]*"
                .prop_filter("Value must be a valid variant",
                    |v| !["console", "honeycomb", "jaeger"].contains(&v.as_str()))) {
            let result: Result<TraceExporter,_> = value.parse();
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
        let exporter_config = TraceExporterConfig::Honeycomb(honey_config.clone());
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
        let exporter_config = TraceExporterConfig::Jaeger(jaeger_config.clone());
        assert!(
            exporter_config
                .jaeger()
                .is_some_and(|config| config.endpoint == jaeger_config.endpoint)
        );
    }

    #[test]
    fn console_does_not_have_config() {
        let exporter_config = TraceExporterConfig::Console;
        assert!(exporter_config.clone().honeycomb().is_none());
        assert!(exporter_config.jaeger().is_none());
    }
}
