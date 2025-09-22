//! This module defines the trace formatting styles

use std::fmt;
use std::io::{self, IsTerminal};
use std::str::FromStr;

use tracing_subscriber::fmt::format::{Compact, Format, Full, Pretty};
use tracing_subscriber::fmt::time::SystemTime;

/// [`EventFormat`] indicates the event formatter that should be used.
#[non_exhaustive]
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
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

impl EventFormat {
    /// Use a less verbose compacted out format
    pub fn compact(&self) -> Format<Compact, ()> {
        self.full()
            .compact()
            .with_target(false)
            .with_thread_ids(false)
            .with_thread_names(false)
            .with_file(false)
            .with_line_number(false)
            .without_time()
    }

    /// Use a full formatter trace output
    pub fn full(&self) -> Format<Full, SystemTime> {
        Format::default().with_ansi(io::stderr().is_terminal())
    }

    /// Pretty format event output
    pub fn pretty(&self) -> Format<Pretty, SystemTime> {
        self.full().pretty()
    }
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
    fn display_correct_trace_format(#[case] event_format: EventFormat, #[case] display: &str) {
        assert_that!(event_format.to_string(), eq(display))
    }

    proptest! {
        #[test]
        fn parse_valid_event_format_successfully(fmt in "compact|full|pretty") {
            let result: Result<EventFormat,_> = fmt.parse();
            assert_that!(result, ok(anything()))
        }

        #[gtest]
        fn parsing_invalid_event_format_fails(
            fmt in "[a-zA-Z]*"
            .prop_filter("Values must not be in enumerated values",
                |fmt| !["compact", "full", "pretty"].contains(&fmt.as_str()))) {
                let result: Result<EventFormat, _> = fmt.parse();
                assert_that!(result, err(anything()))
        }
    }
}
