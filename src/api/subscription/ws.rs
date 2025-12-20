use crate::api::request_util::SUPPORTED_INTERVALS;
use crate::client::HyperliquidClient;
use crate::error::{HyperliquidError, Result};
use crate::types::ws::SubscriptionConfirmation;
use crate::types::ws::SubscriptionResponse;
use futures::{stream::SplitStream, StreamExt};
use futures_util::stream::SplitSink;
use futures_util::SinkExt;
use serde::Serialize;
use serde_json::json;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use tokio::sync::broadcast::{channel, Receiver, Sender};
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

type WsWriteStream =
    SplitSink<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>, Message>;

type WsReadStream =
    SplitStream<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>;

pub enum StreamMessage {
    Subscription(SubscriptionConfirmation),
    Heartbeat,
}

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

#[derive(Clone, Eq)]
pub enum SubscriptionSpec {
    AllMids {
        dex: Option<String>,
    },
    Candle {
        coin: String,
        interval: String,
    },
    L2Book {
        coin: String,
        n_sig_figs: Option<u32>,
        mantissa: Option<u32>,
    },
    Trades {
        coin: String,
    },
    Notifications {
        user: String,
    },
    WebData3 {
        user: String,
    },
    TwapStates {
        user: String,
    },
    ClearinghouseState {
        user: String,
    },
    OpenOrders {
        user: String,
    },
    UserEvents {
        user: String,
    },
    UserFills {
        user: String,
    },
    UserFunding {
        user: String,
    },
    UserNonFundingLedgerUpdates {
        user: String,
    },
    ActiveAssetCtx {
        coin: String,
    },
    ActiveAssetData {
        user: String,
        coin: String,
    },
    UserTwapSliceFills {
        user: String,
    },
    UserTwapHistory {
        user: String,
    },
    Bbo {
        coin: String,
    },
}

impl PartialEq for SubscriptionSpec {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::AllMids { dex: a }, Self::AllMids { dex: b }) => a == b,

            (
                Self::Candle {
                    coin: a1,
                    interval: a2,
                },
                Self::Candle {
                    coin: b1,
                    interval: b2,
                },
            ) => a1 == b1 && a2 == b2,

            (
                Self::L2Book {
                    coin: a,
                    n_sig_figs: a2,
                    mantissa: a3,
                },
                Self::L2Book {
                    coin: b,
                    n_sig_figs: b2,
                    mantissa: b3,
                },
            ) => a == b && a2 == b2 && a3 == b3,

            (Self::Trades { coin: a }, Self::Trades { coin: b }) => a == b,

            (Self::Notifications { user: a }, Self::Notifications { user: b }) => a == b,
            (Self::WebData3 { user: a }, Self::WebData3 { user: b }) => a == b,
            (Self::TwapStates { user: a }, Self::TwapStates { user: b }) => a == b,
            (Self::ClearinghouseState { user: a }, Self::ClearinghouseState { user: b }) => a == b,
            (Self::OpenOrders { user: a }, Self::OpenOrders { user: b }) => a == b,
            (Self::UserEvents { user: a }, Self::UserEvents { user: b }) => a == b,
            (Self::UserFills { user: a }, Self::UserFills { user: b }) => a == b,
            (Self::UserFunding { user: a }, Self::UserFunding { user: b }) => a == b,
            (
                Self::UserNonFundingLedgerUpdates { user: a },
                Self::UserNonFundingLedgerUpdates { user: b },
            ) => a == b,

            (Self::ActiveAssetCtx { coin: a }, Self::ActiveAssetCtx { coin: b }) => a == b,

            (
                Self::ActiveAssetData { user: a1, coin: a2 },
                Self::ActiveAssetData { user: b1, coin: b2 },
            ) => a1 == b1 && a2 == b2,

            (Self::UserTwapSliceFills { user: a }, Self::UserTwapSliceFills { user: b }) => a == b,

            (Self::UserTwapHistory { user: a }, Self::UserTwapHistory { user: b }) => a == b,

            (Self::Bbo { coin: a }, Self::Bbo { coin: b }) => a == b,

            _ => false,
        }
    }
}

impl Hash for SubscriptionSpec {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            Self::AllMids { dex } => dex.hash(state),

            Self::Candle { coin, interval } => {
                coin.hash(state);
                interval.hash(state);
            }

