use crate::types::exchange::{
    base::{CancelStatus, TwapCancelStatus, TwapOrderStatus},
    OrderStatus,
};
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

/// Response data for the `cancel_order` function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelResponseData {
    pub statuses: Vec<CancelStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwapOrderResponseData {
    pub status: TwapOrderStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwapCancelResponseData {
    pub status: TwapCancelStatus,
}

// Order responses
pub type OrderResponse = Response<ResponseBody<OrderResponseData>>;
pub type CancelResponse = Response<ResponseBody<CancelResponseData>>;

/// Response type for canceling orders by client order ID.
/// Currently unused but part of the complete API surface.
pub type CancelByCloidResponse = CancelResponse;
pub type ModifyResponse = OrderResponse;
pub type BatchModifyResponse = OrderResponse;

// TWAP responses
pub type TwapOrderResponse = Response<ResponseBody<TwapOrderResponseData>>;
pub type TwapCancelResponse = Response<ResponseBody<TwapCancelResponseData>>;

// Default responses (return {'status': 'ok', 'response': {'type': 'default'}})
pub type DefaultResponse = Response<DefaultResponseBody>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultResponseBody {
    #[serde(rename = "type")]
    pub type_: String,
}

pub type ScheduleCancelResponse = DefaultResponse;
pub type UpdateLeverageResponse = DefaultResponse;
pub type UpdateIsolatedMarginResponse = DefaultResponse;
pub type UsdSendResponse = DefaultResponse;
pub type SpotSendResponse = DefaultResponse;
pub type WithdrawResponse = DefaultResponse;
pub type UsdClassTransferResponse = DefaultResponse;
pub type SendAssetResponse = DefaultResponse;
pub type CDepositResponse = DefaultResponse;
pub type CWithdrawResponse = DefaultResponse;
pub type TokenDelegateResponse = DefaultResponse;
pub type VaultTransferResponse = DefaultResponse;
pub type ApproveAgentResponse = DefaultResponse;
pub type ApproveBuilderFeeResponse = DefaultResponse;
pub type ReserveRequestWeightResponse = DefaultResponse;
pub type NoopResponse = DefaultResponse;
pub type UserDexAbstractionResponse = DefaultResponse;
pub type AgentEnableDexAbstractionResponse = DefaultResponse;
pub type ValidatorL1StreamResponse = DefaultResponse;
