use hyperliquid_rs::{
    api::current_time_millis,
    client::{HyperliquidClient, HyperliquidClientBuilder},
    helpers::{testnet_client, user, vault_address},
    init_tracing::init_tracing,
};
use tracing;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    let hyperliquid = testnet_client();
    let user = user();

    let summary = hyperliquid
        .info()
        .staking_summary(&user)
        .await?;

    tracing::info!("{:?}", summary);

    Ok(())
}
