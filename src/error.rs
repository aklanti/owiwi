//! Error module
use tonic::metadata::errors::InvalidMetadataValue;

/// [`Result`] is an alias for std::result::Result with `Error` as error type.
pub type Result<T> = std::result::Result<T, Error>;

/// The error type for subcriber initialization operations.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Invalid tonic metadata value
    #[error("{source}")]
    InvalidMetadata {
        /// Error source
        #[from]
        source: InvalidMetadataValue,
    },
    /// Error building exporter
    #[error("{source}")]
    ExporterBuild {
        /// Error source
        #[from]
        source: opentelemetry_otlp::ExporterBuildError,
    },
    /// Error parsing trace directives
    #[error("parsing RUST_LOG directives: {source}")]
    DirectiveParseError {
        /// Error source
        #[from]
        source: std::env::VarError,
    },

    /// The log or level or trace directive is not set.
    #[error("expected tracing level filter")]
    MissingTracingLevel,
    /// Error source
    #[error("{source}")]
    FilterParser {
        /// Error source
        #[from]
        source: tracing_subscriber::filter::ParseError,
    },
    /// The subscriber initialization failed.
    #[error("{source}")]
    SubcriberInit {
        /// Error source
        #[from]
        source: tracing_subscriber::util::TryInitError,
    },
}
