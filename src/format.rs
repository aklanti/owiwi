//! Trace output formatting styles.

use std::fmt;
use std::io::{self, IsTerminal};
use std::str::FromStr;

use tracing_subscriber::fmt::format::{Compact, Format, Full, Pretty};
use tracing_subscriber::fmt::time::SystemTime;

/// Trace event output format.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
pub enum EventFormat {
    /// Compact, single-line output.
    Compact,
    /// Full verbose output with timestamps.
    #[default]
    Full,
    /// Multi-line, indented output.
    Pretty,
}

impl EventFormat {
    /// Returns a compact formatter with one event per line.
    #[must_use]
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

    /// Returns a full verbose formatter with timestamps.
    #[must_use]
    pub fn full(&self) -> Format<Full, SystemTime> {
        Format::default().with_ansi(io::stderr().is_terminal())
    }

    /// Returns a multi-line, indented formatter for local development.
    #[must_use]
    pub fn pretty(&self) -> Format<Pretty, SystemTime> {
        self.full().pretty()
    }
}

impl EventFormat {
    /// String literals for each variant.
    const LITERALS: &[&str] = &["compact", "full", "pretty"];

    /// Returns the string representation of this format.
    #[must_use]
    pub const fn as_str(&self) -> &str {
        Self::LITERALS[*self as usize]
    }
}

impl fmt::Display for EventFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
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
        assert_that!(event_format.to_string(), eq(display));
    }

    proptest! {
        #[test]
        fn parse_valid_event_format_successfully(fmt in "compact|full|pretty") {
            let result: Result<EventFormat,_> = fmt.parse();
            assert_that!(result, ok(anything()));
        }

        #[gtest]
        fn parsing_invalid_event_format_fails(
            fmt in "[a-zA-Z]*"
            .prop_filter("Values must not be in enumerated values",
                |fmt| !["compact", "full", "pretty"].contains(&fmt.as_str()))) {
                let result: Result<EventFormat, _> = fmt.parse();
                assert_that!(result, err(anything()));
        }
    }
}
