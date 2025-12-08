use serde::Serializer;

use crate::api::request_util::normalize_decimal;

/// Serializes decimal values represented as strings to a valid format for the Hyperliquid
/// API
pub fn serialize_decimal<S>(v: &str, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let normalized = normalize_decimal(v);
    s.serialize_str(&normalized)
}

/// Serializes a u64 value representing the chain id to hex form
#[allow(clippy::trivially_copy_pass_by_ref)] // Given the size of u64 passing by reference will
                                             // consume the same amount of mem
pub fn serialize_chain_id_as_hex<S>(chain_id: &u64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&format!("0x{:x}", chain_id))
}
