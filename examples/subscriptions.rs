use hyperliquid_rs::{example_helpers::testnet_client, init_tracing::init_tracing};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    let client = testnet_client()?;
    let mut rx = client
        .subscriptions()
        .await
        .unwrap()
        .subscribe_all_mids(None)
        .await
        .unwrap();

    while let Some(msg) = rx.recv().await {
        tracing::info!("Received new message from 'allMids' subscription.");
        tracing::info!("{:?}", msg);
    }

    Ok(())
}
