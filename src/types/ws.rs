use rust_decimal::Decimal;
/// Request and response types for WebSocket subscriptions
/// and streaming data.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::primitive::str;

use crate::types::serialize::decimal_array;

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
pub struct WsNotification {
    pub notification: String,
}

/// All mid prices
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsAllMids {
    pub mids: HashMap<String, String>,
}

/// Candlestick data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsCandle {
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
    #[serde(with = "rust_decimal::serde::str")]
    pub o: Decimal,
    /// Close price
    #[serde(with = "rust_decimal::serde::str")]
    pub c: Decimal,
    /// High price
    #[serde(with = "rust_decimal::serde::str")]
    pub h: Decimal,
    /// Low price
    #[serde(with = "rust_decimal::serde::str")]
    pub l: Decimal,
    /// Volume (base unit)
    #[serde(with = "rust_decimal::serde::str")]
    pub v: Decimal,
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
        funding: WsUserFundings,
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
    #[serde(with = "rust_decimal::serde::str")]
    pub mark_px: Decimal,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsUserFundings {
    pub is_snapshot: bool,
    pub user: String,
    pub fundings: Vec<WsUserFundings>
}

/// WebSocket user funding payment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fundings {
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
    #[serde(with = "rust_decimal::serde::str")]
    pub day_ntl_vlm: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub prev_day_px: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub mark_px: Decimal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mid_px: Option<Decimal>,
}

/// Perpetuals asset context
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerpsAssetCtx {
    #[serde(with = "rust_decimal::serde::str")]
    pub day_ntl_vlm: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub prev_day_px: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub mark_px: Decimal,
   #[serde(
        with = "rust_decimal::serde::str_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub mid_px: Option<Decimal>,
    #[serde(with = "rust_decimal::serde::str")]
    pub funding: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub open_interest: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub oracle_px: Decimal,
}

/// Spot asset context
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotAssetCtx {
    #[serde(with = "rust_decimal::serde::str")]
    pub day_ntl_vlm: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub prev_day_px: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub mark_px: Decimal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mid_px: Option<Decimal>,
    #[serde(with = "rust_decimal::serde::str")]
    pub circulating_supply: Decimal,
}


/// Leverage information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Leverage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_usd: Option<String>,
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
        #[serde(with = "decimal_array")]
    pub max_trade_szs: [Decimal; 2],
        #[serde(with = "decimal_array")]
    pub available_to_trade: [Decimal; 2],
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
pub struct WsTwapState {
    pub coin: String,
    pub user: String,
    pub side: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub sz: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub executed_sz: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub executed_ntl: Decimal,
    pub minutes: u32,
    pub reduce_only: bool,
    pub randomize: bool,
    pub timestamp: u64,
}

/// TWAP status with description
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsTwapStatusInfo {
    /// "activated" | "terminated" | "finished" | "error"
    pub status: String,
    pub description: String,
}

/// WebSocket TWAP history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsTwapHistory {
    pub state: WsTwapState,
    pub status: WsTwapStatusInfo,
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
    #[serde(with = "rust_decimal::serde::str")]
    pub cum_ledger: Decimal,
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
    #[serde(with = "rust_decimal::serde::str")]
    pub total_vault_equity: Decimal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perps_at_open_interest_cap: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leading_vaults: Option<Vec<LeadingVault>>,
}

/// WebSocket web data (`WebData3`)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsWebData3 {
    pub user_state: UserState,
    pub perp_dex_states: Vec<PerpDexState>,
}

