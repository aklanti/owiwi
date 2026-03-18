//! Trace export backends.

use std::fmt;
use std::str::FromStr;

use crate::error::{Error, ErrorKind};

/// Supported trace export backends
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(rename_all(deserialize = "lowercase"))
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
pub enum TraceBackend {
    /// Writes spans to [`std::io::stdout`]
    #[default]
    Console,
    /// Exports spans to [Honeycomb](https://honeycomb.io)
    Honeycomb,
    /// Any OTLP-compatible backend
    Otlp,
}

impl fmt::Display for TraceBackend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl TraceBackend {
    /// Returns a `&str` value of `self`
    #[must_use]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Console => "console",
            Self::Honeycomb => "honeycomb",
            Self::Otlp => "otlp",
        }
    }
}

impl FromStr for TraceBackend {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let this = match value {
            "console" => Self::Console,
            "honeycomb" => Self::Honeycomb,
            "otlp" => Self::Otlp,
            _ => return Err(ErrorKind::TraceBackend(value.to_owned()).into()),
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

    use super::*;

    #[gtest]
    #[rstest]
    #[case(TraceBackend::Console, "console")]
    #[case(TraceBackend::Honeycomb, "honeycomb")]
    #[case(TraceBackend::Otlp, "otlp")]
    fn display_correct_collector_value(#[case] collector: TraceBackend, #[case] display: &str) {
        assert_that!(collector.to_string(), eq(display));
    }

    proptest! {
        #[gtest]
        fn parse_valid_collector_from_string_successfully(value in "console|honeycomb|otlp") {
            let result: Result<TraceBackend,_> = value.parse();
            assert_that!(result,ok(anything()));
        }

        #[gtest]
        fn parsing_invalid_collector_from_string_fails(
            value in "[a-zA-Z]*"
                .prop_filter("Value must be a valid variant",
                    |v| !["console", "honeycomb", "otlp"].contains(&v.as_str()))) {
            let result: Result<TraceBackend,_> = value.parse();
            assert_that!(result,err(anything()));
        }
    }
}
