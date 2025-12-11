#![allow(clippy::all)]
use rhyperliquid::{
    example_helpers::load_signer,
    init_tracing::init_tracing,
    response::ResponseInner,
    types::exchange::{order::Tpsl, Grouping, OrderRequest, OrderType, TriggerOrder},
    utils::current_time_millis,
    HyperliquidClientBuilder, HyperliquidError,
};
use rust_decimal::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    let pkey_signer = load_signer();

    let mut builder = HyperliquidClientBuilder::new();
    let hyperliquid = builder.testnet().with_wallet(pkey_signer.clone()).build()?;

    let info_api = hyperliquid.info();
    let exchange_api = hyperliquid.exchange();

    let perp_meta = info_api.perpetuals_metadata(None).await?;
    let doge_asset_and_idx = perp_meta
        .universe
        .iter()
        .enumerate()
        .find(|asset| asset.1.name == "DOGE");

    let (asset_idx, _) = doge_asset_and_idx.ok_or(HyperliquidError::Internal(
        "Missing asset in universe".to_string(),
    ))?;
    let asset_id = format!("@{}", asset_idx);

    // Get current price
    let all_mids = info_api.all_mids(None).await?;
    let doge_price = all_mids.get(&asset_id).ok_or(HyperliquidError::Internal(
        "Missing asset in universe".to_string(),
    ))?;
    let price_decimal = Decimal::from_str(&doge_price.to_string())?;

    // Set stop loss 5% below current price
    tracing::info!("\n=== Placing Stop Loss Order ===");
    let stop_loss_price = price_decimal.saturating_mul(Decimal::from_str("0.95")?); // 5% below
    let stop_loss_trigger = stop_loss_price.round_dp(5).to_string();

    let stop_loss_order = OrderRequest {
        a: u32::try_from(asset_idx)?,
        b: true,                      // SELL when triggered
        p: stop_loss_trigger.clone(), // Execution price
        s: "10.0".to_string(),
        r: true,
        t: OrderType::Trigger(TriggerOrder {
            is_market: true, // Execute as market order when triggered
            trigger_px: stop_loss_trigger.clone(),
            tpsl: Tpsl::Sl,
        }),
        c: None,
    };

    tracing::info!(
        "Stop loss will trigger at ${} (5% below current)",
        stop_loss_trigger
    );

    let sl_response = exchange_api
        .place_order(stop_loss_order, Grouping::Na, None, None, None)
        .await?;

    match sl_response.response {
        ResponseInner::Error(e) => {
            tracing::error!("Failed to place stop loss: {}", e);
        }
        ResponseInner::Ok(order_data) => {
            tracing::info!("Stop loss placed: {:?}", order_data);
        }
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Set take profit 10% above current price
    tracing::info!("\n=== Placing Take Profit Order ===");
    let take_profit_price = price_decimal.saturating_mul(Decimal::from_str("1.10")?); // 10% above
    let take_profit_trigger = take_profit_price.round_dp(5).to_string();

    let take_profit_order = OrderRequest {
        a: u32::try_from(asset_idx)?,
        b: false, // SELL when triggered
        p: take_profit_trigger.clone(),
        s: "10.0".to_string(),
        r: true,
        t: OrderType::Trigger(TriggerOrder {
            is_market: true,
            trigger_px: take_profit_trigger.clone(),
            tpsl: Tpsl::Tp,
        }),
        c: None,
    };

    tracing::info!(
        "Take profit will trigger at ${} (10% above current)",
        take_profit_trigger
    );

    let tp_response = exchange_api
        .place_order(take_profit_order, Grouping::Na, None, None, None)
        .await?;

    match tp_response.response {
        ResponseInner::Error(e) => {
            tracing::error!("Failed to place take profit: {}", e);
        }
        ResponseInner::Ok(order_data) => {
            tracing::info!("Take profit placed: {:?}", order_data);
        }
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    let cancel_time = current_time_millis().saturating_add(30_000);

    match exchange_api
        .schedule_cancel(Some(cancel_time), None, None)
        .await
    {
        Ok(response) => {
            tracing::debug!("{:?}", response);
            tracing::info!("Scheduled to cancel all orders at: {}", cancel_time);
            tracing::info!("Orders will auto-cancel in ~30 seconds");
        }
        Err(e) => {
            if e.to_string().contains("volume traded") {
                tracing::warn!("Schedule cancel requires $1M trading volume (testnet limitation)");
                tracing::info!("Skipping dead man's switch demo");
            } else {
                tracing::error!("Failed to schedule cancel: {}", e);
            }
        }
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    let wallet_address = pkey_signer.address();
    let open_orders = info_api
        .open_orders(&wallet_address.to_string(), None)
        .await?;

    tracing::info!("Open orders: {} orders", open_orders.len());
    for order in &open_orders {
        tracing::info!(
            "  Order: coin={}, side={}, size={}",
            order.coin,
            if order.side == "B" { "BUY" } else { "SELL" },
            order.sz
        );
    }

    tracing::info!("\nAll orders will be automatically canceled in ~30 seconds");
    tracing::info!("(Due to the scheduled dead man's switch)");

    Ok(())
}
