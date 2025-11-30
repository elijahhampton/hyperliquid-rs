use rust_decimal::Decimal;
/// Response types for the info endpoints that are specific to spot.
/// Additional information for endpoint responses can be found
/// here: https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/info-endpoint/spot
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvmContract {
    address: String,
    #[serde(
        default,
        alias = "evm_extra_wei_decimals",
        alias = "evmExtraWeiDecimals"
    )]
    evm_extra_wei_decimals: i64,
}

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
    pub evm_contract: Option<EvmContract>,
    pub full_name: Option<String>,
    pub deployer_trading_fee_share: Decimal,
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
    pub day_ntl_vlm: Decimal,
    pub mark_px: Option<Decimal>,
    pub mid_px: Option<Decimal>,
    pub prev_day_px: Option<Decimal>,
}

pub type SpotMetaAndAssetContexts = (SpotMetadata, Vec<SpotAssetContext>);

/// Response type for POST /info with type "spotClearinghouseState"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotBalance {
    pub coin: String,
    pub token: i64,
    pub hold: Decimal,
    pub total: Decimal,
    pub entry_ntl: Decimal,
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
    pub token: i64,
    pub spec: TokenSpec,
    pub full_name: String,
    pub spots: Vec<u32>,
    pub max_supply: u64,
    pub hyperliquidity_genesis_balance: u64,
    pub total_genesis_balance_wei: u64,
    /// Array of [address, balance] pairs
    pub user_genesis_balances: Vec<(String, Decimal)>,
    /// Array of [token_id, balance] pairs
    pub existing_token_genesis_balances: Vec<(i64, Decimal)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GasAuction {
    pub start_time_seconds: u64,
    pub duration_seconds: u64,
    pub start_gas: Decimal,
    pub current_gas: Option<Decimal>,
    pub end_gas: Decimal,
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
    pub start_gas: Decimal,
    pub current_gas: Option<Decimal>,
    pub end_gas: Option<Decimal>,
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
