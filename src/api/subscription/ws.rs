use crate::api::subscription::sender::StreamSenders;
use crate::api::SUPPORTED_INTERVALS;
use crate::client::HyperliquidClient;
use crate::error::{HyperliquidError, Result, SubscriptionError};
use crate::types::ws::{
    SubscriptionConfirmation, WsActiveAssetData, WsAllMids, WsAssetCtx, WsBbo, WsBook, WsCandle,
    WsClearinghouseState, WsNotification, WsOpenOrders, WsTrade, WsTwapStates, WsUserEvent,
    WsUserFills, WsUserFunding, WsUserNonFundingLedgerUpdate, WsUserTwapHistory,
    WsUserTwapSliceFills, WsWebData3,
};
use crate::types::ws::{SubscriptionKey, SubscriptionResponse};
use futures::{stream::SplitStream, StreamExt};
use futures_util::stream::SplitSink;
use futures_util::SinkExt;
use serde::Serialize;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::broadcast::{channel, Receiver, Sender};
use tokio::sync::RwLock;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

/// Configurable parameters for subscriptions.
pub struct SubscriptionConfig {
    // Capacity for the underlying broadcast channel
    channel_capacity: usize,
}

impl Default for SubscriptionConfig {
    fn default() -> Self {
        Self {
            channel_capacity: 1000,
        }
    }
}

/// A WS client providing access to Hyperliquid Subscriptions API.
pub struct SubscriptionClient {
    write_stream: SplitSink<
        WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
        Message,
    >,
    streams: Arc<RwLock<StreamSenders>>,
}

impl SubscriptionClient {
    pub async fn new(client: &HyperliquidClient) -> Result<Self> {
        let endpoint = client.ws_endpoint().ok_or(HyperliquidError::MissingConfiguration { parameter: "ws_endpoint".to_string() })?;
        let (ws_stream, _response) = connect_async(endpoint).await?;
        let (write_stream, read_stream) = ws_stream.split();

        let senders = Arc::new(RwLock::new(StreamSenders::new()));

        let _ = Self::spawn_read_task(senders.clone(), read_stream).await;

        Ok(Self {
            write_stream,
            streams: senders,
        })
    }

    fn subscription_channel<T: Clone>(capacity: Option<usize>) -> (Sender<T>, Receiver<T>) {
        channel::<T>(capacity.unwrap_or(1000))
    }

    async fn send_and_flush(&mut self, confirmation: SubscriptionConfirmation) -> Result<()> {
        self.write_stream
            .send(Message::Text(serde_json::to_string(&confirmation)?.into()))
            .await?;
        self.write_stream.flush().await?;

        Ok(())
    }

