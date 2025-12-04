#![allow(dead_code)]

/// Request and response types for WebSocket subscriptions and streaming data.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// WebSocket trade data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsTrade {
    pub coin: String,
    pub side: String,
    pub px: String,
    pub sz: String,
    pub hash: String,
    pub time: u64,
    /// 50-bit hash of (`buyer_oid`, `seller_oid`)
    /// For globally unique trade id, use (`block_time`, coin, tid)
    pub tid: u64,
    /// [buyer, seller]
    pub users: [String; 2],
}

/// WebSocket order book level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsLevel {
    /// Price
    pub px: String,
    /// Size
    pub sz: String,
    /// Number of orders
    pub n: u32,
}

/// WebSocket order book snapshot
/// Pushed on each block that is at least 0.5s since last push
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsBook {
    pub coin: String,
    /// [bids, asks]
    pub levels: [Vec<WsLevel>; 2],
    pub time: u64,
}

/// WebSocket best bid/offer
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsBbo {
    pub coin: String,
    pub time: u64,
    /// [bid, ask]
    pub bbo: [Option<WsLevel>; 2],
}

/// WebSocket notification
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub notification: String,
}

/// All mid prices
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllMids {
    pub mids: HashMap<String, String>,
}

/// Candlestick data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candle {
    /// Open time in milliseconds
    pub t: u64,
    /// Close time in milliseconds
    #[serde(rename = "T")]
    pub close_time: u64,
    /// Coin
    pub s: String,
    /// Interval
    pub i: String,
    /// Open price
    pub o: f64,
    /// Close price
    pub c: f64,
    /// High price
    pub h: f64,
    /// Low price
    pub l: f64,
    /// Volume (base unit)
    pub v: f64,
    /// Number of trades
    pub n: u64,
}

/// WebSocket user event
/// Can be fills, funding, liquidation, or non-user cancels
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WsUserEvent {
    Fills {
        fills: Vec<WsFill>,
    },
    Funding {
        funding: WsUserFunding,
    },
    Liquidation {
        liquidation: WsLiquidation,
    },
    NonUserCancel {
        non_user_cancel: Vec<WsNonUserCancel>,
    },
}

/// WebSocket user fills with optional snapshot flag
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsUserFills {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_snapshot: Option<bool>,
    pub user: String,
    pub fills: Vec<WsFill>,
}

/// Fill liquidation details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FillLiquidation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liquidated_user: Option<String>,
    pub mark_px: f64,
    /// "market" | "backstop"
    pub method: String,
}

/// WebSocket fill data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsFill {
    pub coin: String,
    /// Price
    pub px: String,
    /// Size
    pub sz: String,
    pub side: String,
    pub time: u64,
    pub start_position: String,
    /// Used for frontend display
    pub dir: String,
    pub closed_pnl: String,
    /// L1 transaction hash
    pub hash: String,
    /// Order ID
    pub oid: u64,
    /// Whether order crossed the spread (was taker)
    pub crossed: bool,
    /// Negative means rebate
    pub fee: String,
    /// Unique trade ID
    pub tid: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liquidation: Option<FillLiquidation>,
    /// The token the fee was paid in
    pub fee_token: String,
    /// Amount paid to builder, also included in fee
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builder_fee: Option<String>,
}

/// WebSocket user funding payment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsUserFunding {
    pub time: u64,
    pub coin: String,
    pub usdc: String,
    pub szi: String,
    pub funding_rate: String,
}

/// WebSocket liquidation event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsLiquidation {
    pub lid: u64,
    pub liquidator: String,
    pub liquidated_user: String,
    pub liquidated_ntl_pos: String,
    pub liquidated_account_value: String,
}

/// WebSocket non-user cancel
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsNonUserCancel {
    pub coin: String,
    pub oid: u64,
}

/// WebSocket order with status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsOrder {
    pub order: WsBasicOrder,
    pub status: String,
    pub status_timestamp: u64,
}

/// WebSocket basic order data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsBasicOrder {
    pub coin: String,
    pub side: String,
    pub limit_px: String,
    pub sz: String,
    pub oid: u64,
    pub timestamp: u64,
    pub orig_sz: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloid: Option<String>,
}

