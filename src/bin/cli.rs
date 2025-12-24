#![allow(unused_imports, clippy::too_many_lines)]
use alloy::signers::local::LocalSigner;
use clap::{Parser, Subcommand};
use rhyperliquid::{
    cli::{Cli, Commands},
    init_tracing::init_tracing,
    types::info::user::CandleSnapshotRequest,
    HyperliquidClientBuilder,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "cli")]
    init_tracing();

    #[cfg(feature = "cli")]
    let cli = Cli::parse();

    #[allow(clippy::expect_used)]
    #[cfg(feature = "cli")]
    let signer = env::var("HL_PRIVATE_KEY").expect("HL_PRIVATE_KEY env var is missing");

    #[cfg(feature = "cli")]
    let mut hyperliquid = &mut HyperliquidClientBuilder::new();

    // Check if the user provided a network, default to testnet
    #[cfg(feature = "cli")]
    if let Some(network) = cli.network {
        match network.to_lowercase().as_str() {
            "testnet" => {
                hyperliquid = hyperliquid.testnet();
            }
            "mainnet" => {
                hyperliquid = hyperliquid.mainnet();
            }
            _ => hyperliquid = hyperliquid.testnet(),
        }
    } else {
        hyperliquid = hyperliquid.testnet();
    }

    // Check if user provides permission to check env var for signer key
    #[cfg(feature = "cli")]
    if let Some(signer_permission) = cli.allow_signer_key_env {
        if signer_permission {
            hyperliquid = hyperliquid.with_wallet(LocalSigner::from_slice(signer.as_bytes())?);
        }
    }

    #[cfg(feature = "cli")]
    let client = hyperliquid.build()?;
    #[cfg(feature = "cli")]
    let info_api = &client.info();

    #[cfg(feature = "cli")]
    match cli.command {
        Commands::AllMids { dex } => {
            let all_mids = info_api.all_mids(dex).await?;
            tracing::info!("{:?}", all_mids);
        }
        Commands::OpenOrders { user, dex } => {
            let open_orders = info_api.open_orders(&user, dex.as_deref()).await?;
            tracing::info!("{:?}", open_orders);
        }
        Commands::FrontendOpenOrders { user, dex } => {
            let frontend_open_orders = info_api
                .open_orders_with_additional_info(&user, dex.as_deref())
                .await?;
            tracing::info!("{:?}", frontend_open_orders);
        }
        Commands::UserFills {
            user,
            aggregate_by_time,
        } => {
            let user_fills = info_api.fills(&user, aggregate_by_time).await?;
            tracing::info!({"{:?}", user_fills});
        }
        Commands::UserFillsByTime {
            user,
            start_time,
            end_time,
            aggregate_by_time,
        } => {
            let fills_by_time = info_api
                .fills_by_time(&user, start_time, end_time, aggregate_by_time)
                .await?;
            tracing::info!("{:?}", fills_by_time);
        }
        Commands::UserRateLimit { user } => {
            let user_rate_limit = info_api.rate_limits(&user).await?;
            tracing::info!("{:?}", user_rate_limit);
        }
        Commands::OrderStatus { user, oid } => {
            let order_status = info_api
                .order_status(&user, rhyperliquid::types::info::OrderId::Numeric(oid))
                .await?;
            tracing::info!("{:?}", order_status);
        }
        Commands::L2Book {
            coin,
            n_sig_figs,
            mantissa,
        } => {
            let l2_book = info_api
                .l2_book_snapshot(&coin, n_sig_figs, mantissa)
                .await?;
            tracing::info!("{:?}", l2_book);
        }
        Commands::CandleSnapshot {
            coin,
            interval,
            start_time,
            end_time,
        } => {
            let snapshot = info_api
                .candle_snapshot(CandleSnapshotRequest {
                    coin,
                    interval,
                    start_time,
                    end_time,
                })
                .await?;

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
        Commands::VaultDetails {
            vault_address,
            user,
        } => {
            let vault_details = info_api
                .vault_details(&vault_address, user.as_deref())
                .await?;
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
