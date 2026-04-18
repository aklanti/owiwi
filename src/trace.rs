//! OpenTelemetry trace setup.

#[cfg(feature = "honeycomb")]
mod honeycomb;
pub(crate) mod otlp;
#[cfg(feature = "honeycomb")]
#[doc(inline)]
pub use honeycomb::HoneycombConfig;
use opentelemetry_otlp::SpanExporter;
use opentelemetry_sdk::resource::Resource;
use opentelemetry_sdk::trace::Sampler;
use opentelemetry_sdk::trace::SdkTracerProvider;
pub use otlp::OtlpConfig;

use crate::env_vars;
use crate::error::Error;
use crate::error::ErrorKind;

/// Trace backend selection
#[derive(Clone, Debug)]
pub enum TraceExporter {
    /// Export span to stdout.
    #[cfg(feature = "console")]
    Console,

    /// Export to Honeycomb.
    #[cfg(feature = "honeycomb")]
    Honeycomb(HoneycombConfig),

    /// Export via OTLP/gRPC to a collector
    Otlp(OtlpConfig),
}

impl TraceExporter {
    /// Build tracer provider from the exporter backend.
    pub fn build_provider(
        self,
        resource: Resource,
        sampler: Option<Sampler>,
    ) -> Result<SdkTracerProvider, Error> {
        match self {
            #[cfg(feature = "console")]
            Self::Console => {
                let provider = SdkTracerProvider::builder()
                    .with_resource(resource)
                    .with_simple_exporter(opentelemetry_stdout::SpanExporter::default())
                    .build();
                Ok(provider)
            }
            #[cfg(feature = "honeycomb")]
            Self::Honeycomb(config) => {
                let exporter = OtlpConfig::from(config).build_exporter()?;
                build_tracer_provider(exporter, resource, sampler)
            }
            Self::Otlp(config) => {
                let exporter = config.build_exporter()?;
                build_tracer_provider(exporter, resource, sampler)
            }
        }
    }
}

impl Default for TraceExporter {
    fn default() -> Self {
        Self::Otlp(OtlpConfig::default())
    }
}

/// Builds a tracer provider from an exporter, resource, and optional sampler.
fn build_tracer_provider(
    exporter: SpanExporter,
    resource: Resource,
    sampler: Option<Sampler>,
) -> Result<SdkTracerProvider, Error> {
    let mut builder = SdkTracerProvider::builder().with_resource(resource);
    match sampler {
        Some(sampler) => {
            builder = builder.with_sampler(sampler);
        }
        None => {
            if let Ok(sampler) = std::env::var(env_vars::OTEL_TRACES_SAMPLER) {
                let arg = std::env::var(env_vars::OTEL_TRACES_SAMPLER_ARG).ok();
                let sampler = parse_sampler(&sampler, arg.as_deref())?;
                builder = builder.with_sampler(sampler);
            }
        }
    }

    Ok(builder.with_batch_exporter(exporter).build())
}

/// Parses trace sampler
fn parse_sampler(name: &str, arg: Option<&str>) -> Result<Sampler, Error> {
    match name {
        "always_on" => Ok(Sampler::AlwaysOn),
        "always_off" => Ok(Sampler::AlwaysOff),
        "traceidratio" => {
            let ratio: f64 = arg
                .ok_or_else(|| ErrorKind::ExporterConfig {
                    reason: String::from("missing trace id ratio"),
                })?
                .parse()
                .map_err(|err| ErrorKind::ExporterConfig {
                    reason: format!("unable to parse trace id argument `{err}`"),
                })?;
            Ok(Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(
                ratio,
            ))))
        }
        other => Err(ErrorKind::ExporterConfig {
            reason: format!("invalid sampler `{other}`"),
        }
        .into()),
    }
}

#[cfg(test)]
mod tests {
    use googletest::expect_that;
    use googletest::gtest;
    use googletest::matchers::anything;
    use googletest::matchers::err;
    use googletest::matchers::ok;

    use super::*;

    #[gtest]
    fn parse_sampler_always_on() {
        let sampler = parse_sampler("always_on", None);
        expect_that!(sampler, ok(anything()));
    }

    #[gtest]
    fn parse_sampler_always_off() {
        let sampler = parse_sampler("always_off", None);
        expect_that!(sampler, ok(anything()));
    }

    #[gtest]
    fn parse_sampler_traceidratio() {
        let sampler = parse_sampler("traceidratio", Some("0.5"));
        expect_that!(sampler, ok(anything()));
    }

    #[gtest]
    fn parse_sampler_traceidratio_missing_arg() {
        let sampler = parse_sampler("traceidratio", None);
        expect_that!(sampler, err(anything()));
    }

    #[gtest]
    fn parse_sampler_invalid_name() {
        let sampler = parse_sampler("bogus", None);
        expect_that!(sampler, err(anything()));
    }
}
