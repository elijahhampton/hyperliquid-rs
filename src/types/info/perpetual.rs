/// Response types for the info endpoints that are specific to perpetuals.
/// Additional information for endpoint responses can be found
/// here: `<https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/info-endpoint/perpetuals>`
use serde::{Deserialize, Serialize};

/// Response type for POST /info with type "perpDexs"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerpetualDex {
    pub name: String,
    pub full_name: String,
    pub deployer: String,
    pub oracle_updater: Option<String>,
    pub fee_recipient: Option<String>,
    /// Array of [coin, cap] pairs
    pub asset_to_streaming_oi_cap: Vec<(String, String)>,
}

pub type PerpetualDexs = Vec<Option<PerpetualDex>>;

/// Response type for POST /info with type "meta"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetInfo {
    pub name: String,
    pub sz_decimals: u8,
    pub max_leverage: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_isolated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_delisted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// "strictIsolated" | "noCross"
    pub margin_mode: Option<String>,
    pub margin_table_id: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginTier {
    pub lower_bound: String,
    pub max_leverage: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginTable {
    pub description: String,
    pub margin_tiers: Vec<MarginTier>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerpetualsMetadata {
    pub universe: Vec<AssetInfo>,
    /// Array of [id, table] pairs
    pub margin_tables: Vec<(u32, MarginTable)>,
}

/// Response type for POST /info with type "metaAndAssetCtxs"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetContext {
    pub day_ntl_vlm: String,
    pub funding: String,
    pub impact_pxs: Option<[String; 2]>,
    pub mark_px: String,
    pub mid_px: Option<String>,
    pub open_interest: String,
    pub oracle_px: String,
    pub premium: Option<String>,
    pub prev_day_px: String,
}

pub type MetaAndAssetContexts = (PerpetualsMetadata, Vec<AssetContext>);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CumulativeFunding {
    pub all_time: String,
    pub since_change: String,
    pub since_open: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Leverage {
    /// "isolated" | "cross"
    #[serde(rename = "type")]
    pub type_: String,
    pub value: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub coin: String,
    pub cum_funding: CumulativeFunding,
    pub entry_px: String,
    pub leverage: Leverage,
    pub liquidation_px: String,
    pub margin_used: String,
    pub max_leverage: u32,
    pub position_value: String,
    pub return_on_equity: String,
    pub szi: String,
    pub unrealized_pnl: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetPosition {
    pub position: Position,
    /// "oneWay"
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginSummary {
    pub account_value: String,
    pub total_margin_used: String,
    pub total_ntl_pos: String,
    pub total_raw_usd: String,
}

/// Response type for POST /info with type "clearinghouseState"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearinghouseState {
    pub asset_positions: Vec<AssetPosition>,
    pub cross_maintenance_margin_used: String,
    pub cross_margin_summary: MarginSummary,
    pub margin_summary: MarginSummary,
    pub time: u64,
    pub withdrawable: String,
}

/// Response type for POST /info with type "userFunding" | "userNonFundingLedgerUpdates"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingDelta {
    pub coin: String,
    pub funding_rate: String,
    pub szi: String,
    /// "funding"
    #[serde(rename = "type")]
    pub type_: String,
    pub usdc: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LedgerUpdate {
    pub delta: FundingDelta,
    pub hash: String,
    pub time: u64,
}

pub type LedgerUpdates = Vec<LedgerUpdate>;

/// Response type for POST /info with type "fundingHistory"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingRate {
    pub coin: String,
    pub funding_rate: String,
    pub premium: String,
    pub time: u64,
}

pub type FundingHistory = Vec<FundingRate>;

/// Response type for POST /info with type "predictedFundings"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PredictedFunding {
    pub funding_rate: String,
    pub next_funding_time: u64,
}

/// Array of [`venue_name`, `predicted_funding`] pairs
pub type VenueFundings = Vec<(String, PredictedFunding)>;

/// Array of [coin, `venue_fundings`] pairs
pub type PredictedFundings = Vec<(String, VenueFundings)>;

/// Response type for POST /info with type "perpsAtOpenInterestCap"
pub type PerpsAtOpenInterestCap = Vec<String>;

/// Response type for POST /info with type "perpDeployAuctionStatus"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerpDeployAuctionStatus {
    pub start_time_seconds: u64,
    pub duration_seconds: u64,
    pub start_gas: String,
    pub current_gas: String,
    pub end_gas: Option<String>,
}

/// Response type for POST /info with type "activeAssetData"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveAssetData {
    pub user: String,
    pub coin: String,
    pub leverage: Leverage,
    pub max_trade_szs: [String; 2],
    pub available_to_trade: [String; 2],
    pub mark_px: String,
}

/// Response type for POST /info with type "perpDexLimits"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerpDexLimits {
    pub total_oi_cap: String,
    pub oi_sz_cap_per_perp: String,
    pub max_transfer_ntl: String,
    /// Array of [coin, cap] pairs
    pub coin_to_oi_cap: Vec<(String, String)>,
}

/// Response type for POST /info with type "perpDexStatus"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerpDexStatus {
    pub total_net_deposit: String,
}
