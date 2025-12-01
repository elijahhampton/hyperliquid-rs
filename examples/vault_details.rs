use hyperliquid_rs::{
    helpers::{testnet_client, vault_address},
    init_tracing::init_tracing,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    let hyperliquid = testnet_client()?;
    let vault_address = vault_address();

    let vault_details = hyperliquid
        .info()
        .vault_details(&vault_address, None)
        .await?;
    tracing::info!("{:?}", vault_details);

    Ok(())
}
