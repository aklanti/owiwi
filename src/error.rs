//! Error module
use tonic::metadata::errors::InvalidMetadataValue;

/// A specialized [`std::result::Result`] type for telemetry setup operation.
///
/// This type is  used to avoid writing out [`owiwi::Error`](crate::Error);
pub type Result<T> = std::result::Result<T, Error>;

/// The error type for subcriber initialization operations.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Invalid tonic metadata value
    #[error("{source}")]
    InvalidMetadataValue {
        /// Error source
        #[from]
        source: InvalidMetadataValue,
    },
    /// Error building exporter
    #[error("{source}")]
    BuildExporterError {
        /// Error source
        #[from]
        source: opentelemetry_otlp::ExporterBuildError,
    },
    /// Error parsing trace directives
    #[error("parsing RUST_LOG directives: {source}")]
    ParseDirectiveError {
        /// Error source
        #[from]
        source: std::env::VarError,
    },
    /// The log or level or trace directive is not set.
    #[error("expected tracing level filter")]
    TraceLevelMissing,
    /// Error source
    #[error("{source}")]
    ParseFilterError {
        /// Error source
        #[from]
        source: tracing_subscriber::filter::ParseError,
    },
    /// The subscriber initialization failed.
    #[error("{source}")]
    InitSubscriberError {
        /// Error source
        #[from]
        source: tracing_subscriber::util::TryInitError,
    },
}
