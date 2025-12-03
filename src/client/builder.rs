use crate::{client::HyperliquidClient, error::HyperliquidError};
use alloy::signers::local::PrivateKeySigner;

/// Represents the Hyperliquid Mainnet and Testnet chains.
enum Network {
    Mainnet,
    Testnet,
}

/// Builder for configuring and produce a [`HyperliquidClient`].
pub struct HyperliquidClientBuilder {
    base_url: Option<String>,
    network: Option<Network>,
    wallet: Option<PrivateKeySigner>,
}

impl Default for HyperliquidClientBuilder {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl HyperliquidClientBuilder {
    #[inline]
    pub const fn new() -> Self {
        Self {
            base_url: None,
            network: None,
            wallet: None,
        }
    }

    #[inline]
    pub fn testnet(&mut self) -> &mut Self {
        self.base_url = Some("https://api.hyperliquid-testnet.xyz".to_owned());
        self.network = Some(Network::Testnet);
        self
    }

    #[inline]
    pub fn mainnet(&mut self) -> &mut Self {
        self.base_url = Some("https://api.hyperliquid.xyz".to_owned());
        self.network = Some(Network::Mainnet);
        self
    }

    #[inline]
    pub fn with_custom_url(&mut self, endpoint: String) -> &mut Self {
        self.base_url = Some(endpoint);
        self
    }

    #[inline]
    pub fn with_wallet(&mut self, signer: PrivateKeySigner) -> &mut Self {
        self.wallet = Some(signer);
        self
    }

    #[inline]
    pub fn build(&self) -> Result<HyperliquidClient, HyperliquidError> {
        let base_url = self
            .base_url
            .clone()
            .ok_or(HyperliquidError::MissingConfiguration {
                parameter: "base_url".to_owned(),
            })?;
        HyperliquidClient::new(base_url)
    }
}
