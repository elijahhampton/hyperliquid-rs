#![allow(dead_code, unused_variables)]
use crate::api::subscription::sender::StreamSenders;
use crate::client::HyperliquidClient;
use crate::error::{HyperliquidError, Result, SubscriptionError};
use crate::types::ws::{SubscriptionConfirmation, WsAllMids, WsClearinghouseState, WsNotification, WsTwapStates, WsWebData3};
use crate::types::{ws::{SubscriptionResponse}};
use futures::{stream::SplitStream, StreamExt};
use futures_util::stream::SplitSink;
use futures_util::SinkExt;
use serde_json::json;
use tokio_tungstenite::connect_async;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::{
    sync::broadcast::{Sender, Receiver, channel}
};
use tokio_tungstenite::{
    tungstenite:: Message,
    WebSocketStream,
};

pub struct SubscriptionConfig {
    channel_capacity: usize
}

impl Default for SubscriptionConfig {
    fn default() -> Self {
        Self {
            channel_capacity: 1000
        }
    }
}

/// A client providing access to Hyperliquid Subscriptions API.
pub struct SubscriptionClient<'client> {
    write_stream: SplitSink<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>, Message>,
    streams: Arc<RwLock<StreamSenders>>,
    client: &'client HyperliquidClient
}

impl<'client> SubscriptionClient<'client> {
    pub async fn new(ws_endpoint: String, client: &'client HyperliquidClient) -> Result<Self> {
        let (ws_stream, _response) = connect_async(&ws_endpoint).await?;
        let (write_stream, read_stream) = ws_stream.split();

        let senders = Arc::new(RwLock::new(StreamSenders::new()));

        Self::spawn_read_task(senders.clone(), read_stream);

        Ok(Self {
            write_stream,
            streams: senders,
            client
        })
    }

    fn subscription_channel<T: Clone>(&self, capacity: Option<usize>) -> (Sender<T>, Receiver<T>) {
        channel::<T>(capacity.unwrap_or(1000))
    }

    async fn send_and_flush(&mut self, confirmation: SubscriptionConfirmation) -> Result<()> {
          self.write_stream
            .send(Message::Text(serde_json::to_string(&confirmation)?.into()))
            .await?;
        self.write_stream.flush().await?;
        Ok(())
    }

