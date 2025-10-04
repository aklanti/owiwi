//! Error module
use tonic::metadata::errors::InvalidMetadataValue;

/// A specialized [`std::result::Result`] type for telemetry setup operation.
///
/// This type is  used to avoid writing out [`owiwi::Error`](crate::Error);
pub type Result<T> = std::result::Result<T, Error>;

/// The error type for subcriber initialization operations.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error building exporter
    #[error(transparent)]
    BuildTraceExporter(#[from] opentelemetry_otlp::ExporterBuildError),
    #[cfg(feature = "metrics")]
    /// Prometheus exporter build error
    #[error(transparent)]
    BuildPrometheusExporter(#[from] metrics_exporter_prometheus::BuildError),
    /// The subscriber initialization failed.
    #[error(transparent)]
    InitSubscriberError(#[from] tracing_subscriber::util::TryInitError),
    /// Invalid tonic metadata value
    #[error(transparent)]
    InvalidMetadataValue(#[from] InvalidMetadataValue),
    /// Error parsing trace directives
    #[error("parsing RUST_LOG directives: {source}")]
    ParseDirectiveError {
        /// Error source
        #[from]
        source: std::env::VarError,
    },
    /// Error parsing filter
    #[error(transparent)]
    ParseFilterError(#[from] tracing_subscriber::filter::ParseError),
    /// Error parsing string to URL
    #[error(transparent)]
    ParseUrlError(#[from] url::ParseError),
    /// Collector configuration error
    #[error("collector configuration error")]
    TraceCollectorConfigError,
    /// The log or level or trace directive is not set.
    #[error("expected tracing level filter")]
    TraceLevelMissing,
    /// Unsupported metrics collector
    #[error("unsupported metrics collector: {0}")]
    UnsupportedMetricsCollector(String),
    /// Unsupported traces collector
    #[error("unsupported traces collector: {0}")]
    UnsupportedTracesCollector(String),
}
