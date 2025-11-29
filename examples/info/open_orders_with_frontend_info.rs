use hyperliquid_rs::{
    client::{HyperliquidClient, HyperliquidClientBuilder},
    init_tracing::init_tracing,
};
use tracing;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    let hyperliquid = HyperliquidClientBuilder::new().testnet().build()?;
    let user = "0x06B825B202450B598E962AA3F0816e8d336337d9";

    let open_orders = hyperliquid
        .info()
        .open_orders_with_additional_info(user, None)
        .await?;
    tracing::info!("{:?}", open_orders);

    Ok(())
}
