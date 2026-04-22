//! Tracing and telemetry initialization.

use std::env::VarError;
use std::error::Error as _;

use bon::Builder;
#[cfg(feature = "clap")]
use clap_verbosity_flag::Verbosity;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::trace::Sampler;
use opentelemetry_sdk::trace::SdkTracerProvider;
use tracing::Subscriber;
use tracing_error::ErrorLayer;
use tracing_subscriber::filter::Directive;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::layer::Layer;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::reload;
use tracing_subscriber::util::SubscriberInitExt as _;

use super::FilterHandle;
#[cfg(feature = "clap")]
use super::HELP_HEADING;
use super::OwiwiGuard;
use super::env_vars;
use super::error::ErrorKind;
use super::error::Result;
use super::trace::TraceExporter;
use crate::EventFormat;

/// Default service name
const DEFAULT_SERVICE_NAME: &str = "unknown_service";

/// Configuration for initializing a [`tracing`] subscriber with OpenTelemetry.
///
/// When the `clap` feature is enabled, this type can be flattened into a CLI parser.
#[must_use]
#[derive(Clone, Debug, Builder)]
#[cfg_attr(
    feature = "clap",
    derive(clap::Args),
    command(next_help_heading=HELP_HEADING)
)]
pub struct Owiwi {
    /// Service name.
    #[cfg_attr(
        feature = "clap",
        arg(
            long,
            help = "Service name for telemetry",
            default_value = DEFAULT_SERVICE_NAME,
            env = env_vars::OTEL_SERVICE_NAME,
        )
    )]
    #[builder(default, into)]
    pub service_name: String,
    /// Resource attributes.
    #[cfg_attr(
        feature = "clap",
        arg(
            name = "resource-attributes",
            long = "resource-attrs",
            help = "Resource attributes (key=value,key=value)",
            value_parser = env_vars::parse_key_values,
            env = env_vars::OTEL_RESOURCE_ATTRIBUTES,
        )
    )]
    #[builder(default)]
    pub resource_attrs: Vec<(String, String)>,

    /// Trace backend. Defaults to OTLP with spec values.
    #[cfg_attr(feature = "clap", arg(skip))]
    #[builder(default)]
    pub traces: TraceExporter,

    /// Span sampler. Defaults to the SDK default value
    /// when not set and `OTEL_TRACES_SAMPLER` is absent.
    #[cfg_attr(feature = "clap", arg(skip))]
    pub sampler: Option<Sampler>,

    /// Metric backend. Defaults to no metrics export.
    #[cfg(feature = "metrics")]
    #[cfg_attr(feature = "clap", arg(skip))]
    #[builder(default)]
    pub metrics: super::metrics::MetricExporter,

    /// Metrics exports interval
    #[cfg(feature = "metrics")]
    #[cfg_attr(
    feature = "clap",
    arg(
        name = "metrics-interval",
        long,
        help = "Metrics export interval (e.g. 30s, 1m)",
        env = env_vars::OWIWI_METRICS_INTERVAL,
    ),
)]
    pub metrics_interval: Option<jiff::SignedDuration>,

    /// Trace filter directives to overwrite the default level and `RUST_LOG`.
    #[cfg_attr(
        feature = "clap",
        arg(
            long = "trace-directive",
            help = "Trace filter",
            value_delimiter = ',',
            num_args = 1..,
        )
    )]
    #[builder(default)]
    pub tracing_directives: Vec<Directive>,
    /// Filter directives for the OpenTelemetry export layer
    /// Defaults to `info`.
    #[cfg_attr(
        feature = "clap",
        arg(
            long = "export-directive",
            help = "Export filter",
            value_delimiter = ',',
            num_args = 1..,
            env = env_vars::OWIWI_EXPORT_LOG,
        )
    )]
    #[builder(default)]
    pub export_directives: Vec<Directive>,
    /// Event output format.
    #[cfg_attr(
        feature = "clap",
        arg(
            name = "event-format",
            long,
            value_enum,
            help = "Output format for trace events",
            default_value_t = Default::default(),
        )
    )]
    #[builder(default)]
    pub event_format: EventFormat,
    /// Verbosity flags
    #[cfg(feature = "clap")]
    #[command(flatten)]
    #[builder(default)]
    pub verbose: Verbosity,

    /// Disables all telemetry when `true`.
    #[cfg_attr(
        feature = "clap",
        arg(
            name = "no-telemetry",
            long,
            help = "Disable all telemetry",
            env = env_vars::OTEL_SDK_DISABLED,
        )
    )]
    #[builder(default)]
    pub no_telemetry: bool,
}

