use crate::error::HyperliquidError::InvalidRequestParameter;
use crate::error::{HyperliquidError, Result};
use once_cell::sync::Lazy;
use serde::de::DeserializeOwned;
use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing;

pub static SUPPORTED_INTERVALS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    [
        "1m", "3m", "5m", "15m", "30m", "1h", "2h", "4h", "8h", "12h", "1d", "3d", "1w", "1M",
    ]
    .into_iter()
    .collect()
});

#[inline]
pub const fn err_request_invalid_hyperliquid_address(
    method: String,
    parameter: String,
    reason: String,
) -> HyperliquidError {
    InvalidRequestParameter {
        method,
        parameter,
        reason,
    }
}

#[allow(clippy::expect_used)]
pub fn current_time_millis() -> u64 {
    let dur = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time before UNIX epoch!");

    dur.as_secs()
        .saturating_mul(1_000)
        .saturating_add(u64::from(dur.subsec_millis()))
}

pub async fn post_json<T>(client: &reqwest::Client, url: &str, body: serde_json::Value) -> Result<T>
where
    T: DeserializeOwned,
{
    tracing::trace!("Executing POST request with payload {:?}", body);

    let res = client
        .post(url)
        .json(&body)
        .send()
        .await?
        .error_for_status()
        .map_err(|e| {
            if let Some(status) = e.status() {
                HyperliquidError::Api {
                    status: status.as_u16(),
                    body: e.to_string(),
                }
            } else {
                HyperliquidError::from(e)
            }
        })?;

    let text = res.text().await?;
    tracing::debug!("Server response: {}", text);

    let json: T = serde_json::from_str(&text)?;
    Ok(json)
}
