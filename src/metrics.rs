//! Metrics export.
#[cfg(feature = "prometheus")]
mod prometheus;

use std::time::Duration;

use opentelemetry_sdk::Resource;
#[cfg(any(feature = "console", feature = "prometheus"))]
use opentelemetry_sdk::metrics::PeriodicReader;
use opentelemetry_sdk::metrics::SdkMeterProvider;
#[doc(inline)]
#[cfg(feature = "prometheus")]
pub use prometheus::PrometheusConfig;

use crate::Error;

/// Metric backend selectionn.
#[allow(
    clippy::large_enum_variant,
    reason = "short-lived init value, not stored"
)]
#[derive(Clone, Debug, Default)]
pub enum MetricExporter {
    /// Export metrics to stdout on a periodical interval.
    #[cfg(feature = "console")]
    Console,
    /// No metrics export.
    #[default]
    None,
    /// Export metrics via OTLP to a Prometheus-compatible endpoint
    #[cfg(feature = "prometheus")]
    Prometheus(PrometheusConfig),
}

impl MetricExporter {
    /// Builds the meter provider for this backend.
    ///
    /// It returns `None` when no backend is configured.
    pub fn build_provider(
        self,
        resource: Resource,
        interval: Option<Duration>,
    ) -> Result<Option<SdkMeterProvider>, Error> {
        #[cfg(not(any(feature = "console", feature = "prometheus")))]
        let _ = (resource, interval);
        match self {
            #[cfg(feature = "console")]
            Self::Console => {
                let exporter = opentelemetry_stdout::MetricExporter::default();
                let mut builder = PeriodicReader::builder(exporter);
                if let Some(interval) = interval {
                    builder = builder.with_interval(interval);
                }
                let provider = SdkMeterProvider::builder()
                    .with_resource(resource)
                    .with_reader(builder.build())
                    .build();
                Ok(Some(provider))
            }

            Self::None => Ok(None),
            #[cfg(feature = "prometheus")]
            Self::Prometheus(config) => {
                let exporter = config.try_into()?;
                let provider = meter_provider(exporter, resource, interval);
                Ok(Some(provider))
            }
        }
    }
}

#[cfg(feature = "prometheus")]
fn meter_provider(
    exporter: opentelemetry_otlp::MetricExporter,
    resource: Resource,
    interval: Option<Duration>,
) -> SdkMeterProvider {
    let mut builder = PeriodicReader::builder(exporter);
    if let Some(interval) = interval {
        builder = builder.with_interval(interval);
    }
    SdkMeterProvider::builder()
        .with_resource(resource)
        .with_reader(builder.build())
        .build()
}
