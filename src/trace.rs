//! OpenTelemetry trace setup.

#[cfg(feature = "honeycomb")]
mod honeycomb;
pub(crate) mod otlp;
#[cfg(feature = "honeycomb")]
#[doc(inline)]
pub use honeycomb::HoneycombConfig;
pub use otlp::OtlpConfig;