impl Default for Owiwi {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl Owiwi {
    /// Initializes the tracing and optionally metrics provider.
    ///
    /// Sets up a [`tracing_subscriber`] registry with an OpenTelemetry layer,
    /// error layer, and a formatting layer. It dispatches on [`Self::trace_exporter`]
    /// and [`Self::metric_exporter`] to build the configured providers.
    ///
    /// Returns an [`OwiwiGuard`] that must be held for the lifetime of the program.
    ///
    /// # Errors
    ///
    /// Returns an error if the exporter cannot be built, filter directives
    /// are invalid, or a global subscriber is already set, or no tokio runtime is available.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use owiwi::Owiwi;
    ///
    /// let guard = Owiwi::builder()
    ///     .service_name("owiwi-test")
    ///     .build()
    ///     .try_init()?;
    /// # Ok::<_, owiwi::Error>(())
    /// ```
    pub fn try_init(mut self) -> Result<OwiwiGuard> {
        if self.is_disabled() {
            return self.noop();
        }
        let resource = self.build_resource();

        #[cfg(feature = "metrics")]
        let meter_provider = {
            let interval = self
                .metrics_interval
                .take()
                .map(|d| {
                    std::time::Duration::try_from(d).map_err(|err| ErrorKind::ExporterConfig {
                        reason: format!("invalid metrics interval: {err}"),
                    })
                })
                .transpose()?;
            std::mem::take(&mut self.metrics).build_provider(resource.clone(), interval)?
        };

        let exporter = std::mem::take(&mut self.traces);
        let tracer_provider = exporter.build_provider(resource, self.sampler.take())?;

        self.finish(
            tracer_provider,
            #[cfg(feature = "metrics")]
            meter_provider,
        )
    }

    /// Sets the global tracing subscriber and returns the provider guard.
    fn finish(
        self,

        tracer_provider: SdkTracerProvider,
        #[cfg(feature = "metrics")] meter_provider: Option<
            opentelemetry_sdk::metrics::SdkMeterProvider,
        >,
    ) -> Result<OwiwiGuard> {
        if tokio::runtime::Handle::try_current().is_err() {
            return Err(ErrorKind::NoTokioRuntime.into());
        }
        let tracer = tracer_provider.tracer(self.service_name.clone());

        let (filter_layer, reload_handle) = self.filter_layer().map(reload::Layer::new)?;
        let export_filter = self.export_filter_layer()?;

        let otel_layer = tracing_opentelemetry::layer()
            .with_tracer(tracer)
            .with_filter(export_filter);

        let fmt_layer = self.fmt_layer().with_filter(filter_layer);

        tracing_subscriber::registry()
            .with(otel_layer)
            .with(ErrorLayer::default())
            .with(fmt_layer)
            .try_init()?;

        #[cfg(feature = "metrics")]
        if let Some(meter_provider) = &meter_provider {
            opentelemetry::global::set_meter_provider(meter_provider.clone());
        }

        Ok(OwiwiGuard {
            tracer_provider,
            #[cfg(feature = "metrics")]
            meter_provider,
            filter_handle: Some(FilterHandle {
                inner: Box::new(move |filter| {
                    reload_handle
                        .reload(filter)
                        .map_err(|err| ErrorKind::FilterReload(err).into())
                }),
            }),
        })
    }

    /// Builds an OpenTelemetry [`Resource`].
    fn build_resource(&mut self) -> Resource {
        let service_name = if self.service_name.is_empty() {
            std::env::var(env_vars::OTEL_SERVICE_NAME)
                .unwrap_or_else(|_| DEFAULT_SERVICE_NAME.to_owned())
        } else {
            self.service_name.clone()
        };

        let attrs = {
            let taken = std::mem::take(&mut self.resource_attrs);
            if taken.is_empty() {
                std::env::var(env_vars::OTEL_RESOURCE_ATTRIBUTES)
                    .ok()
                    .and_then(|raw| env_vars::parse_key_values(&raw).ok())
                    .unwrap_or_default()
            } else {
                taken
            }
        };

        Resource::builder()
            .with_service_name(service_name)
            .with_attributes(
                attrs
                    .into_iter()
                    .map(|(k, v)| opentelemetry::KeyValue::new(k, v)),
            )
            .build()
    }