/// Margin summary
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginSummary {
    #[serde(with = "rust_decimal::serde::str")]
    pub account_value: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub total_ntl_pos: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub total_raw_usd: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub total_margin_used: Decimal,
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
pub struct WsClearinghouseState {
    pub dex: String,
    pub user: String,
    pub clearinghouse_state: ClearinghouseStateData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearinghouseStateData {
    pub asset_positions: Vec<AssetPosition>,
    pub margin_summary: MarginSummary,
    pub cross_margin_summary: MarginSummary,
    #[serde(with = "rust_decimal::serde::str")]
    pub cross_maintenance_margin_used: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub withdrawable: Decimal,
    pub time: u64,
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
pub struct WsOpenOrders {
    pub dex: String,
    pub user: String,
    pub orders: Vec<Order>,
}

/// TWAP states for a user
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsTwapStates {
    pub dex: String,
    pub user: String,
    pub states: Vec<(u64, WsTwapState)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsNonFundingLedgerUpdate {
    pub time: u64,
    pub hash: String,
    pub delta: WsLedgerUpdate,
}

/// WebSocket user non-funding ledger update
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsUserNonFundingLedgerUpdate {
    pub is_snapshot: bool,
    pub user: String,
    pub non_funding_ledger_updates: Vec<WsNonFundingLedgerUpdate>
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
    #[serde(with = "rust_decimal::serde::str")]
    pub usdc: Decimal,
}

/// Withdraw ledger update
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsWithdraw {
    #[serde(with = "rust_decimal::serde::str")]
    pub usdc: Decimal,
    pub nonce: u64,
    #[serde(with = "rust_decimal::serde::str")]
    pub fee: Decimal,
}

/// Internal transfer ledger update
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsInternalTransfer {
    #[serde(with = "rust_decimal::serde::str")]
    pub usdc: Decimal,
    pub user: String,
    pub destination: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub fee: Decimal,
}

/// Sub-account transfer ledger update
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsSubAccountTransfer {
    #[serde(with = "rust_decimal::serde::str")]
    pub usdc: Decimal,
    pub user: String,
    pub destination: String,
}

/// Liquidated position
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiquidatedPosition {
    pub coin: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub szi: Decimal,
}

/// Liquidation ledger update
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsLedgerLiquidation {
    /// For isolated positions this is the isolated account value
    #[serde(with = "rust_decimal::serde::str")]
    pub account_value: Decimal,
    /// "Cross" | "Isolated"
    pub leverage_type: String,
    pub liquidated_positions: Vec<LiquidatedPosition>,
}

/// Vault delta ledger update
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsVaultDelta {
    pub vault: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub usdc: Decimal,
}

/// Vault withdrawal ledger update
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsVaultWithdrawal {
    pub vault: String,
    pub user: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub requested_usd: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub commission: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub closing_cost: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub basis: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub net_withdrawn_usd: Decimal,
}

/// Vault leader commission ledger update
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsVaultLeaderCommission {
    pub user: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub usdc: Decimal,
}

/// Spot transfer ledger update
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsSpotTransfer {
    pub token: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub amount: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub usdc_value: Decimal,
    pub user: String,
    pub destination: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub fee: Decimal,
}

/// Account class transfer ledger update
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsAccountClassTransfer {
    #[serde(with = "rust_decimal::serde::str")]
    pub usdc: Decimal,
    pub to_perp: bool,
}

/// Spot genesis ledger update
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsSpotGenesis {
    pub token: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub amount: Decimal,
}

/// Rewards claim ledger update
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsRewardsClaim {
    #[serde(with = "rust_decimal::serde::str")]
    pub amount: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SubscriptionError {
    pub data: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SubscriptionConfirmation {
    pub method: String,
    pub subscription: serde_json::Value,
}

/// WebSocket active perpetuals asset context
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WsActiveAssetCtx {
    pub coin: String,
    pub ctx: PerpsAssetCtx,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WsActiveSpotAssetCtx {
    pub coin: String,
    pub ctx: SpotAssetCtx,
}


/// Identifies a concrete websocket subscription type supported by the feed.
///
/// Each variant corresponds to a distinct server-side stream and determines
/// both the subscription parameters and the shape of messages received.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SubscriptionKey {
    AllMids,
    Candle,
    Trades,
    L2Book,
    Notification,
    WebData3,
    TwapStates,
    OpenOrders,
    UserEvents,
    UserNonFundingLedgerUpdate,
    ActiveAssetCtx,
    ActiveAssetData,
    UserTwapSliceFills,
    UserTwapHistory,
    Bbo,
    Ping,
}

impl std::fmt::Display for SubscriptionKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = match *self {
            Self::AllMids => f.write_str("allMids"),
            Self::Candle => f.write_str("candle"),
            Self::Trades => f.write_str("trades"),
            Self::L2Book => f.write_str("l2Book"),
            Self::Notification => f.write_str("notification"),
            Self::WebData3 => f.write_str("webData3"),
            Self::TwapStates => f.write_str("twapStates"),
            Self::OpenOrders => f.write_str("openOrders"),
            Self::UserEvents => f.write_str("userEvents"),
            Self::UserNonFundingLedgerUpdate => f.write_str("userNonFundingLedgerUpdate"),
            Self::ActiveAssetCtx => f.write_str("activeAssetCtx"),
            Self::ActiveAssetData => f.write_str("activeAssetData"),
            Self::UserTwapSliceFills => f.write_str("userTwapSliceFills"),
            Self::UserTwapHistory => f.write_str("userTwapHistory"),
            Self::Bbo => f.write_str("bbo"),
            Self::Ping => f.write_str("pong"),
        };

        Ok(())
    }
}

/// Messages delivered over websocket subscription channels.
///
/// The `channel` field selects the subscription stream, while `data` contains
/// the stream-specific payload deserialized into a strongly typed variant.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "channel", content = "data", rename_all = "camelCase")]
pub enum SubscriptionResponse {
    /// Server-side error emitted over the websocket connection.
    #[serde(rename = "error")]
    Error(String),

    /// Acknowledgement or status response for subscribe / unsubscribe requests.
    SubscriptionResponse(SubscriptionConfirmation),

    /// Pong response to client ping (keepalive)
    #[serde(rename = "pong")]
    Pong,

    AllMids(WsAllMids),
    Candle(WsCandle),
    Trades(Vec<WsTrade>),
    L2Book(WsBook),
    Notification(WsNotification),
    WebData3(WsWebData3),
    ClearinghouseState(WsClearinghouseState),
    TwapStates(WsTwapStates),
    UserFills(WsUserFills),
    OpenOrders(WsOpenOrders),
    UserEvents(WsUserEvent),
    UserFundings(WsUserFundings),
    UserNonFundingLedgerUpdates(WsUserNonFundingLedgerUpdate),
    ActiveAssetCtx(WsActiveAssetCtx),
    ActiveAssetData(WsActiveAssetData),
    UserTwapSliceFills(WsUserTwapSliceFills),
    UserTwapHistory(WsUserTwapHistory),
    Bbo(WsBbo),
}
