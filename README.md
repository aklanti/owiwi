![LICENSE-MIT](https://img.shields.io/badge/license-MIT-blue)

# Overview

Owiwi is a library for initializing tracing subscriber with OpenTelemetry


# Usage

To use `owiwi`, add the following to your `Cargo.toml`

```toml
[dependencies]
owiwi = "0.1"
tracing = "0.1"
```

Then initiate the subscriber using the `init()` method

```rust
use owiwi::Owiwi
use tracing::info;

fn main() {
  Owiwi::init("demo-service");
  info!("All good!");
}
```

You can also initialize the subscriber in asynchronous code.

```rust
use owiwi::Owiwi
use tracing::info;

async fn main() {
  Owiwi::init("demo-service");
  info!("All good!");
}
```

# Supported Rust Versions
`Owiwi` currently only support the latest stable version.

## License

This project is licensed under the [MIT license](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Owiwi by you, shall be licensed as MIT, without any additional
terms or conditions.

