[![Build Status][badge-actions]][url-actions]
[![Crates.io][badge-crate]][url-crate]
[![Documentation][badge-docs]][url-docs]
[![MPL-2.0 license][badge-license]][url-license]

# owiwi

Opinionated [`tracing`][url-tracing] subscriber with OpenTelemetry export.

## Install

```toml
[dependencies]
owiwi = { version = "0.1.0", features = ["console"] }
tracing = "0.1"
```

## Usage

```rust,no_run
use owiwi::Owiwi;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut owiwi = Owiwi::new();
    owiwi.service_name = "my-service".to_owned();
    let guard = owiwi.try_init_console()?;

    tracing::info!("credential issues");

    guard.shutdown()?;
    Ok(())
}
```

Hold the returned [`OwiwiGuard`][url-owiwi-guard] until shutdown. Dropping it stops export.

## OTLP export

```rust,no_run
use std::time::Duration;
use owiwi::{Owiwi, OtlpConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = OtlpConfig::builder()
        .endpoint("http://localhost:4317".parse()?)
        .timeout(Duration::from_secs(10))
        .build();
    let guard = Owiwi::new().try_init(config)?;

    tracing::info!("credential issues");

    guard.shutdown()?;
    Ok(())
}
```

## CLI integration

Flatten `Owiwi` into your CLI struct. Requires the `clap` feature.

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
owiwi = { version = "0.1.0", features = ["clap", "honeycomb"] }
tracing = "0.1"
```

```rust,no_run
use clap::Parser;
use owiwi::{Owiwi, HoneycombConfig};

#[derive(Debug, Clone, Parser)]
struct Cli {
    #[command(flatten)]
    owiwi: Owiwi,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let config = HoneycombConfig::builder()
        .endpoint("https://api.honeycomb.io".parse()?)
        .api_key("your-api-key".into())
        .timeout(std::time::Duration::from_secs(5))
        .build();
    let guard = cli.owiwi.try_init(config)?;
    guard.shutdown()?;
    Ok(())
}
```

## Backends

| Backend | Config type | Feature |
|---------|-------------|---------|
| Any OTLP collector | `OtlpConfig` | *(default)* |
| Console (stdout) | — | `console` |
| [Honeycomb](https://honeycomb.io) | `HoneycombConfig` | `honeycomb` |

## Environment variables

Per the [OpenTelemetry spec][url-otel-env]. With `clap`, each has a CLI flag.

| Variable | Flag | |
|----------|------|-|
| `OTEL_SERVICE_NAME` | `--otel-service-name` | Service name |
| `OTEL_SDK_DISABLED` | `--otel-sdk-disabled` | Disable telemetry |
| `OTEL_RESOURCE_ATTRIBUTES` | `--otel-resource-attributes` | `key=value,key=value` |
| `OTEL_EXPORTER_OTLP_ENDPOINT` | `--otlp-endpoint` | Exporter endpoint |
| `OTEL_EXPORTER_OTLP_HEADERS` | `--otlp-headers` | Extra gRPC headers |
| `OTEL_EXPORTER_OTLP_TIMEOUT` | `--otlp-timeout` | Export timeout |
| `OTEL_TRACES_EXPORTER` | `--trace-exporter` | `console` · `honeycomb` · `otlp` |
| `OTEL_METRICS_EXPORTER` | `--metrics-exporter` | `console` · `prometheus` |
| `RUST_LOG` | `--trace-directive` | `info` · `my_crate=debug` |

## Features

| Feature | | Default |
|---------|--|---------|
| `clap` | CLI flags via [`clap::Args`][url-clap-args] | yes |
| `serde` | [`Deserialize`][url-serde-deserialize] on config types | yes |
| `console` | Stdout exporters | no |
| `honeycomb` | [Honeycomb](https://honeycomb.io) exporter | no |
| `metrics` | Metrics via `SdkMeterProvider` | no |
| `prometheus` | Prometheus OTLP export (implies `metrics`) | no |
| `full` | Everything | no |

## MSRV

1.94.0

## Acknowledgments

Inspired by [Instrumenting Axum][url-instrumenting-axum-blog].

## License

[MPL-2.0][url-license]

[badge-actions]: https://github.com/aklanti/owiwi/workflows/CI/badge.svg
[url-actions]: https://github.com/aklanti/owiwi/actions/workflows/main.yaml
[badge-crate]: https://img.shields.io/crates/v/owiwi
[url-crate]: https://crates.io/crates/owiwi
[badge-docs]: https://img.shields.io/docsrs/owiwi/latest
[url-docs]: https://docs.rs/owiwi/latest/owiwi
[badge-license]: https://img.shields.io/badge/License-MPL_2.0-blue.svg
[url-license]: LICENSE
[url-clap-args]: https://docs.rs/clap/4/clap/trait.Args.html
[url-serde-deserialize]: https://docs.rs/serde/1/serde/trait.Deserialize.html
[url-tracing]: https://docs.rs/tracing/0.1
[url-owiwi-guard]: https://docs.rs/owiwi/latest/owiwi/struct.OwiwiGuard.html
[url-otel-env]: https://opentelemetry.io/docs/specs/otel/configuration/sdk-environment-variables/
[url-instrumenting-axum-blog]: https://determinate.systems/blog/instrumenting-axum/
