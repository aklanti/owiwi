use std::time::Duration;

use bon::Builder;
use opentelemetry_otlp::tonic_types::metadata::MetadataMap;
use opentelemetry_otlp::tonic_types::transport::ClientTlsConfig;
use opentelemetry_otlp::{SpanExporter, WithExportConfig, WithTonicConfig};
use url::Url;

use crate::error::Error;

/// `OTlP` exporter configuration
#[derive(Debug, Clone, Builder)]
pub struct OtlpConfig {
    /// Connection endpoint
    pub endpoint: Url,
    ///  Export timeout duration
    pub timeout: Duration,
    /// Additional gRPC metadata headers
    #[builder(default)]
    pub headers: Vec<(String, String)>,
}

impl OtlpConfig {
    /// Builds the gRPC metadata map from all header sources.
    fn metadata(&self) -> Result<MetadataMap, Error> {
        let mut map = MetadataMap::with_capacity(self.headers.len());
        for (key, val) in &self.headers {
            let val = val.try_into().map_err(|_e| Error::ExporterConfig)?;
            map.entry(key.as_str())
                .map_err(|_e| Error::ExporterConfig)?
                .or_insert(val);
        }
        Ok(map)
    }
}

impl TryFrom<OtlpConfig> for SpanExporter {
    type Error = Error;
    fn try_from(config: OtlpConfig) -> Result<Self, Self::Error> {
        let metadata = config.metadata()?;
        let mut builder = SpanExporter::builder()
            .with_tonic()
            .with_endpoint(config.endpoint.as_ref())
            .with_metadata(metadata);

        if config.endpoint.scheme() == "https" {
            builder = builder.with_tls_config(ClientTlsConfig::default().with_enabled_roots());
        }

        Ok(builder.build()?)
    }
}
