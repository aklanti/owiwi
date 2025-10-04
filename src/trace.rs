//! This module defines the abstractions for OpenTelemetry traces setup.

pub mod collector;
pub mod format;
pub mod provider;

#[doc(inline)]
pub use self::collector::{TraceCollector, TraceCollectorConfig};
#[doc(inline)]
pub use self::format::EventFormat;
#[doc(inline)]
pub use self::provider::TracerProviderOptions;
