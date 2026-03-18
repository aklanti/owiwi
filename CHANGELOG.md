# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

[1.1.0]: https://github.com/aklanti/owiwi/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/aklanti/owiwi/releases/tag/v1.0.0
