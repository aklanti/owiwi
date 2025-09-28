//! This module defines the abstractions for setting OpenTelemetry collectors.

use std::fmt;
use std::str::FromStr;

use secrecy::SecretString;
use url::Url;

/// This type enumerates the telemetry exporters
#[non_exhaustive]
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
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
        let value = match self {
            Self::Console => "console",
            Self::Honeycomb => "honeycomb",
            Self::Jaeger => "jaeger",
        };

        write!(f, "{value}")
    }
}

impl FromStr for Collector {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let this = match value {
            "console" => Self::Console,
            "honeycomb" => Self::Honeycomb,
            "jaeger" => Self::Jaeger,
            _ => return Err(format!("invalid collector value {value}")),
        };
        Ok(this)
    }
}

/// Collector configuration data
#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub enum ExporterConfig {
    /// This is the default configuration representing `std::io::stdout`
    #[default]
    Console,
    /// This is Jaeger's configuration data
    #[cfg_attr(feature = "serde", serde(rename(deserialize = "jaeger")))]
    Jaeger(JaegerExporter),
    /// This is the configuration data for honeycomb.io
    #[cfg_attr(feature = "serde", serde(rename(deserialize = "honeycomb")))]
    Honeycomb(HoneycombExporter),
}

impl ExporterConfig {
    /// Convert the `ExporterConfig` to an `Option<HoneycombExporter>`
    ///
    /// # Examples
    ///
    /// ```
    /// # use owiwi::collector::{ExporterConfig, HoneycombExporter};
    /// let honey_config = HoneycombExporter {endpoint: "https://honeycom.io".parse()?, api_key: "".into()};
    /// let exporter_config = ExporterConfig::Honeycomb(honey_config.clone());
    /// assert!(exporter_config.honeycomb().is_some_and(|config| config.endpoint == honey_config.endpoint));
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn honeycomb(self) -> Option<HoneycombExporter> {
        match self {
            Self::Honeycomb(config) => Some(config),
            _ => None,
        }
    }

    /// Convert the `ExporterConfig` to an `Option<JaegerExporter>`
    ///
    /// # Examples
    ///
    /// ```
    /// # use owiwi::collector::{ExporterConfig, JaegerExporter};
    /// let jaeger_config = JaegerExporter{endpoint: "http://localhost:4317".parse()?};
    /// let exporter_config = ExporterConfig::Jaeger(jaeger_config.clone());
    /// assert!(exporter_config.jaeger().is_some_and(|config| config.endpoint == jaeger_config.endpoint));
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn jaeger(self) -> Option<JaegerExporter> {
        match self {
            Self::Jaeger(config) => Some(config),
            _ => None,
        }
    }
}

/// This is the configuration data for honeycomb.io
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct HoneycombExporter {
    /// Connection endpoint
    pub endpoint: Url,
    /// API Key
    pub api_key: SecretString,
}

/// This is the configuration data for Jaeger
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct JaegerExporter {
    /// Connection endpoint
    pub endpoint: Url,
}

#[cfg(test)]
mod tests {
    use googletest::matchers::{anything, eq, err, ok};
    use googletest::{assert_that, gtest};
    use proptest::proptest;
    use proptest::strategy::Strategy;
    use rstest::rstest;

    use super::{Collector, ExporterConfig, HoneycombExporter, JaegerExporter};

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
        let honey_config = HoneycombExporter {
            endpoint: "https://honeycom.io"
                .parse()
                .expect("it's a well formatter URL"),
            api_key: "".into(),
        };
        let exporter_config = ExporterConfig::Honeycomb(honey_config.clone());
        assert!(
            exporter_config
                .honeycomb()
                .is_some_and(|config| config.endpoint == honey_config.endpoint)
        );
    }

    #[test]
    fn get_jaeger_config() {
        let jaeger_config = JaegerExporter {
            endpoint: "https://127.0.0.1:4317"
                .parse()
                .expect("it's a well formatter URL"),
        };
        let exporter_config = ExporterConfig::Jaeger(jaeger_config.clone());
        assert!(
            exporter_config
                .jaeger()
                .is_some_and(|config| config.endpoint == jaeger_config.endpoint)
        );
    }

    #[test]
    fn console_does_not_have_config() {
        let exporter_config = ExporterConfig::Console;
        assert!(exporter_config.clone().honeycomb().is_none());
        assert!(exporter_config.jaeger().is_none());
    }
}
