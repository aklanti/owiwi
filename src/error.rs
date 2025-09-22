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
}
