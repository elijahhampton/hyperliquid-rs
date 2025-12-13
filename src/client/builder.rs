use crate::{client::HyperliquidClient, error::HyperliquidError, types::chain::NetworkType};
use alloy::signers::local::PrivateKeySigner;

/// Builder for configuring and produce a [`HyperliquidClient`].
pub struct HyperliquidClientBuilder {
    base_url: Option<String>,
    network: Option<NetworkType>,
    wallet: Option<PrivateKeySigner>,
    ws_endpoint: Option<String>
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
            ws_endpoint: None
        }
    }

    #[inline]
    pub fn testnet(&mut self) -> &mut Self {
        self.base_url = Some("https://api.hyperliquid-testnet.xyz".to_owned());
        self.network = Some(NetworkType::Testnet);
        self
    }

    #[inline]
    pub fn mainnet(&mut self) -> &mut Self {
        self.base_url = Some("https://api.hyperliquid.xyz".to_owned());
        self.network = Some(NetworkType::Mainnet);
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

    pub fn with_subscriptions(&mut self) -> &mut Self {
        match &self.network {
            Some(network) => {
                match network {
                    NetworkType::Mainnet => self.ws_endpoint = Some("wss://api.hyperliquid.xyz/ws".to_string()),
                    NetworkType::Testnet => self.ws_endpoint = Some("wss://api.hyperliquid-testnet.xyz/ws".to_string())
                }
            }
            None =>
             panic!("Builder must first call mainnet() or testnet()")
        }
        self
    }

    #[inline]
    pub fn build(&self) -> Result<HyperliquidClient, HyperliquidError> {
        let base_url = self
            .base_url
            .clone()
            .ok_or(HyperliquidError::MissingConfiguration {
                parameter: "Must create client with mainnet or testnet configuration.".to_owned(),
            })?;

        // Default to Testnet
        let network = self.network.clone().unwrap_or(NetworkType::Testnet);

        HyperliquidClient::new(network, base_url, self.ws_endpoint.clone(), self.wallet.clone())
    }
}
