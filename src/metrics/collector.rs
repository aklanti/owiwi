//! This module defines the metrics collector abstraction.

use std::fmt;
use std::str::FromStr;

use crate::error::Error;

/// This type enumerates the metric collectors
#[non_exhaustive]
#[derive(Debug, Default)]
pub enum MetricCollector {
    /// Export metrics to `std::io::stdout`
    /// This variant is only suitable for development and debugging
    #[default]
    Console,
    /// Promethus metric exporter
    #[cfg(feature = "prometheus")]
    Prometheus,
}

impl fmt::Display for MetricCollector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Console => "console".fmt(f),
            #[cfg(feature = "prometheus")]
            Self::Prometheus => "prometheus".fmt(f),
        }
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
