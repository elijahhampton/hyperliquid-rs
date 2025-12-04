use crate::{
    api::{exchange::ExchangeApi, info::InfoApi},
    client::HyperliquidClientBuilder,
    error::Result,
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
}

impl Inner {
    pub fn new(base_url: String, wallet: Option<PrivateKeySigner>) -> Result<Self> {
        let http_client = ClientBuilder::new().build()?;

        Ok(Self {
            http_client,
            base_url,
            wallet,
        })
    }
}

impl HyperliquidClient {
    pub fn new(base_url: String, wallet: Option<PrivateKeySigner>) -> Result<Self> {
        let inner = Arc::new(Inner::new(base_url, wallet)?);

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

    pub fn info(&self) -> InfoApi<'_> {
        InfoApi::new(self)
    }

    pub fn exchange(&self) -> ExchangeApi<'_> {
        ExchangeApi::new(self)
    }
}
