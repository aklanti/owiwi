//! Honeycomb exporter setup.
//!
//! Requires a Honeycomb API key.
//!
//! ```sh
//! HONEYCOMB_API_KEY=your-key cargo run --example honeycomb --features honeycomb
//! ```

#![allow(warnings)]
use std::time::Duration;

use owiwi::HoneycombConfig;
use owiwi::Owiwi;
use owiwi::TraceExporter;

fn main() -> owiwi::Result<()> {
    let api_key = std::env::var("HONEYCOMB_API_KEY").expect("HONEYCOMB_API_KEY must be set");

    let config = HoneycombConfig::builder()
        .endpoint("https://api.honeycomb.io".parse().expect("valid URL"))
        .api_key(api_key.into())
        .timeout(Duration::from_secs(5))
        .build();
    let mut owiwi = Owiwi::builder()
        .service_name("honeycomb-example")
        .traces(TraceExporter::Honeycomb(config))
        .build();

    let guard = owiwi.try_init()?;

    tracing::info!("sending spans to Honeycomb");

    {
        let _span = tracing::info_span!("process_event", event_id = 123).entered();
        tracing::info!("event processed");
    }

    guard.shutdown()
}
