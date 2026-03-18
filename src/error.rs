//! Error types for subscriber initialization.

/// Convenience alias for `Result<T, owiwi::Error>`.
pub type Result<T> = std::result::Result<T, Error>;

/// An error that occurred during subscriber initialization.
#[derive(Debug, thiserror::Error)]
#[error("{kind}")]
pub struct Error {
    #[source]
    kind: ErrorKind,
}

impl<E: Into<ErrorKind>> From<E> for Error {
    fn from(err: E) -> Self {
        Self { kind: err.into() }
    }
}

/// Internal error variants for initialization failures.
#[derive(Debug, thiserror::Error)]
pub(crate) enum ErrorKind {
    /// Error building exporter
    #[error(transparent)]
    BuildTraceExporter(#[from] opentelemetry_otlp::ExporterBuildError),
    /// The subscriber initialization failed.
    #[error(transparent)]
    InitSubscriber(#[from] tracing_subscriber::util::TryInitError),
    /// Error parsing trace directives
    #[error("parsing RUST_LOG directives: {source}")]
    ParseDirective {
        #[from]
        source: std::env::VarError,
    },
    /// Error parsing filter
    #[error(transparent)]
    ParseFilter(#[from] tracing_subscriber::filter::ParseError),
    /// Error parsing string to URL
    #[error(transparent)]
    ParseUrl(#[from] url::ParseError),
    #[error("unexpected error parsing env filter: {0}")]
    UnexpectedFilter(String),
    /// Failed to shutdown a provider
    #[error("failed to shutdown provider: {0}")]
    Shutdown(opentelemetry_sdk::error::OTelSdkError),
    /// Invalid Span exporter configuration data
    #[error("invalid span exporter configuration")]
    ExporterConfig,
}
