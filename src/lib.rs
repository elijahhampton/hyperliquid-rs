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

/// Tracing initialization utilities.
pub mod init_tracing;
mod signature;

mod chain;

pub mod prelude {
    pub use crate::api::{
        request_util::{current_time_millis, normalize_decimal, SUPPORTED_INTERVALS},
        response,
    };
    pub use crate::client::{HyperliquidClient, HyperliquidClientBuilder};
    pub use crate::error::{HyperliquidError, Result};
    pub use crate::signature::agent::l1::Agent;
    pub use crate::signature::sign::{
        sig_v_from_bool, sign_l1_action, sign_typed_data, sign_user_signed_action,
    };
    pub use crate::types::exchange;
    pub use crate::types::info;
    pub use crate::types::ws;
}
