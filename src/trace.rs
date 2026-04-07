//! OpenTelemetry trace setup.

#[cfg(feature = "honeycomb")]
mod honeycomb;
mod otlp;
#[cfg(feature = "honeycomb")]
#[doc(inline)]
pub use honeycomb::HoneycombConfig;
#[doc(hidden)]
pub use otlp::OtlpConfig;
