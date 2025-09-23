//! Error module

pub type Result<T> = std::result::Result<T, Error>;

use tonic::metadata::errors::InvalidMetadataValue;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{source}")]
    InvalidMetadata {
        #[from]
        source: InvalidMetadataValue,
    },
    #[error("{source}")]
    ExporterBuild {
        #[from]
        source: opentelemetry_otlp::ExporterBuildError,
    },
    #[error("parsing RUST_LOG directives: {source}")]
    DirectiveParseError {
        #[from]
        source: std::env::VarError,
    },

    #[error("expected tracing level filter")]
    MissingTracingLevel,
    #[error("{source}")]
    FilterParser {
        #[from]
        source: tracing_subscriber::filter::ParseError,
    },
    #[error("{source}")]
    SubcriberInit {
        #[from]
        source: tracing_subscriber::util::TryInitError,
    },
}
