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
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub enum ExporterConfig {
    /// This is Jaeger's configuration data
    #[cfg_attr(feature = "serde", serde(rename(deserialize = "jaeger")))]
    Jaeger {
        /// Connection endpoint
        endpoint: Url,
    },
    /// This is the configuration data for honeycomb.io
    #[cfg_attr(feature = "serde", serde(rename(deserialize = "honeycomb")))]
    Honeycomb {
        /// Connection endpoint
        endpoint: Url,
        /// API Key
        api_key: SecretString,
    },
}

#[cfg(test)]
mod tests {
    use googletest::matchers::{anything, eq, err, ok};
    use googletest::{assert_that, gtest};
    use proptest::proptest;
    use proptest::strategy::Strategy;
    use rstest::rstest;

    use super::Collector;

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
}
