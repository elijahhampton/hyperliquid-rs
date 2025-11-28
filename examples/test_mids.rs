use hyperliquid_rs::HyperliquidClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HyperliquidClient::new("https://api.hyperliquid.xyz".to_string())?;

    let mids = client.get_all_mids().await?;

    println!("Found all trading pairs!");
    println!("{:?}", mids);

    Ok(())
}
