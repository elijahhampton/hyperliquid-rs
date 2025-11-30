/// Request and response types for the exchange endpoint used to interact with and trade on the Hyperliquid chain.
use serde::{Deserialize, Serialize};

/// Client Order ID - optional 128 bit hex string
pub type Cloid = String;

/// Time-in-force for limit orders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Tif {
    /// Add liquidity only (post only)
    #[serde(rename = "Alo")]
    Alo,
    /// Immediate or cancel
    #[serde(rename = "Ioc")]
    Ioc,
    /// Good til canceled
    #[serde(rename = "Gtc")]
    Gtc,
}

/// Limit order type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitOrderType {
    pub tif: Tif,
}

/// Trigger order type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TriggerOrderType {
    pub is_market: bool,
    pub trigger_px: String,
    /// "tp" | "sl"
    pub tpsl: String,
}

/// Order type (limit or trigger)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OrderType {
    #[serde(rename = "limit")]
    Limit(LimitOrderType),
    #[serde(rename = "trigger")]
    Trigger(TriggerOrderType),
}

/// Builder fee configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Builder {
    /// Address that should receive the additional fee
    pub b: String,
    /// Fee size in tenths of a basis point (e.g. 10 = 1bp)
    pub f: u32,
}

/// Single order request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderRequest {
    /// Asset index
    pub a: u32,
    /// Is buy
    pub b: bool,
    /// Price
    pub p: String,
    /// Size
    pub s: String,
    /// Reduce only
    pub r: bool,
    /// Order type
    pub t: OrderType,
    /// Client order ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c: Option<Cloid>,
}

/// Grouping type for orders
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Grouping {
    #[serde(rename = "na")]
    Na,
    #[serde(rename = "normalTpsl")]
    NormalTpsl,
    #[serde(rename = "positionTpsl")]
    PositionTpsl,
}

/// Request type for POST /exchange with type "order"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderAction {
    #[serde(rename = "type")]
    /// "order"
    pub type_: String,
    pub orders: Vec<OrderRequest>,
    pub grouping: Grouping,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builder: Option<Builder>,
}

/// Request type for POST /exchange with type "cancel"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelRequest {
    /// Asset index
    pub a: u32,
    /// Order ID
    pub o: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAction {
    /// "cancel"
    #[serde(rename = "type")]
    pub type_: String,
    pub cancels: Vec<CancelRequest>,
}

/// Request type for POST /exchange with type "cancelByCloid"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelByCloidRequest {
    pub asset: u32,
    pub cloid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelByCloidAction {
    /// "cancelByCloid"
    #[serde(rename = "type")]
    pub type_: String,
    pub cancels: Vec<CancelByCloidRequest>,
}

/// Request type for POST /exchange with type "scheduleCancel"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleCancelAction {
    /// "scheduleCancel"
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<u64>,
}

/// Request type for POST /exchange with type "modify"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyAction {
    /// "modify"
    #[serde(rename = "type")]
    pub type_: String,
    /// Can also be Cloid
    pub oid: u64,
    pub order: OrderRequest,
}

/// Request type for POST /exchange with type "batchModify"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyRequest {
    /// Can also be Cloid
    pub oid: u64,
    pub order: OrderRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchModifyAction {
    /// "batchModify"
    #[serde(rename = "type")]
    pub type_: String,
    pub modifies: Vec<ModifyRequest>,
}

/// Request type for POST /exchange with type "updateLeverage"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateLeverageAction {
    /// "updateLeverage"
    #[serde(rename = "type")]
    pub type_: String,
    pub asset: u32,
    pub is_cross: bool,
    pub leverage: u32,
}

/// Request type for POST /exchange with type "updateIsolatedMargin"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateIsolatedMarginAction {
    /// "updateIsolatedMargin"
    #[serde(rename = "type")]
    pub type_: String,
    pub asset: u32,
    pub is_buy: bool,
    pub ntli: i64,
}

/// Request type for POST /exchange with type "usdSend"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsdSendAction {
    /// "usdSend"
    #[serde(rename = "type")]
    pub type_: String,
    /// "Mainnet" | "Testnet"
    pub hyperliquid_chain: String,
    /// e.g. "0xa4b1"
    pub signature_chain_id: String,
    pub destination: String,
    pub amount: String,
    pub time: u64,
}

/// Request type for POST /exchange with type "spotSend"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotSendAction {
    /// "spotSend"
    #[serde(rename = "type")]
    pub type_: String,
    /// "Mainnet" | "Testnet"
    pub hyperliquid_chain: String,
    pub signature_chain_id: String,
    pub destination: String,
    /// e.g. "PURR:0xc4bf3f870c0e9465323c0b6ed28096c2"
    pub token: String,
    pub amount: String,
    pub time: u64,
}

