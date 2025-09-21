//! This module define the instrumentation type.

#[cfg(feature = "clap")]
use clap::Args;
#[cfg(feature = "clap")]
use clap_verbosity_flag::Verbosity;
use tracing::Subscriber;
use tracing_subscriber::filter::Directive;
use tracing_subscriber::layer::Layer;
use tracing_subscriber::registry::LookupSpan;

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
    pub event_format: EventFormat,

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

impl Owiwi {
    impl_fmt_layer::define_layer!("Creates a compact event formatted tracing layer" => fmt_layer_compact => compact);
    impl_fmt_layer::define_layer!("Creates a full tracing formatting layer" => fmt_layer_full => full);
    impl_fmt_layer::define_layer!("Creates a pretty printed event formatting layer" => fmt_layer_pretty => pretty);
}
///  Formatting layer module
mod impl_fmt_layer {
    /// Defines a new formatting layer method.
    macro_rules! define_layer {
        ($doc:expr => $method:ident => $format: ident) => {
            #[doc=$doc]
            pub fn $method<S>(&self) -> impl Layer<S>
            where
                S: Subscriber + for<'span> LookupSpan<'span>,
            {
                let format = self.event_format.$format();
                tracing_subscriber::fmt::layer().event_format(format)
            }
        };
    }

    pub(super) use define_layer;
}