    /// Creates a formatting layer.
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

    /// Creates a filter layer from the configuration.
    fn filter_layer(&self) -> Result<EnvFilter> {
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
                    #[cfg(feature = "clap")]
                    let level = self.verbose.tracing_level().unwrap_or(tracing::Level::INFO);
                    #[cfg(not(feature = "clap"))]
                    let level = tracing::Level::INFO;
                    EnvFilter::try_new(level.as_str())?
                } else {
                    EnvFilter::builder().parse("")?
                }
            }
        };

        for directive in &self.tracing_directives {
            layer = layer.add_directive(directive.clone());
        }
        Ok(layer)
    }

    fn export_filter_layer(&self) -> Result<EnvFilter> {
        if !self.export_directives.is_empty() {
            let mut filter = EnvFilter::builder().parse("")?;
            for directive in &self.export_directives {
                filter = filter.add_directive(directive.clone());
            }

            return Ok(filter);
        }

        #[cfg(not(feature = "clap"))]
        if let Ok(val) = std::env::var(env_vars::OWIWI_EXPORT_LOG) {
            return Ok(EnvFilter::try_new(val)?);
        }

        Ok(EnvFilter::try_new("info")?)
    }

    #[allow(
        clippy::missing_const_for_fn,
        reason = "cannot be constify when clap is enabled"
    )]
    fn is_disabled(&self) -> bool {
        if self.no_telemetry {
            return true;
        }

        #[cfg(not(feature = "clap"))]
        {
            return std::env::var(env_vars::OTEL_SDK_DISABLED)
                .map(|v| v.eq_ignore_ascii_case("true"))
                .unwrap_or(false);
        }

        #[cfg(feature = "clap")]
        false
    }

    fn noop(self) -> Result<OwiwiGuard> {
        let filter_layer = self.filter_layer()?;
        let fmt_layer = self.fmt_layer().with_filter(filter_layer);
        tracing_subscriber::registry().with(fmt_layer).try_init()?;
        Ok(OwiwiGuard::noop())
    }
}

#[cfg(test)]
mod tests {
    use googletest::expect_that;
    use googletest::gtest;
    use googletest::matchers::anything;
    use googletest::matchers::eq;
    use googletest::matchers::ok;
    use googletest::matchers::pat;
    use googletest::matchers::some;
    use opentelemetry::Key;

    use super::*;

    #[gtest]
    fn build_resource_sets_service_name() {
        let mut owiwi = Owiwi::builder().service_name("test_service").build();
        let resource = owiwi.build_resource();
        expect_that!(resource, pat!(Resource { .. }));
        expect_that!(resource.is_empty(), eq(false));
        let key = Key::new("service.name");
        let name = resource.get(&key).map(|v| String::from(v.as_str()));
        expect_that!(name, some(eq("test_service")));
    }

    #[gtest]
    fn filter_layer_with_directives() {
        let owiwi = Owiwi::builder()
            .tracing_directives(vec![
                "info".parse().expect("valid directive"),
                "my_crate=debug".parse().expect("valid directive"),
            ])
            .build();

        expect_that!(owiwi.filter_layer(), ok(anything()));
    }

    #[gtest]
    fn build_resource_with_custom_attributes() {
        let mut owiwi = Owiwi::builder()
            .resource_attrs(vec![
                ("env".to_owned(), "staging".to_owned()),
                ("region".to_owned(), "us-east-1".to_owned()),
            ])
            .build();

        let resource = owiwi.build_resource();
        let env_key = Key::new("env");
        let env_val = resource.get(&env_key).map(|v| String::from(v.as_str()));
        expect_that!(env_val, some(eq("staging")));
    }

    #[gtest]
    fn filter_layer_defaults_to_info() {
        let owiwi = Owiwi::default();
        let filter = owiwi.filter_layer();
        expect_that!(filter, ok(anything()));
    }
}
