//! Minimal console exporter setup.

#![allow(warnings)]

use owiwi::Owiwi;

fn main() -> owiwi::Result<()> {
    let guard = Owiwi::builder()
        .service_name("console-example")
        .build()
        .try_init_console()?;
    tracing::info!("hello from console  exporter");

    {
        let _span = tracing::info_span!("cat", name = "miri").entered();
        tracing::debug!("doing work");
    }
    guard.shutdown()
}
