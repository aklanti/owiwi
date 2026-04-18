# Changelog

## [3.0.0] - 2026-04-19

### Added

- `TraceExporter` enum for trace backend selection (`Otlp`, `Honeycomb`, `Console`)
- `MetricExporter` enum for metric backend selection (`None`, `Prometheus`, `Console`)
- `Owiwi::sampler` field (moved from `OtlpConfig`), applies to any trace backend
- `TraceExporter::build_provider` and `MetricExporter::build_provider` for constructing providers directly
- `OtlpConfig::default()` reads `OTEL_EXPORTER_OTLP_ENDPOINT`, `OTEL_EXPORTER_OTLP_TIMEOUT`, and `OTEL_EXPORTER_OTLP_HEADERS` env vars

### Changed

- Single `try_init` method dispatches on `trace_exporter` and `metric_exporter` fields
- `Owiwi` fields are now public again; mutation after clap parsing is supported
- `is_disabled` takes `&self` (no longer mutates `no_telemetry`)
- `disable_sdk` field renamed to `no_telemetry`
- Service name resolution now reads `OTEL_SERVICE_NAME` in the builder path (previously only the clap path read it)
- `OtlpConfig` no longer derives `clap::Args`. Endpoint, timeout, headers, and TLS are programmatic only.
- `HoneycombConfig` converts to `OtlpConfig` via `From` (trait-based wiring removed)
- `DEFAULT_OTLP_TIMEOUT` is a `Duration` const instead of a parsed string

### Removed

- `Owiwi::try_init_with` method. Replaced by `TraceExporter::Honeycomb` + `try_init`.
- `Owiwi::try_init_console` method. Replaced by `TraceExporter::Console` + `try_init`.
- `Owiwi::try_init_with_metrics` method. Set `metric_exporter` field and call `try_init`.
- `Owiwi::new`. Use `Owiwi::default()` or `Owiwi::builder().build()`.
- `OtlpConfig::new`. Use `OtlpConfig::default()` or `OtlpConfig::builder()...build()`.
- `SpanExporterConfig` trait
- `OtlpConfig::init_provider`. Use `TraceExporter::build_provider` instead.
- `OtlpConfig::sampler` field. Moved to `Owiwi::sampler`.
- `OtlpConfig::build_exporter` is now pub (was indirectly called via trait)
- `--otlp-endpoint`, `--otlp-timeout`, `--otlp-headers` CLI flags (and `env = OTEL_EXPORTER_OTLP_*` wiring)

### Migration

```diff
- let guard = Owiwi::new().try_init_console()?;
+ let guard = Owiwi::builder()
+     .trace_exporter(TraceExporter::Console)
+     .build()
+     .try_init()?;

- let guard = Owiwi::builder().build().try_init_with(honeycomb_config)?;
+ let guard = Owiwi::builder()
+     .trace_exporter(TraceExporter::Honeycomb(honeycomb_config))
+     .build()
+     .try_init()?;

- let guard = owiwi.try_init_with_metrics(prom_config)?;
+ let guard = Owiwi::builder()
+     .metric_exporter(MetricExporter::Prometheus(prom_config))
+     .build()
+     .try_init()?;
```

CLI users of 2.x who relied on `--otlp-endpoint` etc. must configure OTLP programmatically or flatten `OtlpConfig` separately into their CLI struct.

## [2.1.0] - 2026-04-18

### Changed

- `--otlp-endpoint` defaults to `http://localhost:4317` and `--otlp-timeout` defaults to `10s`. CLI parsing no longer fails when these flags and env vars are absent.

## [2.0.0] - 2026-04-10

### Added