/// Request type for POST /exchange with type "withdraw3"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawAction {
    /// "withdraw3"
    #[serde(rename = "type")]
    pub type_: String,
    pub hyperliquid_chain: String,
    pub signature_chain_id: String,
    pub amount: String,
    pub time: u64,
    pub destination: String,
}

/// Request type for POST /exchange with type "usdClassTransfer"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsdClassTransferAction {
    /// "usdClassTransfer"
    #[serde(rename = "type")]
    pub type_: String,
    pub hyperliquid_chain: String,
    pub signature_chain_id: String,
    pub amount: String,
    pub to_perp: bool,
    pub nonce: u64,
}

/// Request type for POST /exchange with type "sendAsset"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendAssetAction {
    /// "sendAsset"
    #[serde(rename = "type")]
    pub type_: String,
    pub hyperliquid_chain: String,
    pub signature_chain_id: String,
    pub destination: String,
    pub source_dex: String,
    pub destination_dex: String,
    pub token: String,
    pub amount: String,
    pub from_sub_account: String,
    pub nonce: u64,
}

/// Request type for POST /exchange with type "cDeposit"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CDepositAction {
    /// "cDeposit"
    #[serde(rename = "type")]
    pub type_: String,
    pub hyperliquid_chain: String,
    pub signature_chain_id: String,
    pub wei: u64,
    pub nonce: u64,
}

/// Request type for POST /exchange with type "cWithdraw"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CWithdrawAction {
    /// "cWithdraw"
    #[serde(rename = "type")]
    pub type_: String,
    pub hyperliquid_chain: String,
    pub signature_chain_id: String,
    pub wei: u64,
    pub nonce: u64,
}

/// Request type for POST /exchange with type "tokenDelegate"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenDelegateAction {
    /// "tokenDelegate"
    #[serde(rename = "type")]
    pub type_: String,
    pub hyperliquid_chain: String,
    pub signature_chain_id: String,
    pub validator: String,
    pub is_undelegate: bool,
    pub wei: u64,
    pub nonce: u64,
}

/// Request type for POST /exchange with type "vaultTransfer"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultTransferAction {
    /// "vaultTransfer"
    #[serde(rename = "type")]
    pub type_: String,
    pub vault_address: String,
    pub is_deposit: bool,
    pub usd: u64,
}

/// Request type for POST /exchange with type "approveAgent"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApproveAgentAction {
    /// "approveAgent"
    #[serde(rename = "type")]
    pub type_: String,
    pub hyperliquid_chain: String,
    pub signature_chain_id: String,
    pub agent_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<String>,
    pub nonce: u64,
}

/// Request type for POST /exchange with type "approveBuilderFee"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApproveBuilderFeeAction {
    /// "approveBuilderFee"
    #[serde(rename = "type")]
    pub type_: String,
    pub hyperliquid_chain: String,
    pub signature_chain_id: String,
    /// e.g. "0.001%"
    pub max_fee_rate: String,
    pub builder: String,
    pub nonce: u64,
}

/// Request type for POST /exchange with type "twapOrder"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwapRequest {
    /// Asset index
    pub a: u32,
    /// Is buy
    pub b: bool,
    /// Size
    pub s: String,
    /// Reduce only
    pub r: bool,
    /// Minutes
    pub m: u32,
    /// Randomize
    pub t: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwapOrderAction {
    /// "twapOrder"
    #[serde(rename = "type")]
    pub type_: String,
    pub twap: TwapRequest,
}

/// Request type for POST /exchange with type "twapCancel"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwapCancelAction {
    /// "twapCancel"
    #[serde(rename = "type")]
    pub type_: String,
    /// Asset index
    pub a: u32,
    /// TWAP ID
    pub t: u64,
}

/// Request type for POST /exchange with type "reserveRequestWeight"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReserveRequestWeightAction {
    /// "reserveRequestWeight"
    #[serde(rename = "type")]
    pub type_: String,
    pub weight: u32,
}

/// Request type for POST /exchange with type "noop"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoopAction {
    /// "noop"
    #[serde(rename = "type")]
    pub type_: String,
}

/// Request type for POST /exchange with type "userDexAbstraction"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDexAbstractionAction {
    /// "userDexAbstraction"
    #[serde(rename = "type")]
    pub type_: String,
    pub hyperliquid_chain: String,
    pub signature_chain_id: String,
    pub user: String,
    pub enabled: bool,
    pub nonce: u64,
}

/// Request type for POST /exchange with type "agentEnableDexAbstraction"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentEnableDexAbstractionAction {
    /// "agentEnableDexAbstraction"
    #[serde(rename = "type")]
    pub type_: String,
}

/// Request type for POST /exchange with type "validatorL1Stream"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidatorL1StreamAction {
    /// "validatorL1Stream"
    #[serde(rename = "type")]
    pub type_: String,
    /// e.g. "0.04"
    pub risk_free_rate: String,
}
