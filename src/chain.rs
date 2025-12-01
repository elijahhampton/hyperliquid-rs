use alloy::primitives::Address;

/// Returns true only if the string is a valid Hyperliquid user/vault address
pub fn is_valid_hyperliquid_address(s: &str) -> bool {
    // check length and prefix first
    if !s.starts_with("0x") || s.len() != 42 {
        return false;
    }

    // alloy will validate that the remaining 40 chars are valid hex
    // and exactly 20 bytes.
    Address::parse_checksummed(s, None).is_ok()
}
