#![allow(unused_imports, clippy::too_many_lines)]
use alloy::signers::local::LocalSigner;
use clap::{Parser, Subcommand};
use rhyperliquid::{
    cli::{Cli, Commands},
    init_tracing::init_tracing,
    types::{info::user::CandleSnapshotRequest, ws::SubscriptionResponse},
    HyperliquidClientBuilder,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    let cli = Cli::parse();

    #[allow(clippy::expect_used)]
    let signer = env::var("HL_PRIVATE_KEY").expect("HL_PRIVATE_KEY env var is missing");
    let mut hyperliquid = &mut HyperliquidClientBuilder::new();

    // Check if the user provided a network, default to testnet
    if let Some(network) = cli.network {
        match (network as String).to_lowercase().as_str() {
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

    if let Some(subscriptions) = cli.subscriptions {
        if subscriptions == true {
            hyperliquid = hyperliquid.with_subscriptions();
        }
    }

    // Check if user provides permission to check env var for signer key
    if let Some(signer_permission) = cli.allow_signer_key_env {
        if signer_permission {
            hyperliquid = hyperliquid.with_wallet(LocalSigner::from_slice(signer.as_bytes())?);
        }
    }

    let client = hyperliquid.build()?;
    let info_api = &client.info();
    let mut subs = client.subscriptions().await?;

    match cli.command {
        Commands::AllMids { dex } => {
            let all_mids = info_api.all_mids(dex).await?;
            tracing::info!("{:?}", all_mids);
            return Ok(());
        }
        Commands::OpenOrders { user, dex } => {
            let open_orders = info_api.open_orders(&user, dex.as_deref()).await?;
            tracing::info!("{:?}", open_orders);
            return Ok(());
        }
        Commands::FrontendOpenOrders { user, dex } => {
            let frontend_open_orders = info_api
                .open_orders_with_additional_info(&user, dex.as_deref())
                .await?;
            tracing::info!("{:?}", frontend_open_orders);
            return Ok(());
        }
        Commands::UserFills {
            user,
            aggregate_by_time,
        } => {
            let user_fills = info_api.fills(&user, aggregate_by_time).await?;
            tracing::info!({"{:?}", user_fills});
            return Ok(());
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
            return Ok(());
        }
        Commands::UserRateLimit { user } => {
            let user_rate_limit = info_api.rate_limits(&user).await?;
            tracing::info!("{:?}", user_rate_limit);
            return Ok(());
        }
        Commands::OrderStatus { user, oid } => {
            let order_status = info_api
                .order_status(&user, rhyperliquid::types::info::OrderId::Numeric(oid))
                .await?;
            tracing::info!("{:?}", order_status);
            return Ok(());
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
            return Ok(());
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
            return Ok(());
        }
        Commands::HistoricalOrders { user } => {
            let historical_orders = info_api.historical_orders(&user).await?;
            tracing::info!("{:?}", historical_orders);
            return Ok(());
        }
        Commands::SubAccounts { user } => {
            let sub_accounts = info_api.subaccounts(&user).await?;
            if let Some(accounts) = sub_accounts {
                tracing::info!("{:?}", accounts);
            } else {
                tracing::info!("User {:?} does not have subaccounts.", user);
            }
            return Ok(());
        }
        Commands::VaultDetails {
            vault_address,
            user,
        } => {
            let vault_details = info_api
                .vault_details(&vault_address, user.as_deref())
                .await?;
            tracing::info!("{:?}", vault_details);
            return Ok(());
        }
        Commands::UserVaultEquities { user } => {
            let equities = info_api.vault_deposits(&user).await?;
            tracing::info!("{:?}", equities);
            return Ok(());
        }
        Commands::UserRole { user } => {
            let role = info_api.role(&user).await?;
            tracing::info!("{:?}", role);
            return Ok(());
        }
        Commands::Portfolio { user } => {
            let portfolio = info_api.portfolio(&user).await?;
            tracing::info!("{:?}", portfolio);
            return Ok(());
        }
        Commands::Referral { user } => {
            let referral = info_api.portfolio(&user).await?;
            tracing::info!("{:?}", referral);
            return Ok(());
        }
        Commands::UserFees { user } => {
            let user_fees = info_api.fees(&user).await?;
            tracing::info!("{:?}", user_fees);
            return Ok(());
        }
        Commands::SubscribeOpenOrders { user } => {
            subs.subscribe_open_orders(user).await?;
        }
        Commands::SubscribeAllMids { dex } => {
            subs.subscribe_all_mids(dex).await?;
        }
        Commands::SubscribeL2Book {
            coin,
            n_sig_figs,
            mantissa,
        } => {
            subs.subscribe_l2_book(coin, n_sig_figs, mantissa).await?;
        }
        Commands::SubscribeCandleSnapshot { coin, interval } => {
            subs.subscribe_candle_snapshot(coin, interval).await?;
        }
        Commands::SubscribeNotifications { user } => {
            subs.subscribe_notifications(user).await?;
        }
        Commands::SubscribeWebData3 { user } => {
            subs.subscribe_webdata3(user).await?;
        }
        Commands::SubscribeTwapStates { user } => {
            subs.subscribe_twap_states(user).await?;
        }
        Commands::SubscribeClearinghouseState { user } => {
            subs.subscribe_clearinghouse_state(user).await?;
        }
        Commands::SubscribeUserEvents { user } => {
            subs.subscribe_user_events(user).await?;
        }
        Commands::SubscribeUserFills { user } => {
            subs.subscribe_user_fills(user).await?;
        }
        Commands::SubscribeUserFunding { user } => {
            subs.subscribe_user_funding(user).await?;
        }
        Commands::SubscribeUserNonFundingLedgerUpdates { user } => {
            subs.subscribe_user_non_funding_ledger_updates(user).await?;
        }
        Commands::SubscribeActiveAssetCtx { coin } => {
            subs.subscribe_active_asset_ctx(coin).await?;
        }
        Commands::SubscribeActiveAssetData { user, coin } => {
            subs.subscribe_active_asset_data(user, coin).await?;
        }
        Commands::SubscribeUserTwapSliceFills { user } => {
            subs.subscribe_user_twap_slice_fills(user).await?;
        }
        Commands::SubscribeUserTwapHistory { user } => {
            subs.subscribe_user_twap_history(user).await?;
        }
        Commands::SubscribeBbo { user } => {
            subs.subscribe_bbo(user).await?;
        }
    }

    let mut events = subs.events;

    loop {
        match events.recv().await? {
            SubscriptionResponse::AllMids(ws_all_mids) => {
                tracing::info!("{}", serde_json::to_string_pretty(&ws_all_mids)?);
            }
            SubscriptionResponse::L2Book(ws_l2_book) => {
                tracing::info!("{}", serde_json::to_string_pretty(&ws_l2_book)?);
            }
            SubscriptionResponse::Candle(ws_candle) => {
                tracing::info!("{}", serde_json::to_string_pretty(&ws_candle)?);
            }
            SubscriptionResponse::Notification(ws_notification) => {
                tracing::info!("{}", serde_json::to_string_pretty(&ws_notification)?);
            }
            SubscriptionResponse::WebData3(ws_webdata3) => {
                tracing::info!("{}", serde_json::to_string_pretty(&ws_webdata3)?);
            }
            SubscriptionResponse::TwapStates(ws_twap_states) => {
                tracing::info!("{}", serde_json::to_string_pretty(&ws_twap_states)?);
            }
            SubscriptionResponse::ClearinghouseState(ws_clearinghouse_state) => {
                tracing::info!("{}", serde_json::to_string_pretty(&ws_clearinghouse_state)?);
            }
            SubscriptionResponse::OpenOrders(ws_open_orders) => {
                tracing::info!("{}", serde_json::to_string_pretty(&ws_open_orders)?);
            }
            SubscriptionResponse::UserEvents(ws_user_events) => {
                tracing::info!("{}", serde_json::to_string_pretty(&ws_user_events)?);
            }
            SubscriptionResponse::UserFills(ws_fills) => {
                tracing::info!("{}", serde_json::to_string_pretty(&ws_fills)?);
            }
            SubscriptionResponse::UserFundings(ws_fundings) => {
                tracing::info!("{}", serde_json::to_string_pretty(&ws_fundings)?);
            }
            SubscriptionResponse::UserNonFundingLedgerUpdates(
                ws_user_non_funding_ledger_updates,
            ) => {
                tracing::info!(
                    "{}",
                    serde_json::to_string_pretty(&ws_user_non_funding_ledger_updates)?
                );
            }
            SubscriptionResponse::ActiveAssetCtx(ws_active_asset_ctx) => {
                tracing::info!("{}", serde_json::to_string_pretty(&ws_active_asset_ctx)?);
            }
            SubscriptionResponse::ActiveAssetData(ws_active_asset_data) => {
                tracing::info!("{}", serde_json::to_string_pretty(&ws_active_asset_data)?);
            }
            SubscriptionResponse::UserTwapSliceFills(ws_user_twap_slice_fills) => {
                tracing::info!(
                    "{}",
                    serde_json::to_string_pretty(&ws_user_twap_slice_fills)?
                );
            }
            SubscriptionResponse::UserTwapHistory(ws_user_twap_history) => {
                tracing::info!("{}", serde_json::to_string_pretty(&ws_user_twap_history)?);
            }
            SubscriptionResponse::Bbo(ws_bbo) => {
                tracing::info!("{}", serde_json::to_string_pretty(&ws_bbo)?);
            }
            SubscriptionResponse::Error(ws_error_response) => {
                tracing::info!("{:?}", serde_json::to_string_pretty(&ws_error_response));
            }
            SubscriptionResponse::SubscriptionResponse(subscription_confirmation) => {
                tracing::info!("Subscription confirmed {:?}", subscription_confirmation);
            }
            SubscriptionResponse::Pong => {
                tracing::info!("Received `pong` response from the server");
            }
            SubscriptionResponse::Trades(ws_trades) => {
                tracing::info!("{:?}", ws_trades);
            }
        }
    }
}
