# hyperliquid-rs

A unofficial Rust SDK for [Hyperliquid](https://hyperliquid.xyz), the high-performance perpetuals decentralized exchange. Built with type safety in mind and async-first architecture.

⚠️ **Not suitable for production trading. This SDK is still under development.**

## Project Status

- [x] Complete REST Info API (market data)
- [x] EIP-712 authentication & message signing
- [x] Exchange API (order placement, cancellation)
- [ ] WebSocket streaming (real-time data feeds)
- [ ] CLI application

**Not suitable for production trading.** API surface and semantics subject to change before 1.0 release.

## Quick Start

```rust
use hyperliquid_rs::HyperliquidClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HyperliquidClient::builder()
        .testnet()
        .build()?;

    // Get real-time mid prices for all assets
    let mids = client.info().get_all_mids().await?;
    println!("BTC: ${}", mids.get("BTC").unwrap());

    // Fetch L2 orderbook snapshot
    let book = client.info().get_l2_book("ETH").await?;
    println!("Best bid: ${}", book.levels[0].px);

    Ok(())
}
```

## Installation

### As a Library Dependency

Add to your `Cargo.toml`:

```toml
[dependencies]
hyperliquid-rs = "0.1"
tokio = { version = "1.41", features = ["full"] }
```

### From Source

Clone and build the repository:

```bash
# Clone the repository
git clone https://github.com/elijahhampton/hyperliquid-rs.git
cd hyperliquid-rs

# Build the library
cargo build --release

# Run tests
cargo test --all-features

# Build documentation
cargo doc --open
```

### Minimum Supported Rust Version (MSRV)

Rust **1.75.0** or higher is required. This project uses Rust 2021 edition features.

```bash
# Verify your Rust version
rustc --version

rustup update stable
```

### Async-First Architecture

Built on `tokio` for high-concurrency workloads:

```rust
// Concurrent requests with structured concurrency
let (mids, meta, book) = tokio::join!(
    client.info().get_all_mids(),
    client.info().get_meta(),
    client.info().get_l2_book("BTC"),
);
```
