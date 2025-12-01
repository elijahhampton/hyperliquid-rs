use std::result::Result as StdResult;
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
    InvalidRequestParameter {
        method: String,
        parameter: String,
        reason: String,
    },
    #[error("Missing configuration for parameter {parameter}")]
    MissingConfiguration { parameter: String },
    #[error("{0}")]
    JsonSerialization(#[from] serde_json::Error),
    #[error["{0}"]]
    Internal(String),
    #[error["{0}"]]
    SignatureFailure(String),
    #[error["{0}"]]
    Wallet(String),
    #[error["{0}"]]
    GenericParse(String),
}

pub type Result<T> = StdResult<T, HyperliquidError>;
