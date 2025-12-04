use crate::types::exchange::{base::CancelStatus, OrderStatus};
use serde::{Deserialize, Serialize};

/// General response type returns from a SDK function call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response<T> {
    pub status: String,
    pub response: ResponseInner<T>,
}

/// The `inner` portion of a response representing an OK response status, else an
/// error response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponseInner<T> {
    Error(String),
    Ok(T),
}

/// The response body given a successful request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseBody<T> {
    #[serde(rename = "type")]
    pub type_: String,
    pub data: T,
}

/// Response data for `place_order` function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponseData {
    pub statuses: Vec<OrderStatus>,
}

pub type OrderResponse = Response<ResponseBody<OrderResponseData>>;

/// Response data for the `cancel_order` function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelResponseData {
    pub statuses: Vec<CancelStatus>,
}

pub type CancelResponse = Response<ResponseBody<CancelResponseData>>;
