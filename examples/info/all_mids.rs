use hyperliquid_rs::{
    client::{HyperliquidClient, HyperliquidClientBuilder},
    init_tracing::init_tracing,
};
use tracing;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    let hyperliquid = HyperliquidClientBuilder::new().testnet().build()?;

    let all_mids = hyperliquid.info().all_mids(None).await?;
    tracing::info!("{:?}", all_mids);

    Ok(())
}
