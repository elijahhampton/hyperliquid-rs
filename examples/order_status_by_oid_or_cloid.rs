use hyperliquid_rs::{
    client::HyperliquidClientBuilder, example_helpers::user, init_tracing::init_tracing,
    types::info::user::OrderId,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    let hyperliquid = HyperliquidClientBuilder::new().testnet().build()?;
    let user = user();
    let oid = OrderId::Numeric(0);

    let order_status = hyperliquid.info().order_status(&user, oid).await?;
    tracing::info!("{:?}", order_status);

    Ok(())
}
