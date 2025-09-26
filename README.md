[![Crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![MIT licensed][mit-badge]][mit-license]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/owiwi
[crates-url]: https://crates.io/crates/owiwi
[docs-badge]: https://img.shields.io/docsrs/owiwi/latest
[docs-url]: https://docs.rs/owiwi/latest/owiwi/
[mit-badge]: https://img.shields.io/badge/license-MIT-blue
[mit-license]: LICENSE
[actions-badge]: https://github.com/aklanti/owiwi/workflows/CI/badge.svg
[actions-url]: https://github.com/aklanti/owiwi/actions/workflows/main.yaml



## Overview

Owiwi is a library for initializing tracing subscriber with OpenTelemetry


## Usage

To use `owiwi`, add the following to your `Cargo.toml`

```toml
[dependencies]
owiwi = "0.2"
tracing = "0.1"
```

Then initiate the subscriber using the `init()` method

```rust
use owiwi::Owiwi

fn main() {
  // Initialize the subscriber with an exporter that prints telemetry
  // (logs, metrics and traces) to the standard output.
  Owiwi::init("demo-service");
  tracing::info!("All good!");
}
```

You can also initialize the subscriber in asynchronous code.

```rust
use owiwi::Owiwi
use tracing::info;

async fn main() {
  // Initialize the subscriber with an exporter that prints telemetry
  // (logs, metrics and traces) to the standard output.
  Owiwi::init("demo-service");
  tracing::info!("All good!");
}
```

## Optional features
By default `owiwi` depends on `clap` and `serde`. There are optional features that enable 
- `bon` adds [`bon::Builder`][bon-builder-url] implementation for [`TracerProviderOptions`][tracer-provider-doc] and [`Owiwi`][owiwi-doc]
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


[bon-builder-url]: https://docs.rs/bon/latest/bon/derive.Builder.html
[tracer-provider-doc]: https://docs.rs/owiwi/latest/owiwi/provider/struct.TracerProviderOptions.html 
[owiwi-doc]: https://docs.rs/owiwi/latest/owiwi/struct.Owiwi.html
[instrumenting-axum]: https://determinate.systems/blog/instrumenting-axum/ 
