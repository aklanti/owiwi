[![Crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![MIT licensed][mit-badge]][mit-license]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/owiwi-tracing-opentelemetry
[crates-url]: https://crates.io/crates/owiwi-tracing-opentelemetry
[docs-badge]: https://img.shields.io/docsrs/owiwi-tracing-opentelemetry/latest
[docs-url]: https://docs.rs/owiwi-tracing-opentelemetry/latest/owiwi-tracing-opentelemetry/
[mit-badge]: https://img.shields.io/badge/license-MIT-blue
[mit-license]: LICENSE
[actions-badge]: https://github.com/aklanti/owiwi-tracing-opentelemetry/workflows/CI/badge.svg
[actions-url]: https://github.com/aklanti/owiwi-tracing-opentelemetry/actions/workflows/main.yaml

## Overview

`owiwi-tracing-opentelemetry` is a crate that provides an opinionated abstraction for initializing tracing subscriber with OpenTelemetry.

It allows sending telemetry to any of the collector define in the [`trace::collector`][trace-collector] module.

## Usage

The `owiwi-tracing-opentelemetry` crate is [on crates.io][crates-url] and can be
used by adding `owiwi-tracing-opentelemetry` to your dependencies in your project's `Cargo.toml`.
Or more simply, just run `cargo add owiwi-tracing-opentelemetry`.

Additionally, You must add the tracing crate to your dependencies.

### Example with the feature `clap`

The main type of this crate is originally design to work binary application that defines a command line interface, we need to enable the `clap` flag.

```toml
[dependencies]
clap = { version = "4.5.48", features = ["derive"] }
owiwi-tracing-opentelemetry = { version = "0.2", features = ["clap"] }
tracing = "0.1"
```

The following is a complete program that initializes a subscriber and emit some traces.

```rust
use clap::Parser;
use owiwi_tracing_opentelemetry::Owiwi;
use owiwi_tracing_opentelemetry::trace::TraceCollectorConfig;
use owiwi_tracing_opentelemetry::trace::collector::HoneycombConfig;

#[derive(Debug, Clone, Parser)]
struct Cli {
     #[command(flatten)]
     owiwi: Owiwi,
}

fn main() {
     let cli = Cli::parse();
     // Create a configuration to send traces to honeycomb.io
     let honeycomb_config = HoneycombConfig.builder()
         .endpoint("https://api.honeycomb.io/traces/api".parse().expect("to be valid URL"))
         .api_key("super_secret_key".into())
         .timeout(std::time::Duration::from_secs(5))
         .build();
     let collector_config = TraceCollectorConfig::Honeycomb(honeycomb_config);
     let _guard = cli.owiwi.try_init("example", collector_config);
     tracing::info!("the subscriber was initialized");
}

```

### Example without the feature `clap`

The following is a complete program that initializes a subscriber and emit some traces.

```rust
use owiwi_tracing_opentelemetry::Owiwi;
use owiwi_tracing_opentelemetry::trace::TraceCollectorConfig;
use owiwi_tracing_opentelemetry::format::EventFormat;

fn main() {
     // The default collector configuration sends traces to std::io::stdout
     let collector_config = TraceCollectorConfig::default();
     let service_name = "example";
     // Initializes the subscriber
     let _guard = Owiwi::default().try_init(service_name,  collector_config);
     tracing::info!("the Subscriber was initialized!");
}
```

## Optional features

There are some optional features that enable additional dependencies:
- `serde` adds [`Deserialize`][deserialize] implementations for some types. It also allow deserializing [`humantime`][humantime] using [`humantime-serde`][humantime-serde]
- `clap`: adds [`Args`][clap-args] implementation to [`Owiwi`][owiwi] and various other types.

## Supported Rust Versions
`owiwi-tracing-opentelemetry` currently only supports the latest stable version.

## License

This project is licensed under the [MIT license](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `owiwi-tracing-opentelemetry` by you, shall be licensed as MIT, without any additional
terms or conditions.

### Acknowledgments
This project was inspired by this [blog][instrumenting-axum] post.


[deserialize]: https://docs.rs/serde/1/serde/trait.Deserialize.html
[humantime]: https://docs.rs/humantime/2/humantime/
[humantime-serde]: https://docs.rs/humantime-serde/1/humantime_serde/
[clap-args]: https://docs.rs/clap/4/clap/trait.Args.html
[owiwi]: https://docs.rs/owiwi-tracing-opentelemetry/latest/owiwi/struct.Owiwi.html
[trace-collector]: https://docs.rs/owiwi-tracing-opentelemetry/latest/trace/collector/index.html
[instrumenting-axum]: https://determinate.systems/blog/instrumenting-axum/ 
