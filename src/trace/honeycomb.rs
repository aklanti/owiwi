//! This module provides the confuguration data for honeycomb.io

use std::time::Duration;

use bon::Builder;
use opentelemetry_otlp::tonic_types::metadata::MetadataMap;
use opentelemetry_otlp::{SpanExporter, WithExportConfig, WithTonicConfig};
use secrecy::{ExposeSecret, SecretString};
use url::Url;

use super::exporter::SpanExporterConfig;
use crate::Error;

/// Configuration data for honeycomb.io
#[derive(Debug, Clone, Builder)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct HoneycombConfig {
    /// Connection endpoint
    pub endpoint: Url,
    /// API Key
    pub api_key: SecretString,
    /// Set export timeout duration
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "humantime_serde::deserialize")
    )]
    pub timeout: Duration,
}

impl TryFrom<HoneycombConfig> for SpanExporter {
    type Error = Error;

    fn try_from(config: HoneycombConfig) -> crate::Result<Self> {
        let mut metadata = MetadataMap::with_capacity(1);
        metadata.insert(
            "x-honeycomb-team",
            config.api_key.expose_secret().try_into()?,
        );
        let exporter = SpanExporter::builder()
            .with_tonic()
            .with_endpoint(config.endpoint.as_ref())
            .with_timeout(config.timeout)
            .with_metadata(metadata)
            .with_tls_config(
                opentelemetry_otlp::tonic_types::transport::ClientTlsConfig::default()
                    .with_enabled_roots(),
            )
            .build()?;
        Ok(exporter)
    }
}

impl SpanExporterConfig for HoneycombConfig {
    fn with_endpoint(&mut self, endpoint: Url) {
        self.endpoint = endpoint;
    }

    fn with_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }
}
