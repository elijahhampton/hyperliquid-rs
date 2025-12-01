use crate::client::HyperliquidClient;
use crate::error::Result;

pub fn testnet_client() -> Result<HyperliquidClient> {
    HyperliquidClient::builder().testnet().build()
}

pub fn mainnet_client() -> Result<HyperliquidClient> {
    HyperliquidClient::builder().mainnet().build()
}

#[allow(clippy::must_use_candidate)]
pub fn user() -> String {
    "0x06B825B202450B598E962AA3F0816e8d336337d9".to_owned()
}

#[allow(clippy::must_use_candidate)]
pub fn builder() -> String {
    "0xeCe9Aa540e6dA813aF0356342E45167A67F54962".to_owned()
}

#[allow(clippy::must_use_candidate)]
pub fn vault_address() -> String {
    String::new()
}
