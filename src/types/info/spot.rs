/// Response types for the info endpoints that are specific to spot.
/// Additional information for endpoint responses can be found
/// here: https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/info-endpoint/spot


use serde::{Deserialize, Serialize};

/// Response type for POST /info with type "spotMeta"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotToken {
    pub name: String,
    pub sz_decimals: u8,
    pub wei_decimals: u8,
    pub index: u32,
    pub token_id: String,
    pub is_canonical: bool,
    pub evm_contract: Option<String>,
    pub full_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotMarket {
    pub name: String,
    pub tokens: [u32; 2],
    pub index: u32,
    pub is_canonical: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotMetadata {
    pub tokens: Vec<SpotToken>,
    pub universe: Vec<SpotMarket>,
}

/// Response type for POST /info with type "spotMetaAndAssetCtxs"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotAssetContext {
    pub day_ntl_vlm: String,
    pub mark_px: String,
    pub mid_px: String,
    pub prev_day_px: String,
}

pub type SpotMetaAndAssetContexts = (SpotMetadata, Vec<SpotAssetContext>);

/// Response type for POST /info with type "spotClearinghouseState"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotBalance {
    pub coin: String,
    pub token: u32,
    pub hold: String,
    pub total: String,
    pub entry_ntl: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotClearinghouseState {
    pub balances: Vec<SpotBalance>,
}

/// Response type for POST /info with type "spotDeployState"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenSpec {
    pub name: String,
    pub sz_decimals: u8,
    pub wei_decimals: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeployState {
    pub token: u32,
    pub spec: TokenSpec,
    pub full_name: String,
    pub spots: Vec<u32>,
    pub max_supply: u64,
    pub hyperliquidity_genesis_balance: String,
    pub total_genesis_balance_wei: String,
    /// Array of [address, balance] pairs
    pub user_genesis_balances: Vec<(String, String)>,
    /// Array of [token_id, balance] pairs
    pub existing_token_genesis_balances: Vec<(u32, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GasAuction {
    pub start_time_seconds: u64,
    pub duration_seconds: u64,
    pub start_gas: String,
    pub current_gas: Option<String>,
    pub end_gas: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotDeployState {
    pub states: Vec<DeployState>,
    pub gas_auction: GasAuction,
}

/// Response type for POST /info with type "spotPairDeployAuctionStatus"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotPairDeployAuctionStatus {
    pub start_time_seconds: u64,
    pub duration_seconds: u64,
    pub start_gas: String,
    pub current_gas: String,
    pub end_gas: Option<String>,
}

/// Response type for POST /info with type "tokenDetails"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenGenesis {
    /// Array of [address, balance] pairs
    pub user_balances: Vec<(String, String)>,
    pub existing_token_balances: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenDetails {
    pub name: String,
    pub max_supply: String,
    pub total_supply: String,
    pub circulating_supply: String,
    pub sz_decimals: u8,
    pub wei_decimals: u8,
    pub mid_px: String,
    pub mark_px: String,
    pub prev_day_px: String,
    pub genesis: TokenGenesis,
    pub deployer: String,
    pub deploy_gas: String,
    pub deploy_time: String,
    pub seeded_usdc: String,
    pub non_circulating_user_balances: Vec<serde_json::Value>,
    pub future_emissions: String,
}
