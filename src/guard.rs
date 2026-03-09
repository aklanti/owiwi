//! RAII guard for the tracing and telemetry providers.

use opentelemetry_sdk::trace::SdkTracerProvider;

use crate::error::Error;

/// A type returned by [`Owiwi::try_init`](crate::Owiwi::try_init) or [`Owiwi::try_init_console`](crate::Owiwi::try_init_console).
///
/// It ensures that all buffered spans are flushed and the underlying [`SdkTracerProvider`]
/// is shutdown when the guard is dropped
///
/// It must be held for the lifetime of the program. Dropping it early will
/// immediately shut down telemetry export.
#[derive(Debug)]
pub struct OwiwiGuard {
    pub(crate) tracer_provider: SdkTracerProvider,
    #[cfg(feature = "metrics")]
    pub(crate) meter_provider: opentelemetry_sdk::metrics::SdkMeterProvider,
}

impl OwiwiGuard {
    /// Explicitly shutdown all providers
    pub fn shutdown(self) -> Result<(), Error> {
        self.tracer_provider.shutdown().map_err(Error::Shutdown)?;
        #[cfg(feature = "metrics")]
        self.meter_provider.shutdown().map_err(Error::Shutdown)?;
        std::mem::forget(self);
        Ok(())
    }
}
impl Drop for OwiwiGuard {
    fn drop(&mut self) {
        if let Err(err) = self.tracer_provider.shutdown() {
            eprintln!("failed to shutdown tracer provider {err}");
        }

        #[cfg(feature = "metrics")]
        if let Err(err) = self.meter_provider.shutdown() {
            eprintln!("failed to shutdown meter provider {err}");
        }
    }
}
