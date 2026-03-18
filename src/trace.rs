//! OpenTelemetry trace setup.

#[cfg(feature = "honeycomb")]
mod honeycomb;
pub mod otlp;
pub mod provider;

#[cfg(feature = "honeycomb")]
#[doc(inline)]
pub use honeycomb::HoneycombConfig;
pub(crate) use provider::TracerProviderOptions;
