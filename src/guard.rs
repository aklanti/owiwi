//! RAII guard for the tracing and telemetry providers.
//!
//! This module contains [`OwiwiGuard`], the type returned by [`Owiwi::try_init`].
//! The guard ensures that all buffered spans are flushed and the underlying [`SdkTracerProvider`]
//! is shutdown when the guard is dropped
//!
//! The guard **must** be held for the lifetime of the program. Dropping it early will
//! immediately shut down telemetry export.

use opentelemetry_sdk::trace::SdkTracerProvider;

/// Owiwi guard for tracing and metrics providers
#[derive(Debug)]
pub struct OwiwiGuard {
    pub(crate) tracer_provider: SdkTracerProvider,
    #[cfg(feature = "metrics")]
    pub(crate) metrics_provider: opentelemetry_sdk::metrics::SdkMeterProvider,
}

impl Drop for OwiwiGuard {
    fn drop(&mut self) {
        if let Err(err) = self.tracer_provider.shutdown() {
            eprintln!("failed to shutdown tracer provider {err}");
        }
    }
}
