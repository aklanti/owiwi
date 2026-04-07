//! Error types for subscriber initialization.

/// Convenience alias for `Result<T, owiwi::Error>`.
pub type Result<T> = std::result::Result<T, Error>;

/// An error that occurred during subscriber initialization.
#[derive(Debug, thiserror::Error)]
#[error("owiwi initialization failed: {kind}")]
pub struct Error {
    #[source]
    kind: ErrorKind,
}

impl<E: Into<ErrorKind>> From<E> for Error {
    fn from(err: E) -> Self {
        Self { kind: err.into() }
    }
}

/// Error variants for initialization failures.
#[derive(Debug, thiserror::Error)]
pub(crate) enum ErrorKind {
    /// Failed to build exporter.
    #[error(transparent)]
    BuildTraceExporter(#[from] opentelemetry_otlp::ExporterBuildError),
    /// Subscriber initialization failed.
    #[error(transparent)]
    InitSubscriber(#[from] tracing_subscriber::util::TryInitError),
    /// Invalid trace directive.
    #[error("parsing RUST_LOG directives: {source}")]
    ParseDirective {
        #[from]
        source: std::env::VarError,
    },
    /// Invalid filter.
    #[error(transparent)]
    ParseFilter(#[from] tracing_subscriber::filter::ParseError),
    /// Invalid URL.
    #[error(transparent)]
    ParseUrl(#[from] url::ParseError),
    #[error("unexpected error parsing env filter: {0}")]
    UnexpectedFilter(String),
    /// Failed to shut down a provider.
    #[error(transparent)]
    Shutdown(#[from] opentelemetry_sdk::error::OTelSdkError),
    /// Invalid span exporter configuration.
    #[error("invalid span exporter configuration: {reason}")]
    ExporterConfig { reason: String },
    /// Failed to replace active filter
    #[error(transparent)]
    FilterReload(#[from] tracing_subscriber::reload::Error),
    #[error("no tokio runtime found. owiwi requires a running tokio runtime for batch export")]
    NoTokioRuntime,
}
