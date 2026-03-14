//! This module defines the abstractions for OpenTelemetry traces setup.

pub mod exporter;
#[cfg(feature = "honeycomb")]
mod honeycomb;
pub mod otlp;
pub mod provider;

#[doc(inline)]
pub use exporter::TraceBackend;
#[cfg(feature = "honeycomb")]
#[doc(inline)]
pub use honeycomb::HoneycombConfig;
#[doc(inline)]
pub use provider::TracerProviderOptions;
