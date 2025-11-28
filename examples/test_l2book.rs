use hyperliquid_rs::HyperliquidClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HyperliquidClient::new("https://api.hyperliquid.xyz".to_string())?;

    let l2book = client.get_l2_book("BTC".to_string(), None, None).await?;

    println!("Retrieved L2Book!");
    println!("{:?}", l2book);

    Ok(())
}
