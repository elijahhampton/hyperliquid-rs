use crate::types::ws::{
    WsActiveAssetData, WsAllMids, WsAssetCtx, WsBbo, WsBook, WsCandle, WsClearinghouseState,
    WsNotification, WsOpenOrders, WsTrade, WsTwapStates, WsUserEvent, WsUserFills, WsUserFunding,
    WsUserNonFundingLedgerUpdate, WsUserTwapHistory, WsUserTwapSliceFills, WsWebData3,
};
use tokio::sync::broadcast::Sender;

/// A mapping of subscription identifiers to sender channels and subscription
/// parameters.
#[derive(Clone, Default)]
pub struct StreamSenders {
    pub(crate) all_mids: (Option<Sender<WsAllMids>>, Option<String>),
    pub(crate) candle: (Option<Sender<WsCandle>>, Option<String>, Option<String>),
    pub(crate) trades: (
        Option<Sender<WsTrade>>,
        Option<String>,
        Option<u32>,
        Option<u32>,
    ),
    pub(crate) l2book: (Option<Sender<WsBook>>, Option<String>, Option<u32>, Option<u32>),
    pub(crate) notifications: (Option<Sender<WsNotification>>, Option<String>),
    pub(crate) webdata3: (Option<Sender<WsWebData3>>, Option<String>),
    pub(crate) twap_states: (Option<Sender<WsTwapStates>>, Option<String>),
    pub(crate) clearinghouse_state: (Option<Sender<WsClearinghouseState>>, Option<String>),
    pub(crate) open_orders: (Option<Sender<WsOpenOrders>>, Option<String>),
    pub(crate) user_events: (Option<Sender<WsUserEvent>>, Option<String>),
    pub(crate) user_fills: (Option<Sender<WsUserFills>>, Option<String>),
    pub(crate) user_funding: (Option<Sender<WsUserFunding>>, Option<String>),
    pub(crate) user_non_funding_ledger_updates:
        (Option<Sender<WsUserNonFundingLedgerUpdate>>, Option<String>),
    pub(crate) active_asset_ctx: (Option<Sender<WsAssetCtx>>, Option<String>),
    pub(crate) active_asset_data: (
        Option<Sender<WsActiveAssetData>>,
        Option<String>,
        Option<String>,
    ),
    pub(crate) user_twap_slice_fills: (Option<Sender<WsUserTwapSliceFills>>, Option<String>),
    pub(crate) user_twap_history: (Option<Sender<WsUserTwapHistory>>, Option<String>),
    pub(crate) bbo: (Option<Sender<WsBbo>>, Option<String>),
}

impl StreamSenders {
    pub fn new() -> Self {
        Self {
            all_mids: (None, None),
            candle: (None, None, None),
            trades: (None, None, None, None),
            l2book: (None, None, None, None),
            notifications: (None, None),
            webdata3: (None, None),
            twap_states: (None, None),
            clearinghouse_state: (None, None),
            open_orders: (None, None),
            user_events: (None, None),
            user_fills: (None, None),
            user_funding: (None, None),
            user_non_funding_ledger_updates: (None, None),
            active_asset_ctx: (None, None),
            active_asset_data: (None, None, None),
            user_twap_slice_fills: (None, None),
            user_twap_history: (None, None),
            bbo: (None, None),
        }
    }
}
