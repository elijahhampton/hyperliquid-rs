use alloy::signers::local::LocalSigner;
use clap::{Parser, Subcommand};
use rhyperliquid::{init_tracing::init_tracing, types::info::user::CandleSnapshotRequest, HyperliquidClientBuilder};
use std::env;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    network: Option<String>,

    #[arg(short, long)]
    allow_signer_key_env: Option<bool>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    AllMids {
        #[arg(short, long)]
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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    let cli = Cli::parse();

    #[allow(clippy::expect_used)]
    let signer = env::var("HL_PRIVATE_KEY").expect("HL_PRIVATE_KEY env var is missing");

    let mut hyperliquid = &mut HyperliquidClientBuilder::new();

    // Check if the user provided a network, default to testnet
    if let Some(network) = cli.network {
        match network.to_lowercase().as_str() {
            "testnet" => {
                hyperliquid = hyperliquid.testnet();
            },
            "mainnet" => {
                hyperliquid = hyperliquid.mainnet();
            },
            _ => hyperliquid = hyperliquid.testnet()
        }
    } else {
        hyperliquid = hyperliquid.testnet();
    }

    // Check if user provides permission to check env var for signer key
    if let Some(signer_permission) = cli.allow_signer_key_env {
        if signer_permission {
            hyperliquid = hyperliquid.with_wallet(LocalSigner::from_slice(signer.as_bytes())?);
        }
    }

    let client = hyperliquid.build()?;
    let info_api = &client.info();

    match cli.command {
        Commands::AllMids{dex}=>{
            let all_mids = info_api.all_mids(dex).await?;
            tracing::info!("{:?}", all_mids);
        },
        Commands::OpenOrders{user,dex}=>{
            let open_orders = info_api.open_orders(&user, dex.as_deref()).await?;
            tracing::info!("{:?}", open_orders);
        }
        Commands::FrontendOpenOrders { user, dex } => {
            let frontend_open_orders = info_api.open_orders_with_additional_info(&user, dex.as_deref()).await?;
            tracing::info!("{:?}", frontend_open_orders);
        }
        Commands::UserFills { user, aggregate_by_time } => {
            let user_fills = info_api.fills(&user, aggregate_by_time).await?;
            tracing::info!({"{:?}", user_fills});
        }
        Commands::UserFillsByTime { user, start_time, end_time, aggregate_by_time } => {
            let fills_by_time = info_api.fills_by_time(&user, start_time, end_time, aggregate_by_time).await?;
            tracing::info!("{:?}", fills_by_time);
        }
        Commands::UserRateLimit { user } => {
            let user_rate_limit = info_api.rate_limits(&user).await?;
            tracing::info!("{:?}", user_rate_limit);
        }
        Commands::OrderStatus { user, oid } => {
            let order_status = info_api.order_status(&user, rhyperliquid::types::info::OrderId::Numeric(oid)).await?;
            tracing::info!("{:?}", order_status);
        }
        Commands::L2Book { coin, n_sig_figs, mantissa } => {
            let l2_book = info_api.l2_book_snapshot(&coin, n_sig_figs, mantissa).await?;
            tracing::info!("{:?}", l2_book);
        }
        Commands::CandleSnapshot { coin, interval, start_time, end_time } => {
            let snapshot = info_api.candle_snapshot(CandleSnapshotRequest {
                coin,
                interval,
                start_time,
                end_time
            }).await?;

            tracing::info!("{:?}", snapshot);
        }
        Commands::HistoricalOrders { user } => {
            let historical_orders = info_api.historical_orders(&user).await?;
            tracing::info!("{:?}", historical_orders);
        }
        Commands::SubAccounts { user } => {
            let sub_accounts = info_api.subaccounts(&user).await?;
            if let Some(accounts) = sub_accounts {
                tracing::info!("{:?}", accounts);
            } else {
                tracing::info!("User {:?} does not have subaccounts.", user);
            }
        }
        Commands::VaultDetails { vault_address, user } => {
            let vault_details = info_api.vault_details(&vault_address, user.as_deref()).await?;
            tracing::info!("{:?}", vault_details);
        }
        Commands::UserVaultEquities { user } => {
            let equities = info_api.vault_deposits(&user).await?;
            tracing::info!("{:?}", equities);
        }
        Commands::UserRole { user } => {
            let role = info_api.role(&user).await?;
            tracing::info!("{:?}", role);
        }
        Commands::Portfolio { user } => {
            let portfolio = info_api.portfolio(&user).await?;
            tracing::info!("{:?}", portfolio);
        }
        Commands::Referral { user } => {
            let referral = info_api.portfolio(&user).await?;
            tracing::info!("{:?}", referral);
        }
        Commands::UserFees { user } => {
            let user_fees = info_api.fees(&user).await?;
            tracing::info!("{:?}", user_fees);
        }
    }

    Ok(())
}
