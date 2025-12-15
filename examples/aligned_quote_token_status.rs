#![allow(clippy::all)]
use rhyperliquid::{
    example_helpers::{testnet_client, user},
    init_tracing::init_tracing,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    let hyperliquid = testnet_client()?;
    let user = user();

    let status = hyperliquid.info().aligned_quote_token_status(&user).await?;

    tracing::info!("{:?}", status);

    Ok(())
}