    fn spawn_read_task(senders: Arc<RwLock<StreamSenders>>, mut read_stream: SplitStream<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>) {
        tokio::spawn(async move {
            while let Some(msg) = read_stream.next().await {
                if let Ok(msg) = msg {
                    match msg {
                        Message::Text(utf8_bytes) => {
                            let string_from_bytes = String::from_utf8(utf8_bytes.as_bytes().to_vec()).unwrap();
                            let value =
                                serde_json::from_str::<SubscriptionResponse>(&string_from_bytes)
                                    .unwrap();

                            match value {
                                SubscriptionResponse::Error(error) => {
                                    tracing::error!("{:?}", error);
                                }
                                SubscriptionResponse::AllMids(mids) => {
                                    let all_mids_tx = &senders.read().await.all_mids;
                                    if let Some(tx) = all_mids_tx {
                                        if let Err(send_err) = tx.send(mids) {
                                            tracing::error!("{:?}", send_err);
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
            }
        });
    }

    /// Subscribes to the [`WsAllMids`] websocket feed.
    ///
    /// # Arguments
    /// * `dex` (optional) Represents the perp dex to source mids from. If not provided,
    /// then the first perp dex is used. Spot mids are only included with the first perp dex.
    ///
    /// # Returns
    /// A bounded tokio::sync::broadcast::Sender
    pub async fn subscribe_all_mids(&mut self, subscription_config: Option<SubscriptionConfig>, dex: Option<String>) -> Result<Receiver<WsAllMids>> {
        if self.streams.read().await.all_mids.is_some() {
            tracing::error!("Already subscribed to `AllMids`");
            return Err(HyperliquidError::SubscriptionError(SubscriptionError::SubscriptionExist { method: "allMids".to_string() }));
        }

        let capacity = subscription_config.unwrap_or_default().channel_capacity;
        let (tx, rx) = self.subscription_channel::<WsAllMids>(Some(capacity));

        let mut subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription: json!({
                "type": "allMids",
            })
        };

        if let Some(opt_dex) = dex {
            subscription_message.subscription["dex"] = serde_json::Value::String(opt_dex);
        }

        self.send_and_flush(subscription_message).await?;

        self.streams.write().await.all_mids = Some(tx);

        Ok(rx)
    }

    /// Subscribes to the [`WsNotification`] websocket feed.
    ///
    /// # Arguments
    /// * `user`
    ///
    /// # Returns
    /// A bounded tokio::sync::broadcast::Sender
    pub async fn subscribe_notifications(&mut self,
        subscription_config: Option<SubscriptionConfig>,
        user: impl Into<String>) -> Result<Receiver<WsNotification>> {
        if self.streams.read().await.notifications.is_some() {
            tracing::error!("Already subscribed to `notifications`");
            return Err(HyperliquidError::SubscriptionError(SubscriptionError::SubscriptionExist { method: "notifications".to_string() }));
        }

        let capacity = subscription_config.unwrap_or_default().channel_capacity;
        let (tx, rx) = self.subscription_channel::<WsNotification>(Some(capacity));

        let subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription: json!({
                "type": "notification",
                "user": json!(user.into())
            })
        };

        self.send_and_flush(subscription_message).await?;

        self.streams.write().await.notifications = Some(tx);

        Ok(rx)
    }

    /// Subscribes to the [`WsWebData3`] websocket feed.
    ///
    /// # Arguments
    /// * `user`
    ///
    /// # Returns
    /// A bounded tokio::sync::broadcast::Sender.
    pub async fn subscribe_webdata3(&mut self,
        subscription_config: Option<SubscriptionConfig>,
        user: impl Into<String>) -> Result<Receiver<WsWebData3>> {
        if self.streams.read().await.webdata3.is_some() {
            tracing::error!("Already subscribed to `webdata3`");
            return Err(HyperliquidError::SubscriptionError(SubscriptionError::SubscriptionExist { method: "webData3".to_string() }));
        }

        let capacity = subscription_config.unwrap_or_default().channel_capacity;
        let (tx, rx) = self.subscription_channel::<WsWebData3>(Some(capacity));

        let subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription: json!({
                "type": "webdata3",
                "user": json!(user.into())
            })
        };

        self.send_and_flush(subscription_message).await?;

        self.streams.write().await.webdata3 = Some(tx);

        Ok(rx)
    }

    /// Subscribes to the [`WsTwapStates`] websocket feed.
    ///
    /// # Arguments
    /// * `user`
    ///
    /// # Returns
    /// A bounded tokio::sync::broadcast::Sender.
    pub async fn subscribe_twap_states(&mut self,
        subscription_config: Option<SubscriptionConfig>,
        user: impl Into<String>) -> Result<Receiver<WsTwapStates>> {
        if self.streams.read().await.twap_states.is_some() {
            tracing::error!("Already subscribed to `twapStates`");
            return Err(HyperliquidError::SubscriptionError(SubscriptionError::SubscriptionExist { method: "twapStates".to_string() }));
        }

        let capacity = subscription_config.unwrap_or_default().channel_capacity;
        let (tx, rx) = self.subscription_channel::<WsTwapStates>(Some(capacity));

        let subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription: json!({
                "type": "twapStates",
                "user": json!(user.into())
            })
        };

        self.send_and_flush(subscription_message).await?;

        self.streams.write().await.twap_states = Some(tx);

        Ok(rx)
    }


    /// Subscribes to the [`WsClearinghouseState`] websocket feed.
    ///
    /// # Arguments
    /// * `user`
    ///
    /// # Returns
    /// A bounded tokio::sync::broadcast::Sender.
    pub async fn subscribe_clearinghouse_state(&mut self,
        subscription_config: Option<SubscriptionConfig>,
        user: impl Into<String>) -> Result<Receiver<WsClearinghouseState>> {
        if self.streams.read().await.clearinghouse_state.is_some() {
            tracing::error!("Already subscribed to `clearinghouseState");
            return Err(HyperliquidError::SubscriptionError(SubscriptionError::SubscriptionExist { method: "clearinghouseState".to_string() }));
        }

        let capacity = subscription_config.unwrap_or_default().channel_capacity;
        let (tx, rx) = self.subscription_channel::<WsClearinghouseState>(Some(capacity));

        let subscription_message = SubscriptionConfirmation {
            method: "subscribe".to_string(),
            subscription: json!({
                "type": "clearinghouseState",
                "user": json!(user.into())
            })
        };

        self.send_and_flush(subscription_message).await?;

        self.streams.write().await.clearinghouse_state = Some(tx);

        Ok(rx)
    }
}
