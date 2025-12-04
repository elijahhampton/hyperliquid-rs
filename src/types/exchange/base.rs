use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RestingOrder {
    pub oid: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrderStatus {
    Resting { resting: RestingOrder },
    Error { error: String },
}

impl OrderStatus {
    pub fn is_resting(&self) -> bool {
        matches!(self, Self::Resting { .. })
    }

    pub fn oid(&self) -> Option<u64> {
        match self {
            Self::Resting { resting } => Some(resting.oid),
            Self::Error { .. } => None,
        }
    }

    pub fn error_message(&self) -> Option<&str> {
        match self {
            Self::Error { error } => Some(error),
            Self::Resting { .. } => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CancelStatus {
    Success(String),
    Error { error: String },
}

impl CancelStatus {
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success(s) if s == "success")
    }

    pub fn error_message(&self) -> Option<&str> {
        match self {
            Self::Error { error } => Some(error),
            Self::Success(_) => None,
        }
    }
}

/// Response type for TWAP order
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunningTwap {
    pub twap_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwapStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running: Option<RunningTwap>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwapOrderResponseData {
    pub status: TwapStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwapOrderResponseInner {
    /// "twapOrder"
    #[serde(rename = "type")]
    pub type_: String,
    pub data: TwapOrderResponseData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwapOrderResponse {
    /// "ok"
    pub status: String,
    pub response: TwapOrderResponseInner,
}

/// Response type for TWAP cancel
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwapCancelResponseData {
    /// "success"
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwapCancelResponseInner {
    /// "twapCancel"
    #[serde(rename = "type")]
    pub type_: String,
    pub data: TwapCancelResponseData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwapCancelResponse {
    /// "ok"
    pub status: String,
    pub response: TwapCancelResponseInner,
}

/// Generic default response for actions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultResponseInner {
    /// "default"
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultResponse {
    /// "ok"
    pub status: String,
    pub response: DefaultResponseInner,
}
