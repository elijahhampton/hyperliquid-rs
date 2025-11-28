# hyperliquid-rs

An unofficial Hyperliquid sdk written in Rust for [Hyperliquid](https://hyperliquid.xyz) — the high-performance perpetuals DEX.

## Quick Start

```rust
use hyperliquid_rs::HyperliquidClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HyperliquidClient::new();

    // Get all mid prices
    let mids = client.info().get_all_mids().await?;
    println!("BTC: ${}", mids.get("BTC").unwrap());

    Ok(())
}
```

## Installation

```toml
[dependencies]
hyperliquid-rs = "0.1"
```

## Current Status

Phase 2 Current Progress
- [x] Info API (Partially implemented)
- [ ] Authentication & signing (EIP-712)
- [ ] Exchange API
- [ ] WS and Subscriptions


## Documentation

Run `cargo doc --open` for full API documentation.

## License

MIT

---

⚠️ Currently public API only. Not production-ready for trading.
