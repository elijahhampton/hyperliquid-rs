use hyperliquid_rs::{
    api::current_time_millis,
    client::{HyperliquidClient, HyperliquidClientBuilder},
    init_tracing::init_tracing,
};
use tracing;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    let hyperliquid = HyperliquidClientBuilder::new().testnet().build()?;

    let l2_book_snapshot = hyperliquid
        .info()
        .l2_book_snapshot("BTC", None, None)
        .await?;
    tracing::info!("{:?}", l2_book_snapshot);

    Ok(())
}
