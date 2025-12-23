# Changelog

## [0.2.0] - 2025-12-XX

### Added
- WebSocket streaming support for real-time market data
- Subscription APIs for orderbook, trades, candles, and user events
- CLI binary for terminal-based queries (`--features=cli`)
- CLI commands for market data and account management
- Network selection via `--network` flag (mainnet/testnet)
- Environment-based authentication for CLI via `HL_PRIVATE_KEY`

### Changed
- Project status: WebSocket and CLI marked as complete

## [0.1.0] - 2025-12-10

### Added
- Complete REST API implementation (Info + Exchange)
- EIP-712 signature support for authentication
- Comprehensive examples for common trading workflows

### Notes
- Initial release
- Testnet and mainnet support
- Pre-1.0: API may have breaking changes
