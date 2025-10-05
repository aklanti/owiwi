//! This module defines the abstraction for exporting metrics
pub mod collector;

#[doc(inline)]
pub use collector::{MetricOptions, MetricsConfig};
