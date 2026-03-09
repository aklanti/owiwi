//! Error module

/// A specialized [`std::result::Result`] type for telemetry setup operation.
///
/// This type is used to avoid writing out [`owiwi::Error`](crate::Error);
pub type Result<T> = std::result::Result<T, Error>;

/// The error type for subcriber initialization operations.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error building exporter
    #[error(transparent)]
    BuildTraceExporter(#[from] opentelemetry_otlp::ExporterBuildError),
    /// The subscriber initialization failed.
    #[error(transparent)]
    InitSubscriber(#[from] tracing_subscriber::util::TryInitError),
    /// Error parsing trace directives
    #[error("parsing RUST_LOG directives: {source}")]
    ParseDirective {
        /// Error source
        #[from]
        source: std::env::VarError,
    },
    /// Error parsing filter
    #[error(transparent)]
    ParseFilter(#[from] tracing_subscriber::filter::ParseError),
    /// Error parsing string to URL
    #[error(transparent)]
    ParseUrl(#[from] url::ParseError),
    /// The log or level or trace directive is not set.
    #[error("expected tracing level filter")]
    TraceLevelMissing,
    /// Unsupported metrics backend
    #[error("unsupported metrics backend: {0}")]
    TraceBackend(String),
    /// Unsupported traces backend
    #[error("unsupported traces backend: {0}")]
    MetricBackend(String),
    /// Invalid filter
    #[error("unexpected error parsing env filter: {0}")]
    UnexpectedFilter(String),
    /// Failed to shutdown a provider
    #[error("failed to shutdown provider: {0}")]
    Shutdown(opentelemetry_sdk::error::OTelSdkError),
    /// Invalid Span exporter configuration data
    #[error("invalid span exporter configuration")]
    ExporterConfig,
}
