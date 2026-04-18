//! Honeycomb configuration.

use std::time::Duration;

use bon::Builder;
use secrecy::ExposeSecret;
use secrecy::SecretString;
use url::Url;

use crate::OtlpConfig;

/// Configuration for [Honeycomb](https://honeycomb.io) trace export.
#[derive(Debug, Clone, Builder)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct HoneycombConfig {
    /// Exporter endpoint.
    pub endpoint: Url,
    /// API key.
    pub api_key: SecretString,
    /// Export timeout.
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "humantime_serde::deserialize")
    )]
    pub timeout: Duration,
}

impl From<HoneycombConfig> for OtlpConfig {
    fn from(config: HoneycombConfig) -> Self {
        OtlpConfig::builder()
            .endpoint(config.endpoint)
            .timeout(config.timeout)
            .headers(vec![(
                "x-honeycomb-team".to_owned(),
                config.api_key.expose_secret().to_owned(),
            )])
            .build()
    }
}
