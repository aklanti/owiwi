//! This module defines the abstractions for OpenTelemetry traces setup.

pub mod collector;
pub mod provider;

#[doc(inline)]
pub use collector::{HoneycombConfig, JaegerConfig, TraceCollector, TraceCollectorConfig};
#[doc(inline)]
pub use format::EventFormat;
#[doc(inline)]
pub use provider::TracerProviderOptions;