    fn spawn_read_task(
        senders: Arc<RwLock<StreamSenders>>,
        mut read_stream: SplitStream<
            WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
        >,
    ) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            while let Some(Ok(Message::Text(text))) = read_stream.next().await {
                let value = match serde_json::from_str::<SubscriptionResponse>(&text) {
                    Ok(v) => v,
                    Err(e) => {
                        tracing::error!("json error: {:?}", e);
                        continue;
                    }
                };

                match value {
                    SubscriptionResponse::Error(err) => {
                        tracing::error!("{:?}", err);
                    }
                    SubscriptionResponse::AllMids(mids) => {
                        if let (Some(tx), Some(_)) = &senders.read().await.all_mids {
                            if let Err(e) = tx.send(mids) {
                                tracing::error!("{:?}", e);
                            }
                        }
                    }
                    _ => {}
                }
            }
        })
    }

    pub async fn unsubscribe(&mut self, key: &SubscriptionKey) -> Result<()> {
        let subscription = {
            let streams = self.streams.read().await;
            Self::build_unsubscribe_payload(&streams, key)?
        };

        let msg = SubscriptionConfirmation {
            method: "unsubscribe".into(),
            subscription: serde_json::Value::Object(subscription),
        };

        self.send_and_flush(msg).await?;

        {
            let mut streams = self.streams.write().await;
            Self::clear_subscription(&mut streams, key);
        }

        Ok(())
    }

    fn build_unsubscribe_payload(
        streams: &StreamSenders,
        key: &SubscriptionKey,
    ) -> Result<serde_json::Map<String, serde_json::Value>> {
        use serde_json::{Map, Value};

        let mut sub = Map::new();

        match key {
            SubscriptionKey::AllMids => {
                let (_, dex) = &streams.all_mids;
                Self::ensure_present(&streams.all_mids.0, key)?;
                sub.insert("type".into(), Value::String("all_mids".into()));
                if let Some(d) = dex {
                    sub.insert("dex".into(), Value::String(d.clone()));
                }
            }

            SubscriptionKey::Candle => {
                let (_, coin, interval) = &streams.candle;
                Self::ensure_present(&streams.candle.0, key)?;
                sub.insert("type".into(), Value::String("candle".into()));
                sub.insert("coin".into(), Value::String(Self::req(coin, key)?));
                sub.insert("interval".into(), Value::String(Self::req(interval, key)?));
            }

            SubscriptionKey::Trades => {
                let (_, coin, _, _) = &streams.trades;
                Self::ensure_present(&streams.trades.0, key)?;
                sub.insert("coin".into(), Value::String(Self::req(coin, key)?));
            }

            SubscriptionKey::L2Book => {
                let (_, coin, _, _) = &streams.l2book;
                Self::ensure_present(&streams.l2book.0, key)?;
                sub.insert("coin".into(), Value::String(Self::req(coin, key)?));
            }

            SubscriptionKey::Notification => {
                Self::user_only(&mut sub, &streams.notifications, key)?;
            }
            SubscriptionKey::WebData3 => Self::user_only(&mut sub, &streams.webdata3, key)?,
            SubscriptionKey::TwapStates => Self::user_only(&mut sub, &streams.twap_states, key)?,
            SubscriptionKey::OpenOrders => Self::user_only(&mut sub, &streams.open_orders, key)?,
            SubscriptionKey::UserEvents => Self::user_only(&mut sub, &streams.user_events, key)?,
            SubscriptionKey::UserNonFundingLedgerUpdate => {
                Self::user_only(&mut sub, &streams.user_non_funding_ledger_updates, key)?;
            }

            SubscriptionKey::ActiveAssetCtx => {
                let (_, coin) = &streams.active_asset_ctx;
                Self::ensure_present(&streams.active_asset_ctx.0, key)?;
                sub.insert("coin".into(), Value::String(Self::req(coin, key)?));
            }

            SubscriptionKey::ActiveAssetData => {
                let (_, user, coin) = &streams.active_asset_data;
                Self::ensure_present(&streams.active_asset_data.0, key)?;
                sub.insert("user".into(), Value::String(Self::req(user, key)?));
                sub.insert("coin".into(), Value::String(Self::req(coin, key)?));
            }

            SubscriptionKey::UserTwapSliceFills => {
                Self::user_only(&mut sub, &streams.user_twap_slice_fills, key)?;
            }

            SubscriptionKey::UserTwapHistory => {
                Self::user_only(&mut sub, &streams.user_twap_history, key)?;
            }

            SubscriptionKey::Bbo => Self::user_only(&mut sub, &streams.bbo, key)?,
        }

        Ok(sub)
    }

    fn ensure_present<T>(sender: Option<&Sender<T>>, key: &SubscriptionKey) -> Result<()> {
        if sender.is_none() {
            Err(Self::missing(key))
        } else {
            Ok(())
        }
    }

    fn req(v: Option<&String>, key: &SubscriptionKey) -> Result<String> {
        v.clone().ok_or_else(|| Self::missing(key))
    }

    fn user_only<T>(
        sub: &mut serde_json::Map<String, serde_json::Value>,
        entry: &(Option<Sender<T>>, Option<String>),
        key: &SubscriptionKey,
    ) -> Result<()> {
        Self::ensure_present(&entry.0, key)?;
        sub.insert(
            "user".into(),
            serde_json::Value::String(Self::req(&entry.1, key)?),
        );
        Ok(())
    }

    fn missing(key: &SubscriptionKey) -> HyperliquidError {
        HyperliquidError::SubscriptionError(SubscriptionError::MissingSubscription(key.clone()))
    }
    fn clear_subscription(streams: &mut StreamSenders, key: &SubscriptionKey) {
        match key {
            SubscriptionKey::AllMids => streams.all_mids = Default::default(),
            SubscriptionKey::Candle => streams.candle = Default::default(),
            SubscriptionKey::Trades => streams.trades = Default::default(),
            SubscriptionKey::L2Book => streams.l2book = Default::default(),
            SubscriptionKey::Notification => streams.notifications = Default::default(),
            SubscriptionKey::WebData3 => streams.webdata3 = Default::default(),
            SubscriptionKey::TwapStates => streams.twap_states = Default::default(),
            SubscriptionKey::OpenOrders => streams.open_orders = Default::default(),
            SubscriptionKey::UserEvents => streams.user_events = Default::default(),
            SubscriptionKey::UserNonFundingLedgerUpdate => {
                streams.user_non_funding_ledger_updates = Default::default();
            }
            SubscriptionKey::ActiveAssetCtx => streams.active_asset_ctx = Default::default(),
            SubscriptionKey::ActiveAssetData => streams.active_asset_data = Default::default(),
            SubscriptionKey::UserTwapSliceFills => {
                streams.user_twap_slice_fills = Default::default();
            }
            SubscriptionKey::UserTwapHistory => streams.user_twap_history = Default::default(),
            SubscriptionKey::Bbo => streams.bbo = Default::default(),
        }
    }

    /// Subscribes to the [`WsAllMids`] websocket feed.
    ///
    /// # Arguments
    /// * `dex` (optional) Represents the perp dex to source mids from. If not provided,
    /// then the first perp dex is used. Spot mids are only included with the first perp dex.
    ///
    /// # Returns
    /// A bounded `tokio::sync::broadcast::Sender`
    pub async fn subscribe_all_mids(
        &mut self,
        subscription_config: Option<SubscriptionConfig>,
        dex: Option<String>,
    ) -> Result<Receiver<WsAllMids>> {
        let subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription: json!({
                "type": "allMids",
            }),
        };

        if let Some(dex_param) = dex.clone() {
            if let Some(dex_obj) = subscription_message.clone().subscription.get_mut("dex") {
                if let Some(obj) = dex_obj.as_object_mut() {
                    let val = serde_json::Value::String(dex_param);

                    obj.insert("dex".to_string(), val);
                }
            }
        }

        self.send_and_flush(subscription_message).await?;

        let capacity = subscription_config.unwrap_or_default().channel_capacity;
        let (tx, rx) = Self::subscription_channel::<WsAllMids>(Some(capacity));

        {
            let write_lock = &mut self.streams.write().await.all_mids;

            if write_lock.0.is_some() {
                tracing::error!("Already subscribed to `AllMids`");
                return Err(HyperliquidError::SubscriptionError(
                    SubscriptionError::SubscriptionExist {
                        method: "allMids".to_string(),
                    },
                ));
            }

            write_lock.1 = dex;
            write_lock.0 = Some(tx);
        }

        Ok(rx)
    }

    /// Subscribes to the [`WsCandle`] websocket feed.
    ///
    /// # Arguments
    /// * `coin` The desired asset for returning candle data.
    /// * `interval` The interval at which the candle data is returned. Supported intervals include: "1m", "3m",
    /// "5m", "15m", "30m", "1h", "2h", "4h", "8h", "12h", "1d", "3d", "1w", "1M"
    ///
    /// # Returns
    /// A bounded `tokio::sync::broadcast::Sender`
    pub async fn subscribe_candle(
        &mut self,
        subscription_config: Option<SubscriptionConfig>,
        coin: impl Into<String> + Serialize + Clone,
        interval: String,
    ) -> Result<Receiver<WsCandle>> {
        if !SUPPORTED_INTERVALS.contains(interval.as_str()) {
            return Err(HyperliquidError::InvalidRequestParameter {
                method: "subscribe_candle".to_string(),
                parameter: "interval".to_string(),
                reason: format!("Supported intervals include: {:?}", SUPPORTED_INTERVALS),
            });
        }

        let subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription: json!({
                "type": "candle",
                "coin": coin,
                "interval": interval
            }),
        };

        self.send_and_flush(subscription_message).await?;

        let capacity = subscription_config.unwrap_or_default().channel_capacity;
        let (tx, rx) = Self::subscription_channel::<WsCandle>(Some(capacity));

        {
            let mut candle = self.streams.write().await.candle;

            if candle.0.is_some() {
                return Err(HyperliquidError::SubscriptionError(
                    SubscriptionError::SubscriptionExist {
                        method: "candle".to_string(),
                    },
                ));
            }

            candle.0 = Some(tx);
            candle.1 = Some(coin.clone().into());
            candle.2 = Some(interval.clone());
        }

        Ok(rx)
    }

    /// Subscribes to the [`WsBook`] websocket feed.
    ///
    /// # Arguments
    /// * `coin` The desired asset for returning l2book data.
    ///
    /// # Returns
    /// A bounded `tokio::sync::broadcast::Sender`
    pub async fn subscribe_l2_book(
        &mut self,
        subscription_config: Option<SubscriptionConfig>,
        coin: impl Into<String> + Serialize,
        n_sig_figs: Option<u32>,
        mantissa: Option<u32>,
    ) -> Result<Receiver<WsBook>> {
        let coin = coin.into();

        let mut subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription: json!({
                "type": "l2book",
                "coin": coin.clone(),
            }),
        };

        let subscription_data = subscription_message.subscription.as_object_mut().expect("subscription object to be valid");
        if let Some(sig_figs) = n_sig_figs {
            subscription_data.insert("nSigFigs".to_string(), serde_json::Value::Number(sig_figs.into()));
        }

        if let Some(mantissa) = mantissa {
             subscription_data.insert("mantissa".to_string(), serde_json::Value::Number(mantissa.into()));
        }

        self.send_and_flush(subscription_message).await?;

        let capacity = subscription_config.unwrap_or_default().channel_capacity;
        let (tx, rx) = Self::subscription_channel::<WsBook>(Some(capacity));

        {
            let stream_lock = &mut self.streams.write().await.l2book;
            if stream_lock.0.is_some() {
                tracing::error!("Already subscribed to `l2book`");
                return Err(HyperliquidError::SubscriptionError(
                    SubscriptionError::SubscriptionExist {
                        method: "l2book".to_string(),
                    },
                ));
            }

            stream_lock.0 = Some(tx);
            stream_lock.1 = Some(coin);
            stream_lock.2 = n_sig_figs;
            stream_lock.3 = mantissa;
        }

        Ok(rx)
    }

    /// Subscribes to the [`WsTrade`] websocket feed.
    ///
    /// # Arguments
    /// * `coin` The desired asset for returning trade data.
    ///
    /// # Returns
    /// A bounded `tokio::sync::broadcast::Sender`
    pub async fn subscribe_trades(
        &mut self,
        subscription_config: Option<SubscriptionConfig>,
        coin: impl Into<String> + Serialize + Clone,
    ) -> Result<Receiver<WsTrade>> {
        let subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription: json!({
                "type": "trades",
                "coin": coin,
            }),
        };

        self.send_and_flush(subscription_message).await?;

        let capacity = subscription_config.unwrap_or_default().channel_capacity;
        let (tx, rx) = Self::subscription_channel::<WsTrade>(Some(capacity));

        {
            let stream_lock = &mut self.streams.write().await.trades;
            if stream_lock.0.is_some() {
                tracing::error!("Already subscribed to `trades`");
                return Err(HyperliquidError::SubscriptionError(
                    SubscriptionError::SubscriptionExist {
                        method: "trades".to_string(),
                    },
                ));
            }

            stream_lock.0 = Some(tx);
            stream_lock.1 = Some(coin.clone().into());
        }

        Ok(rx)
    }

    /// Subscribes to the [`WsNotification`] websocket feed.
    ///
    /// # Arguments
    /// * `user`
    ///
    /// # Returns
    /// A bounded `tokio::sync::broadcast::Sender`
    pub async fn subscribe_notifications(
        &mut self,
        subscription_config: Option<SubscriptionConfig>,
        user: impl Into<String>,
    ) -> Result<Receiver<WsNotification>> {
        let subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription: json!({
                "type": "notification",
                "user": json!(user.into())
            }),
        };

        self.send_and_flush(subscription_message).await?;

        let capacity = subscription_config.unwrap_or_default().channel_capacity;
        let (tx, rx) = Self::subscription_channel::<WsNotification>(Some(capacity));

        {
            let stream_lock = &mut self.streams.write().await.notifications;
            if stream_lock.0.is_some() {
                tracing::error!("Already subscribed to `notifications`");
                return Err(HyperliquidError::SubscriptionError(
                    SubscriptionError::SubscriptionExist {
                        method: "notifications".to_string(),
                    },
                ));
            }

            stream_lock.0 = Some(tx);
        }

        Ok(rx)
    }

    /// Subscribes to the [`WsWebData3`] websocket feed.
    ///
    /// # Arguments
    /// * `user`
    ///
    /// # Returns
    /// A bounded `tokio::sync::broadcast::Sender`.
    pub async fn subscribe_webdata3(
        &mut self,
        subscription_config: Option<SubscriptionConfig>,
        user: impl Into<String> + Serialize + Clone,
    ) -> Result<Receiver<WsWebData3>> {
        let user = user.into();

        let subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription: json!({
                "type": "webdata3",
                "user": json!(user.clone())
            }),
        };

        self.send_and_flush(subscription_message).await?;

        let capacity = subscription_config.unwrap_or_default().channel_capacity;
        let (tx, rx) = Self::subscription_channel::<WsWebData3>(Some(capacity));

        {
            let (opt_tx, opt_user) = &mut self.streams.write().await.webdata3;
            if opt_tx.is_some() {
                tracing::error!("Already subscribed to `webdata3`");
                return Err(HyperliquidError::SubscriptionError(
                    SubscriptionError::SubscriptionExist {
                        method: "webData3".to_string(),
                    },
                ));
            }

            *opt_tx = Some(tx);
            *opt_user = Some(user);
        }

        Ok(rx)
    }

    /// Subscribes to the [`WsTwapStates`] websocket feed.
    ///
    /// # Arguments
    /// * `user`
    ///
    /// # Returns
    /// A bounded `tokio::sync::broadcast::Sender`.
    pub async fn subscribe_twap_states(
        &mut self,
        subscription_config: Option<SubscriptionConfig>,
        user: impl Into<String>,
    ) -> Result<Receiver<WsTwapStates>> {
        let user = user.into();

        let subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription: json!({
                "type": "twapStates",
                "user": json!(user.clone())
            }),
        };

        self.send_and_flush(subscription_message).await?;

        let capacity = subscription_config.unwrap_or_default().channel_capacity;
        let (tx, rx) = Self::subscription_channel::<WsTwapStates>(Some(capacity));

        {
            let (opt_tx, opt_user) = &mut self.streams.write().await.twap_states;
            if opt_tx.is_some() {
                tracing::error!("Already subscribed to `twapStates`");
                return Err(HyperliquidError::SubscriptionError(
                    SubscriptionError::SubscriptionExist {
                        method: "twapStates".to_string(),
                    },
                ));
            }

            *opt_tx = Some(tx);
            *opt_user = Some(user);
        }

        Ok(rx)
    }

    /// Subscribes to the [`WsClearinghouseState`] websocket feed.
    ///
    /// # Arguments
    /// * `user`
    ///
    /// # Returns
    /// A bounded `tokio::sync::broadcast::Sender`.
    pub async fn subscribe_clearinghouse_state(
        &mut self,
        subscription_config: Option<SubscriptionConfig>,
        user: impl Into<String>,
    ) -> Result<Receiver<WsClearinghouseState>> {
        if self.streams.read().await.clearinghouse_state.0.is_some() {
            tracing::error!("Already subscribed to `clearinghouseState");
            return Err(HyperliquidError::SubscriptionError(
                SubscriptionError::SubscriptionExist {
                    method: "clearinghouseState".to_string(),
                },
            ));
        }

        let user = user.into();

        let subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription: json!({
                "type": "clearinghouseState",
                "user": json!(user.clone())
            }),
        };

        self.send_and_flush(subscription_message).await?;

        let capacity = subscription_config.unwrap_or_default().channel_capacity;
        let (tx, rx) = Self::subscription_channel::<WsClearinghouseState>(Some(capacity));

        let write_lock = &mut self.streams.write().await.clearinghouse_state;
        write_lock.0 = Some(tx);
        write_lock.1 = Some(user);

        Ok(rx)
    }

    /// Subscribes to the [`WsOpenOrders`] websocket feed.
    ///
    /// # Arguments
    /// * `user`
    ///
    /// # Returns
    /// A bounded `tokio::sync::broadcast::Sender`.
    pub async fn subscribe_open_orders(
        &mut self,
        subscription_config: Option<SubscriptionConfig>,
        user: impl Into<String>,
    ) -> Result<Receiver<WsOpenOrders>> {
        if self.streams.read().await.open_orders.0.is_some() {
            tracing::error!("Already subscribed to `openOrders");
            return Err(HyperliquidError::SubscriptionError(
                SubscriptionError::SubscriptionExist {
                    method: "openOrders".to_string(),
                },
            ));
        }

        let user = user.into();

        let subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription: json!({
                "type": "openOrders",
                "user": json!(user.clone())
            }),
        };

        self.send_and_flush(subscription_message).await?;

        let capacity = subscription_config.unwrap_or_default().channel_capacity;
        let (tx, rx) = Self::subscription_channel::<WsOpenOrders>(Some(capacity));

        let write_lock = &mut self.streams.write().await.open_orders;
        write_lock.0 = Some(tx);
        write_lock.1 = Some(user);

        Ok(rx)
    }

    /// Subscribes to the [`WsUserEvent`] websocket feed.
    ///
    /// # Arguments
    /// * `user`
    ///
    /// # Returns
    /// A bounded `tokio::sync::broadcast::Sender`.
    pub async fn subscribe_user_events(
        &mut self,
        subscription_config: Option<SubscriptionConfig>,
        user: impl Into<String>,
    ) -> Result<Receiver<WsUserEvent>> {
        if self.streams.read().await.user_events.0.is_some() {
            tracing::error!("Already subscribed to `userEvents`");
            return Err(HyperliquidError::SubscriptionError(
                SubscriptionError::SubscriptionExist {
                    method: "userEvents".to_string(),
                },
            ));
        }

        let user = user.into();

        let subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription: json!({
                "type": "userEvents",
                "user": json!(user.clone())
            }),
        };

        self.send_and_flush(subscription_message).await?;

        let capacity = subscription_config.unwrap_or_default().channel_capacity;
        let (tx, rx) = Self::subscription_channel::<WsUserEvent>(Some(capacity));

        let write_lock = &mut self.streams.write().await.user_events;
        write_lock.0 = Some(tx);
        write_lock.1 = Some(user);

        Ok(rx)
    }

    /// Subscribes to the [`WsUserFills`] websocket feed.
    ///
    /// # Arguments
    /// * `user`
    ///
    /// # Returns
    /// A bounded `tokio::sync::broadcast::Sender`.
    pub async fn subscribe_user_fills(
        &mut self,
        subscription_config: Option<SubscriptionConfig>,
        user: impl Into<String>,
    ) -> Result<Receiver<WsUserFills>> {
        if self.streams.read().await.user_fills.0.is_some() {
            tracing::error!("Already subscribed to `userFills`");
            return Err(HyperliquidError::SubscriptionError(
                SubscriptionError::SubscriptionExist {
                    method: "userFills".to_string(),
                },
            ));
        }

        let user = user.into();

        let subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription: json!({
                "type": "userFills",
                "user": json!(user.clone())
            }),
        };

        self.send_and_flush(subscription_message).await?;

        let capacity = subscription_config.unwrap_or_default().channel_capacity;
        let (tx, rx) = Self::subscription_channel::<WsUserFills>(Some(capacity));

        let write_lock = &mut self.streams.write().await.user_fills;
        write_lock.0 = Some(tx);
        write_lock.1 = Some(user);

        Ok(rx)
    }

    /// Subscribes to the [`WsUserFunding`] websocket feed.
    ///
    /// # Arguments
    /// * `user`
    ///
    /// # Returns
    /// A bounded `tokio::sync::broadcast::Sender`.
    pub async fn subscribe_user_funding(
        &mut self,
        subscription_config: Option<SubscriptionConfig>,
        user: impl Into<String>,
    ) -> Result<Receiver<WsUserFunding>> {
        if self.streams.read().await.user_funding.0.is_some() {
            tracing::error!("Already subscribed to `userFunding`");
            return Err(HyperliquidError::SubscriptionError(
                SubscriptionError::SubscriptionExist {
                    method: "userFunding".to_string(),
                },
            ));
        }

        let user = user.into();

        let subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription: json!({
                "type": "userFunding",
                "user": json!(user.clone())
            }),
        };

        self.send_and_flush(subscription_message).await?;

        let capacity = subscription_config.unwrap_or_default().channel_capacity;
        let (tx, rx) = Self::subscription_channel::<WsUserFunding>(Some(capacity));

        let write_lock = &mut self.streams.write().await.user_funding;
        write_lock.0 = Some(tx);
        write_lock.1 = Some(user);

        Ok(rx)
    }

    /// Subscribes to the [`WsUserNonFundingLedgerUpdate`] websocket feed.
    ///
    /// # Arguments
    /// * `user`
    ///
    /// # Returns
    /// A bounded `tokio::sync::broadcast::Sender`.
    pub async fn subscribe_user_non_funding_ledger_updates(
        &mut self,
        subscription_config: Option<SubscriptionConfig>,
        user: impl Into<String>,
    ) -> Result<Receiver<WsUserNonFundingLedgerUpdate>> {
        if self
            .streams
            .read()
            .await
            .user_non_funding_ledger_updates
            .0
            .is_some()
        {
            tracing::error!("Already subscribed to `userNonFundingLedgerUpdates`");
            return Err(HyperliquidError::SubscriptionError(
                SubscriptionError::SubscriptionExist {
                    method: "userNonFundingLedgerUpdates".to_string(),
                },
            ));
        }

        let user = user.into();

        let subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription: json!({
                "type": "userNonFundingLedgerUpdates",
                "user": json!(user.clone())
            }),
        };

        self.send_and_flush(subscription_message).await?;

        let capacity = subscription_config.unwrap_or_default().channel_capacity;
        let (tx, rx) = Self::subscription_channel::<WsUserNonFundingLedgerUpdate>(Some(capacity));

        let write_lock = &mut self.streams.write().await.user_non_funding_ledger_updates;
        write_lock.0 = Some(tx);
        write_lock.1 = Some(user);

        Ok(rx)
    }

    /// Subscribes to the [`WsActiveAssetCtx`] websocket feed.
    ///
    /// # Arguments
    /// * `coin` The symbol of the coin, i.e. the asset for receiving asset context
    ///
    /// # Returns
    /// A bounded `tokio::sync::broadcast::Sender`.
    pub async fn subscribe_active_asset_ctx(
        &mut self,
        subscription_config: Option<SubscriptionConfig>,
        coin: impl Into<String> + Clone,
    ) -> Result<Receiver<WsAssetCtx>> {
        if self.streams.read().await.active_asset_ctx.0.is_some() {
            tracing::error!("Already subscribed to `activeAssetCtx`");
            return Err(HyperliquidError::SubscriptionError(
                SubscriptionError::SubscriptionExist {
                    method: "activeAssetCtx".to_string(),
                },
            ));
        }

        let coin = coin.into();

        let subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription: json!({
                "type": "activeAssetCtx",
                "coin": json!(coin.clone())
            }),
        };

        self.send_and_flush(subscription_message).await?;

        let capacity = subscription_config.unwrap_or_default().channel_capacity;
        let (tx, rx) = Self::subscription_channel::<WsAssetCtx>(Some(capacity));

        let write_lock = &mut self.streams.write().await.active_asset_ctx;
        write_lock.0 = Some(tx);
        write_lock.1 = Some(coin);

        Ok(rx)
    }

    /// Subscribes to the [`WsActiveAssetData`] websocket feed.
    ///
    /// # Arguments
    /// * `coin` The symbol of the coin, i.e. the asset for receiving asset context
    /// * `user`
    ///
    /// # Returns
    /// A bounded `tokio::sync::broadcast::Sender`.
    pub async fn subscribe_active_asset_data(
        &mut self,
        subscription_config: Option<SubscriptionConfig>,
        user: impl Into<String>,
        coin: impl Into<String>,
    ) -> Result<Receiver<WsActiveAssetData>> {
        if self.streams.read().await.active_asset_data.0.is_some() {
            tracing::error!("Already subscribed to `activeAssetData`");
            return Err(HyperliquidError::SubscriptionError(
                SubscriptionError::SubscriptionExist {
                    method: "activeAssetData".to_string(),
                },
            ));
        }

        let user = user.into();
        let coin = coin.into();

        let subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription: json!({
                "type": "activeAssetData",
                "user": json!(user.clone()),
                "coin": json!(coin.clone())
            }),
        };

        self.send_and_flush(subscription_message).await?;

        let capacity = subscription_config.unwrap_or_default().channel_capacity;
        let (tx, rx) = Self::subscription_channel::<WsActiveAssetData>(Some(capacity));

        let write_lock = &mut self.streams.write().await.active_asset_data;
        write_lock.0 = Some(tx);
        write_lock.1 = Some(user);
        write_lock.2 = Some(coin);

        Ok(rx)
    }

    /// Subscribes to the [`WsUserTwapSliceFills`] websocket feed.
    ///
    /// # Arguments
    /// * `coin` The symbol of the coin, i.e. the asset for receiving asset context
    /// * `user`
    ///
    /// # Returns
    /// A bounded `tokio::sync::broadcast::Sender`.
    pub async fn subscribe_user_twap_slice_fills(
        &mut self,
        subscription_config: Option<SubscriptionConfig>,
        user: impl Into<String>,
    ) -> Result<Receiver<WsUserTwapSliceFills>> {
        if self.streams.read().await.user_twap_slice_fills.0.is_some() {
            tracing::error!("Already subscribed to `userTwapSliceFills`");
            return Err(HyperliquidError::SubscriptionError(
                SubscriptionError::SubscriptionExist {
                    method: "userTwapSliceFills".to_string(),
                },
            ));
        }

        let user = user.into();

        let subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription: json!({
                "type": "userTwapSliceFills",
                "user": json!(user.clone()),
            }),
        };

        self.send_and_flush(subscription_message).await?;

        let capacity = subscription_config.unwrap_or_default().channel_capacity;
        let (tx, rx) = Self::subscription_channel::<WsUserTwapSliceFills>(Some(capacity));

        let write_lock = &mut self.streams.write().await.user_twap_slice_fills;
        write_lock.0 = Some(tx);
        write_lock.1 = Some(user);

        Ok(rx)
    }

    /// Subscribes to the [`WsUserTwapHistory`] websocket feed.
    ///
    /// # Arguments
    /// * `coin` The symbol of the coin, i.e. the asset for receiving asset context
    /// * `user`
    ///
    /// # Returns
    /// A bounded `tokio::sync::broadcast::Sender`.
    pub async fn subscribe_user_twap_history(
        &mut self,
        subscription_config: Option<SubscriptionConfig>,
        user: impl Into<String>,
    ) -> Result<Receiver<WsUserTwapHistory>> {
        if self.streams.read().await.user_twap_history.0.is_some() {
            tracing::error!("Already subscribed to `userTwapHistory`");
            return Err(HyperliquidError::SubscriptionError(
                SubscriptionError::SubscriptionExist {
                    method: "userTwapHistory".to_string(),
                },
            ));
        }

        let user = user.into();

        let subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription: json!({
                "type": "userTwapHistory",
                "user": json!(user.clone()),
            }),
        };

        self.send_and_flush(subscription_message).await?;

        let capacity = subscription_config.unwrap_or_default().channel_capacity;
        let (tx, rx) = Self::subscription_channel::<WsUserTwapHistory>(Some(capacity));

        let write_lock = &mut self.streams.write().await.user_twap_history;
        write_lock.0 = Some(tx);
        write_lock.1 = Some(user);

        Ok(rx)
    }

    /// Subscribes to the [`WsBbo`] websocket feed.
    ///
    /// # Arguments
    /// * `coin` The symbol of the coin, i.e. the asset for receiving asset context
    /// * `user`
    ///
    /// # Returns
    /// A bounded `tokio::sync::broadcast::Sender`.
    pub async fn subscribe_bbo(
        &mut self,
        subscription_config: Option<SubscriptionConfig>,
        user: impl Into<String>,
    ) -> Result<Receiver<WsBbo>> {
        if self.streams.read().await.bbo.0.is_some() {
            tracing::error!("Already subscribed to `bbo`");
            return Err(HyperliquidError::SubscriptionError(
                SubscriptionError::SubscriptionExist {
                    method: "bbo".to_string(),
                },
            ));
        }

        let user = user.into();

        let subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription: json!({
                "type": "bbo",
                "user": json!(user.clone()),
            }),
        };

        self.send_and_flush(subscription_message).await?;

        let capacity = subscription_config.unwrap_or_default().channel_capacity;
        let (tx, rx) = Self::subscription_channel::<WsBbo>(Some(capacity));

        let write_lock = &mut self.streams.write().await.bbo;
        write_lock.0 = Some(tx);
        write_lock.1 = Some(user);

        Ok(rx)
    }
}
