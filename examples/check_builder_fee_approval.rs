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
    let builder = "0xeCe9Aa540e6dA813aF0356342E45167A67F54962";

    let fee_approval = hyperliquid
        .info()
        .check_builder_fee_approval(user, builder)
        .await?;
    tracing::info!("{:?}", fee_approval);

    Ok(())
}
