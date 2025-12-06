use crate::{signature::eip712::Eip712, types::info::perpetual::AssetInfo};
/// Request and response types for the exchange endpoint used to interact
/// with and trade on the Hyperliquid chain.
///
use alloy::{
    dyn_abi::Eip712Domain,
    primitives::{keccak256, Address, B256},
    sol_types::{eip712_domain, SolValue},
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::types::serialize::serialize_decimal;

fn eip_712_domain(chain_id: u64) -> Eip712Domain {
    eip712_domain! {
        name: "HyperliquidSignTransaction",
        version: "1",
        chain_id: chain_id,
        verifying_contract: Address::ZERO,
    }
}

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
pub struct LimitOrder {
    pub tif: Tif,
}

/// Trigger order type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TriggerOrder {
    pub is_market: bool,
    pub trigger_px: String,
    /// "tp" | "sl"
    pub tpsl: Tpsl,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Tpsl {
    #[serde(rename = "tp")]
    Tp,
    #[serde(rename = "sl")]
    Sl
}

/// Order type (limit or trigger)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OrderType {
    #[serde(rename = "limit")]
    Limit(LimitOrder),
    #[serde(rename = "trigger")]
    Trigger(TriggerOrder),
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
    #[serde(serialize_with = "serialize_decimal")]
    pub p: String,
    /// Size
    #[serde(serialize_with = "serialize_decimal")]
    pub s: String,
    /// Reduce only
    pub r: bool,
    /// Order type
    pub t: OrderType,
    /// Client order ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c: Option<Cloid>,
}

impl OrderRequest {
    /// Create a new market order with automatic price/size rounding
    pub fn new_market_order(
        asset_idx: u32,
        asset_info: &AssetInfo,
        is_buy: bool,
        price: Decimal,
        size: Decimal,
    ) -> Self {
        let sz_decimals = asset_info.sz_decimals as u32;

        // BTC and most perps use whole number ticks (0 decimals)
        let px_decimals = 0;

        Self {
            a: asset_idx,
            b: is_buy,
            p: price.round_dp(px_decimals).to_string(),
            s: size.round_dp(sz_decimals).to_string(),
            r: false,
            t: OrderType::Limit(LimitOrder { tif: Tif::Ioc }),
            c: None,
        }
    }
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
    pub ntli: u32,
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
    pub signature_chain_id: u64,
    pub destination: String,
    pub amount: String,
    pub time: u64,
}

impl Eip712 for UsdSendAction {
    fn domain(&self) -> Eip712Domain {
        eip_712_domain(self.signature_chain_id)
    }

    fn struct_hash(&self) -> B256 {
        let items = (
            keccak256("HyperliquidTransaction:UsdSend(string hyperliquidChain,string destination,string amount,uint64 time)"),
            keccak256(&self.hyperliquid_chain),
            keccak256(&self.destination),
            keccak256(&self.amount),
            &self.time
        );
        keccak256(items.abi_encode())
    }
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
    pub signature_chain_id: u64,
    pub destination: String,
    /// e.g. "PURR:0xc4bf3f870c0e9465323c0b6ed28096c2"
    pub token: String,
    pub amount: String,
    pub time: u64,
}

impl Eip712 for SpotSendAction {
    fn domain(&self) -> Eip712Domain {
        eip_712_domain(self.signature_chain_id)
    }

    fn struct_hash(&self) -> B256 {
        let items = (
            keccak256("HyperliquidTransaction:SpotSend(string hyperliquidChain,string destination,string token,string amount,uint64 time)"),
            keccak256(&self.hyperliquid_chain),
            keccak256(&self.destination),
            keccak256(&self.token),
            keccak256(&self.amount),
            &self.time,
        );
        keccak256(items.abi_encode())
    }
}

/// Request type for POST /exchange with type "withdraw3"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawAction {
    /// "withdraw3"
    #[serde(rename = "type")]
    pub type_: String,
    pub hyperliquid_chain: String,
    pub signature_chain_id: u64,
    pub amount: String,
    pub time: u64,
    pub destination: String,
}

