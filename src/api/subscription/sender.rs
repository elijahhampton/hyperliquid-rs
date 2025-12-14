use tokio::sync::broadcast::Sender;
use crate::types::ws::{
    WsActiveAssetData,
    WsAllMids,
    WsAssetCtx,
    WsBbo,
    WsBook,
    WsCandle,
    WsClearinghouseState,
    WsNotification,
    WsOpenOrders,
    WsTrade,
    WsTwapStates,
    WsUserEvent,
    WsUserFills,
    WsUserFunding,
    WsUserNonFundingLedgerUpdate,
    WsUserTwapHistory,
    WsUserTwapSliceFills,
    WsWebData3
};

#[derive(Clone, Default)]
pub struct StreamSenders {
    pub(crate) all_mids: Option<Sender<WsAllMids>>,
    pub(crate) candle: Option<Sender<WsCandle>>,
    pub(crate) trades: Option<Sender<WsTrade>>,
    pub(crate) l2book: Option<Sender<WsBook>>,
    pub(crate) notifications: Option<Sender<WsNotification>>,
    pub(crate) webdata3: Option<Sender<WsWebData3>>,
    pub(crate) twap_states: Option<Sender<WsTwapStates>>,
    pub(crate) clearinghouse_state: Option<Sender<WsClearinghouseState>>,
    pub(crate) open_orders: Option<Sender<WsOpenOrders>>,
    pub(crate) user_events: Option<Sender<WsUserEvent>>,
    pub(crate) user_fills: Option<Sender<WsUserFills>>,
    pub(crate) user_funding: Option<Sender<WsUserFunding>>,
    pub(crate) user_non_funding_ledger_updates: Option<Sender<WsUserNonFundingLedgerUpdate>>,
    pub(crate) active_asset_ctx: Option<Sender<WsAssetCtx>>,
    pub(crate) active_asset_data: Option<Sender<WsActiveAssetData>>,
    pub(crate) user_twap_slice_fills: Option<Sender<WsUserTwapSliceFills>>,
    pub(crate) user_twap_history: Option<Sender<WsUserTwapHistory>>,
    pub(crate) bbo: Option<Sender<WsBbo>>
}

impl StreamSenders {
    pub fn new() -> Self {
        Self {
            all_mids: None,
            candle: None,
            trades: None,
            l2book: None,
            notifications: None,
            webdata3: None,
            twap_states: None,
            clearinghouse_state: None,
            open_orders: None,
            user_events: None,
            user_fills: None,
            user_funding: None,
            user_non_funding_ledger_updates: None,
            active_asset_ctx: None,
            active_asset_data: None,
            user_twap_slice_fills: None,
            user_twap_history: None,
            bbo: None
        }
    }
}
