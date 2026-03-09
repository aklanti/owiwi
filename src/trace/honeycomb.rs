//! This module provides the confuguration data for honeycomb.io

use std::time::Duration;

use bon::Builder;
use opentelemetry_otlp::SpanExporter;
use secrecy::{ExposeSecret, SecretString};
use url::Url;

use super::otlp::OtlpConfig;
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
        OtlpConfig::from(config).try_into()
    }
}

impl From<HoneycombConfig> for OtlpConfig {
    fn from(config: HoneycombConfig) -> Self {
        Self::builder()
            .endpoint(config.endpoint)
            .timeout(config.timeout)
            .headers(vec![(
                "x-honeycomb-team".to_owned(),
                config.api_key.expose_secret().to_owned(),
            )])
            .build()
    }
}