impl Eip712 for WithdrawAction {
    fn domain(&self) -> Eip712Domain {
        eip_712_domain(self.signature_chain_id)
    }

    fn struct_hash(&self) -> B256 {
        let items = (
            keccak256("HyperliquidTransaction:Withdraw(string hyperliquidChain,string destination,string amount,uint64 time)"),
            keccak256(&self.hyperliquid_chain),
            keccak256(&self.destination),
            keccak256(&self.amount),
            &self.time,
        );
        keccak256(items.abi_encode())
    }
}

/// Request type for POST /exchange with type "usdClassTransfer"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsdClassTransferAction {
    /// "usdClassTransfer"
    #[serde(rename = "type")]
    pub type_: String,
    pub hyperliquid_chain: String,
    pub signature_chain_id: u64,
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
    pub signature_chain_id: u64,
    pub destination: String,
    pub source_dex: String,
    pub destination_dex: String,
    pub token: String,
    pub amount: String,
    pub from_sub_account: String,
    pub nonce: u64,
}

impl Eip712 for SendAssetAction {
    fn domain(&self) -> Eip712Domain {
        eip_712_domain(self.signature_chain_id)
    }

    fn struct_hash(&self) -> B256 {
        let items = (
            keccak256("HyperliquidTransaction:SendAsset(string hyperliquidChain,string destination,string sourceDex,string destinationDex,string token,string amount,string fromSubAccount,uint64 nonce)"),
            keccak256(&self.hyperliquid_chain),
            keccak256(&self.destination),
            keccak256(&self.source_dex),
            keccak256(&self.destination_dex),
            keccak256(&self.token),
            keccak256(&self.amount),
            keccak256(&self.from_sub_account),
            &self.nonce,
        );
        keccak256(items.abi_encode())
    }
}

/// Request type for POST /exchange with type "cDeposit"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CDepositAction {
    /// "cDeposit"
    #[serde(rename = "type")]
    pub type_: String,
    pub hyperliquid_chain: String,
    pub signature_chain_id: u64,
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
    pub signature_chain_id: u64,
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
    pub signature_chain_id: u64,
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
    pub signature_chain_id: u64,
    pub agent_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<String>,
    pub nonce: u64,
}

impl Eip712 for ApproveAgentAction {
    fn domain(&self) -> Eip712Domain {
        eip_712_domain(self.signature_chain_id)
    }

    fn struct_hash(&self) -> B256 {
        let items = (
            keccak256("HyperliquidTransaction:ApproveAgent(string hyperliquidChain,address agentAddress,string agentName,uint64 nonce)"),
            keccak256(&self.hyperliquid_chain),
            &self.agent_address,
            keccak256(self.agent_name.as_deref().unwrap_or("")),
            &self.nonce
        );
        keccak256(items.abi_encode())
    }
}

/// Request type for POST /exchange with type "approveBuilderFee"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApproveBuilderFeeAction {
    /// "approveBuilderFee"
    #[serde(rename = "type")]
    pub type_: String,
    pub hyperliquid_chain: String,
    pub signature_chain_id: u64,
    /// e.g. "0.001%"
    pub max_fee_rate: String,
    pub builder: String,
    pub nonce: u64,
}

impl Eip712 for ApproveBuilderFeeAction {
    fn domain(&self) -> Eip712Domain {
        eip_712_domain(self.signature_chain_id)
    }

    fn struct_hash(&self) -> B256 {
        let items = (
            keccak256("HyperliquidTransaction:ApproveBuilderFee(string hyperliquidChain,string maxFeeRate,address builder,uint64 nonce)"),
            keccak256(&self.hyperliquid_chain),
            keccak256(&self.max_fee_rate),
            &self.builder,
            &self.nonce,
        );
        keccak256(items.abi_encode())
    }
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
    pub a: usize,
    /// TWAP ID
    pub t: u32,
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
    pub signature_chain_id: u64,
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
    /// e.g. "0.04" for 4%
    pub risk_free_rate: String,
}
