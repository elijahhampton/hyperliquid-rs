#[allow(unused_imports)]
use rust_decimal::prelude::*;

use hyperliquid_rs::{
    example_helpers::load_signer,
    init_tracing::init_tracing,
    prelude::{
        exchange::{Grouping, OrderRequest},
        response::ResponseInner,
        HyperliquidClientBuilder,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    let pkey_signer = load_signer();

    let mut builder = HyperliquidClientBuilder::new();
    let hyperliquid = builder.testnet().with_wallet(pkey_signer.clone()).build()?;

    let info_api = hyperliquid.info();
    let exchange_api = hyperliquid.exchange();

    // Get current price from meta and asset contexts
    let (perp_meta, asset_ctxs) = info_api.perpetuals_asset_contexts().await?;
    let btc_asset_and_idx = perp_meta
        .universe
        .iter()
        .enumerate()
        .find(|asset| asset.1.name == "BTC");

    let (asset_idx, asset_info) = btc_asset_and_idx.unwrap();

    let btc_ctx = &asset_ctxs[asset_idx];
    let mark_price = Decimal::from_str(&btc_ctx.mark_px)?;

    tracing::info!("BTC mark price: ${}", mark_price);

    let multiplier = Decimal::from_str("1.01")?;
    let order_price_decimal = mark_price * multiplier;

    let min_notional = Decimal::from(10);
    let min_size = min_notional / order_price_decimal;

    // Round up to sz_decimals precision
    let sz_decimals = asset_info.sz_decimals as u32;
    let size = min_size.round_dp_with_strategy(sz_decimals, RoundingStrategy::AwayFromZero);

    tracing::info!("Calculated size: {}", size);
    tracing::info!("Notional value: {}", size * order_price_decimal);

    tracing::info!("Setting leverage to 5x isolated");
    let leverage_response = exchange_api
        .update_leverage(asset_idx as u32, false, 5, None, None)
        .await?;

    match leverage_response.response {
        ResponseInner::Error(e) => {
            tracing::error!("Failed to update leverage: {:?}", e);
            return Ok(());
        }
        ResponseInner::Ok(_) => {
            tracing::info!("Successfully set leverage to 5x isolated");
        }
    }

    // Place market order using the builder
    tracing::info!("Placing market buy order");
    let order_req = OrderRequest::new_market_order(
    asset_idx as u32,
    asset_info,
    true,
    order_price_decimal,
    size
);

    let order_response = exchange_api
        .place_order(order_req, Grouping::Na, None, None, None)
        .await?;

    match order_response.response {
        ResponseInner::Error(e) => {
            tracing::error!("Failed to place order: {:?}", e);
            return Ok(());
        }
        ResponseInner::Ok(order_data) => {
            tracing::info!("Order placed: {:?}", order_data);
        }
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    tracing::info!("Adding 5 USDC to isolated margin");
    let add_margin_response = exchange_api
        .update_isolated_margin(asset_idx as u32, true, 5_000_000, None, None)
        .await?;

    match add_margin_response.response {
        ResponseInner::Error(e) => {
            tracing::error!("Failed to add margin: {:?}", e);
        }
        ResponseInner::Ok(_) => {
            tracing::info!("Successfully added 5 USDC margin");
        }
    }

    let wallet_address = pkey_signer.address();
    let user_state = info_api
        .perpetuals_account_summary(&wallet_address.to_string(), None)
        .await?;

    tracing::info!("Final account state:");
    tracing::info!("Margin summary: {:?}", user_state.margin_summary);
    tracing::info!("Asset positions: {:?}", user_state.asset_positions);

    Ok(())
}
