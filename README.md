[![Build Status][badge-actions]][url-actions]
[![Crates.io][badge-crate]][url-crate]
[![Documentation][badge-docs]][url-docs]
[![MPL-2.0 license][badge-license]][url-license]

## owiwi

An opinionated library for initializing tracing subscriber with OpenTelemetry.

It allows sending telemetry to any of the collector define in the [`trace::collector`][url-trace-collector] module.

## Usage

The `owiwi` crate is [on crates.io][url-crate] and can be
used by adding `owiwi` to your dependencies in your project's `Cargo.toml`.
Or more simply, just run `cargo add owiwi`.

Additionally, You must add the tracing crate to your dependencies.

### Example

The main type of this crate is originally design to work binary application that defines a command line interface, we need to enable the `clap` flag.

```toml
[dependencies]
clap = { version = "4.5.60", features = ["derive"] }
owiwi = { version = "0.1.0" features = ["clap"] }
tracing = "0.1"
```

The following is a complete program that initializes a subscriber and emit some traces.

```rust
use clap::Parser;
use owiwi::{Owiwi, HoneycombConfig, ConfigureExporter};

#[derive(Debug, Clone, Parser)]
struct Cli {
     #[command(flatten)]
     owiwi: Owiwi,
}

fn main() {
     let cli = Cli::parse();
     // Create a configuration to send traces to honeycomb.io
     let exporter_config = HoneycombConfig::builder()
         .endpoint("https://api.honeycomb.io/traces/api".parse().expect("to be valid URL"))
         .api_key("super_secret_key".into())
         .timeout(std::time::Duration::from_secs(5))
         .build();
     let _guard = cli.owiwi.try_init(exporter_config);
     tracing::info!("the subscriber was initialized");
}

```

### Example without `clap`

The following is a complete program that initializes a subscriber and emit some traces.

```rust
use owiwi::{Owiwi, EventFormat};

fn main() {
     // Initializes the subscriber
     let _guard = Owiwi::new("example".into()).try_init_console();
     tracing::info!("the Subscriber was initialized!");
}
```

## Optional features

There are some optional features that enable additional dependencies:
- **serde:** adds [`Deserialize`][url-serde-deserialize] implementations for some types. It also allow deserializing [`humantime`][url-humantime] using [`humantime-serde`][url-humantime-serde]
- **clap**: adds [`Args`][url-clap-args] implementation to [`Owiwi`][url-owiwi-struct] and various other types.

## Supported Rust versions

The minimum supported Rust version is **1.94.0**.

## License

Unless otherwise noted, this project is licensed under the [Mozilla Public License Version 2.0][url-license].

### Acknowledgments
This project was inspired by this [blog][url-instrumenting-axum-blog] post.


[badge-actions]: https://github.com/aklanti/owiwi/workflows/CI/badge.svg
[url-actions]: https://github.com/aklanti/owiwi/actions/workflows/main.yaml
[badge-crate]: https://img.shields.io/crates/v/owiwi
[url-crate]: https://crates.io/crates/owiwi
[badge-docs]: https://img.shields.io/docsrs/owiwi/latest
[url-docs]: https://docs.rs/owiwi/latest/owiwi
[badge-license]: https://img.shields.io/badge/License-MPL_2.0-blue.svg
[url-license]: LICENSE
[url-serde-serialize]: https://docs.rs/serde/1/serde/trait.Serialize.html
[url-serde-deserialize]: https://docs.rs/serde/1/serde/trait.Deserialize.html
[url-humantime]: https://docs.rs/humantime/2/humantime/
[url-humantime-serde]: https://docs.rs/humantime-serde/1/humantime_serde/
[url-clap-args]: https://docs.rs/clap/4/clap/trait.Args.html
[url-owiwi-struct]: https://docs.rs/owiwi/latest/owiwi/struct.Owiwi.html
[url-trace-collector]: https://docs.rs/owiwi/latest/trace/collector/index.html
[url-instrumenting-axum-blog]: https://determinate.systems/blog/instrumenting-axum/ 
