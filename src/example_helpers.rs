use alloy::signers::local::PrivateKeySigner;

use crate::client::HyperliquidClient;
use std::env;

pub fn testnet_client() -> Result<HyperliquidClient, Box<dyn std::error::Error>> {
    let signer = load_signer();
    Ok(HyperliquidClient::builder()
        .testnet()
        .with_wallet(signer)
        .with_subscriptions()
        .build()?)
}

pub fn mainnet_client() -> Result<HyperliquidClient, Box<dyn std::error::Error>> {
    let signer = load_signer();
    Ok(HyperliquidClient::builder()
        .mainnet()
        .with_wallet(signer)
        .with_subscriptions()
        .build()?)
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

pub fn load_signer() -> PrivateKeySigner {
    #[allow(clippy::expect_used)]
    let pkey = env::var("HL_PRIVATE_KEY").expect("HL_PRIVATE_KEY env var is missing");

    #[allow(clippy::expect_used)]
    pkey.parse().expect("Invalid private key format")
}
