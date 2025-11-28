use std::sync::Arc;
use crate::{api::info::InfoApi, error::{HyperliquidError, Result}};
use reqwest::{Client, ClientBuilder};

#[derive(Clone)]
pub struct HyperliquidClient {
    inner: Arc<Inner>
}

pub struct Inner {
    http_client: Client,
    base_url: String,
}

impl Inner {
    pub fn new(base_url: String) -> Result<Self> {
         let http_client = ClientBuilder::new().build()?;

         Ok(Self {
            http_client,
            base_url
         })
    }
}

impl HyperliquidClient {
    pub fn new(base_url: String) -> Result<Self> {
        let inner = Arc::new(Inner::new(base_url)?);

        Ok(Self { inner })
    }

    pub fn http_client(&self) -> &Client {
        &self.inner.http_client
    }

    pub fn base_url(&self) -> &str {
        &self.inner.base_url
    }

    pub fn info(&self) -> InfoApi<'_> {
        InfoApi::new(self)
    }
}
