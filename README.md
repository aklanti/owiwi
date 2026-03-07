[![Build Status][badge-actions]][url-actions]
[![Crates.io][badge-crate]][url-crate]
[![Documentation][badge-docs]][url-docs]
[![MPL-2.0 license][badge-license]][url-license]

## owiwi-tracing-opentelemetry

An opinionated library for initializing tracing subscriber with OpenTelemetry.

It allows sending telemetry to any of the collector define in the [`trace::collector`][url-trace-collector] module.

## Usage

The `owiwi-tracing-opentelemetry` crate is [on crates.io][url-crate] and can be
used by adding `owiwi-tracing-opentelemetry` to your dependencies in your project's `Cargo.toml`.
Or more simply, just run `cargo add owiwi-tracing-opentelemetry`.

Additionally, You must add the tracing crate to your dependencies.

### Example

The main type of this crate is originally design to work binary application that defines a command line interface, we need to enable the `clap` flag.

```toml
[dependencies]
clap = { version = "4.5.60", features = ["derive"] }
owiwi-tracing-opentelemetry = { version = "0.2.1", features = ["clap"] }
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

### Example without `clap`

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
- **serde:** adds [`Deserialize`][url-deserialize] implementations for some types. It also allow deserializing [`humantime`][url-humantime] using [`humantime-serde`][url-humantime-serde]
- **clap**: adds [`Args`][url-clap-args] implementation to [`Owiwi`][url-owiwi-struct] and various other types.

## Supported Rust versions

The minimum supported Rust version is **1.94.0**.

## License

Unless otherwise noted, this project is licensed under the [Mozilla Public License Version 2.0][url-license].

### Acknowledgments
This project was inspired by this [blog][url-instrumenting-axum-blog] post.


[badge-actions]: https://github.com/aklanti/owiwi-tracing-opentelemetry/workflows/CI/badge.svg
[url-actions]: https://github.com/aklanti/owiwi-tracing-opentelemetry/actions/workflows/main.yaml
[badge-crate]: https://img.shields.io/crates/v/owiwi-tracing-opentelemetry
[url-crate]: https://crates.io/crates/owiwi-tracing-opentelemetry
[badge-docs]: https://img.shields.io/docsrs/owiwi-tracing-opentelemetry/latest
[url-docs]: https://docs.rs/owiwi-tracing-opentelemetry/latest/owiwi-tracing-opentelemetry
[badge-license]: https://img.shields.io/badge/License-MPL_2.0-blue.svg
[url-license]: LICENSE
[url-serde-serialize]: https://docs.rs/serde/1/serde/trait.Serialize.html
[url-serde-deserialize]: https://docs.rs/serde/1/serde/trait.Deserialize.html
[url-humantime]: https://docs.rs/humantime/2/humantime/
[url-humantime-serde]: https://docs.rs/humantime-serde/1/humantime_serde/
[url-clap-args]: https://docs.rs/clap/4/clap/trait.Args.html
[url-owiwi-struct]: https://docs.rs/owiwi-tracing-opentelemetry/latest/owiwi/struct.Owiwi.html
[url-trace-collector]: https://docs.rs/owiwi-tracing-opentelemetry/latest/trace/collector/index.html
[url-instrumenting-axum-blog]: https://determinate.systems/blog/instrumenting-axum/ 
