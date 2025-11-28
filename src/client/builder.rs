use crate::{client::{HyperliquidClient}, error::HyperliquidError};

/// Represents the Hyperliquid Mainnet and Testnet chains.
enum Network {
    Mainnet,
    Testnet
}

/// Builder for configuring and produce a [`HyperliquidClient`].
pub struct HyperliquidClientBuilder {
    base_url:  Option<String>,
}

impl HyperliquidClientBuilder {
    pub fn new() -> Self {
        Self {
            base_url: None
        }
    }

    pub fn with_network(&mut self, network: Network) -> &HyperliquidClientBuilder {
        match network {
            Network::Mainnet => self.base_url = Some("".to_string()),
            Network::Testnet => self.base_url = Some("".to_string()),
            _ => self.base_url = Some("".to_string()),
        }

        self
    }

    pub fn build(&self) -> Result<HyperliquidClient, HyperliquidError> {
        HyperliquidClient::new(self.base_url.clone().expect(""))
    }
}
