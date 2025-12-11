#![allow(clippy::all)]
use rhyperliquid::{
    example_helpers::{builder, user},
    init_tracing::init_tracing,
    utils::current_time_millis,
    HyperliquidClientBuilder,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    let hyperliquid = HyperliquidClientBuilder::new().testnet().build()?;

    let user = user();
    let builder = builder();

    let rate_limits = hyperliquid.info().rate_limits(&user).await?;
    tracing::info!("Rate Limits: {:?}", rate_limits);

    let subaccounts = hyperliquid.info().subaccounts(&user).await?;
    tracing::info!("{:?}", subaccounts);

    let referral_information = hyperliquid.info().referral_information(&user).await?;
    tracing::info!("{:?}", referral_information);

    let fee_approval = hyperliquid
        .info()
        .check_builder_fee_approval(&user, &builder)
        .await?;
    tracing::info!("{:?}", fee_approval);

    let user_role = hyperliquid.info().role(&user).await?;
    tracing::info!("{:?}", user_role);

    let user_portfolio = hyperliquid.info().portfolio(&user).await?;
    tracing::info!("{:?}", user_portfolio);

    let historical_orders = hyperliquid.info().historical_orders(&user).await?;
    tracing::info!("{:?}", historical_orders);

    let open_orders = hyperliquid.info().open_orders(&user, None).await?;
    tracing::info!("{:?}", open_orders);

    let open_orders_with_additional_frontend_info = hyperliquid
        .info()
        .open_orders_with_additional_info(&user, None)
        .await?;
    tracing::info!("{:?}", open_orders_with_additional_frontend_info);

    let fees = hyperliquid.info().fees(&user).await?;
    tracing::info!("{:?}", fees);

    let fills = hyperliquid.info().fills(&user, Some(true)).await?;
    tracing::info!("Fills: {:?}", fills);

    let start_time = current_time_millis();

    let fills_by_time = hyperliquid
        .info()
        .fills_by_time(&user, start_time, None, None)
        .await?;
    tracing::info!("Fills by time: {:?}", fills_by_time);

    let twap_slice_fills = hyperliquid.info().twap_slice_fills(&user).await?;
    tracing::info!("{:?}", twap_slice_fills);

    let vault_deposits = hyperliquid.info().vault_deposits(&user).await?;
    tracing::info!("{:?}", vault_deposits);

    let state = hyperliquid.info().hip3_dex_abstraction_state(&user).await?;
    tracing::info!("{:?}", state);

    let history = hyperliquid.info().staking_history(&user).await?;

    tracing::info!("{:?}", history);

    let rewards = hyperliquid.info().staking_rewards(&user).await?;

    tracing::info!("{:?}", rewards);

    let delegations = hyperliquid.info().staking_delegations(&user).await?;
    tracing::info!("{:?}", delegations);

    let summary = hyperliquid.info().staking_summary(&user).await?;

    tracing::info!("{:?}", summary);

    Ok(())
}
