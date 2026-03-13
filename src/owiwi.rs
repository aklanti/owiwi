//! This module defines the instrumentation type.

use std::env::VarError;
use std::error::Error as _;

#[cfg(feature = "clap")]
use clap_verbosity_flag::Verbosity;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::trace::SdkTracerProvider;
use tracing::Subscriber;
use tracing_error::ErrorLayer;
use tracing_subscriber::filter::{Directive, EnvFilter};
use tracing_subscriber::layer::{Layer, SubscriberExt as _};
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::util::SubscriberInitExt as _;

#[cfg(feature = "clap")]
use super::HELP_HEADING;
use super::OwiwiGuard;
use super::env_vars;
use super::error::{Error, ErrorKind};
use super::trace::TracerProviderOptions;
use crate::EventFormat;
use crate::OtlpConfig;

/// Default service name
const DEFAULT_SERVICE_NAME: &str = "unknown_service";

/// Instrumentation type.
#[must_use]
#[derive(Clone, Default, Debug)]
#[cfg_attr(feature = "clap", derive(clap::Args))]
pub struct Owiwi {
    /// Service name
    #[cfg_attr(
        feature = "clap",
        arg(
            name="otel-service-name",
            default_value = DEFAULT_SERVICE_NAME,
            env=env_vars::OTEL_SERVICE_NAME,
         )
    )]
    pub service_name: String,

    /// The event formatter to use
    #[cfg_attr(
        feature = "clap",
        arg(
            name = "event-format",
            long,
            value_enum,
            default_value_t = Default::default(),
            help_heading = HELP_HEADING,
        )
    )]
    pub event_format: EventFormat,

    /// Traces filter directives
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

    /// Meter provider configuration options
    #[cfg(feature = "metrics")]
    #[cfg_attr(feature = "clap", command(flatten))]
    pub meter_options: super::metrics::MeterProviderOptions,

    /// Resource attributes
    #[cfg_attr(
        feature = "clap",
        arg(
            name = "otel-resource-attributes",
            long,
            value_parser = env_vars::parse_key_values,
            env = env_vars::OTEL_RESOURCE_ATTRIBUTES,
            help_heading = HELP_HEADING,
        ))]
    pub resource_attrs: Vec<(String, String)>,

    /// Disables all telemetry
    #[cfg_attr(
        feature = "clap",
        arg(
            name = "otel-sdk-disabled",
            long,
            env=env_vars::OTEL_SDK_DISABLED
        )
    )]
    pub disable_sdk: bool,

    #[expect(missing_docs, reason = "is flatten command")]
    #[cfg(feature = "clap")]
    #[command(flatten)]
    pub verbose: Verbosity,
}

impl Owiwi {
    /// Creates new subscriber
    pub fn new() -> Self {
        Self::default()
    }

    /// Initializes the tracing and metrics providers with the given exporter configuration
    ///
    /// Sets up a [`tracing_subscriber`] registry with an OpenTelemetry layer,
    /// error layer, env filter, and the configured event formatter.
    ///
    /// Returns an [`OwiwiGuard`] that must be held for the lifetime of the program.
    pub fn try_init(
        mut self,
        config: impl Into<OtlpConfig>,
        #[cfg(feature = "metrics")] metrics_exporter: impl TryInto<
            opentelemetry_otlp::MetricExporter,
            Error = Error,
        >,
    ) -> Result<OwiwiGuard, Error> {
        if self.disable_sdk || is_disabled() {
            cfg_if::cfg_if! {
                if #[cfg(feature = "console")] {
                    return self.try_init_console();
                } else {
                    let filter_layer = self.filter_layer()?;
                    let fmt_layer = self.fmt_layer();
                    tracing_subscriber::registry().with(filter_layer).with(fmt_layer).try_init()?;
                    return Ok(OwiwiGuard::noop());
                }
            }
        }

        let resource = self.build_resource();

        #[cfg(feature = "metrics")]
        let meter_provider = self
            .meter_options
            .init_provider(resource.clone(), metrics_exporter)?;
        let tracer_provider = self
            .tracer_provider_options
            .init_provider(config, resource)?;