/// Shared asset context fields
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedAssetCtx {
    pub day_ntl_vlm: f64,
    pub prev_day_px: f64,
    pub mark_px: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mid_px: Option<f64>,
}

/// Perpetuals asset context
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerpsAssetCtx {
    pub day_ntl_vlm: f64,
    pub prev_day_px: f64,
    pub mark_px: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mid_px: Option<f64>,
    pub funding: f64,
    pub open_interest: f64,
    pub oracle_px: f64,
}

/// Spot asset context
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotAssetCtx {
    pub day_ntl_vlm: f64,
    pub prev_day_px: f64,
    pub mark_px: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mid_px: Option<f64>,
    pub circulating_supply: f64,
}

/// WebSocket active perpetuals asset context
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsActiveAssetCtx {
    pub coin: String,
    pub ctx: PerpsAssetCtx,
}

/// WebSocket active spot asset context
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsActiveSpotAssetCtx {
    pub coin: String,
    pub ctx: SpotAssetCtx,
}

/// Leverage information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Leverage {
    pub raw_usd: String,
    #[serde(rename = "type")]
    pub leverage_type: String,
    pub value: u32,
}

/// WebSocket active asset data for a user
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsActiveAssetData {
    pub user: String,
    pub coin: String,
    pub leverage: Leverage,
    pub max_trade_szs: [f64; 2],
    pub available_to_trade: [f64; 2],
}

/// WebSocket TWAP slice fill
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsTwapSliceFill {
    pub fill: WsFill,
    pub twap_id: u64,
}

/// WebSocket user TWAP slice fills
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsUserTwapSliceFills {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_snapshot: Option<bool>,
    pub user: String,
    pub twap_slice_fills: Vec<WsTwapSliceFill>,
}

/// TWAP state
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwapState {
    pub coin: String,
    pub user: String,
    pub side: String,
    pub sz: f64,
    pub executed_sz: f64,
    pub executed_ntl: f64,
    pub minutes: u32,
    pub reduce_only: bool,
    pub randomize: bool,
    pub timestamp: u64,
}

/// TWAP status with description
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwapStatusInfo {
    /// "activated" | "terminated" | "finished" | "error"
    pub status: String,
    pub description: String,
}

/// WebSocket TWAP history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsTwapHistory {
    pub state: TwapState,
    pub status: TwapStatusInfo,
    pub time: u64,
}

/// WebSocket user TWAP history
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsUserTwapHistory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_snapshot: Option<bool>,
    pub user: String,
    pub history: Vec<WsTwapHistory>,
}

/// User state information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserState {
    pub agent_address: Option<String>,
    pub agent_valid_until: Option<u64>,
    pub server_time: u64,
    pub cum_ledger: f64,
    pub is_vault: bool,
    pub user: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_out_of_spot_dusting: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dex_abstraction_enabled: Option<bool>,
}

/// Leading vault information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeadingVault {
    pub address: String,
    pub name: String,
}

/// Perpetuals DEX state
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerpDexState {
    pub total_vault_equity: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perps_at_open_interest_cap: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leading_vaults: Option<Vec<LeadingVault>>,
}

/// WebSocket web data (`WebData3`)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebData3 {
    pub user_state: UserState,
    pub perp_dex_states: Vec<PerpDexState>,
}

/// Margin summary
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginSummary {
    pub account_value: f64,
    pub total_ntl_pos: f64,
    pub total_raw_usd: f64,
    pub total_margin_used: f64,
}

/// Position information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub coin: String,
    pub entry_px: String,
    pub leverage: Leverage,
    pub liquidation_px: Option<String>,
    pub margin_used: String,
    pub position_value: String,
    pub return_on_equity: String,
    pub szi: String,
    pub unrealized_pnl: String,
}

/// Asset position (one-way)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetPosition {
    #[serde(rename = "type")]
    pub type_: String,
    pub position: Position,
}

/// Clearinghouse state
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearinghouseState {
    pub asset_positions: Vec<AssetPosition>,
    pub margin_summary: MarginSummary,
    pub cross_margin_summary: MarginSummary,
    pub cross_maintenance_margin_used: f64,
    pub withdrawable: f64,
}

