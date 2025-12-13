use tokio::sync::broadcast::Sender;

use crate::types::ws::{WsAllMids, WsClearinghouseState, WsNotification, WsTwapStates, WsWebData3};

#[derive(Clone, Default)]
pub struct StreamSenders {
    pub(crate) all_mids: Option<Sender<WsAllMids>>,
    pub(crate) notifications: Option<Sender<WsNotification>>,
    pub(crate) webdata3: Option<Sender<WsWebData3>>,
    pub(crate) twap_states: Option<Sender<WsTwapStates>>,
    pub(crate) clearinghouse_state: Option<Sender<WsClearinghouseState>>
}

impl StreamSenders {
    pub fn new() -> Self {
        Self {
            all_mids: None,
            notifications: None,
            webdata3: None,
            twap_states: None,
            clearinghouse_state: None
        }
    }
}
