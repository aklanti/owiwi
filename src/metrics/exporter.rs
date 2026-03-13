//! This module defines the metrics collector abstraction.

use crate::error::{Error, ErrorKind};
use std::fmt;
use std::str::FromStr;

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
            _ => return Err(ErrorKind::MetricBackend(value.to_owned()).into()),
        };

        Ok(this)
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
