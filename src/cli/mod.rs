#[allow(unused_imports)]
use crate::types::exchange::OrderRequest;
use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum Commands {
    AllMids {
        #[arg(short, long)]
        dex: Option<String>,
    },
    SubscribeAllMids {
        #[arg(long)]
        dex: Option<String>,
    },
    OpenOrders {
        #[arg(short, long)]
        user: String,
        #[arg(short, long)]
        dex: Option<String>,
    },
    FrontendOpenOrders {
        #[arg(short, long)]
        user: String,
        #[arg(short, long)]
        dex: Option<String>,
    },
    UserFills {
        #[arg(short, long)]
        user: String,
        #[arg(short, long)]
        aggregate_by_time: Option<bool>,
    },
    UserFillsByTime {
        #[arg(short, long)]
        user: String,
        #[arg(short, long)]
        start_time: u64,
        #[arg(short, long)]
        end_time: Option<u64>,
        #[arg(short, long)]
        aggregate_by_time: Option<bool>,
    },
    UserRateLimit {
        #[arg(short, long)]
        user: String,
    },
    OrderStatus {
        #[arg(short, long)]
        user: String,
        #[arg(short, long)]
        oid: u64, // Can be u64 or hex string
    },
    L2Book {
        #[arg(short, long)]
        coin: String,
        #[arg(short, long)]
        n_sig_figs: Option<u8>,
        #[arg(short, long)]
        mantissa: Option<u8>,
    },
    SubscribeL2Book {
        #[arg(short, long)]
        coin: String,
        #[arg(short, long)]
        n_sig_figs: Option<u8>,
        #[arg(short, long)]
        mantissa: Option<u8>,
    },
    CandleSnapshot {
        #[arg(short, long)]
        coin: String,
        #[arg(short, long)]
        interval: String,
        #[arg(short, long)]
        start_time: u64,
        #[arg(short, long)]
        end_time: u64,
    },
    SubscribeCandleSnapshot {
        #[arg(short, long)]
        coin: String,
        #[arg(short, long)]
        interval: String,
    },
    HistoricalOrders {
        #[arg(short, long)]
        user: String,
    },
    SubAccounts {
        #[arg(short, long)]
        user: String,
    },
    VaultDetails {
        #[arg(short, long)]
        vault_address: String,
        #[arg(short, long)]
        user: Option<String>,
    },
    UserVaultEquities {
        #[arg(short, long)]
        user: String,
    },
    UserRole {
        #[arg(short, long)]
        user: String,
    },
    Portfolio {
        #[arg(short, long)]
        user: String,
    },
    Referral {
        #[arg(short, long)]
        user: String,
    },
    UserFees {
        #[arg(short, long)]
        user: String,
    },
    SubscribeNotifications {
        #[arg(short, long)]
        user: String,
    },
    SubscribeWebData3 {
        #[arg(short, long)]
        user: String,
    },
    SubscribeTwapStates {
        #[arg(short, long)]
        user: String,
    },
    SubscribeClearinghouseState {
        #[arg(short, long)]
        user: String,
    },
    SubscribeOpenOrders {
        #[arg(short, long)]
        user: String,
    },
    SubscribeUserEvents {
        #[arg(short, long)]
        user: String,
    },
    SubscribeUserFills {
        #[arg(short, long)]
        user: String,
    },
    SubscribeUserFunding {
        #[arg(short, long)]
        user: String,
    },
    SubscribeUserNonFundingLedgerUpdates {
        #[arg(short, long)]
        user: String,
    },
    SubscribeActiveAssetCtx {
        #[arg(short, long)]
        coin: String,
    },
    SubscribeActiveAssetData {
        #[arg(short, long)]
        user: String,
        #[arg(short, long)]
        coin: String,
    },
    SubscribeUserTwapSliceFills {
        #[arg(short, long)]
        user: String,
    },
    SubscribeUserTwapHistory {
        #[arg(short, long)]
        user: String,
    },
    SubscribeBbo {
        #[arg(short, long)]
        user: String,
    },
}

#[derive(Parser)]
pub struct Cli {
    /// Enables subscriptions through the CLI
    #[arg(short, long)]
    pub subscriptions: Option<bool>,

    /// Picks a specific network, i.e. 'testnet' or 'mainnet'
    #[arg(short, long)]
    pub network: Option<String>,

    /// Allows CLI to use HL_PRIVATE_KEY env var in configuration (requires HL_PRIVATE_KEY end var)
    #[arg(short, long)]
    pub allow_signer_key_env: Option<bool>,

    #[command(subcommand)]
    pub command: Commands,
}
