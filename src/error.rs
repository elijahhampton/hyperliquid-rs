use alloy::primitives::SignatureError;
use std::{result::Result as StdResult, string::FromUtf8Error, sync::mpsc::SendError};
use thiserror::Error;
use tokio::sync::broadcast::error::RecvError;

use crate::{api::StreamMessage, types::ws::SubscriptionKey};

#[derive(Error, Debug)]
pub enum SubscriptionError {
    #[error("Subscription already exist for method {method}")]
    SubscriptionExist { method: String },
    #[error("Caller not subscribed to the {0} feed.")]
    MissingSubscription(SubscriptionKey),
}

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
    #[error("{0}")]
    SubscriptionError(#[from] SubscriptionError),
    #[error("Function requires wallet/signer.")]
    SignerRequired,
    #[error("{0}")]
    SendError(#[from] SendError<StreamMessage>),
    #[error("{0}")]
    RecvError(#[from] RecvError),
    #[error("{0}")]
    WebSocketError(String),
    #[error("{0}")]
    AlloySignError(#[from] alloy::signers::Error),
    #[error("{0}")]
    RmpSerde(#[from] rmp_serde::encode::Error),
    #[error("{0}")]
    FromUtf8Error(#[from] FromUtf8Error),
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    #[error("{0}")]
    TungsteniteError(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("{0}")]
    SignatureError(#[from] SignatureError),
    #[error("Missing configuration for parameter {parameter}")]
    MissingConfiguration { parameter: String },
    #[error("{0}")]
    JsonSerialization(#[from] serde_json::Error),
    #[error["{0}"]]
    Internal(String),
    #[error["{0}"]]
    InvalidPrice(String),
    #[error["{0}"]]
    SignatureFailure(String),
    #[error["{0}"]]
    Wallet(String),
    #[error["{0}"]]
    GenericParse(String),
}

impl<T> From<tokio::sync::mpsc::error::SendError<T>> for HyperliquidError {
    fn from(e: tokio::sync::mpsc::error::SendError<T>) -> Self {
        Self::WebSocketError(format!("Channel send failed: {}", e))
    }
}

pub type Result<T> = StdResult<T, HyperliquidError>;
