//! This module defines the abstractions for OpenTelemetry traces setup.

pub mod exporter;
#[cfg(feature = "honeycomb")]
mod honeycomb;
#[cfg(feature = "jaeger")]
mod jaeger;
pub mod otlp;
pub mod provider;

#[doc(inline)]
pub use exporter::TraceBackend;
#[cfg(feature = "honeycomb")]
#[doc(inline)]
pub use honeycomb::HoneycombConfig;
#[cfg(feature = "jaeger")]
#[doc(inline)]
pub use jaeger::JaegerConfig;
#[doc(inline)]
pub use provider::TracerProviderOptions;
