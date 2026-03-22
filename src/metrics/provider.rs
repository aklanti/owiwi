//! Meter provider configuration.

use std::time::Duration;

use bon::Builder;
use opentelemetry::global;
use opentelemetry_otlp::MetricExporter;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::metrics::PeriodicReader;
use opentelemetry_sdk::metrics::SdkMeterProvider;

#[cfg(feature = "clap")]
use crate::HELP_HEADING;
use crate::error::{Error, Result};

/// Meter provider configuration.
#[must_use]
#[derive(Clone, Debug, Default, Builder)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "clap", derive(clap::Args))]
pub struct MeterProviderOptions {
    /// Metrics export interval.
    #[cfg_attr(
        feature = "clap",
        arg(
            name="metrics-interval",
            long,
            value_parser = humantime::parse_duration,
            help_heading = HELP_HEADING,
        ),
    )]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "humantime_serde::deserialize")
    )]
    pub interval: Option<Duration>,
}

impl MeterProviderOptions {
    /// Initializes the meter provider.
    pub fn init_provider(
        &self,
        resource: Resource,
        config: impl TryInto<MetricExporter, Error = Error>,
    ) -> Result<SdkMeterProvider> {
        let exporter = config.try_into()?;
        let mut builder = PeriodicReader::builder(exporter);
        if let Some(interval) = self.interval {
            builder = builder.with_interval(interval);
        }
        let reader = builder.build();
        let meter_provider = SdkMeterProvider::builder()
            .with_resource(resource)
            .with_reader(reader)
            .build();

        global::set_meter_provider(meter_provider.clone());
        Ok(meter_provider)
    }
}