        self.finish(
            tracer_provider,
            #[cfg(feature = "metrics")]
            meter_provider,
        )
    }

    #[cfg(feature = "console")]
    /// Initialize tracing with a console exporter for local development.
    ///
    /// Uses a simple exporter to write spans to stdout.
    pub fn try_init_console(mut self) -> Result<OwiwiGuard, Error> {
        let resource = self.build_resource();
        #[cfg(feature = "metrics")]
        let meter_provider = {
            let exporter = opentelemetry_stdout::MetricExporter::default();
            opentelemetry_sdk::metrics::SdkMeterProvider::builder()
                .with_resource(resource.clone())
                .with_periodic_exporter(exporter)
                .build()
        };
        let tracer_provider = SdkTracerProvider::builder()
            .with_resource(resource)
            .with_simple_exporter(opentelemetry_stdout::SpanExporter::default())
            .build();

        self.finish(
            tracer_provider,
            #[cfg(feature = "metrics")]
            meter_provider,
        )
    }

    /// Install the subscriber and returns the provider guard
    fn finish(
        self,
        tracer_provider: SdkTracerProvider,
        #[cfg(feature = "metrics")] meter_provider: opentelemetry_sdk::metrics::SdkMeterProvider,
    ) -> Result<OwiwiGuard, Error> {
        let tracer = tracer_provider.tracer(self.service_name.clone());
        let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);
        let filter_layer = self.filter_layer()?;

        let fmt_layer = self.fmt_layer();
        tracing_subscriber::registry()
            .with(otel_layer)
            .with(ErrorLayer::default())
            .with(filter_layer)
            .with(fmt_layer)
            .try_init()?;

        Ok(OwiwiGuard {
            tracer_provider,
            #[cfg(feature = "metrics")]
            meter_provider,
        })
    }
    /// Initializes the resource.
    fn build_resource(&mut self) -> Resource {
        let service_name = if self.service_name.is_empty() {
            cfg_if::cfg_if! {
                if #[cfg(not(feature = "clap"))] {
                   std::env::var(env_vars::OTEL_SERVICE_NAME).unwrap_or(DEFAULT_SERVICE_NAME.into())
                } else {
                    self.service_name.clone()
                }
            }
        } else {
            self.service_name.clone()
        };

        let mut attrs = std::mem::take(&mut self.resource_attrs);

        #[cfg(feature = "clap")]
        if attrs.is_empty() {
            attrs = std::env::var(env_vars::OTEL_RESOURCE_ATTRIBUTES)
                .ok()
                .and_then(|raw| env_vars::parse_key_values(&raw).ok())
                .unwrap_or_default();
        }

        Resource::builder()
            .with_service_name(service_name)
            .with_attributes(
                attrs
                    .into_iter()
                    .map(|(k, v)| opentelemetry::KeyValue::new(k, v)),
            )
            .build()
    }

    fn fmt_layer<S>(&self) -> impl Layer<S>
    where
        S: Subscriber + for<'span> LookupSpan<'span>,
    {
        let layer: Box<dyn Layer<_> + Send + Sync> = match self.event_format {
            EventFormat::Compact => {
                let format = self.event_format.compact();
                Box::new(tracing_subscriber::fmt::layer().event_format(format))
            }

            EventFormat::Full => {
                let format = self.event_format.full();
                Box::new(tracing_subscriber::fmt::layer().event_format(format))
            }
            EventFormat::Pretty => {
                let format = self.event_format.pretty();
                Box::new(tracing_subscriber::fmt::layer().event_format(format))
            }
        };

        layer
    }

    fn filter_layer(&self) -> Result<EnvFilter, Error> {
        let mut layer = match EnvFilter::try_from_default_env() {
            Ok(layer) => layer,
            Err(err) => {
                if let Some(source) = err.source() {
                    match source.downcast_ref::<VarError>() {
                        Some(VarError::NotPresent) => (),
                        Some(err) => {
                            tracing::error!("{err:?}");
                            return Err(ErrorKind::ParseDirective {
                                source: err.clone(),
                            }
                            .into());
                        }
                        None => {
                            tracing::error!("{err:?}");
                            return Err(ErrorKind::UnexpectedFilter(err.to_string()).into());
                        }
                    }
                }
                if self.tracing_directives.is_empty() {
                    cfg_if::cfg_if! {
                           if #[cfg(feature = "clap")] {
                               let level = self.verbose
                               .tracing_level()
                               .unwrap_or(tracing::Level::INFO);
                           } else {
                               let level = tracing::Level::INFO;
                           }
                    }
                    EnvFilter::try_new(level.as_str())?
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
}

fn is_disabled() -> bool {
    std::env::var(env_vars::OTEL_SDK_DISABLED)
        .map(|v| v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}
