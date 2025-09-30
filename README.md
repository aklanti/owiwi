[![Crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![MIT licensed][mit-badge]][mit-license]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/owiwi-tracing-opentelemetry
[crates-url]: https://crates.io/crates/owiwi-tracing-opentelemetry
[docs-badge]: https://img.shields.io/docsrs/owiwi-tracing-opentelemetry/latest
[docs-url]: https://docs.rs/owiwi/latest/owiwi-tracing-opentelemetry/
[mit-badge]: https://img.shields.io/badge/license-MIT-blue
[mit-license]: LICENSE
[actions-badge]: https://github.com/aklanti/owiwi-tracing-opentelemetry/workflows/CI/badge.svg
[actions-url]: https://github.com/aklanti/owiwi-tracing-opentelemetry/actions/workflows/main.yaml



## Overview

`owiwi-tracing-opentelemetry` is a library for initializing tracing subscriber with OpenTelemetry


## Usage

To use `owiwi-tracing-opentelemetry`, add the following to your `Cargo.toml`

```toml
[dependencies]
owiwi-tracing-opentelemtry = "0.1"
tracing = "0.1"
```

Then initiate the subscriber using the `init()` method

```rust
use owiwi_tracing_opentelemetry::Owiwi
use owiwi_tracing_opentelemetry::collector::CollectorConfig;

fn main() {
  // Initialize the subscriber with an exporter that prints telemetry
  // (logs, metrics and traces) to the standard output.
  let config = CollectorConfig::default();
  Owiwi::init("demo-service");
  tracing::info!("All good!", config);
}
```

You can also initialize the subscriber in asynchronous code.

```rust
use owiwi_tracing_opentelemetry::Owiwi
use owiwi_tracing_opentelemetry::collector::CollectorConfig;
use tracing::info;

async fn main() {
  // Initialize the subscriber with an exporter that prints telemetry
  // (logs, metrics and traces) to the standard output.
  let config = CollectorConfig::default();
  Owiwi::init("demo-service", config);
  tracing::info!("All good!");
}
```

## Supported Rust Versions
`Owiwi` currently only support the latest stable version.

## License

This project is licensed under the [MIT license](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Owiwi by you, shall be licensed as MIT, without any additional
terms or conditions.

### Acknowledgments
This project was inspired by this [blog][instrumenting-axum] post.


[instrumenting-axum]: https://determinate.systems/blog/instrumenting-axum/ 
