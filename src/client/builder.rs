use crate::{client::HyperliquidClient, error::HyperliquidError};

/// Represents the Hyperliquid Mainnet and Testnet chains.
enum Network {
    Mainnet,
    Testnet,
}

/// Builder for configuring and produce a [`HyperliquidClient`].
pub struct HyperliquidClientBuilder {
    base_url: Option<String>,
    network: Option<Network>,
}

impl HyperliquidClientBuilder {
    pub fn new() -> Self {
        Self {
            base_url: None,
            network: None,
        }
    }

    pub fn testnet(&mut self) -> &HyperliquidClientBuilder {
        self.base_url = Some("https://api.hyperliquid-testnet.xyz".to_string());
        self.network = Some(Network::Testnet);
        self
    }

    pub fn mainnet(&mut self) -> &HyperliquidClientBuilder {
        self.base_url = Some("https://api.hyperliquid.xyz".to_string());
        self.network = Some(Network::Mainnet);
        self
    }

    pub fn endpoint(&mut self, endpoint: String) -> &HyperliquidClientBuilder {
        self.base_url = Some(endpoint);
        self
    }

    pub fn build(&self) -> Result<HyperliquidClient, HyperliquidError> {
        HyperliquidClient::new(self.base_url.clone().expect(""))
    }
}
