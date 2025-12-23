# rhyperliquid

A Rust SDK for [Hyperliquid](https://hyperliquid.xyz), the high-performance perpetuals decentralized exchange. Built with type safety in mind and async-first architecture.

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
- Command-line interface for quick queries

## Project Status

- [x] Complete REST Info API (market data)
- [x] EIP-712 authentication & message signing
- [x] Exchange API (order placement, cancellation)
- [x] WebSocket streaming (real-time data feeds)
- [x] CLI for market data and account queries

## Quick Start

### Library Usage
```rust
use rhyperliquid::HyperliquidClient;

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

### CLI Usage

The CLI provides quick access to market data and account information from your terminal.
```bash
# Run CLI commands
cargo run --bin cli --features=cli -- [OPTIONS] <COMMAND>

# Examples:

# Get all mid prices
cargo run --bin cli --features=cli -- all-mids

# Get all mid prices on testnet
cargo run --bin cli --features=cli -- --network testnet all-mids

# Get L2 orderbook for BTC
cargo run --bin cli --features=cli -- l2-book --coin BTC

# Get user's open orders (requires HL_PRIVATE_KEY env var)
export HL_PRIVATE_KEY=your_private_key_here
cargo run --bin cli --features=cli -- --allow-signer-key-env open-orders --user 0x...

# Get candle data
cargo run --bin cli --features=cli -- candle-snapshot \
  --coin ETH \
  --interval 1h \
  --start-time 1640000000000 \
  --end-time 1640100000000

# Get vault details
cargo run --bin cli --features=cli -- vault-details --vault-address 0x...
```

#### CLI Options

**Global Flags:**
- `--network <mainnet|testnet>` - Network to connect to (default: mainnet)
- `--allow-signer-key-env` - Allow reading private key from `HL_PRIVATE_KEY` environment variable

**Available Commands:**
- `all-mids` - Get mid prices for all assets
- `open-orders` - Get user's open orders
- `frontend-open-orders` - Get frontend-formatted open orders
- `user-fills` - Get user's fill history
- `user-fills-by-time` - Get fills within time range
- `user-rate-limit` - Check user's rate limit status
- `order-status` - Get status of specific order
- `l2-book` - Get L2 orderbook snapshot
- `candle-snapshot` - Get historical candle data
- `historical-orders` - Get user's historical orders
- `sub-accounts` - Get user's sub-accounts
- `vault-details` - Get vault information
- `user-vault-equities` - Get user's vault equity
- `user-role` - Get user's role information
- `portfolio` - Get user's portfolio
- `referral` - Get user's referral information
- `user-fees` - Get user's fee information

Use `--help` on any command for detailed parameter information:
```bash
cargo run --bin cli --features=cli -- l2-book --help
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
rhyperliquid = "0.1"
tokio = { version = "1.41", features = ["full"] }
```

### CLI Installation
```bash
# Install from source with CLI support
cargo install --path . --features=cli --bin cli

# Or clone and build
git clone https://github.com/elijahhampton/rhyperliquid.git
cd rhyperliquid
cargo build --release --features=cli --bin cli

# Binary will be at target/release/cli
```

### From Source

Clone and build the repository:
```bash
# Clone the repository
git clone https://github.com/elijahhampton/rhyperliquid.git
cd rhyperliquid

# Build the library
cargo build --release

# Build with CLI
cargo build --release --features=cli

# Run tests
cargo test --all-features

# Build documentation
cargo doc --open
```

## Getting Help
If you have any questions, first see if the answer to your question can be found in the [Hyperliquid Docs](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api).

If the answer is not there:

- Open a discussion with your question, or
- Open an issue with the bug

### Minimum Supported Rust Version (MSRV)

Rust **1.75.0** or higher is required.
```bash
# Verify your Rust version
rustc --version

# Update if needed
rustup update stable
```
