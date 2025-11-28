use hyperliquid_rs::{HyperliquidClient, types::CandleSnapshotRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HyperliquidClient::new("https://api.hyperliquid.xyz".to_string())?;
    let req = CandleSnapshotRequest {
        coin: "BTC".to_string(),
        interval: "15m".to_string(),
        end_time: 1681924499999,
        start_time: 1681923600000,
    };

    let l2book = client.get_candles(req).await?;

    println!("Retrieved L2Book!");
    println!("{:?}", l2book);

    Ok(())
}
