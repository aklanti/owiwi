//! Minimal console exporter setup.

#![allow(warnings)]

use owiwi::Owiwi;

fn main() -> owiwi::Result<()> {
    let mut owiwi = Owiwi::builder().service_name("console-example").build();
    let guard = owiwi.try_init_console()?;
    tracing::info!("hello from console  exporter");

    {
        let _span = tracing::info_span!("cat", name = "miri").entered();
        tracing::debug!("doing work");
    }
    guard.shutdown()
}