            Self::L2Book {
                coin,
                n_sig_figs,
                mantissa,
            } => {
                coin.hash(state);
                n_sig_figs.hash(state);
                mantissa.hash(state);
            }

            Self::Trades { coin } | Self::Bbo { coin } => coin.hash(state),

            Self::Notifications { user }
            | Self::WebData3 { user }
            | Self::TwapStates { user }
            | Self::ClearinghouseState { user }
            | Self::OpenOrders { user }
            | Self::UserEvents { user }
            | Self::UserFills { user }
            | Self::UserFunding { user }
            | Self::UserNonFundingLedgerUpdates { user }
            | Self::UserTwapSliceFills { user }
            | Self::UserTwapHistory { user } => user.hash(state),

            Self::ActiveAssetCtx { coin } => coin.hash(state),

            Self::ActiveAssetData { user, coin } => {
                user.hash(state);
                coin.hash(state);
            }
        }
    }
}

/// A WS client providing access to Hyperliquid Subscriptions API.
pub struct SubscriptionClient {
    pub events: Receiver<SubscriptionResponse>,
    config: SubscriptionConfig,
    active_subs: tokio::sync::RwLock<HashSet<SubscriptionSpec>>,
    write_stream_tx: tokio::sync::mpsc::UnboundedSender<StreamMessage>,
}

impl SubscriptionClient {
    pub async fn new(
        client: &HyperliquidClient,
        config: Option<SubscriptionConfig>,
    ) -> Result<Self> {
        let endpoint = client
            .ws_endpoint()
            .ok_or(HyperliquidError::MissingConfiguration {
                parameter: "ws_endpoint".to_string(),
            })?;
        let (ws_stream, _response) = connect_async(endpoint).await?;
        let (write_stream, read_stream) = ws_stream.split();

        let config = config.unwrap_or_default();
        let (tx, rx) = channel::<SubscriptionResponse>(config.channel_capacity);
        let (write_stream_tx, write_stream_rx) =
            tokio::sync::mpsc::unbounded_channel::<StreamMessage>();

        Self::spawn_write_task(write_stream, write_stream_rx);
        Self::spawn_heartbeat(write_stream_tx.clone());
        Self::spawn_read_task(tx, read_stream);

        Ok(Self {
            config,
            events: rx,
            active_subs: RwLock::new(HashSet::new()),
            write_stream_tx,
        })
    }

