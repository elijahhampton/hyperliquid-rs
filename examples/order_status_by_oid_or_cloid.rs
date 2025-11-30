use hyperliquid_rs::{
    api::current_time_millis,
    client::{HyperliquidClient, HyperliquidClientBuilder},
    init_tracing::init_tracing,
    types::info::user::OrderId,
};
use tracing;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    let hyperliquid = HyperliquidClientBuilder::new().testnet().build()?;
    let user = user();
    let oid = OrderId::Numeric(0);

    let order_status = hyperliquid.info().order_status(user, oid).await?;
    tracing::info!("{:?}", order_status);

    Ok(())
}
