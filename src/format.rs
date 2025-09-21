//! This module defines the trace formatting styles

use std::fmt;
use std::str::FromStr;

/// [`EventFormat`] indicates the event formatter that should be used.
#[non_exhaustive]
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
pub enum EventFormat {
    /// Compact traces
    #[default]
    Compact,
    /// Emits full verbose traces
    Full,
    /// Prettier traces
    Pretty,
}

impl fmt::Display for EventFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Compact => "compact",
            Self::Full => "full",
            Self::Pretty => "pretty",
        };
        write!(f, "{value}")
    }
}

impl FromStr for EventFormat {
    type Err = String;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trace_fmt = match value {
            "compact" => Self::Compact,
            "full" => Self::Full,
            "pretty" => Self::Pretty,
            _ => return Err("invalid trace format".into()),
        };
        Ok(trace_fmt)
    }
}

#[cfg(test)]
mod tests {
    use googletest::matchers::{anything, eq, err, ok};
    use googletest::{assert_that, gtest};
    use proptest::proptest;
    use proptest::strategy::Strategy;
    use rstest::rstest;

    use super::EventFormat;

    #[rstest]
    #[case(EventFormat::Compact, "compact")]
    #[case(EventFormat::Full, "full")]
    #[case(EventFormat::Pretty, "pretty")]
    fn display_correct_trace_format(#[case] trace_format: EventFormat, #[case] display: &str) {
        assert_that!(trace_format.to_string(), eq(display))
    }

    proptest! {
        #[test]
        fn parse_valid_trace_format_successfully(fmt in "compact|full|pretty") {
            let result: Result<EventFormat,_> = fmt.parse();
            assert_that!(result, ok(anything()))
        }
    }

    proptest! {
        #[gtest]
        fn parsing_invalid_trace_format_fails(
            fmt in "[a-zA-Z]"
            .prop_filter("Values must not be in enumerated values",
                |fmt| !["compact", "full", "pretty"].contains(&fmt.as_str()))) {
                let result: Result<EventFormat, _> = fmt.parse();
                assert_that!(result, err(anything()))
        }
    }
}
