use hyperliquid_rs::{
    example_helpers::user,
    init_tracing::init_tracing,
    prelude::{
        current_time_millis,
        info::{perpetual::PerpetualDex, user::CandleSnapshotRequest},
        HyperliquidClientBuilder,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    let hyperliquid = HyperliquidClientBuilder::new().testnet().build()?;
    let user = user();

    let all_mids = hyperliquid.info().all_mids(None).await?;
    tracing::info!("{:?}", all_mids);

    let l2_book_snapshot = hyperliquid
        .info()
        .l2_book_snapshot("BTC", None, None)
        .await?;
    tracing::info!("{:?}", l2_book_snapshot);

    let candle_snapshot_start_time = current_time_millis();
    let candle_snapshot_end_time = candle_snapshot_start_time.saturating_add(30 * 60);

    let candle_snapshot_req = CandleSnapshotRequest {
        coin: "BTC".to_owned(),
        interval: "15m".to_owned(),
        start_time: candle_snapshot_start_time,
        end_time: candle_snapshot_end_time,
    };

    let candle_snapshot = hyperliquid
        .info()
        .candle_snapshot(candle_snapshot_req)
        .await?;
    tracing::info!("{:?}", candle_snapshot);

    let perpetual_dexs = hyperliquid.info().perpetual_dexs().await?;
    tracing::info!("{:?}", perpetual_dexs);

    let perp_dexs = perpetual_dexs
        .iter()
        .filter_map(|dex| dex.clone())
        .collect::<Vec<PerpetualDex>>();

    #[allow(clippy::indexing_slicing)]
    let perp_dex = perp_dexs[0].clone();

    let perpetual_metadata = hyperliquid
        .info()
        .perpetuals_metadata(Some(&perp_dex.name))
        .await?;
    tracing::info!("{:?}", perpetual_metadata);

    let user_perpetual_account_summary = hyperliquid
        .info()
        .perpetuals_account_summary(&user, Some(&perp_dex.name))
        .await?;
    tracing::info!("{:?}", user_perpetual_account_summary);

    let perpetual_asset_ctxs = hyperliquid.info().perpetuals_asset_contexts().await?;
    tracing::info!("{:?}", perpetual_asset_ctxs);

    let user_perpetuals_account_summary = hyperliquid
        .info()
        .perpetuals_account_summary(&user, Some(&perp_dex.name))
        .await?;
    tracing::info!("{:?}", user_perpetuals_account_summary);

    let start_time = current_time_millis();

    let user_funding_history = hyperliquid
        .info()
        .funding_history_updates(&user, start_time, None)
        .await?;
    tracing::info!("{:?}", user_funding_history);

    let non_funding_ledger_updates = hyperliquid
        .info()
        .non_funding_ledger_updates(&user, start_time, None)
        .await?;
    tracing::info!("{:?}", non_funding_ledger_updates);

    let historical_funding_rates = hyperliquid
        .info()
        .historical_funding_rates("BTC", start_time, None)
        .await?;
    tracing::info!("{:?}", historical_funding_rates);

    let perps_at_open_interest_caps = hyperliquid
        .info()
        .query_perps_at_open_interest_caps()
        .await?;
    tracing::info!("{:?}", perps_at_open_interest_caps);

    let active_asset_data = hyperliquid.info().active_asset_data(user, "BTC").await?;
    tracing::info!("{:?}", active_asset_data);

    let perp_dex_limits = hyperliquid
        .info()
        .builder_deployed_perp_market_limits(&perp_dex.name)
        .await?;
    tracing::info!("{:?}", perp_dex_limits);

    let perp_market_status = hyperliquid
        .info()
        .perp_market_status(&perp_dex.name)
        .await?;
    tracing::info!("{:?}", perp_market_status);

    Ok(())
}
