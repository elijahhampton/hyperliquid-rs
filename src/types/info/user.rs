/// Response types for the info endpoints that are used to fetch information about the exchange and specific users.
/// Additional information for all endpoints can be found
/// here: `<https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/info-endpoint>`
///
use rust_decimal::{self, Decimal};
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

/// Response to "allMids" request type.
pub type AllMids = HashMap<String, Decimal>;

/// Request for "openOrders" request type.
#[derive(Debug, Serialize)]
pub struct OpenOrdersRequest {
    /// "allMids"
    #[serde(rename = "type")]
    pub type_: String,
    /// Address in 42-character hexadecimal format; e.g. 0x0000000000000000000000000000000000000000.
    pub user: String,
    /// The name of the perpetuals DEX to query. Defaults to the empty string which represents
    /// the first perp dex. Spot mids are only included with the first perp dex.
    pub dex: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct OpenOrder {
    pub coin: String,
    #[serde(rename = "limitPx")]
    pub limit_px: Decimal,
    pub oid: u64,
    pub side: String,
    pub sz: Decimal,
    pub timestamp: u64,
}

/// Response for "openOrders" request type.
pub type OpenOrders = Vec<OpenOrder>;

/// Request for "frontendOpenOrders" request type.
#[derive(Debug, Serialize)]
pub struct FrontendOpenOrdersRequest {
    #[serde(rename = "type")]
    pub type_: String,
    /// Address in 42-character hexadecimal format; e.g. 0x0000000000000000000000000000000000000000.
    pub user: String,
    /// The name of the perpetuals DEX to query. Defaults to the empty string which represents
    /// the first perp dex. Spot mids are only included with the first perp dex.
    pub dex: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FrontendOpenOrder {
    pub coin: String,
    #[serde(rename = "isPositionTpsl")]
    pub is_position_tpsl: bool,
    #[serde(rename = "isTrigger")]
    pub is_trigger: bool,
    #[serde(rename = "limitPx")]
    pub limit_px: Decimal,
    pub oid: u64,
    #[serde(rename = "orderType")]
    pub order_type: String,
    #[serde(rename = "origSz")]
    pub orig_sz: Decimal,
    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,
    pub side: String,
    pub sz: Decimal,
    pub timestamp: u64,
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: String,
    #[serde(rename = "triggerPx")]
    pub trigger_px: Decimal,
}

/// Response for `FrontendOpenOrders` request type.
pub type FrontendOpenOrders = Vec<FrontendOpenOrder>;

/// Request for "userFills" request type.
#[derive(Debug, Serialize)]
pub struct UserFillsRequest {
    /// "userFills"
    #[serde(rename = "type")]
    pub type_: String,
    /// Address in 42-character hexadecimal format; e.g. 0x0000000000000000000000000000000000000000.
    pub user: String,
    /// When true, partial fills are combined when a crossing order gets filled by multiple
    /// different resting orders. Resting orders filled by multiple crossing orders are only
    /// aggregated if in the same block.
    #[serde(rename = "aggregateByTime")]
    pub aggregate_by_time: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct PerpetualFill {
    #[serde(rename = "closedPnl")]
    pub closed_pnl: Decimal,
    /// Refer to `<https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/asset-ids>` for more information
    /// on how asset IDs work.
    pub coin: String,
    pub crossed: bool,
    pub dir: String,
    pub hash: String,
    pub oid: u64,
    pub px: Decimal,
    pub side: String,
    #[serde(rename = "startPosition")]
    pub start_position: Decimal,
    pub sz: Decimal,
    pub time: u64,
    // The total fee, inclusive of builderFee.
    pub fee: Decimal,
    #[serde(rename = "feeToken")]
    pub fee_token: String,
    #[serde(rename = "builderFee")]
    // This is optional and will not be present if 0.
    #[serde(default, deserialize_with = "deserialize_builder_fee")]
    pub builder_fee: Option<Decimal>,
    pub tid: u64,
}

fn deserialize_builder_fee<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt_str = Option::<String>::deserialize(deserializer)?;
    match opt_str {
        None => Ok(None),
        Some(s) if s == "0" || s == "0.0" || s == "0.00" => Ok(None),
        Some(s) => s.parse::<Decimal>().map(Some).map_err(D::Error::custom),
    }
}

#[derive(Debug, Deserialize)]
pub struct SpotFill {
    /// Refer to `<https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/asset-ids>` for more information
    /// on how asset IDs work.
    pub coin: String,
    pub px: Decimal,
    pub sz: Decimal,
    pub side: String,
    pub time: u64,
    #[serde(rename = "startPosition")]
    pub start_position: Decimal,
    pub dir: String,
    pub closed_pnl: Decimal,
    pub hash: String,
    pub oid: u64,
    pub crossed: bool,
    pub fee: Decimal,
    pub tid: u64,
    #[serde(rename = "feeToken")]
    pub fee_token: String,
}

/// Response for "userFills" request type.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Fill {
    Perp(PerpetualFill),
    Spot(SpotFill),
}

pub type UserFills = Vec<Fill>;

/// Request for "userFillsByTime" request type.
#[derive(Debug, Serialize)]
pub struct UserFillsByTimeRequest {
    /// "userFillsByTime"
    #[serde(rename = "type")]
    pub type_: String,
    /// Address in 42-character hexadecimal format; e.g. 0x0000000000000000000000000000000000000000.
    pub user: String,
    /// Start time in milliseconds, inclusive.
    #[serde(rename = "startTime")]
    pub start_time: u32,
    /// End time in milliseconds, inclusive. Defaults to current time.
    #[serde(rename = "endTime")]
    pub end_time: Option<u32>,
    /// When true, partial fills are combined when a crossing order gets filled by multiple
    /// different resting orders. Resting orders filled by multiple crossing orders are only
    /// aggregated if in the same block.
    #[serde(rename = "aggregateByTime")]
    pub aggregate_by_time: Option<bool>,
}

/// Response for "userFillsByTime" request type.
pub type UserFillsByTime = (PerpetualFill, SpotFill);

/// Request for "userRateLimit" request type.
#[derive(Debug, Serialize)]
pub struct UserRateLimitsRequest {
    /// "userRateLimit"
    #[serde(rename = "type")]
    pub type_: String,
    /// Address in 42-character hexadecimal format; e.g. 0x0000000000000000000000000000000000000000.
    pub user: String,
}

/// Response for "userRateLimit" request type.
#[derive(Debug, Deserialize)]
pub struct UserRateLimits {
    #[serde(rename = "cumVlm")]
    pub cum_vlm: Decimal,
    /// max(0, `cumulative_used` minus reserved)
    #[serde(rename = "nRequestsUsed")]
    pub n_requests_used: u64,
    #[serde(rename = "nRequestsCap")]
    pub n_requests_cap: u64,
    // max(0, reserved minus `cumulative_used`)
    #[serde(rename = "nRequestsSurplus")]
    pub n_requests_surplus: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrderId {
    Numeric(u64),
    /// A 16-byte hex value
    ClientId(String),
}

/// Request for request with type "orderStatus"
#[derive(Debug, Deserialize)]
pub struct OrderStatusRequest {
    pub user: String,
    /// "orderStatus"
    #[serde(rename = "type")]
    pub type_: String,
    pub oid: OrderId,
}
#[derive(Debug, Deserialize)]
pub struct ServerOrder {
    pub order: Order,
    pub status: String,
    #[serde(rename = "statusTimestamp")]
    pub status_timestamp: u64,
}

/// Top-level response from the server for an "order" request
#[derive(Debug, Deserialize)]
pub struct OrderStatus {
    pub status: String, // e.g., "order"
    pub order: ServerOrder,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum OrderWithStatus {
    Success { order: Box<ServerOrder> },
    UnknownOid { status: String },
}
/// Represents a Bid or Ask in the [`L2BookSnapshot`].
#[derive(Debug, Serialize, Deserialize)]
pub struct BidOrAsk {
    px: rust_decimal::Decimal,
    sz: rust_decimal::Decimal,
    n: u64,
}

/// Response for request with type "l2Book"
#[derive(Debug, Serialize, Deserialize)]
pub struct L2BookSnapshot {
    coin: String,
    time: u64,
    levels: (Vec<BidOrAsk>, Vec<BidOrAsk>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CandleSnapshotRequest {
    pub coin: String,
    pub interval: String,
    #[serde(rename = "startTime")]
    pub start_time: u64,
    #[serde(rename = "endTime")]
    pub end_time: u64,
}

#[allow(nonstandard_style)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Candle {
    T: u64,
    c: String,
    h: String,
    i: String,
    l: String,
    n: u64,
    o: String,
    s: String,
    t: u64,
    v: String,
}

pub type CandleSnapshot = Vec<Candle>;

#[derive(Debug, Serialize)]
pub struct CandleSnapshotRequestBody {
    #[serde(rename = "type")]
    pub type_: String,
    pub req: CandleSnapshotRequest,
}

/// Request type for request with type "maxBuilderFee"
#[derive(Debug, Serialize)]
pub struct BuilderFeeApprovalRequest {
    /// "maxBuilderFee"
    pub type_: String,
    /// Address in 42-character hexadecimal format; e.g. 0x0000000000000000000000000000000000000000.
    pub user: String,
    /// Address in 42-character hexadecimal format; e.g. 0x0000000000000000000000000000000000000000.
    pub builder: String,
}

/// Resposne type for request with type "maxBuilderFee". Maximum fee approved in tenths of a basis
/// point i.e. 1 means 0.001%.
pub type BuilderFeeApproval = u64;

#[derive(Debug, Serialize, Deserialize)]
pub struct Children;

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub coin: String,
    pub side: String,
    #[serde(rename = "limitPx")]
    pub limit_px: Decimal,
    pub sz: Decimal,
    pub oid: u64,
    pub timestamp: u64,
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: String,
    #[serde(rename = "isTrigger")]
    pub is_trigger: bool,
    #[serde(rename = "triggerPx")]
    pub trigger_px: Decimal,
    pub children: Vec<Order>,
    #[serde(rename = "isPositionTpsl")]
    pub is_position_tpsl: bool,
    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,
    #[serde(rename = "orderType")]
    pub order_type: String,
    #[serde(rename = "origSz")]
    pub orig_sz: Decimal,
    pub tif: String,
    pub cloid: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct HistoricalOrder {
    pub order: Order,
    pub status: String,
    #[serde(rename = "statusTimestamp")]
    pub status_timestamp: u64,
}

/// Response for request with type "historicalOrders"
pub type UserHistoricalOrders = Vec<HistoricalOrder>;

#[derive(Debug, Deserialize)]
pub struct SliceFill {
    #[serde(rename = "closedPnl")]
    pub closed_pnl: Decimal,
    pub coin: String,
    pub crossed: bool,
    pub dir: String,
    pub hash: String,
    pub oid: u64,
    pub px: Decimal,
    pub side: String,
    #[serde(rename = "startPosition")]
    pub start_position: Decimal,
    pub sz: Decimal,
    pub time: u64,
    pub fee: Decimal,
    #[serde(rename = "feeToken")]
    pub fee_token: String,
    pub tid: u64,
}

#[derive(Debug, Deserialize)]
pub struct Twap {
    pub fill: SliceFill,
    #[serde(rename = "twapId")]
    pub twap_id: u32,
}

/// Response for request with type "userTwapSliceFills"
pub type TwapSliceFills = Vec<Twap>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClearinghouseState {
    #[serde(rename = "marginSummary")]
    pub margin_summary: MarginSummary,
    #[serde(rename = "crossMarginSummary")]
    pub cross_margin_summary: MarginSummary,
    #[serde(rename = "crossMaintenanceMarginUsed")]
    pub cross_maintenance_margin_used: Decimal,
    pub withdrawable: Decimal,
    #[serde(rename = "assetPositions")]
    pub asset_positions: Vec<AssetPosition>,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarginSummary {
    #[serde(rename = "accountValue")]
    pub account_value: Decimal,
    #[serde(rename = "totalNtlPos")]
    pub total_ntl_pos: Decimal,
    #[serde(rename = "totalRawUsd")]
    pub total_raw_usd: Decimal,
    #[serde(rename = "totalMarginUsed")]
    pub total_margin_used: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetPosition {
    #[serde(rename = "type")]
    pub position_type: String, // "oneWay"
    pub position: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub coin: String,
    pub szi: String,
    pub leverage: Leverage,
    pub entry_px: String,
    pub position_value: String,
    pub unrealized_pnl: String,
    pub return_on_equity: String,
    pub liquidation_px: String,
    pub margin_used: String,
    pub max_leverage: u32,
    pub cum_funding: CumFunding,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Leverage {
    #[serde(rename = "type")]
    pub leverage_type: String, // "isolated" or "cross"
    pub value: u32,
    pub raw_usd: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CumFunding {
    pub all_time: String,
    pub since_open: String,
    pub since_change: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpotState {
    pub balances: Vec<SpotBalance>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpotBalance {
    pub coin: String,
    pub token: u64,
    pub total: Decimal,
    pub hold: Decimal,
    #[serde(rename = "entryNtl")]
    pub entry_ntl: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubAccount {
    pub name: String,
    #[serde(rename = "subAccountUser")]
    pub sub_account_user: String,
    pub master: String,
    #[serde(rename = "clearinghouseState")]
    pub clearinghouse_state: ClearinghouseState,
    #[serde(rename = "spotState")]
    pub spot_state: SpotState,
}

/// Response for request with types "subAccounts"
pub type UserSubAccounts = Option<Vec<SubAccount>>;

/// Request for request with type "vaultDetails"
#[derive(Debug, Serialize)]
pub struct VaultDetailsRequest {
    #[serde(rename = "type")]
    pub type_: String,
    /// Address in 42-character hexadecimal format; e.g. 0x0000000000000000000000000000000000000000.
    #[serde(rename = "vaultAddress")]
    pub vault_address: String,
    /// Address in 42-character hexadecimal format; e.g. 0x0000000000000000000000000000000000000000.
    pub user: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PortfolioEntry(
    pub String, // "day", "week", "month", etc.
    pub HistoryEntry,
);

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryPoint(pub u64, pub Decimal);

#[derive(Debug, Serialize, Deserialize)]
pub struct FollowerState;

#[derive(Debug, Serialize, Deserialize)]
pub struct Follower {
    pub user: String,
    #[serde(rename = "vaultEquity")]
    pub vault_equity: Decimal,
    pub pnl: Decimal,
    #[serde(rename = "allTimePnl")]
    pub all_time_pnl: Decimal,
    #[serde(rename = "daysFollowing")]
    pub days_following: u64,
    #[serde(rename = "vaultEntryTime")]
    pub vault_entry_time: u64,
    #[serde(rename = "lockupUntil")]
    pub lockup_until: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VaultRelationship {
    #[serde(rename = "type")]
    pub relationship_type: String, // "parent"
    pub data: RelationshipData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelationshipData {
    #[serde(rename = "childAddresses")]
    pub child_addresses: Vec<String>,
}

/// Response for request with type "vaultDetails"
#[derive(Debug, Serialize, Deserialize)]
pub struct VaultDetails {
    pub name: String,
    #[serde(rename = "vaultAddress")]
    pub vault_address: String,
    pub leader: String,
    pub description: String,
    pub portfolio: Vec<PortfolioEntry>,
    pub apr: Decimal,
    #[serde(rename = "followerState")]
    pub follower_state: Option<FollowerState>, // null in example
    #[serde(rename = "leaderFraction")]
    pub leader_fraction: f64,
    #[serde(rename = "leaderComission")]
    pub leader_commission: f64,
    pub followers: Vec<Follower>,
    #[serde(rename = "maxDistributable")]
    pub max_distributable: f64,
    #[serde(rename = "maxWithdrawable")]
    pub max_wthdrawable: f64,
    #[serde(rename = "isClosed")]
    pub is_closed: bool,
    pub relationship: VaultRelationship,
    #[serde(rename = "allowDeposits")]
    pub allow_deposits: bool,
    #[serde(rename = "alwaysCloseOnWithdraw")]
    pub always_close_on_withdraw: bool,
}

#[derive(Debug, Serialize)]
pub struct UserRequest {
    #[serde(rename = "type")]
    pub type_: String,
    /// Address in 42-character hexadecimal format; e.g. 0x0000000000000000000000000000000000000000.
    pub user: String,
}

#[derive(Debug, Deserialize)]
pub struct VaultDeposit {
    #[serde(rename = "vaultAddress")]
    pub vault_address: String,
    pub equity: Decimal,
}

/// Response for equest with type "userVaultEquities"
pub type UserVaultDeposits = Vec<VaultDeposit>;

#[derive(Debug, Deserialize)]
/// Response for request with type "userRole"
pub struct UserRole {
    /// "user" | "agent" | "vault" | "subaccount | "missing""
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryEntry {
    #[serde(rename = "accountValueHistory")]
    pub account_value_history: Vec<HistoryPoint>,
    #[serde(rename = "pnlHistory")]
    pub pnl_history: Vec<HistoryPoint>,
    pub vlm: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryOverview(
    pub String, // "day" | "week" | "month" | "allTime" | "perpDay" | "perpWeek" | "perpMonth" | "perpAllTime"
    pub HistoryEntry,
);

/// Response for request with type "portfolio"
pub type UserPortfolio = Vec<HistoryOverview>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReferredBy {
    pub referrer: String,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TokenStateEntry {
    Index(u64),
    State(TokenState),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenState {
    #[serde(rename = "cumVlm")]
    pub cum_vlm: Decimal,
    #[serde(rename = "unclaimedRewards")]
    pub unclaimed_rewards: Decimal,
    #[serde(rename = "claimedRewards")]
    pub claimed_rewards: Decimal,
    #[serde(rename = "builderRewards")]
    pub builder_rewards: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReferrerState {
    pub stage: String,
    pub data: Option<ReferrerData>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReferrerData {
    Ready {
        code: String,
        #[serde(rename = "referralStates")]
        referral_states: Vec<ReferralState>,
    },
    NeedToTrade {
        required: Decimal,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReferralState {
    #[serde(rename = "cumVlm")]
    pub cum_vlm: Decimal,
    #[serde(rename = "cumRewardedFeesSinceReferred")]
    pub cum_rewarded_fees_since_referred: Decimal,
    #[serde(rename = "cumFeesRewardedToReferrer")]
    pub cum_fees_rewarded_to_referrer: Decimal,
    #[serde(rename = "timeJoined")]
    pub time_joined: u64,
    pub user: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RewardHistory; // empty array type

/// Response for request with type "referral"
#[derive(Debug, Serialize, Deserialize)]
pub struct UserReferralInformation {
    #[serde(rename = "referredBy")]
    pub referred_by: Option<ReferredBy>,
    #[serde(rename = "cumVlm")]
    pub cum_vlm: Decimal, // USDC Only
    #[serde(rename = "unclaimedRewards")]
    pub unclaimed_rewards: Decimal, // USDC Only
    #[serde(rename = "claimedRewards")]
    pub claimed_rewards: Decimal, // USDC Only
    #[serde(rename = "builderRewards")]
    pub builder_rewards: Decimal, // USDC Only
    #[serde(rename = "tokenToState")]
    pub token_to_state: Vec<TokenStateEntry>,
    #[serde(rename = "referrerState")]
    pub referrer_state: Option<ReferrerState>,
    #[serde(rename = "rewardHistory")]
    pub reward_history: Vec<RewardHistory>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyUserVlm {
    pub date: String,
    #[serde(rename = "userCross")]
    pub user_cross: Decimal,
    #[serde(rename = "userAdd")]
    pub user_add: Decimal,
    pub exchange: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeSchedule {
    pub cross: Decimal,
    pub add: Decimal,
    #[serde(rename = "spotCross")]
    pub spot_cross: Decimal,
    #[serde(rename = "spotAdd")]
    pub spot_add: Decimal,
    pub tiers: FeeTiers,
    #[serde(rename = "referralDiscount")]
    pub referral_discount: Decimal,
    #[serde(rename = "stakingDiscountTiers")]
    pub staking_discount_tiers: Vec<StakingDiscount>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeTiers {
    pub vip: Vec<VipTier>,
    pub mm: Vec<MmTier>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VipTier {
    #[serde(rename = "ntlCutoff")]
    pub ntl_cutoff: Decimal,
    pub cross: Decimal,
    pub add: Decimal,
    #[serde(rename = "spotCross")]
    pub spot_cross: Decimal,
    #[serde(rename = "spotAdd")]
    pub spot_add: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MmTier {
    #[serde(rename = "makerFractionCutoff")]
    pub maker_fraction_cutoff: Decimal,
    pub add: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StakingLink {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "stakingUser")]
    pub staking_user: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StakingDiscount {
    #[serde(rename = "bpsOfMaxSupply")]
    pub bps_of_max_supply: Decimal,
    pub discount: Decimal,
}

/// Response for request with type "userFees".
#[derive(Debug, Serialize, Deserialize)]
pub struct UserFees {
    #[serde(rename = "dailyUserVlm")]
    pub daily_user_vlm: Vec<DailyUserVlm>,
    #[serde(rename = "feeSchedule")]
    pub fee_schedule: FeeSchedule,
    #[serde(rename = "userCrossRate")]
    pub user_cross_rate: Decimal,
    #[serde(rename = "userAddRate")]
    pub user_add_rate: Decimal,
    #[serde(rename = "userSpotCrossRate")]
    pub user_spot_cross_rate: Decimal,
    #[serde(rename = "userSpotAddRate")]
    pub user_spot_add_rate: Decimal,
    #[serde(rename = "activeReferralDiscount")]
    pub active_referral_discount: Decimal,
    pub trial: Option<Decimal>,
    #[serde(rename = "feeTrialEscrow")]
    pub fee_trial_escrow: Decimal,
    #[serde(rename = "nextTrialAvailableTimestamp")]
    pub next_trial_available_timestamp: Option<Decimal>,
    #[serde(rename = "stakingLink")]
    pub staking_link: Option<StakingLink>,
    #[serde(rename = "activeStakingDiscount")]
    pub active_staking_discount: StakingDiscount,
}

#[derive(Debug, Deserialize)]
pub struct StakingDelegation {
    pub validator: String,
    pub amount: Decimal,
    #[serde(rename = "lockedUntilTimestamp")]
    pub locked_until_timestamp: u64,
}

/// Response for the "delegations" request type.
pub type UserStakingDelegations = Vec<StakingDelegation>;

/// Response for the "delegatorSummary" request type.
#[derive(Debug, Deserialize)]
pub struct UserStakingSummary {
    pub delegated: Decimal,
    pub undelegated: Decimal,
    #[serde(rename = "totalPendingWithdrawal")]
    pub total_pending_withdrawal: Decimal,
    #[serde(rename = "nPendingWithdrawals")]
    pub n_pending_withdrawals: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct Delegate {
    pub validator: String,
    pub amount: Decimal,
    #[serde(rename = "isUndelegate")]
    pub is_undelegate: bool,
}

#[derive(Debug, Deserialize)]
pub struct Delta {
    pub delegate: Delegate,
}

#[derive(Debug, Deserialize)]
pub struct StakingHistory {
    pub time: u64,
    pub hash: String,
    pub delta: Delta,
}

/// Response for the "delegatorHistory" request type.
pub type UserStakingHistory = Vec<StakingHistory>;

#[derive(Debug, Deserialize)]
pub struct StakingReward {
    pub time: u64,
    pub source: String,
    #[serde(rename = "totalAmount")]
    pub total_amount: Decimal,
}

/// Response for the "delegatorRewards" request type.
pub type UserStakingRewards = Vec<StakingReward>;

/// Request for the "userDexAbstraction" request type.
#[derive(Debug, Serialize)]
pub struct UserHIP3DexAbstractionStateRequest {
    /// "userDexAbstraction"
    #[serde(rename = "type")]
    pub type_: String,
    /// Address in 42-character hexadecimal format; e.g. 0x0000000000000000000000000000000000000000.
    pub user: String,
}

/// Response for the "userDexAbstraction" request type.
pub type UserHIP3DexAbstractionState = bool;

/// Request for the "alignedQuoteTokenInfo" request type.
#[derive(Debug, Serialize)]
pub struct AlignedQuoteTokenInfoRequest {
    /// "alignedQuoteTokenInfo"
    #[serde(rename = "type")]
    pub type_: String,
    /// Token index
    pub token: usize,
}

pub type DailyAmountOwed = (String, Decimal);

/// Response for the "alignedQuoteTokenInfo" request type.
#[derive(Debug, Deserialize)]
pub struct AlignedQuoteTokenInfo {
    #[serde(rename = "isAligned")]
    pub is_aligned: bool,
    #[serde(rename = "fistAlignedTime")]
    pub first_aligned_time: u64,
    #[serde(rename = "evmMintedSupply")]
    pub evm_minted_supply: Decimal,
    #[serde(rename = "dailyAmountOwed")]
    pub daily_amount_owed: Vec<DailyAmountOwed>,
    #[serde(rename = "predictedRate")]
    pub predicted_rate: Decimal,
}
