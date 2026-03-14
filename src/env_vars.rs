//! Opentelemetry environment variables settings

/// The default value is <http://localhost:4317>. With the `clap` feature,
/// you can overwrite it with `--otlp-endpoint`
pub const OTEL_EXPORTER_OTLP_ENDPOINT: &str = "OTEL_EXPORTER_OTLP_ENDPOINT";
/// Additional headers for OTLP exporter requests. You can overwrite it with the
/// `--otlp-headers` when the `clap` feature is enabled
pub const OTEL_EXPORTER_OTLP_HEADERS: &str = "OTEL_EXPORTER_OTLP_HEADERS";
/// OTLP exporter timeout in milliseconds. You can overwrite it with
/// `--otlp-timeout`
pub const OTEL_EXPORTER_OTLP_TIMEOUT: &str = "OTEL_EXPORTER_OTLP_TIMEOUT";
/// Traces exporter backend selection.
pub const OTEL_TRACES_EXPORTER: &str = "OTEL_TRACES_EXPORTER";
/// Metrics exporter backend selection, when the `clap` feature is enabled,
/// the value can be overwritten it with `--metric-exporter`
pub const OTEL_METRICS_EXPORTER: &str = "OTEL_METRICS_EXPORTER";
/// Additional resource attributes as comma separated key=value pairs
/// You can overwrite or set the value with `--otel-resource-attributes`
pub const OTEL_RESOURCE_ATTRIBUTES: &str = "OTEL_RESOURCE_ATTRIBUTES";
/// Disables all telemetry when set to `true`. Defaults to `false`
/// You can overwrite this value with `--otel-sdk-disabled` with feature `clap`
pub const OTEL_SDK_DISABLED: &str = "OTEL_SDK_DISABLED";
/// Service name for telemetry identification
/// You can overwrite or set this value with `--otel-service-name`
/// when `clap` is enabled
pub const OTEL_SERVICE_NAME: &str = "OTEL_SERVICE_NAME";

/// Parses a comma-separated list of `key=value` entries
///
/// Malformed entries missing `=` are silently skipped.
pub fn parse_key_values(header: &str) -> Result<Vec<(String, String)>, String> {
    header
        .split(',')
        .map(|entry| {
            let (key, val) = entry
                .split_once('=')
                .ok_or_else(|| format!("invalid header: expected `key=value`, got `{entry}`"))?;
            Ok((key.trim().to_owned(), val.trim().to_owned()))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use googletest::matchers::{anything, elements_are, eq, err};
    use googletest::{expect_that, gtest};

    use super::*;

    #[gtest]
    fn parse_key_values_trims_whitespace() {
        let value = parse_key_values(" k1 = v1, k2 = v2").expect("value to be ok");
        expect_that!(
            &value,
            elements_are![(eq("k1"), eq("v1")), (eq("k2"), eq("v2"))]
        );
    }

    #[gtest]
    fn parse_key_values_valid_input() {
        let value = parse_key_values("k1=v1,k2=v2").expect("value to be ok");
        expect_that!(
            &value,
            elements_are![(eq("k1"), eq("v1")), (eq("k2"), eq("v2"))]
        );
    }

    #[gtest]
    fn parse_missing_equals() {
        let value = parse_key_values("k1 v1,k2=v2");
        expect_that!(value, err(anything()));
    }

    #[gtest]
    fn parse_key_values_empty_string() {
        let value = parse_key_values("");
        expect_that!(value, err(anything()));
    }
}
