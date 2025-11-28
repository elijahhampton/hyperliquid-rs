use reqwest::Response;
use crate::error::HyperliquidError;

pub async fn err_from_status_code(response: Response) -> HyperliquidError {
    let status = response.status().as_u16();
    let body = response.text().await.map_err(|e| HyperliquidError::InvalidResponse(e.to_string()))?;

    HyperliquidError::Api {
        status,
        body,
    }
}
