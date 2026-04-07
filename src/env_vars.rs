//! OpenTelemetry environment variables.

/// OTLP exporter endpoint. Defaults to `http://localhost:4317`.
pub const OTEL_EXPORTER_OTLP_ENDPOINT: &str = "OTEL_EXPORTER_OTLP_ENDPOINT";
/// Additional headers for OTLP exporter requests.
pub const OTEL_EXPORTER_OTLP_HEADERS: &str = "OTEL_EXPORTER_OTLP_HEADERS";
/// OTLP exporter timeout.
pub const OTEL_EXPORTER_OTLP_TIMEOUT: &str = "OTEL_EXPORTER_OTLP_TIMEOUT";
/// Additional resource attributes as comma-separated `key=value` pairs.
pub const OTEL_RESOURCE_ATTRIBUTES: &str = "OTEL_RESOURCE_ATTRIBUTES";
/// Disables all telemetry when set to `"true"`. Defaults to `"false"`.
pub const OTEL_SDK_DISABLED: &str = "OTEL_SDK_DISABLED";
/// Service name for telemetry identification.
pub const OTEL_SERVICE_NAME: &str = "OTEL_SERVICE_NAME";
/// Sampler type
pub const OTEL_TRACES_SAMPLER: &str = "OTEL_TRACES_SAMPLER";
/// Sampler argument. For `traceidratio`, this is the ratio.
pub const OTEL_TRACES_SAMPLER_ARG: &str = "OTEL_TRACES_SAMPLER_ARG";

/// Parses a comma-separated list of `key=value` entries.
///
/// Returns an error if any entry is missing `=`.
pub(super) fn parse_key_values(header: &str) -> Result<Vec<(String, String)>, ParseKeyValueError> {
    header
        .split(',')
        .map(|entry| {
            let (key, val) = entry.split_once('=').ok_or_else(|| ParseKeyValueError {
                entry: entry.to_owned(),
            })?;
            Ok((key.trim().to_owned(), val.trim().to_owned()))
        })
        .collect()
}

/// Error parsing a key=value list
#[derive(Debug, thiserror::Error)]
#[error("invalid header: expected `key=value`, got `{entry}`")]
pub(super) struct ParseKeyValueError {
    entry: String,
}

#[cfg(test)]
mod tests {
    use googletest::expect_that;
    use googletest::gtest;
    use googletest::matchers::anything;
    use googletest::matchers::elements_are;
    use googletest::matchers::eq;
    use googletest::matchers::err;

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
