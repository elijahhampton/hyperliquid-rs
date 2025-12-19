use rhyperliquid::{
    example_helpers::{testnet_client, user},
    init_tracing::init_tracing,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    let client = testnet_client()?;
    let mut subs = client.subscriptions().await?;
    let user = user();

    subs.subscribe_all_mids(None).await?;
    subs.subscribe_candle("BTC", "5m".to_string()).await?;
    subs.subscribe_l2_book("BTC", None, None).await?;
    subs.subscribe_trades("BTC").await?;
    subs.subscribe_notifications(user.clone()).await?;
    subs.subscribe_webdata3(user.clone()).await?;
    subs.subscribe_twap_states(user.clone()).await?;
    subs.subscribe_clearinghouse_state(user.clone()).await?;
    subs.subscribe_open_orders(user.clone()).await?;
    subs.subscribe_user_events(user.clone()).await?;
    subs.subscribe_user_fills(user.clone()).await?;
    subs.subscribe_user_funding(user.clone()).await?;
    subs.subscribe_user_non_funding_ledger_updates(user.clone()).await?;
    subs.subscribe_active_asset_ctx("BTC").await?;
    subs.subscribe_active_asset_data(user.clone(), "BTC").await?;
    subs.subscribe_user_twap_slice_fills(user.clone()).await?;
    subs.subscribe_user_twap_history(user.clone()).await?;
    subs.subscribe_bbo("BTC").await?;

    // Match and receive subscription messages
    while let Ok(msg) = subs.events.recv().await {
        match msg {
            rhyperliquid::types::ws::SubscriptionResponse::Error(e)=>{tracing::info!("Error: {:?}",e);}
            rhyperliquid::types::ws::SubscriptionResponse::SubscriptionResponse(subscription_confirmation,)=>{tracing::info!("SubscriptionResponse: {:?}",subscription_confirmation);}
            rhyperliquid::types::ws::SubscriptionResponse::AllMids(ws_all_mids)=>{tracing::info!("AllMids: {:?}",ws_all_mids);}
            rhyperliquid::types::ws::SubscriptionResponse::Candle(ws_candle)=>{tracing::info!("Candle: {:?}",ws_candle);}
            rhyperliquid::types::ws::SubscriptionResponse::Trades(ws_trade)=>{tracing::info!("Trades: {:?}",ws_trade);}
            rhyperliquid::types::ws::SubscriptionResponse::L2Book(ws_book)=>{tracing::info!("L2Book: {:?}",ws_book);}
            rhyperliquid::types::ws::SubscriptionResponse::Notification(ws_notification)=>{tracing::info!("Notification: {:?}",ws_notification);}
            rhyperliquid::types::ws::SubscriptionResponse::WebData3(ws_web_data3)=>{tracing::info!("WebData3: {:?}",ws_web_data3);}
            rhyperliquid::types::ws::SubscriptionResponse::TwapStates(ws_twap_states)=>{tracing::info!("TwapStates: {:?}",ws_twap_states);}
            rhyperliquid::types::ws::SubscriptionResponse::OpenOrders(ws_open_orders)=>{tracing::info!("OpenOrders: {:?}",ws_open_orders);}
            rhyperliquid::types::ws::SubscriptionResponse::UserEvents(ws_user_event)=>{tracing::info!("UserEvents: {:?}",ws_user_event);}
            rhyperliquid::types::ws::SubscriptionResponse::UserNonFundingLedgerUpdates(ws_user_non_funding_ledger_update,)=>{tracing::info!("UserNonFundingLedgerUpdate: {:?}",ws_user_non_funding_ledger_update);}
            rhyperliquid::types::ws::SubscriptionResponse::ActiveAssetCtx(ws_asset_ctx)=>{tracing::info!("ActiveAssetCtx: {:?}",ws_asset_ctx);}
            rhyperliquid::types::ws::SubscriptionResponse::ActiveAssetData(ws_active_asset_data,)=>{tracing::info!("ActiveAssetData: {:?}",ws_active_asset_data);}
            rhyperliquid::types::ws::SubscriptionResponse::UserTwapSliceFills(ws_user_twap_slice_fills,)=>{tracing::info!("UserTwapSliceFills: {:?}",ws_user_twap_slice_fills);}
            rhyperliquid::types::ws::SubscriptionResponse::UserTwapHistory(ws_user_twap_history,)=>{tracing::info!("UserTwapHistory: {:?}",ws_user_twap_history);}
            rhyperliquid::types::ws::SubscriptionResponse::Bbo(ws_bbo)=>{tracing::info!("Bbo: {:?}",ws_bbo);}
            rhyperliquid::types::ws::SubscriptionResponse::Pong=>tracing::info!("Pong"),
            rhyperliquid::types::ws::SubscriptionResponse::ClearinghouseState(ws_clearinghouse_state,)=>{tracing::info!("ClearinghouseState: {:?}",ws_clearinghouse_state);}
            rhyperliquid::types::ws::SubscriptionResponse::UserFills(ws_user_fills)=>{tracing::info!("User Fills: {:?}",ws_user_fills);}
            rhyperliquid::types::ws::SubscriptionResponse::UserFundings(ws_user_fundings) => {tracing::info!("User Fundings: {:?}",ws_user_fundings);}
        }
    }

    Ok(())
}
