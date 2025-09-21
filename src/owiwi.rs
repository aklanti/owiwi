//! This module define the instrumentation type.

#[cfg(feature = "clap")]
use clap::Args;
#[cfg(feature = "clap")]
use clap_verbosity_flag::Verbosity;
use tracing_subscriber::filter::Directive;

use crate::format::EventFormat;

/// Instrumentation type.
#[must_use]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "clap", derive(Args))]
pub struct Owiwi {
    /// The event formatter to use
    #[cfg_attr(
        feature = "clap",
        arg(
            name = "trace-format",
            long,
            value_enum,
            default_value_t = Default::default(),
            help_heading = "Instrumentation options",
        )
    )]
    pub event_formatter: EventFormat,

    #[cfg(feature = "clap")]
    #[command(flatten)]
    pub verbose: Verbosity,

    /// Tracing filter directives
    ///
    /// Use this value to override the default value, and the `RUST_LOG` environment variable.
    #[cfg_attr(
        feature = "clap",
        arg(
            long = "trace-directive",
            global = true,
            value_delimiter = ',',
            num_args = 0..,
            help_heading = "Instrumentation options",
        )
    )]
    pub tracing_directives: Vec<Directive>,
}
