//! OpenTelemetry trace setup.

#[cfg(feature = "honeycomb")]
mod honeycomb;
pub mod otlp;

#[cfg(feature = "honeycomb")]
#[doc(inline)]
pub use honeycomb::HoneycombConfig;
