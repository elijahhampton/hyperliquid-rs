use hyperliquid_rs::{
    api::current_time_millis,
    client::{HyperliquidClient, HyperliquidClientBuilder},
    init_tracing::init_tracing,
    types::info::user::CandleSnapshotRequest,
};
use tracing;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    let hyperliquid = HyperliquidClientBuilder::new().testnet().build()?;
    let start_time = current_time_millis();
    let end_time = start_time + (30 * 60);
    let candle_snapshot_req = CandleSnapshotRequest {
        coin: "BTC".to_string(),
        interval: "15m".to_string(),
        start_time,
        end_time,
    };

    let candle_snapshot = hyperliquid
        .info()
        .candle_snapshot(candle_snapshot_req)
        .await?;
    tracing::info!("{:?}", candle_snapshot);

    Ok(())
}
