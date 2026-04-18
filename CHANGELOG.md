# Changelog

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

[2.1.0]: https://github.com/aklanti/owiwi/compare/v2.0.0...HEAD
[2.0.0]: https://github.com/aklanti/owiwi/compare/v1.1.0...v2.0.0
[1.1.0]: https://github.com/aklanti/owiwi/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/aklanti/owiwi/releases/tag/v1.0.0
