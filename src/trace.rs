//! This module defines the abstractions for OpenTelemetry traces setup.

pub mod exporter;
mod honeycomb;
mod jaeger;
pub mod provider;

#[doc(inline)]
pub use exporter::{TraceExporter, TraceExporterConfig};
#[doc(inline)]
pub use honeycomb::HoneycombConfig;
#[doc(inline)]
pub use jaeger::JaegerConfig;
#[doc(inline)]
pub use provider::TracerProviderOptions;
