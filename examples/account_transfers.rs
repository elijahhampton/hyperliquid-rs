#![allow(clippy::all)]
use hyperliquid_rs::{
    example_helpers::load_signer,
    init_tracing::init_tracing,
    prelude::{response::ResponseInner, HyperliquidClientBuilder},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    let pkey_signer = load_signer();
    let wallet_address = pkey_signer.address();

    let mut builder = HyperliquidClientBuilder::new();
    let hyperliquid = builder.testnet().with_wallet(pkey_signer).build()?;

    let info_api = hyperliquid.info();
    let exchange_api = hyperliquid.exchange();

    // Check initial balances
    let initial_state = info_api
        .perpetuals_account_summary(&wallet_address.to_string(), None)
        .await?;

    tracing::info!(
        "Perp balance: {} USDC",
        initial_state.margin_summary.account_value
    );

    let spot_state = info_api.token_balances(&wallet_address.to_string()).await?;

    tracing::info!("Spot balances: {:?}", spot_state.balances);

    // Transfer 100 USDC from perp to spot
    tracing::info!("\n=== Transferring 100 USDC: Perp → Spot ===");
    let transfer_to_spot = exchange_api
        .transfer_to_or_from_spot_account_to_or_from_perp_account(
            "100".to_string(),
            false, // false = perp → spot
        )
        .await?;

    match transfer_to_spot.response {
        ResponseInner::Error(e) => {
            tracing::error!("Failed to transfer to spot: {}", e);
            return Ok(());
        }
        ResponseInner::Ok(_) => {
            tracing::info!("Successfully transferred 100 USDC to spot");
        }
    }

    // Wait some time for the transfer to settle
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Verify balances after first transfer
    let mid_state = info_api
        .perpetuals_account_summary(&wallet_address.to_string(), None)
        .await?;

    tracing::info!(
        "Perp balance: {} USDC",
        mid_state.margin_summary.account_value
    );

    let mid_spot_state = info_api.token_balances(&wallet_address.to_string()).await?;

    tracing::info!("Spot balances: {:?}", mid_spot_state.balances);

    // Transfer 100 USDC back from spot to perp
    tracing::info!("\n=== Transferring 100 USDC: Spot → Perp ===");
    let transfer_to_perp = exchange_api
        .transfer_to_or_from_spot_account_to_or_from_perp_account(
            "100".to_string(),
            true, // true = spot → perp
        )
        .await?;

    match transfer_to_perp.response {
        ResponseInner::Error(e) => {
            tracing::error!("Failed to transfer to perp: {}", e);
            return Ok(());
        }
        ResponseInner::Ok(_) => {
            tracing::info!("Successfully transferred 100 USDC back to perp");
        }
    }

    // Wait some time for the transfer to settle
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Verify final balances
    let final_state = info_api
        .perpetuals_account_summary(&wallet_address.to_string(), None)
        .await?;

    tracing::info!(
        "Perp balance: {} USDC",
        final_state.margin_summary.account_value
    );

    let final_spot_state = info_api.token_balances(&wallet_address.to_string()).await?;

    tracing::info!("Spot balances: {:?}", final_spot_state.balances);

    tracing::info!(
        "Initial perp: {} → Final perp: {}",
        initial_state.margin_summary.account_value,
        final_state.margin_summary.account_value
    );

    Ok(())
}
