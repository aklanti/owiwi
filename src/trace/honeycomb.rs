//! Honeycomb configuration.

use std::time::Duration;

use bon::Builder;
use opentelemetry_otlp::SpanExporter;
use secrecy::{ExposeSecret, SecretString};
use url::Url;

use super::otlp::OtlpConfig;
use crate::error::{Error, Result};

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

impl TryFrom<HoneycombConfig> for SpanExporter {
    type Error = Error;

    fn try_from(config: HoneycombConfig) -> Result<Self> {
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

#[cfg(test)]
mod tests {
    use googletest::matchers::{elements_are, eq};
    use googletest::{expect_that, gtest};

    use super::*;

    #[gtest]
    fn honeycomb_config_sets_api_key_header() {
        let config = HoneycombConfig::builder()
            .endpoint("https://api.honeycomb.io".parse().expect("to be a valid"))
            .api_key("test-key".into())
            .timeout(Duration::ZERO)
            .build();

        let otlp: OtlpConfig = config.into();
        expect_that!(
            otlp.headers,
            elements_are!((eq("x-honeycomb-team"), eq("test-key")))
        );
    }

    #[gtest]
    fn honeycomb_config_preserves_endpoint() {
        let config = HoneycombConfig::builder()
            .endpoint("https://custom.honeycomb.io".parse().expect("to be valid"))
            .api_key("key".into())
            .timeout(Duration::ZERO)
            .build();

        let otlp: OtlpConfig = config.into();
        expect_that!(otlp.endpoint.as_str(), eq("https://custom.honeycomb.io"));
    }

    #[gtest]
    fn honeycomb_config_preserve_timeout() {
        let config = HoneycombConfig::builder()
            .endpoint("https://api.honeycomb.io".parse().expect("to be valid"))
            .api_key("key".into())
            .timeout(Duration::ZERO)
            .build();

        let otlp: OtlpConfig = config.into();
        expect_that!(otlp.timeout, eq(Duration::ZERO));
    }
}
