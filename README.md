[![Build Status][badge-actions]][url-actions]
[![Crates.io][badge-crate]][url-crate]
[![Documentation][badge-docs]][url-docs]
[![MPL-2.0 license][badge-license]][url-license]

# owiwi

Opinionated [`tracing`][url-tracing] subscriber with OpenTelemetry export.

## Known Limitations

- gRPC only. `OTEL_EXPORTER_OTLP_PROTOCOL` ignored
- Backend selection is programmatic. `OTEL_TRACES_EXPORTER` and `OTEL_METRICS_EXPORTER` are not read.
- OTLP endpoint/timeout/headers are not exposed as CLI flags. They are read from `OTEL_EXPORTER_OTLP_ENDPOINT`, `OTEL_EXPORTER_OTLP_TIMEOUT`, and `OTEL_EXPORTER_OTLP_HEADERS` via `OtlpConfig::default()`, or set programmatically via the builder.

## Install

```toml
[dependencies]
owiwi = { version = "2.0.0", features = ["console"] }
tracing = "0.1"
```

## Usage

Default: OTLP export to `http://localhost:4317`.

```rust,no_run
use owiwi::Owiwi;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let guard = Owiwi::builder()
        .service_name("my-service")
        .build()
        .try_init()?;

    tracing::info!("credential issues");

    guard.shutdown()?;
    Ok(())
}
```

Hold the returned [`OwiwiGuard`][url-owiwi-guard] until shutdown. Dropping it stops export.

## Custom OTLP endpoint

```rust,no_run
use std::time::Duration;
use owiwi::{Owiwi, TraceExporter, OtlpConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let otlp = OtlpConfig::builder()
        .endpoint("http://collector.internal:4317".parse()?)
        .timeout(Duration::from_secs(30))
        .build();

    let guard = Owiwi::builder()
        .service_name("my-service")
        .trace_exporter(TraceExporter::Otlp(otlp))
        .build()
        .try_init()?;

    guard.shutdown()?;
    Ok(())
}
```

## Console (stdout)

```rust,no_run
use owiwi::{Owiwi, TraceExporter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let guard = Owiwi::builder()
        .service_name("my-service")
        .trace_exporter(TraceExporter::Console)
        .build()
        .try_init()?;

    guard.shutdown()?;
    Ok(())
}
```

## Honeycomb

```rust,no_run
use std::time::Duration;
use owiwi::{Owiwi, TraceExporter, HoneycombConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let honeycomb = HoneycombConfig::builder()
        .endpoint("https://api.honeycomb.io".parse()?)
        .api_key("your-api-key".into())
        .timeout(Duration::from_secs(5))
        .build();

    let guard = Owiwi::builder()
        .service_name("my-service")
        .trace_exporter(TraceExporter::Honeycomb(honeycomb))
        .build()
        .try_init()?;

    guard.shutdown()?;
    Ok(())
}
```

## Metrics

Set `metric_exporter` alongside `trace_exporter`:

```rust,no_run
use owiwi::{Owiwi, MetricExporter, PrometheusConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let prom = PrometheusConfig::builder()
        .endpoint("http://prometheus:9090".parse()?)
        .build();

    let guard = Owiwi::builder()
        .service_name("my-service")
        .metric_exporter(MetricExporter::Prometheus(prom))
        .build()
        .try_init()?;

    guard.shutdown()?;
    Ok(())
}
```

## CLI Integration

Flatten `Owiwi` into your CLI struct for the non-backend options (service name,
resource attributes, filters, event format, verbosity, disable flag). Backend
selection remains programmatic.

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
owiwi = { version = "2.0.0", features = ["clap"] }
tracing = "0.1"
```

```rust,no_run
use clap::Parser;
use owiwi::Owiwi;

#[derive(Debug, Clone, Parser)]
struct Cli {
    #[command(flatten)]
    owiwi: Owiwi,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let guard = cli.owiwi.try_init()?;
    guard.shutdown()?;
    Ok(())
}
```

## Backends

| Backend | Variant | Feature |
|---------|---------|---------|
| Any OTLP collector | TraceExporter::Otlp(OtlpConfig) | *(default)* |
| Console (stdout) | TraceExporter::Console | console |
| [Honeycomb](https://honeycomb.io) | TraceExporter::Honeycomb(HoneycombConfig) | honeycomb |
| Prometheus metrics | MetricExporter::Prometheus(PrometheusConfig) | prometheus |
| Console metrics | MetricExporter::Console | console + metrics |

## Environment Variables

Per the [OpenTelemetry spec][url-otel-env] where applicable. With `clap`, each flagged variable also has a CLI flag.

| Variable | Flag | |
|----------|------|-|
| OTEL_SERVICE_NAME | --service-name | Service name |
| OTEL_SDK_DISABLED | --no-telemetry | Disable telemetry |
| OTEL_RESOURCE_ATTRIBUTES | --resource-attrs | key=value,key=value |
| OTEL_EXPORTER_OTLP_ENDPOINT |  | OTLP endpoint (read by OtlpConfig::default) |
| OTEL_EXPORTER_OTLP_TIMEOUT |  | OTLP timeout (read by OtlpConfig::default) |
| OTEL_EXPORTER_OTLP_HEADERS |  | OTLP headers (read by OtlpConfig::default) |
| OTEL_TRACES_SAMPLER |  | always_on, always_off, or traceidratio |
| OTEL_TRACES_SAMPLER_ARG |  | Sampler argument (e.g. ratio for traceidratio) |
| RUST_LOG | --trace-directive | Terminal filter (info, my_crate=debug) |
| OWIWI_EXPORT_LOG | --export-directive | Export filter (default: info) |
| OWIWI_METRICS_INTERVAL | --metrics-interval | Metrics export interval (e.g. 30s) |

## Features

| Feature | | Default |
|---------|--|---------|
| clap | CLI flags via [clap::Args][url-clap-args] | yes |
| serde | [Deserialize][url-serde-deserialize] on config types | yes |
| console | Stdout exporters | no |
| honeycomb | [Honeycomb](https://honeycomb.io) exporter | no |
| metrics | Metrics via SdkMeterProvider | no |
| prometheus | Prometheus OTLP export (implies metrics) | no |

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
