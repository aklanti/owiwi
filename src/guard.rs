//! RAII guard for the tracing and telemetry providers.

use std::fmt;

use opentelemetry_sdk::trace::SdkTracerProvider;
use tracing_subscriber::filter::EnvFilter;

use crate::error::ErrorKind;
use crate::error::Result;

/// Guard returned by [`Owiwi::try_init`](crate::Owiwi::try_init) and
/// [`Owiwi::try_init_console`](crate::Owiwi::try_init_console).
///
/// Flushes buffered spans and shuts down the underlying [`SdkTracerProvider`]
/// when dropped. Must be held for the lifetime of the program; dropping it
/// early stops telemetry export.
#[derive(Debug)]
pub struct OwiwiGuard {
    pub(crate) tracer_provider: SdkTracerProvider,
    #[cfg(feature = "metrics")]
    pub(crate) meter_provider: Option<opentelemetry_sdk::metrics::SdkMeterProvider>,
    pub(crate) filter_handle: Option<FilterHandle>,
}

/// Handle for changing the tracing filter at runtime.
pub struct FilterHandle {
    pub(crate) inner: Box<dyn Fn(EnvFilter) -> Result<()> + Send + Sync>,
}

impl OwiwiGuard {
    /// Shuts down all providers.
    pub fn shutdown(mut self) -> Result<()> {
        let tracer_provider = std::mem::take(&mut self.tracer_provider);
        tracer_provider.shutdown().map_err(ErrorKind::Shutdown)?;

        #[cfg(feature = "metrics")]
        {
            if let Some(meter_provider) = self.meter_provider.take() {
                meter_provider.shutdown().map_err(ErrorKind::Shutdown)?;
            }
        }
        std::mem::forget(self);
        Ok(())
    }

    /// Returns a handle for replacing active filter
    #[must_use]
    pub const fn filter_handle(&self) -> Option<&FilterHandle> {
        self.filter_handle.as_ref()
    }

    #[allow(
        dead_code,
        reason = "Only called when SDK is disabled and cannot be behind a cfg"
    )]
    pub(crate) fn noop() -> Self {
        Self {
            tracer_provider: SdkTracerProvider::default(),
            #[cfg(feature = "metrics")]
            meter_provider: None,
            filter_handle: None,
        }
    }
}

impl fmt::Debug for FilterHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FilterHandle").finish_non_exhaustive()
    }
}

impl Drop for OwiwiGuard {
    fn drop(&mut self) {
        if let Err(err) = self.tracer_provider.shutdown() {
            eprintln!("failed to shutdown tracer provider {err}");
        }

        #[cfg(feature = "metrics")]
        {
            if let Some(meter_provider) = &self.meter_provider
                && let Err(err) = meter_provider.shutdown()
            {
                eprintln!("failed to shutdown meter provider {err}");
            }
        }
    }
}

impl FilterHandle {
    /// Replaces active filter.
    pub fn reload(&self, new_filter: EnvFilter) -> Result<()> {
        (self.inner)(new_filter)
    }
}

#[cfg(test)]
mod tests {
    use googletest::expect_that;
    use googletest::gtest;
    use googletest::matchers::anything;
    use googletest::matchers::ok;

    use super::*;

    #[gtest]
    fn shutdown_returns_ok() {
        let guard = OwiwiGuard::noop();
        expect_that!(guard.shutdown(), ok(anything()));
    }

    #[cfg(feature = "metrics")]
    #[gtest]
    fn noop_guard_has_no_meter_provider() {
        let guard = OwiwiGuard::noop();
        expect_that!(guard.meter_provider, googletest::matchers::none());
    }
}
