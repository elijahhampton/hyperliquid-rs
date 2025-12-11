#![allow(clippy::all)]
use rhyperliquid::{example_helpers::user, init_tracing::init_tracing, HyperliquidClientBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    let hyperliquid = HyperliquidClientBuilder::new().testnet().build()?;
    let user = user();

    let spot_meta = hyperliquid.info().spot_metadata().await?;
    tracing::info!("{:?}", spot_meta);

    let spot_asset_ctxs = hyperliquid.info().spot_asset_context().await?;
    tracing::info!("{:?}", spot_asset_ctxs);

    let spot_clearinghouse_state = hyperliquid.info().token_balances(&user).await?;
    tracing::info!("{:?}", spot_clearinghouse_state);

    let spot_deploy_action_information = hyperliquid
        .info()
        .spot_deploy_auction_information(&user)
        .await?;
    tracing::info!("{:?}", spot_deploy_action_information);

    let spot_pair_deploy_auction_information = hyperliquid
        .info()
        .spot_pair_deploy_auction_information()
        .await?;
    tracing::info!("{:?}", spot_pair_deploy_auction_information);

    let token_information = hyperliquid
        .info()
        .token_information("0x00000000000000000000000000000000")
        .await?;
    tracing::info!("{:?}", token_information);

    Ok(())
}
