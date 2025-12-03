/// Request and response types for the exchange endpoint used to interact with and trade on the Hyperliquid chain.
use serde::{Deserialize, Serialize};

/// Response type for successful order placement (resting)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RestingOrder {
    pub oid: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resting: Option<RestingOrder>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponseData {
    pub statuses: Vec<OrderStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrderResponseInner {
    Error(String),
    Ok(OrderResponseInnerOk),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponseInnerOk {
    #[serde(rename = "type")]
    pub type_: String,
    pub data: OrderResponseData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    pub status: String,
    pub response: OrderResponseInner,
}

/// Response type for cancel actions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelResponseData {
    /// "success" | error message
    pub statuses: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelResponseInner {
    /// "cancel"
    #[serde(rename = "type")]
    pub type_: String,
    pub data: CancelResponseData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelResponse {
    /// "ok"
    pub status: String,
    pub response: CancelResponseInner,
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
