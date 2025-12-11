#![allow(clippy::all)]
#[allow(unused_imports)]
use rhyperliquid::{
    example_helpers::load_signer,
    init_tracing::init_tracing,
    types::exchange::{CancelRequest, Grouping, OrderRequest, OrderType, Tif},
    utils::current_time_millis,
    HyperliquidClientBuilder,
};
use rhyperliquid::{response::ResponseInner, types::exchange::LimitOrder, HyperliquidError};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    let pkey_signer = load_signer();

    // Create a hyperliquid client
    let mut builder = HyperliquidClientBuilder::new();
    let hyperliquid = builder.testnet().with_wallet(pkey_signer.clone()).build()?;

    // Extract the info and exchange APIs
    let info_api = hyperliquid.info();
    let exchange_api = hyperliquid.exchange();

    // Find the asset id of the asset we want to place the order for i.e., the DOGE coin.
    let perp_meta = info_api.perpetuals_metadata(None).await?;
    let doge_asset_and_idx = perp_meta
        .universe
        .iter()
        .enumerate()
        .find(|asset| asset.1.name == "DOGE");

    let (idx, _) = doge_asset_and_idx.ok_or(HyperliquidError::Internal(
        "Missing asset in universe".to_string(),
    ))?;

    let asset_id = format!("@{}", idx);
    let all_mids = info_api.all_mids(None).await?;
    let doge_price = all_mids.get(&asset_id).ok_or(HyperliquidError::Internal(
        "Missing asset in universe".to_string(),
    ))?;

    let limit_order_type = LimitOrder { tif: Tif::Gtc };

    if let Some((asset_idx, _)) = doge_asset_and_idx {
        let order_req = OrderRequest {
            a: u32::try_from(asset_idx)?,
            b: true,
            p: doge_price.to_string(),
            s: "100.0".to_string(),
            r: false,
            t: OrderType::Limit(limit_order_type),
            c: None,
        };

        tracing::info!("Attempting to place order.");

        let order_response = exchange_api
            .place_order(order_req, Grouping::Na, None, None, None)
            .await?;

        tracing::info!("{:?}", order_response);

        match order_response.response {
            ResponseInner::Error(e) => {
                tracing::error!("Failed to place order {:?}", e);
            }
            ResponseInner::Ok(order_response_inner_ok) => {
                let statuses = order_response_inner_ok.data.statuses;

                tracing::info!("Attempting to cancel each order.");
                for order_status in statuses {
                    if let Some(oid) = order_status.oid() {
                        tracing::info!("Canceling order with oid: {}", oid);
                        let cancel = CancelRequest {
                            a: u32::try_from(asset_idx)?,
                            o: oid,
                        };

                        match exchange_api
                            .cancel_order(cancel, None, None)
                            .await?
                            .response
                        {
                            ResponseInner::Error(e) => {
                                tracing::error!("{:?}", e);
                            }
                            ResponseInner::Ok(o) => {
                                tracing::info!("Successfully canceled order.");
                                tracing::info!("{:?}", o);
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
