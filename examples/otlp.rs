//! Generic OTLP exporter setup.
//!
//! Works with any OTLP-compatible backend
//!
//! ```sh
//! # Start a local OTLP collector, then:
//! cargo run --example otlp
//! ```

#![allow(warnings)]

use std::time::Duration;

use owiwi::OtlpConfig;
use owiwi::Owiwi;

fn main() -> owiwi::Result<()> {
    let mut owiwi = Owiwi::builder()
        .service_name("otlp-example")
        .otlp(
            OtlpConfig::builder()
                .endpoint("http://localhost:4317".parse().expect("valid URL"))
                .timeout(Duration::from_secs(10))
                .headers(vec![("x-custom-header".into(), "value".into())])
                .build(),
        )
        .build();

    let guard = owiwi.try_init()?;

    tracing::info!("connected to OTLP backend");

    {
        let _span = tracing::info_span!("request", method = "GET", path = "/api/v1").entered();
        tracing::info!("processing request");
    }

    guard.shutdown()
}
