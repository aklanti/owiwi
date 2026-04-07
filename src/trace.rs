//! OpenTelemetry trace setup.

#[cfg(feature = "honeycomb")]
mod honeycomb;
mod otlp;
#[doc(hidden)]
pub use otlp::OtlpConfig;

#[cfg(feature = "honeycomb")]
#[doc(inline)]
pub use honeycomb::HoneycombConfig;
