use crate::{
    api::{exchange::ExchangeApi, info::InfoApi},
    client::HyperliquidClientBuilder,
    error::Result,
    types::chain::NetworkType,
};
use alloy::signers::local::PrivateKeySigner;
use reqwest::{Client, ClientBuilder};
use std::sync::Arc;

#[derive(Clone)]
pub struct HyperliquidClient {
    inner: Arc<Inner>,
}

pub struct Inner {
    http_client: Client,
    base_url: String,
    wallet: Option<PrivateKeySigner>,
    network: NetworkType,
}

impl Inner {
    pub fn new(
        network: NetworkType,
        base_url: String,
        wallet: Option<PrivateKeySigner>,
    ) -> Result<Self> {
        let http_client = ClientBuilder::new().build()?;

        Ok(Self {
            http_client,
            base_url,
            wallet,
            network,
        })
    }
}

impl HyperliquidClient {
    pub fn new(
        network: NetworkType,
        base_url: String,
        wallet: Option<PrivateKeySigner>,
    ) -> Result<Self> {
        let inner = Arc::new(Inner::new(network, base_url, wallet)?);

        Ok(Self { inner })
    }

    pub fn builder() -> HyperliquidClientBuilder {
        HyperliquidClientBuilder::new()
    }

    pub fn http_client(&self) -> &Client {
        &self.inner.http_client
    }

    pub fn base_url(&self) -> &str {
        &self.inner.base_url
    }

    pub fn signer(&self) -> Option<&PrivateKeySigner> {
        self.inner.wallet.as_ref()
    }

    pub fn network_type(&self) -> &NetworkType {
        &self.inner.network
    }

    pub fn info(&self) -> InfoApi<'_> {
        InfoApi::new(self)
    }

    pub fn exchange(&self) -> ExchangeApi<'_> {
        ExchangeApi::new(self)
    }

    pub fn is_mainnet(&self) -> bool {
        match self.inner.network {
            NetworkType::Mainnet => true,
            NetworkType::Testnet => false,
        }
    }
}
