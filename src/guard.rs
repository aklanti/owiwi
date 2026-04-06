//! RAII guard for the tracing and telemetry providers.

use opentelemetry_sdk::trace::SdkTracerProvider;

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
}

impl OwiwiGuard {
    /// Shuts down all providers.
    pub fn shutdown(self) -> Result<()> {
        self.tracer_provider
            .shutdown()
            .map_err(ErrorKind::Shutdown)?;

        #[cfg(feature = "metrics")]
        {
            if let Some(meter_provider) = &self.meter_provider {
                meter_provider.shutdown().map_err(ErrorKind::Shutdown)?;
            }
        }
        std::mem::forget(self);
        Ok(())
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
        }
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

#[cfg(test)]
mod tests {
    use googletest::expect_that;
    use googletest::gtest;
    use googletest::matchers::anything;
    use googletest::matchers::eq;
    use googletest::matchers::none;
    use googletest::matchers::ok;

    use super::*;

    #[gtest]
    fn shutdown_returns_ok() {
        let guard = OwiwiGuard::noop();
        expect_that!(guard.shutdown(), ok(anything()));
    }

    #[gtest]
    fn noop_guard_has_default_tracer_provider() {
        let guard = OwiwiGuard::noop();
        expect_that!(format!("{:?}", guard.tracer_provider).is_empty(), eq(false));
    }

    #[cfg(feature = "metrics")]
    #[gtest]
    fn noop_guard_has_no_meter_provider() {
        let guard = OwiwiGuard::noop();
        expect_that!(guard.meter_provider, none());
    }
}
