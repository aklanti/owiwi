//! Jaeger configuration data

use std::time::Duration;

use bon::Builder;
use opentelemetry_otlp::SpanExporter;
use url::Url;

use super::otlp::OtlpConfig;
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
        OtlpConfig::from(config).try_into()
    }
}

impl From<JaegerConfig> for OtlpConfig {
    fn from(config: JaegerConfig) -> Self {
        Self::builder()
            .endpoint(config.endpoint)
            .timeout(config.timeout)
            .build()
    }
}
