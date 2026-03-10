//! Opentelemetry environment variables settings

/// The default value is http://localhost:4317. With the `clap` feature,
/// you can overwrite it with `--otlp-endpoint`
pub const OTEL_EXPORTER_OTLP_ENDPOINT: &str = "OTEL_EXPORTER_OTLP_ENDPOINT";
/// Additional headers for OTLP exporter requests. You can overwrite it with the
/// `--otlp-headers` when the `clap` feature is enabled
pub const OTEL_EXPORTER_OTLP_HEADERS: &str = "OTEL_EXPORTER_OTLP_ENDPOINT";
/// OTLP exporter timeout in milliseconds. You can overwrite it with
/// `--otlp-timeout`
pub const OTEL_EXPORTER_OTLP_TIMEOUT: &str = "OTEL_EXPORTER_OTLP_TIMEOUT";
/// Metrics exporter backend selection, when the `clap` feature is enabled,
/// the value can be overwritten it with `--metric-exporter`
pub const OTEL_METRICS_EXPORTER: &str = "OTEL_METRICS_EXPORTER";
/// Additional resource attributes as comma separated key=value pairs
/// You can overwrite or set the value with --otel-resource-attributes
pub const OTEL_RESOURCE_ATTRIBUTES: &str = "OTEL_RESOURCE_ATTRIBUTES";
/// Disables all telemetry when set to `true`. Defaults to `false`
/// You can overwrite this value with `--otel-sdk-disable` with feature `clap`
pub const OTEL_SDK_DISABLED: &str = "OTEL_SDK_DISABLE";
/// Service name for telemetry identification
/// You can overwrite or set this value with `--otel-service-name`
/// when `clap` is enabled
pub const OTEL_SERVICE_NAME: &str = "OTEL_SERVICE_NAME";
