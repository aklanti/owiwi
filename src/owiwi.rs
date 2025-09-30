//! This module define the instrumentation type.

use std::env::VarError;
use std::error::Error as _;

use opentelemetry::trace::TracerProvider as _;
use opentelemetry_sdk::trace::SdkTracerProvider;
use tracing::Subscriber;
use tracing_error::ErrorLayer;
use tracing_subscriber::filter::{Directive, EnvFilter};
use tracing_subscriber::layer::{Layer, SubscriberExt as _};
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::util::SubscriberInitExt as _;

#[cfg(feature = "clap")]
use super::HELP_HEADING;
use super::error::Error;
use super::format::EventFormat;
use super::provider::{self, TracerProviderOptions};
#[cfg(feature = "clap")]
use clap_verbosity_flag::Verbosity;

use crate::collector::CollectorConfig;

/// Instrumentation type.
#[must_use]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "clap", derive(clap::Args))]
pub struct Owiwi {
    /// The event formatter to use
    #[cfg_attr(
        feature = "clap",
        arg(
            name = "trace-format",
            long,
            value_enum,
            default_value_t = Default::default(),
            help_heading = HELP_HEADING,
        )
    )]
    pub event_format: EventFormat,

    #[expect(missing_docs, reason = "is flatten command")]
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
            value_delimiter = ',',
            num_args = 0..,
            help_heading = HELP_HEADING,
        )
    )]
    pub tracing_directives: Vec<Directive>,

    /// Tracer provider configuration options
    #[cfg_attr(feature = "clap", command(flatten))]
    pub tracer_provider_options: TracerProviderOptions,
}

impl Default for Owiwi {
    fn default() -> Self {
        Self::new()
    }
}

impl Owiwi {
    /// Creates new subscriber
    #[cfg(not(feature = "clap"))]
    pub fn new() -> Self {
        Self {
            event_format: EventFormat::default(),
            tracing_directives: Vec::new(),
            tracer_provider_options: TracerProviderOptions::default(),
        }
    }

    /// Creates new subscriber
    #[cfg(feature = "clap")]
    pub fn new() -> Self {
        Self {
            event_format: EventFormat::default(),
            tracing_directives: Vec::new(),
            tracer_provider_options: TracerProviderOptions::default(),
            verbose: Verbosity::default(),
        }
    }

    /// Initializes the tracer
    pub fn init(
        &self,
        service_name: &'static str,
        collector_config: CollectorConfig,
    ) -> Result<OwiwiGuard, Error> {
        let filter_layer = self.filter_layer()?;
        let resource = provider::init_resource(service_name);
        let tracer_provider = self
            .tracer_provider_options
            .init_provider(collector_config, resource)?;
        let tracer = tracer_provider.tracer(service_name);
        let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);
        let registry = tracing_subscriber::registry()
            .with(otel_layer)
            .with(ErrorLayer::default())
            .with(filter_layer);
        match self.event_format {
            EventFormat::Compact => registry.with(self.fmt_layer_compact()).try_init()?,
            EventFormat::Full => registry.with(self.fmt_layer_full()).try_init()?,
            EventFormat::Pretty => registry.with(self.fmt_layer_pretty()).try_init()?,
        }
        Ok(OwiwiGuard { tracer_provider })
    }
    /// Creates a the filter layer
    pub fn filter_layer(&self) -> Result<EnvFilter, Error> {
        let mut layer = match EnvFilter::try_from_default_env() {
            Ok(layer) => layer,
            Err(err) => {
                if let Some(source) = err.source() {
                    match source.downcast_ref::<VarError>() {
                        Some(VarError::NotPresent) => (),
                        Some(err) => {
                            tracing::error!("{err:?}");
                            return Err(Error::ParseDirectiveError {
                                source: err.clone(),
                            });
                        }
                        None => unreachable!(),
                    }
                }
                if self.tracing_directives.is_empty() {
                    #[cfg(feature = "clap")]
                    let level = self
                        .verbose
                        .tracing_level()
                        .ok_or_else(|| Error::TraceLevelMissing)?;

                    #[cfg(not(feature = "clap"))]
                    let level = tracing::Level::INFO;

                    EnvFilter::try_new(format!(
                        "{}={}",
                        env!("CARGO_PKG_NAME").replace('-', "_"),
                        level.as_str()
                    ))?
                } else {
                    EnvFilter::try_new("")?
                }
            }
        };

        for directive in &self.tracing_directives {
            layer = layer.add_directive(directive.clone());
        }
        Ok(layer)
    }

    impl_fmt_layer::define_layer!("Creates a compact event formatted tracing layer" => fmt_layer_compact => compact);
    impl_fmt_layer::define_layer!("Creates a full tracing formatting layer" => fmt_layer_full => full);
    impl_fmt_layer::define_layer!("Creates a pretty printed event formatting layer" => fmt_layer_pretty => pretty);
}

///  Formatting layer module
mod impl_fmt_layer {
    /// Defines a new formatting layer method.
    macro_rules! define_layer {
        ($doc:expr => $func:ident => $format: ident) => {
            #[doc=$doc]
            pub fn $func<S>(&self) -> impl Layer<S>
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

/// Owiwi guard
#[derive(Debug)]
pub struct OwiwiGuard {
    /// SDK tracer provider
    tracer_provider: SdkTracerProvider,
}

impl Drop for OwiwiGuard {
    fn drop(&mut self) {
        if let Err(err) = self.tracer_provider.shutdown() {
            tracing::error!("failed to shutdown tracer provider {err}");
        }
    }
}
