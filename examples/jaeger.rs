//! Jaeger exporter setup
//!
//! Requires a running Jaeger instance with OTLP/gRPC enabled.
//!
//! ```sh
//! # Start Jarger:
//! docker run -d --name jaeger \
//!     -p 4317:4317 \
//!     -p 166886:16686 \
//!     jaegertracing/jaeger:latest
//!
//! # Run the example:
//!
//! cargo run --example jaeger --features jaeger
//!
//! # View traces at http://localhost:16686
//! ```

#![allow(warnings)]

use std::time::Duration;

use owiwi::{JaegerConfig, Owiwi};

fn main() -> owiwi::Result<()> {
    let mut owiwi = Owiwi::new();
    "jaeger-example".clone_into(&mut owiwi.service_name);

    let config = JaegerConfig::builder()
        .endpoint("http://localhost:4317".parse().expect("valid URL"))
        .timeout(Duration::from_secs(10))
        .build();

    let guard = owiwi.try_init(config)?;

    tracing::info!("sending spans to Jaeger");

    {
        let _outer = tracing::info_span!("http_request", method = "POST").entered();
        {
            let _inner = tracing::info_span!("db_query", table = "users").entered();
            tracing::debug!("quering db");
        }

        tracing::info!("request complete");
    }

    guard.shutdown()
}
