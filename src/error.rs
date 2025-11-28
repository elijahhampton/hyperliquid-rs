use reqwest::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HyperliquidError {
    #[error("{0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("Failed to parse response body: {0}")]
    InvalidResponse(String),
    #[error("API error {status}: {body}")]
    Api { status: u16, body: String },
    #[error("Invalid request parameters for method '{method}' for parameter '{parameter}'.")]
    InvalidRequestParameter { method: String, parameter: String, reason: String },
}

pub type Result<T> = std::result::Result<T, HyperliquidError>;
