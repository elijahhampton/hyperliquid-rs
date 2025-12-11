mod api;
mod client;
mod error;

/// Utilities for examples and testing.
///
/// Note: This module is primarily intended for examples and
/// may change between versions.
#[allow(dead_code)]
pub mod example_helpers;
mod types;
pub mod utils;

mod chain;
/// Tracing initialization utilities.
pub mod init_tracing;
mod signature;

pub mod prelude {
    pub use crate::api::response;
    pub use crate::client::{HyperliquidClient, HyperliquidClientBuilder};
    pub use crate::error::{HyperliquidError, Result};
    pub use crate::types::{exchange, info};
}
