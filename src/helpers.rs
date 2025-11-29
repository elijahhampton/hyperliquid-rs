#![allow(dead_code)]
use crate::client::HyperliquidClient;

pub fn testnet_client() -> HyperliquidClient {
    HyperliquidClient::builder().testnet().build().unwrap()
}

pub fn mainnet_client() -> HyperliquidClient {
    HyperliquidClient::builder().mainnet().build().unwrap()
}

pub fn user() -> String {
    "0x06B825B202450B598E962AA3F0816e8d336337d9".to_string()
}

pub fn builder() -> String {
    "0xeCe9Aa540e6dA813aF0356342E45167A67F54962".to_string()
}

pub fn vault_address() -> String {
    "".to_string()
}
