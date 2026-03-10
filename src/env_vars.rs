//! Telemetry environment variables settings

/// Sets the value of the service.name resource attribute
pub const OTEL_SERVICE_NAME: &str = "OTEL_SERVICE_NAME";
/// Specifies which exporter is used for traces
pub const OTEL_TRACES_EXPORTER: &str = "OTEL_TRACES_EXPORTER";
/// Exporter endpoint URL
pub const OTEL_EXPORTER_OTLP_ENDPOINT: &str = "OTEL_EXPORTER_OTLP_ENDPOINT";
