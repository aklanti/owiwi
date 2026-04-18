//! Minimal console exporter setup.

#![allow(warnings)]

use owiwi::Owiwi;
use owiwi::TraceExporter;

fn main() -> owiwi::Result<()> {
    let guard = Owiwi::builder()
        .service_name("console-example")
        .trace_exporter(TraceExporter::Console)
        .build()
        .try_init()?;
    tracing::info!("hello from console  exporter");

    {
        let _span = tracing::info_span!("cat", name = "miri").entered();
        tracing::debug!("doing work");
    }
    guard.shutdown()
}
