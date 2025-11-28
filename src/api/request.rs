use crate::error::{HyperliquidError, Result};
use once_cell::sync::Lazy;
use reqwest::{Client, Error, Response};
use serde::Serialize;
use std::collections::{HashMap, HashSet};

use std::time::{SystemTime, UNIX_EPOCH};

pub fn current_time_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time before UNIX epoch!")
        .as_millis() as u64
}

pub static SUPPORTED_INTERVALS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    [
        "1m", "3m", "5m", "15m", "30m",
        "1h", "2h", "4h", "8h", "12h",
        "1d", "3d", "1w", "1M",
    ].into_iter().collect()
});


pub async fn post_json<T>(client: &reqwest::Client, url: &str, body: serde_json::Value) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let res = client
        .post(url)
        .json(&body)
        .send()
        .await?
        .error_for_status() // turns 4xx/5xx into Err(reqwest::Error)
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

    Ok(res.json().await?)
}