    fn spawn_write_task(
        mut write_stream: WsWriteStream,
        mut rx: tokio::sync::mpsc::UnboundedReceiver<StreamMessage>,
    ) -> JoinHandle<()> {
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                match msg {
                    StreamMessage::Subscription(confirmation) => {
                        if let Err(e) = write_stream
                            .send(Message::Text(
                                serde_json::to_string(&confirmation)
                                    .expect("confirmation to stringify")
                                    .into(),
                            ))
                            .await
                        {
                            tracing::error!("Failed to send subscription: {:?}", e);
                            break;
                        }
                        if let Err(e) = write_stream.flush().await {
                            tracing::error!("Failed to flush: {:?}", e);
                            break;
                        }
                    }
                    StreamMessage::Heartbeat => {
                        if let Err(e) = write_stream
                            .send(Message::Text(r#"{"method":"ping"}"#.into()))
                            .await
                        {
                            tracing::error!("Failed to send ping: {:?}", e);
                            break;
                        }
                    }
                }
            }
            tracing::info!("Write task shutting down");
        })
    }

    pub fn spawn_heartbeat(
        tx: tokio::sync::mpsc::UnboundedSender<StreamMessage>,
    ) -> tokio::task::JoinHandle<()> {
        tokio::task::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(50));
            interval.tick().await;

            loop {
                interval.tick().await;

                if let Err(e) = tx.send(StreamMessage::Heartbeat) {
                    tracing::error!("Failed to send heartbeat: {:?}", e);
                    break;
                }
            }
        })
    }

    pub fn spawn_read_task(
        tx: Sender<SubscriptionResponse>,
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

                if let Err(e) = tx.send(value) {
                    tracing::error!("{:?}", e);
                }
            }
        })
    }

    fn subscription_channel<T: Clone>(capacity: Option<usize>) -> (Sender<T>, Receiver<T>) {
        channel::<T>(capacity.unwrap_or(1000))
    }

    fn send_and_flush(&self, confirmation: SubscriptionConfirmation) -> Result<()> {
        self.write_stream_tx
            .send(StreamMessage::Subscription(confirmation))?;
        Ok(())
    }

    fn build_subscription_json(spec: &SubscriptionSpec) -> serde_json::Value {
        match spec {
            SubscriptionSpec::AllMids { dex } => {
                let mut v = json!({ "type": "allMids" });

                if let Some(d) = dex {
                    if let Some(obj) = v.as_object_mut() {
                        obj.insert("dex".to_string(), json!(d));
                    }
                }

                v
            }

            SubscriptionSpec::Candle { coin, interval } => {
                json!({
                    "type": "candle",
                    "coin": coin,
                    "interval": interval,
                })
            }

            SubscriptionSpec::L2Book {
                coin,
                n_sig_figs,
                mantissa,
            } => {
                let mut v = json!({
                    "type": "l2Book",
                    "coin": coin,
                });

                if let Some(n) = n_sig_figs {
                    if let Some(obj) = v.as_object_mut() {
                        obj.insert("nSigFigs".to_string(), json!(n));
                    }
                }

                if let Some(m) = mantissa {
                    if let Some(obj) = v.as_object_mut() {
                        obj.insert("mantissa".to_string(), json!(m));
                    }
                }

                v
            }

            SubscriptionSpec::Trades { coin } => {
                json!({
                    "type": "trades",
                    "coin": coin,
                })
            }

            SubscriptionSpec::Notifications { user } => {
                json!({ "type": "notification", "user": user })
            }

            SubscriptionSpec::WebData3 { user } => {
                json!({ "type": "webData3", "user": user })
            }

            SubscriptionSpec::TwapStates { user } => {
                json!({ "type": "twapStates", "user": user })
            }

            SubscriptionSpec::ClearinghouseState { user } => {
                json!({ "type": "clearinghouseState", "user": user })
            }

            SubscriptionSpec::OpenOrders { user } => {
                json!({ "type": "openOrders", "user": user })
            }

            SubscriptionSpec::UserEvents { user } => {
                json!({ "type": "userEvents", "user": user })
            }

            SubscriptionSpec::UserFills { user } => {
                json!({ "type": "userFills", "user": user })
            }

            SubscriptionSpec::UserFunding { user } => {
                json!({ "type": "userFundings", "user": user })
            }

            SubscriptionSpec::UserNonFundingLedgerUpdates { user } => {
                json!({ "type": "userNonFundingLedgerUpdates", "user": user })
            }

            SubscriptionSpec::ActiveAssetCtx { coin } => {
                json!({ "type": "activeAssetCtx", "coin": coin })
            }

            SubscriptionSpec::ActiveAssetData { user, coin } => {
                json!({ "type": "activeAssetData", "user": user, "coin": coin })
            }

            SubscriptionSpec::UserTwapSliceFills { user } => {
                json!({ "type": "userTwapSliceFills", "user": user })
            }

            SubscriptionSpec::UserTwapHistory { user } => {
                json!({ "type": "userTwapHistory", "user": user })
            }

            SubscriptionSpec::Bbo { coin } => {
                json!({ "type": "bbo", "coin": coin })
            }
        }
    }

    pub async fn unsubscribe(&mut self, spec: &SubscriptionSpec) -> Result<()> {
        if !self.active_subs.read().await.contains(spec) {
            return Ok(());
        }

        let subscription = Self::build_subscription_json(spec);

        self.send_and_flush(SubscriptionConfirmation {
            method: "unsubscribe".into(),
            subscription,
        })?;

        self.active_subs.write().await.remove(spec);
        Ok(())
    }

    /// Subscribes to the [`WsAllMids`] websocket feed.
    ///
    /// # Arguments
    /// * `dex` (optional) Represents the perp dex to source mids from. If not provided,
    /// then the first perp dex is used. Spot mids are only included with the first perp dex.
    ///
    /// # Returns
    /// A bounded `tokio::sync::broadcast::Sender`
    pub async fn subscribe_all_mids(&mut self, dex: Option<String>) -> Result<()> {
        let spec = SubscriptionSpec::AllMids { dex: dex.clone() };

        if self.active_subs.read().await.contains(&spec) {
            return Ok(());
        }

        let subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription: json!({
                "type": "allMids",
            }),
        };

        if let Some(dex_param) = dex {
            if let Some(dex_obj) = subscription_message.clone().subscription.get_mut("dex") {
                if let Some(obj) = dex_obj.as_object_mut() {
                    let val = serde_json::Value::String(dex_param);

                    obj.insert("dex".to_string(), val);
                }
            }
        }

        self.send_and_flush(subscription_message)?;

        self.active_subs.write().await.insert(spec);

        Ok(())
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
        coin: impl Into<String> + Serialize + Clone,
        interval: String,
    ) -> Result<()> {
        let coin = coin.into();

        let spec = SubscriptionSpec::Candle {
            coin,
            interval: interval.clone(),
        };
        if self.active_subs.read().await.contains(&spec) {
            return Ok(());
        }

        if !SUPPORTED_INTERVALS.contains(interval.as_str()) {
            return Err(HyperliquidError::InvalidRequestParameter {
                method: "subscribe_candle".to_string(),
                parameter: "interval".to_string(),
                reason: format!("Supported intervals include: {:?}", SUPPORTED_INTERVALS),
            });
        }

        let subscription = Self::build_subscription_json(&spec);
        let subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription,
        };

        self.send_and_flush(subscription_message)?;

        self.active_subs.write().await.insert(spec);

        Ok(())
    }

    /// Subscribes to the [`WsBook`] websocket feed.
    ///
    /// # Arguments
    /// * `coin` The desired asset for returning l2book data.
    /// * `n_sig_figs` - Optional number of significant figures. May cause subscription
    ///   to not receive data on testnet if set. Use `None` if unsure.
    /// * `mantissa` - Optional mantissa. May cause subscription to not receive data
    ///   on testnet if set. Use `None` if unsure.
    ///
    /// # Returns
    /// A bounded `tokio::sync::broadcast::Sender`
    pub async fn subscribe_l2_book(
        &mut self,
        coin: impl Into<String> + Serialize,
        n_sig_figs: Option<u32>,
        mantissa: Option<u32>,
    ) -> Result<()> {
        let coin = coin.into();
        let spec = SubscriptionSpec::L2Book {
            coin: coin.clone(),
            n_sig_figs,
            mantissa,
        };

        if self.active_subs.read().await.contains(&spec) {
            return Ok(());
        }

        let subscription = Self::build_subscription_json(&spec);
        let subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription,
        };

        self.send_and_flush(subscription_message)?;

        self.active_subs.write().await.insert(spec);

        Ok(())
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
        coin: impl Into<String> + Serialize + Clone,
    ) -> Result<()> {
        let coin = coin.into();
        let spec = SubscriptionSpec::Trades { coin: coin.clone() };

        if self.active_subs.read().await.contains(&spec) {
            return Ok(());
        }

        let subscription = Self::build_subscription_json(&spec);
        let subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription,
        };

        self.send_and_flush(subscription_message)?;

        self.active_subs.write().await.insert(spec);

        Ok(())
    }

    /// Subscribes to the [`WsNotification`] websocket feed.
    ///
    /// # Arguments
    /// * `user`
    ///
    /// # Returns
    /// A bounded `tokio::sync::broadcast::Sender`
    pub async fn subscribe_notifications(&mut self, user: impl Into<String>) -> Result<()> {
        let user = user.into();

        let spec = SubscriptionSpec::Notifications { user: user.clone() };

        if self.active_subs.read().await.contains(&spec) {
            return Ok(());
        }

        let subscription = Self::build_subscription_json(&spec);
        let subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription,
        };

        self.send_and_flush(subscription_message)?;

        self.active_subs.write().await.insert(spec);

        Ok(())
    }

    /// Subscribes to the [`WsWebData3`] websocket feed.
    ///
    /// # Arguments
    /// * `user`
    ///
    /// # Returns
    /// A bounded `tokio::sync::broadcast::Sender`.
    pub async fn subscribe_webdata3(&mut self, user: impl Into<String>) -> Result<()> {
        let user = user.into();

        let spec = SubscriptionSpec::WebData3 { user: user.clone() };

        if self.active_subs.read().await.contains(&spec) {
            return Ok(());
        }

        let subscription = Self::build_subscription_json(&spec);
        let subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription,
        };

        self.send_and_flush(subscription_message)?;

        self.active_subs.write().await.insert(spec);

        Ok(())
    }

    /// Subscribes to the [`WsTwapStates`] websocket feed.
    ///
    /// # Arguments
    /// * `user`
    ///
    /// # Returns
    /// A bounded `tokio::sync::broadcast::Sender`.
    pub async fn subscribe_twap_states(&mut self, user: impl Into<String>) -> Result<()> {
        let user = user.into();

        let spec = SubscriptionSpec::TwapStates { user: user.clone() };

        if self.active_subs.read().await.contains(&spec) {
            return Ok(());
        }

        let subscription = Self::build_subscription_json(&spec);
        let subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription,
        };

        self.send_and_flush(subscription_message)?;

        self.active_subs.write().await.insert(spec);

        Ok(())
    }

    /// Subscribes to the [`WsClearinghouseState`] websocket feed.
    ///
    /// # Arguments
    /// * `user`
    ///
    /// # Returns
    /// A bounded `tokio::sync::broadcast::Sender`.
    pub async fn subscribe_clearinghouse_state(&mut self, user: impl Into<String>) -> Result<()> {
        let user = user.into();

        let spec = SubscriptionSpec::ClearinghouseState { user: user.clone() };

        if self.active_subs.read().await.contains(&spec) {
            return Ok(());
        }

        let subscription = Self::build_subscription_json(&spec);
        let subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription,
        };

        self.send_and_flush(subscription_message)?;

        self.active_subs.write().await.insert(spec);

        Ok(())
    }

    /// Subscribes to the [`WsOpenOrders`] websocket feed.
    ///
    /// # Arguments
    /// * `user`
    ///
    /// # Returns
    /// A bounded `tokio::sync::broadcast::Sender`.
    pub async fn subscribe_open_orders(&mut self, user: impl Into<String>) -> Result<()> {
        let user = user.into();

        let spec = SubscriptionSpec::OpenOrders { user: user.clone() };

        if self.active_subs.read().await.contains(&spec) {
            return Ok(());
        }

        let subscription = Self::build_subscription_json(&spec);

        let subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription,
        };

        self.send_and_flush(subscription_message)?;
        self.active_subs.write().await.insert(spec);

        Ok(())
    }

    /// Subscribes to the [`WsUserEvent`] websocket feed.
    ///
    /// # Arguments
    /// * `user`
    ///
    /// # Returns
    /// A bounded `tokio::sync::broadcast::Sender`.
    pub async fn subscribe_user_events(&mut self, user: impl Into<String>) -> Result<()> {
        let user = user.into();

        let spec = SubscriptionSpec::UserEvents { user: user.clone() };

        if self.active_subs.read().await.contains(&spec) {
            return Ok(());
        }

        let subscription = Self::build_subscription_json(&spec);
        let subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription,
        };

        self.send_and_flush(subscription_message)?;

        self.active_subs.write().await.insert(spec);

        Ok(())
    }

    /// Subscribes to the [`WsUserFills`] websocket feed.
    ///
    /// # Arguments
    /// * `user`
    ///
    /// # Returns
    /// A bounded `tokio::sync::broadcast::Sender`.
    pub async fn subscribe_user_fills(&mut self, user: impl Into<String>) -> Result<()> {
        let user = user.into();

        let spec = SubscriptionSpec::UserFills { user: user.clone() };

        if self.active_subs.read().await.contains(&spec) {
            return Ok(());
        }

        let subscription = Self::build_subscription_json(&spec);

        let subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription,
        };

        self.send_and_flush(subscription_message)?;

        self.active_subs.write().await.insert(spec);

        Ok(())
    }

    /// Subscribes to the [`WsUserFunding`] websocket feed.
    ///
    /// # Arguments
    /// * `user`
    ///
    /// # Returns
    /// A bounded `tokio::sync::broadcast::Sender`.
    pub async fn subscribe_user_funding(&mut self, user: impl Into<String>) -> Result<()> {
        let user = user.into();

        let spec = SubscriptionSpec::UserFunding { user: user.clone() };

        if self.active_subs.read().await.contains(&spec) {
            return Ok(());
        }

        let subscription = Self::build_subscription_json(&spec);

        let subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription,
        };

        self.send_and_flush(subscription_message)?;

        self.active_subs.write().await.insert(spec);

        Ok(())
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

        user: impl Into<String>,
    ) -> Result<()> {
        let user = user.into();

        let spec = SubscriptionSpec::UserNonFundingLedgerUpdates { user: user.clone() };

        if self.active_subs.read().await.contains(&spec) {
            return Ok(());
        }

        let subscription = Self::build_subscription_json(&spec);

        let subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription,
        };

        self.send_and_flush(subscription_message)?;

        self.active_subs.write().await.insert(spec);

        Ok(())
    }

    /// Subscribes to the [`WsActiveAssetCtx`] websocket feed.
    ///
    /// # Arguments
    /// * `coin` The symbol of the coin, i.e. the asset for receiving asset context
    ///
    /// # Returns
    /// A bounded `tokio::sync::broadcast::Sender`.
    pub async fn subscribe_active_asset_ctx(&mut self, coin: impl Into<String>) -> Result<()> {
        let coin = coin.into();

        let spec = SubscriptionSpec::ActiveAssetCtx { coin: coin.clone() };

        if self.active_subs.read().await.contains(&spec) {
            return Ok(());
        }

        let subscription = Self::build_subscription_json(&spec);

        let subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription,
        };

        self.send_and_flush(subscription_message)?;

        self.active_subs.write().await.insert(spec);

        Ok(())
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

        user: impl Into<String>,
        coin: impl Into<String>,
    ) -> Result<()> {
        let user = user.into();
        let coin = coin.into();

        let spec = SubscriptionSpec::ActiveAssetData {
            coin: coin.clone(),
            user: user.clone(),
        };

        if self.active_subs.read().await.contains(&spec) {
            return Ok(());
        }

        let subscription = Self::build_subscription_json(&spec);

        let subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription,
        };

        self.send_and_flush(subscription_message)?;

        self.active_subs.write().await.insert(spec);

        Ok(())
    }

    /// Subscribes to the [`WsUserTwapSliceFills`] websocket feed.
    ///
    /// # Arguments
    /// * `coin` The symbol of the coin, i.e. the asset for receiving asset context
    /// * `user`
    ///
    /// # Returns
    /// A bounded `tokio::sync::broadcast::Sender`.
    pub async fn subscribe_user_twap_slice_fills(&mut self, user: impl Into<String>) -> Result<()> {
        let user = user.into();

        let spec = SubscriptionSpec::UserTwapSliceFills { user: user.clone() };

        if self.active_subs.read().await.contains(&spec) {
            return Ok(());
        }

        let subscription = Self::build_subscription_json(&spec);

        let subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription,
        };

        self.send_and_flush(subscription_message)?;

        self.active_subs.write().await.insert(spec);

        Ok(())
    }

    /// Subscribes to the [`WsUserTwapHistory`] websocket feed.
    ///
    /// # Arguments
    /// * `coin` The symbol of the coin, i.e. the asset for receiving asset context
    /// * `user`
    ///
    /// # Returns
    /// A bounded `tokio::sync::broadcast::Sender`.
    pub async fn subscribe_user_twap_history(&mut self, user: impl Into<String>) -> Result<()> {
        let user = user.into();

        let spec = SubscriptionSpec::UserTwapHistory { user: user.clone() };

        if self.active_subs.read().await.contains(&spec) {
            return Ok(());
        }

        let subscription = Self::build_subscription_json(&spec);

        let subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription,
        };

        self.send_and_flush(subscription_message)?;

        self.active_subs.write().await.insert(spec);

        Ok(())
    }

    /// Subscribes to the [`WsBbo`] websocket feed.
    ///
    /// # Arguments
    /// * `coin` The symbol of the coin, i.e. the asset for receiving asset context
    /// * `user`
    ///
    /// # Returns
    /// A bounded `tokio::sync::broadcast::Sender`.
    pub async fn subscribe_bbo(&mut self, coin: impl Into<String>) -> Result<()> {
        let coin = coin.into();

        let spec = SubscriptionSpec::Bbo { coin: coin.clone() };

        if self.active_subs.read().await.contains(&spec) {
            return Ok(());
        }

        let subscription = Self::build_subscription_json(&spec);

        let subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription,
        };

        self.send_and_flush(subscription_message)?;

        self.active_subs.write().await.insert(spec);

        Ok(())
    }
}
