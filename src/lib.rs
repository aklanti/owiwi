//! [![Crates.io][crates-badge]][crates-url]
//! [![Documentation][docs-badge]][docs-url]
//!
//! [crates-badge]: https://img.shields.io/crates/v/owiwi-tracing-opentelemetry
//! [crates-url]: https://crates.io/crates/owiwi-tracing-opentelemetry
//! [docs-badge]: https://img.shields.io/docsrs/owiwi-tracing-opentelemetry/latest
//! [docs-url]: https://docs.rs/owiwi/latest/owiwi-tracing-opentelemetry/
//!
//! # Overview
//!
//! `owiwi-tracing-opentelemetry` is a crate that provides an opinionated abstraction for initializing tracing subscriber with OpenTelemetry.
//!
//! It allows sending telemetry to any of the collector define in [`collector::Collector`].
//!
//! <br/>
//!
//! # Usage
//!
//!The `owiwi-tracing-opentelemetry` crate is [on crates.io](https://crates.io/crates/owiwi-tracing-opentelemetry) and can be
//! used by adding `owiwi-tracing-opentelemetry` to your dependencies in your project's `Cargo.toml`.
//! Or more simply, just run `cargo add owiwi-tracing-opentelemetry`.
//!
//! Additionally, You must add the tracing crate to your dependencies.
//!
//! <br/>
//!
//! ## Example with the feature `clap`
//! The main type of this crate is originally design to work binary application that defines a command line interface, we need to enable the `clap` flag.
//!
//! ```toml
//! [dependencies]
//! clap = { version = "4.5.48", features = ["derive"] }
//! owiwi-tracing-opentelemetry = { version = "0.1.0", features = ["clap"] }
//! tracing = "0.1"
//! ```
//!
//! The following is a complete program that initializes a subscriber and emit some traces.
//!
//! ```ignore
//! use clap::Parser;
//! use owiwi_tracing_opentelemetry::Owiwi;
//! use owiwi_tracing_opentelemetry::collector::{CollectorConfig, HoneycombConfig};
//!
//! #[derive(Debug, Clone, Parser)]
//! struct Cli {
//!     #[command(flatten)]
//!     owiwi: Owiwi,
//! }
//!
//! fn main() {
//!     let cli = Cli::parse();
//!     // Create a configuration to send traces to honeycomb.io
//!     let honeycomb_config = HoneycombConfig.builder()
//!         .endpoint("https://api.honeycomb.io/traces/api".parse().expect("to be valid URL"))
//!         .api_key("super_secret_key".into())
//!         .timeout(std::time::Duration::from_secs(5))
//!         .build();
//!     let collector_config = CollectorConfig::Honeycomb(honeycomb_config);
//!     let _guard = cli.owiwi.init("example", collector_config);
//!     tracing::info!("the subscriber was initialized");
//! }
//!```
//!
//! <br/>
//!
//! ## Example without the feature `clap`
//!
//! The following is a complete program that initializes a subscriber and emit some traces.
//!
//! ```
//! use owiwi_tracing_opentelemetry::Owiwi;
//! use owiwi_tracing_opentelemetry::collector::CollectorConfig;
//! use owiwi_tracing_opentelemetry::format::EventFormat;
//!
//! fn main() {
//!     // The default collector configuration sends traces to std::io::stdout
//!     let collector_config = CollectorConfig::default();
//!     let service_name = "example";
//!     // Initializes the subscriber
//!     let _guard = Owiwi::default().init(service_name,  collector_config);
//!     tracing::info!("the Subscriber was initialized!");
//! }
//! ```
//!
//! <br/>
//!
//! # Optional features
//!
//! There are some optional features that enable additional dependencies:
//! - `serde` adds [`Deserialize`](https://docs.rs/serde/1/serde/trait.Deserialize.html) implementations for some types. It also allow deserializing [`humantime`](https://docs.rs/humantime/2/humantime/) using [`humantime-serde`](https://docs.rs/humantime-serde/1/humantime_serde/)
//! - `clap`: adds [`Args`](https://docs.rs/clap/4/clap/trait.Args.html) implementation to [`Owiwi`] and various other types.
//!
#![cfg_attr(test, deny(warnings))]

pub mod collector;
pub mod env_vars;
pub mod error;
pub mod format;
pub mod owiwi;
pub mod provider;

#[doc(inline)]
pub use error::{Error, Result};
#[doc(inline)]
pub use owiwi::Owiwi;

#[cfg(feature = "clap")]
/// Help heading for instrumentation options
pub const HELP_HEADING: &str = "Instrumentation options";