- `SpanExporterConfig` trait for custom exporter backends
- `Owiwi::try_init_with` method accepting any `impl SpanExporterConfig`
- `FilterHandle` for runtime filter reloading via `OwiwiGuard::filter_handle`
- Per-layer filtering: independent `EnvFilter` for fmt and OTel export layers
- `--export-directive` CLI flag and `OWIWI_EXPORT_LOG` env var for export filter control
- `--no-telemetry` CLI flag (replaces `--disable-sdk`)
- `--metrics-interval` env var support (`OWIWI_METRICS_INTERVAL`)
- `OTEL_TRACES_SAMPLER` and `OTEL_TRACES_SAMPLER_ARG` env var support
- `OtlpConfig::sampler` field for programmatic sampler configuration
- `PrometheusConfig::tls_config` field for custom TLS certificates
- `PrometheusConfig::headers` field for auth headers
- `NoTokioRuntime` error variant (returns error instead of panicking)
- Help text with format hints on all CLI flags
- `Debug` impl for `FilterHandle` and `OwiwiGuard`

### Changed

- `Owiwi` fields are now private; use the builder for construction
- `Owiwi::try_init` no longer takes a config argument; uses the builder's `OtlpConfig`
- `OtlpConfig` absorbs `TracerProviderOptions` (endpoint, timeout, headers, sampler, TLS)
- `MeterProviderOptions` removed; `metrics_interval` inlined onto `Owiwi`
- `HoneycombConfig` implements `SpanExporterConfig` directly instead of `From<HoneycombConfig> for OtlpConfig`
- `OtlpConfig` implements `Default` with spec values (`http://localhost:4317`, 10s timeout)
- `global::set_meter_provider` moved from `MeterProviderOptions::init_provider` to `Owiwi::finish`
- Sampler handling extracted into `build_tracer_provider` (shared by `try_init` and `try_init_with`)
- Shutdown uses `mem::take` before provider shutdown (panic-safe)
- `HELP_HEADING` no longer gated behind `clap` feature
- `--service-name` replaces `--otel-service-name`
- `--resource-attrs` replaces `--otel-resource-attributes`
- `--trace-directive` requires at least one value (`num_args = 1..`)

### Removed

- `TracerProviderOptions` struct
- `MeterProviderOptions` struct
- `From<HoneycombConfig> for OtlpConfig` impl
- `TryFrom<OtlpConfig> for SpanExporter` impl (replaced by `SpanExporterConfig` trait)

## [1.1.0] - 2026-03-18

### Removed

- `TraceBackend` and `MetricBackend` enums
- `--trace-exporter` and `--metrics-exporter` CLI flags
- `OTEL_TRACES_EXPORTER` and `OTEL_METRICS_EXPORTER` env var handling
- `full` feature

## [1.0.0] - 2026-03-18

Initial release.

### Added

- Core `Owiwi` configuration struct with builder pattern (via `bon`)
- `OwiwiGuard` RAII guard for tracer and meter provider lifecycle
- Three trace export backends: OTLP (default), Console, and Honeycomb
- Two metrics export backends: Console and Prometheus
- `OtlpConfig`, `HoneycombConfig`, and `PrometheusConfig` builder structs
- `EventFormat` enum with `Compact`, `Full`, and `Pretty` output styles
- OpenTelemetry environment variable support (`OTEL_SERVICE_NAME`, `OTEL_SDK_DISABLED`,
  `OTEL_RESOURCE_ATTRIBUTES`, `OTEL_EXPORTER_OTLP_ENDPOINT`, and others)
- TLS auto-detection for HTTPS endpoints with custom `ClientTlsConfig` support
- Secure API key handling via `secrecy` for Honeycomb
- `clap` feature: flatten `Owiwi` into CLI parsers with `--verbose` support
- `serde` feature: deserialize all config types with `humantime` duration parsing
- `console` feature: stdout exporters for local development
- `honeycomb` feature: Honeycomb.io trace export
- `metrics` feature: `SdkMeterProvider` setup with periodic reader
- `prometheus` feature: Prometheus metrics export (implies `metrics`)

[3.0.0]: https://github.com/aklanti/owiwi/compare/v2.1.0...HEAD
[2.1.0]: https://github.com/aklanti/owiwi/compare/v2.0.0...v2.1.0
[2.0.0]: https://github.com/aklanti/owiwi/compare/v1.1.0...v2.0.0
[1.1.0]: https://github.com/aklanti/owiwi/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/aklanti/owiwi/releases/tag/v1.0.0