/// Order information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub coin: String,
    pub side: String,
    pub limit_px: String,
    pub sz: String,
    pub oid: u64,
    pub timestamp: u64,
    pub orig_sz: String,
}

/// Open orders for a user
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenOrders {
    pub dex: String,
    pub user: String,
    pub orders: Vec<Order>,
}

/// TWAP states for a user
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwapStates {
    pub dex: String,
    pub user: String,
    pub states: Vec<(u64, TwapState)>,
}

/// WebSocket user non-funding ledger update
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsUserNonFundingLedgerUpdate {
    pub time: u64,
    pub hash: String,
    pub delta: WsLedgerUpdate,
}

/// WebSocket ledger update
/// Can be deposit, withdraw, transfer, liquidation, vault operations, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum WsLedgerUpdate {
    #[serde(rename = "deposit")]
    Deposit(WsDeposit),
    #[serde(rename = "withdraw")]
    Withdraw(WsWithdraw),
    #[serde(rename = "internalTransfer")]
    InternalTransfer(WsInternalTransfer),
    #[serde(rename = "subAccountTransfer")]
    SubAccountTransfer(WsSubAccountTransfer),
    #[serde(rename = "liquidation")]
    Liquidation(WsLedgerLiquidation),
    #[serde(rename = "vaultCreate")]
    VaultCreate(WsVaultDelta),
    #[serde(rename = "vaultDeposit")]
    VaultDeposit(WsVaultDelta),
    #[serde(rename = "vaultDistribution")]
    VaultDistribution(WsVaultDelta),
    #[serde(rename = "vaultWithdraw")]
    VaultWithdraw(WsVaultWithdrawal),
    #[serde(rename = "vaultLeaderCommission")]
    VaultLeaderCommission(WsVaultLeaderCommission),
    #[serde(rename = "spotTransfer")]
    SpotTransfer(WsSpotTransfer),
    #[serde(rename = "accountClassTransfer")]
    AccountClassTransfer(WsAccountClassTransfer),
    #[serde(rename = "spotGenesis")]
    SpotGenesis(WsSpotGenesis),
    #[serde(rename = "rewardsClaim")]
    RewardsClaim(WsRewardsClaim),
}

/// Deposit ledger update
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsDeposit {
    pub usdc: f64,
}

/// Withdraw ledger update
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsWithdraw {
    pub usdc: f64,
    pub nonce: u64,
    pub fee: f64,
}

/// Internal transfer ledger update
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsInternalTransfer {
    pub usdc: f64,
    pub user: String,
    pub destination: String,
    pub fee: f64,
}

/// Sub-account transfer ledger update
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsSubAccountTransfer {
    pub usdc: f64,
    pub user: String,
    pub destination: String,
}

/// Liquidated position
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiquidatedPosition {
    pub coin: String,
    pub szi: f64,
}

/// Liquidation ledger update
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsLedgerLiquidation {
    /// For isolated positions this is the isolated account value
    pub account_value: f64,
    /// "Cross" | "Isolated"
    pub leverage_type: String,
    pub liquidated_positions: Vec<LiquidatedPosition>,
}

/// Vault delta ledger update
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsVaultDelta {
    pub vault: String,
    pub usdc: f64,
}

/// Vault withdrawal ledger update
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsVaultWithdrawal {
    pub vault: String,
    pub user: String,
    pub requested_usd: f64,
    pub commission: f64,
    pub closing_cost: f64,
    pub basis: f64,
    pub net_withdrawn_usd: f64,
}

/// Vault leader commission ledger update
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsVaultLeaderCommission {
    pub user: String,
    pub usdc: f64,
}

/// Spot transfer ledger update
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsSpotTransfer {
    pub token: String,
    pub amount: f64,
    pub usdc_value: f64,
    pub user: String,
    pub destination: String,
    pub fee: f64,
}

/// Account class transfer ledger update
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsAccountClassTransfer {
    pub usdc: f64,
    pub to_perp: bool,
}

/// Spot genesis ledger update
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsSpotGenesis {
    pub token: String,
    pub amount: f64,
}

/// Rewards claim ledger update
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsRewardsClaim {
    pub amount: f64,
}
