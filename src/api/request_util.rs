use crate::error::HyperliquidError::InvalidRequestParameter;
use crate::error::{HyperliquidError, Result};
use once_cell::sync::Lazy;
use serde::de::DeserializeOwned;
use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing;

/// Hyperliquid supported time intervals for interval based request
pub static SUPPORTED_INTERVALS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    [
        "1m", "3m", "5m", "15m", "30m", "1h", "2h", "4h", "8h", "12h", "1d", "3d", "1w", "1M",
    ]
    .into_iter()
    .collect()
});

/// Helper function for creating [`InvalidRequestParameter`] errors.
#[inline]
pub(crate) const fn err_request_invalid_hyperliquid_address(
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

/// Returns the current time in millis
#[allow(clippy::expect_used)]
pub fn current_time_millis() -> u64 {
    let dur = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time before UNIX epoch!");

    dur.as_secs()
        .saturating_mul(1_000)
        .saturating_add(u64::from(dur.subsec_millis()))
}

/// Sends a POST request with a JSON body to a url.
pub(crate) async fn post_json<T>(
    client: &reqwest::Client,
    url: &str,
    body: serde_json::Value,
) -> Result<T>
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

/// Parses and normalizes strings representing Decimals, i.e. removes trailing zeros.
pub fn normalize_decimal(s: &str) -> String {
    // Parse as Decimal and normalize (removes trailing zeros)
    s.parse::<f64>()
        .ok()
        .map(|f| {
            let formatted = format!("{:.8}", f);
            let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
            if trimmed.is_empty() {
                "0".to_string()
            } else {
                trimmed.to_string()
            }
        })
        .unwrap_or_else(|| s.to_string())
}
