use crate::cli::OrderRequest;
use crate::types::exchange::order::OrderSide;
use crate::types::exchange::{
    Builder, CancelRequest, Grouping, LimitOrder, Tif, Tpsl, TriggerOrder,
};
#[allow(unused_imports)]
use clap::{Args, Command, Parser, ValueEnum};
use serde::{Deserialize, Serialize};

/// CLI specific order type argument for order request.
#[derive(ValueEnum, Clone, Serialize, Deserialize)]
pub enum OrderTypeArg {
    #[serde(rename = "limit")]
    Limit,
    #[serde(rename = "trigger")]
    Trigger,
}

/// Command for placing an order
#[derive(Args)]
pub struct OrderCmd {
    coin: String,
    #[arg(value_enum)]
    side: OrderSide,
    size: String,

    // Order type (limit vs trigger)
    #[arg(long, default_value = "limit")]
    #[arg(value_enum)]
    order_type: OrderTypeArg,

    // Limit order fields
    #[arg(long, required_if_eq("order_type", "limit"))]
    price: Option<String>,

    #[arg(long, default_value = "gtc")]
    #[arg(value_enum)]
    tif: Tif,

    // Trigger order fields
    #[arg(long, required_if_eq("order_type", "trigger"))]
    trigger_price: Option<String>,

    #[arg(long, requires = "trigger_price")]
    #[arg(value_enum)]
    tpsl: Option<Tpsl>,

    #[arg(long)]
    trigger_market: bool,

    // Optional flags
    #[arg(long)]
    reduce_only: bool,

    #[arg(long)]
    cloid: Option<String>,

    // Function params
    #[arg(long, default_value = "na")]
    #[arg(value_enum)]
    grouping: Grouping,

    #[arg(long)]
    vault_address: Option<String>,

    #[arg(long)]
    expires_after: Option<u64>,

    // Builder fee
    #[arg(long)]
    builder_address: Option<String>,

    #[arg(long, requires = "builder_address")]
    builder_fee: Option<u32>,
}

impl OrderCmd {
    /// Builds the order request using the `asset_idx` and
    /// the command line arguments.
    pub fn build(
        self,
        asset_idx: u32,
    ) -> (
        OrderRequest,
        Grouping,
        Option<Builder>,
        Option<String>,
        Option<u64>,
    ) {
        use crate::types::exchange::{OrderRequest, OrderType};

        let order_type = match self.order_type {
            OrderTypeArg::Limit => OrderType::Limit(LimitOrder { tif: self.tif }),
            OrderTypeArg::Trigger => OrderType::Trigger(TriggerOrder {
                is_market: self.trigger_market,
                trigger_px: self
                    .trigger_price
                    .clone()
                    .expect("--trigger price required for trigger orders"),
                tpsl: self.tpsl.expect("--tpsl required for trigger orders"),
            }),
        };

        let builder = self.builder_address.map(|b| Builder {
            b,
            f: self.builder_fee.expect(""),
        });

        let price = match self.order_type {
            OrderTypeArg::Limit => self.price.ok_or("--price required for limit orders"),
            OrderTypeArg::Trigger => Ok(self.trigger_price.clone().unwrap_or_default()),
        };

        let order = OrderRequest {
            a: asset_idx,
            b: matches!(self.side, OrderSide::Buy),
            p: price.expect("--price required for limit orders"),
            s: self.size,
            r: self.reduce_only,
            t: order_type,
            c: self.cloid,
        };

        (
            order,
            self.grouping,
            builder,
            self.vault_address,
            self.expires_after,
        )
    }

    pub fn coin(&self) -> &str {
        &self.coin
    }
}

/// Cmd arguments for a cancel order request
#[derive(Args)]
pub struct CancelOrderByOidCmd {
    /// Name of coin
    coin: String,

    /// Order id
    o: u64,

    /// Asset index
    #[arg(long)]
    a: Option<u32>,

    #[arg(long)]
    vault_address: Option<String>,

    #[arg(long)]
    expires_after: Option<u64>,
}

impl CancelOrderByOidCmd {
    /// Builds the order request using the `asset_idx` and
    /// the command line arguments.
    pub fn build(self, asset_idx: u32) -> (CancelRequest, Option<String>, Option<u64>) {
        let cancel_req = CancelRequest {
            a: self.a.unwrap_or(asset_idx),
            o: self.o,
        };

        (cancel_req, self.vault_address, self.expires_after)
    }

    pub fn coin(&self) -> &str {
        &self.coin
    }
}
