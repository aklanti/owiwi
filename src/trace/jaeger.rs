//! Jaeger configuration data

use std::time::Duration;

use bon::Builder;
use opentelemetry_otlp::{SpanExporter, WithExportConfig, WithTonicConfig};
use url::Url;

use crate::Error;

/// This is the configuration data for Jaeger
#[derive(Debug, Clone, Builder)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct JaegerConfig {
    /// Connection endpoint
    pub endpoint: Url,
    /// Set export timeout duration
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "humantime_serde::deserialize")
    )]
    pub timeout: Duration,
}

impl TryFrom<JaegerConfig> for SpanExporter {
    type Error = Error;

    fn try_from(config: JaegerConfig) -> Result<Self, Self::Error> {
        let mut builder = Self::builder()
            .with_tonic()
            .with_endpoint(config.endpoint.as_ref())
            .with_timeout(config.timeout);

        if config.endpoint.scheme() == "https" {
            builder = builder.with_tls_config(
                opentelemetry_otlp::tonic_types::transport::ClientTlsConfig::default()
                    .with_enabled_roots(),
            );
        }

        let exporter = builder.build()?;
        Ok(exporter)
    }
}
