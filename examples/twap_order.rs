#![allow(clippy::all)]
#![allow(clippy::too_many_lines)]
use hyperliquid_rs::{
    example_helpers::load_signer,
    init_tracing::init_tracing,
    prelude::{
        exchange::TwapRequest, response::ResponseInner, HyperliquidClientBuilder, HyperliquidError,
    },
};
use rust_decimal::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    let pkey_signer = load_signer();
    let wallet_address = pkey_signer.address();

    let mut builder = HyperliquidClientBuilder::new();
    let hyperliquid = builder.testnet().with_wallet(pkey_signer).build()?;

    let info_api = hyperliquid.info();
    let exchange_api = hyperliquid.exchange();

    let perp_meta = info_api.perpetuals_metadata(None).await?;
    let doge_asset_and_idx = perp_meta
        .universe
        .iter()
        .enumerate()
        .find(|asset| asset.1.name == "DOGE");

    let (asset_idx, asset_info) = doge_asset_and_idx.ok_or(HyperliquidError::Internal(
        "Missing asset in universe".to_string(),
    ))?;
    let asset_id = format!("@{}", asset_idx);

    // Get current price to calculate size
    let all_mids = info_api.all_mids(None).await?;
    let default_decimal = Decimal::new(0, 0);
    let doge_price = all_mids.get(&asset_id).unwrap_or(&default_decimal);
    let price_decimal = Decimal::from_str(&doge_price.to_string())?;

    // Calculate size: $50 notional / price
    let notional = Decimal::from(50);
    let total_size = notional
        .checked_div(price_decimal)
        .ok_or(HyperliquidError::InvalidPrice(
            "Price cannot be zero".to_string(),
        ))?;
    let sz_decimals = u32::from(asset_info.sz_decimals);
    let size_rounded = total_size.round_dp(sz_decimals);

    tracing::info!("\n=== TWAP Order Configuration ===");
    tracing::info!("Asset: DOGE (index {})", asset_idx);
    tracing::info!("Current price: ${}", doge_price);
    tracing::info!("Total size: {} DOGE", size_rounded);
    tracing::info!("Notional value: ${}", notional);
    tracing::info!("Duration: 3 minutes");
    tracing::info!("Execution: Randomized intervals");

    // Place TWAP order
    let twap_request = TwapRequest {
        a: u32::try_from(asset_idx)?,
        b: true, // buy
        s: size_rounded.to_string(),
        r: false, // not reduce-only
        m: 5,     // 3 minutes
        t: true,  // randomize execution
    };

    let twap_response = exchange_api
        .place_twap_order(twap_request, None, None)
        .await?;

    let twap_id = match twap_response.response {
        ResponseInner::Error(e) => {
            tracing::error!("Failed to place TWAP order: {}", e);
            return Ok(());
        }
        ResponseInner::Ok(data) => match data.data.status.twap_id() {
            Some(id) => {
                tracing::info!("TWAP order placed successfully!");
                tracing::info!("TWAP ID: {}", id);
                id
            }
            None => {
                tracing::error!("TWAP order failed: {:?}", data.data.status.error_message());
                return Ok(());
            }
        },
    };

    tracing::info!("\n=== Monitoring TWAP Execution (60 seconds) ===");
    tracing::info!("TWAP will execute small slices over the next 3 minutes");
    tracing::info!("Watching for fills...\n");

    let start_time = std::time::Instant::now();
    let mut last_fill_count = 0;

    while start_time.elapsed().as_secs() < 60 {
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        // Check for slice fills
        let slice_fills = info_api
            .twap_slice_fills(&wallet_address.to_string())
            .await?;

        // Filter fills for our TWAP ID
        let our_fills: Vec<_> = slice_fills
            .iter()
            .filter(|twap| twap.twap_id == twap_id)
            .collect();

        let fills_len = our_fills.len();

        if fills_len > last_fill_count {
            let new_fills = fills_len.saturating_sub(last_fill_count);
            tracing::info!(
                "New slice executed! Total slices: {} (+{})",
                our_fills.len(),
                new_fills
            );

            if let Some(latest) = our_fills.last() {
                tracing::info!(
                    "   Size: {} @ ${}, Time: {}",
                    latest.fill.sz,
                    latest.fill.px,
                    latest.fill.time
                );
            }

            last_fill_count = our_fills.len();
        }
    }

    // Cancel the remaining TWAP
    let cancel_response = exchange_api
        .cancel_twap_order(asset_idx, twap_id, None, None)
        .await?;

    match cancel_response.response {
        ResponseInner::Error(e) => {
            tracing::warn!("TWAP cancel returned error: {}", e);
        }
        ResponseInner::Ok(data) => {
            if data.data.status.is_success() {
                tracing::info!("TWAP order canceled successfully");
            } else if let Some(err) = data.data.status.error_message() {
                tracing::warn!("TWAP cancel status: {}", err);
            }
        }
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    let final_fills = info_api
        .twap_slice_fills(&wallet_address.to_string())
        .await?;
    let our_final_fills: Vec<_> = final_fills
        .iter()
        .filter(|twap| twap.twap_id == twap_id)
        .collect();

    tracing::info!("Total slices executed: {}", our_final_fills.len());

    if !our_final_fills.is_empty() {
        let total_executed: Decimal = our_final_fills
            .iter()
            .map(|f| Decimal::from_str(&f.fill.sz.to_string()).unwrap_or_default())
            .sum();

        tracing::info!("Total size filled: {} DOGE", total_executed);
        tracing::info!(
            "Remaining: {} DOGE",
            size_rounded.saturating_sub(total_executed)
        );
    }

    Ok(())
}
