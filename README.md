# hyperliquid-rs

An unofficial Rust SDK for [Hyperliquid](https://hyperliquid.xyz), the high-performance perpetuals decentralized exchange. Built with type safety in mind and async-first architecture.

⚠️ **Please test all desired operations on Testnet before operating on Mainnet**

## Features

- Market data queries (spot & perpetuals)
- Account information retrieval
- Order book snapshots
- Historical candle data
- User fills and funding history
- Order placement and cancellation
- Position management
- Vault operations

## Project Status

- [x] Complete REST Info API (market data)
- [x] EIP-712 authentication & message signing
- [x] Exchange API (order placement, cancellation)
- [ ] WebSocket streaming (real-time data feeds)
- [ ] CLI

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

## Trading

For order placement and cancellation examples, see [`examples/basic_order.rs`](examples/basic_order.rs).

For account transfer examples, see [`examples/account_transfer.rs`](examples/account_transfer.rs).

For position leverage and managing an isolated position see [`examples/leverage_position.rs`](examples/leverage_position.rs).

For advanced order examples see [`examples/advanced_order.rs`](examples/advanced_order.rs).

For TWAP order placement see [`examples/twap_order.rs`](examples/twap_order.rs).

## Stability and API Guarantees
This crate is under active development.

**Breaking changes** may occur between minor versions (0.1 -> 0.2).

**Public API** is subject to refinement based on usage feedback.

**Testnet testing** is strongly recommended before mainnet use.

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

## Getting Help
If you have any questions, first see if the answer to your question can be found in the [Hyperliquid Docs](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api).

If the answer is not there:

Open a discussion with your question, or
Open an issue with the bug


### Minimum Supported Rust Version (MSRV)

Rust **1.75.0** or higher is required.

```bash
# Verify your Rust version
rustc --version

rustup update stable
```
