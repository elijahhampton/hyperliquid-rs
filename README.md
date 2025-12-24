# rhyperliquid

## What is rhyperliquid?
rhyperliquid is a production-grade Rust client library for the Hyperliquid decentralized perpetuals exchange. It provides both a programmatic SDK and terminal-based interfaces for traders and developers who demand type-safety, performance, and reliability in their trading infrastructure. rhyperliquid is built with async-first architecture and is licensed under the Apache and MIT licenses.

# Design Principles

As a comprehensive Hyperliquid client, rhyperliquid enables developers to build automated trading strategies, analytics tools, and terminal interfaces while maintaining the safety guarantees and performance characteristics that Rust provides. Building a production-ready trading client requires balancing financial precision, cryptographic correctness, and developer experience with practical performance.

More concretely, our principles are:

1. **Type Safety & Financial Precision**: Every API response, order type, and market data structure is modeled in Rust's type system to catch invalid requests at compile time. We use `rust_decimal` for all financial calculations to eliminate floating-point errors. While network and API errors remain runtime concerns, the library prevents entire classes of invalid requests before they're sent.

2. **Correctness Over Speed**: rhyperliquid prioritizes getting trading operations right. This means proper EIP-712 signing implementation, correct msgpack serialization for action hashing, and careful handling of Hyperliquid's WebSocket heartbeat protocol. We optimize client-side performance where it matters, but understand that network latency to Hyperliquid's servers will dominate round-trip times.

3. **Modularity**: The crate is structured as composable components. Whether you need just the REST client for backtesting, WebSocket streams for live data, or the full CLI for manual trading, you can depend on only what you need. All public APIs are documented with examples showing real-world usage patterns.

4. **Developer Experience**: We believe trading infrastructure should be approachable. The library provides builder patterns for configuration, comprehensive error messages that explain what went wrong, and examples organized by user workflows rather than individual functions. Both library users and CLI users should find the interface intuitive.

5. **Battle-Tested Foundations**: By leveraging proven crates (tokio for async, reqwest for HTTP, Alloy for Ethereum cryptography), we build on solid foundations rather than reinventing implementations. Our authentication follows Hyperliquid's exact specifications for both testnet and mainnet environments.

6. **Open & Extensible**: rhyperliquid is free open source software licensed under Apache/MIT. This enables anyone to build proprietary strategies, modify the client for their needs, or integrate it into larger systems without licensing concerns. We welcome contributions that improve reliability, add features, or enhance documentation.

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
