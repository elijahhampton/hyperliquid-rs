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
    let user = "0x06B825B202450B598E962AA3F0816e8d336337d9";

    let fills = hyperliquid.info().fills(user, Some(true)).await?;
    tracing::info!("Fills: {:?}", fills);

    let start_time = current_time_millis();
    let fills_by_time = hyperliquid
        .info()
        .fills_by_time(user, start_time, None, None)
        .await?;
    tracing::info!("Fills by time: {:?}", fills_by_time);

    Ok(())
}
