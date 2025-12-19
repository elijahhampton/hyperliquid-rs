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

pub(crate) mod decimal_array {
    use rust_decimal::Decimal;
    use serde::{Deserialize, Deserializer, Serializer};

    /// Serializer for a fixed array of length(2)
    pub fn serialize<S>(value: &[Decimal; 2], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeTuple;
        let mut seq = serializer.serialize_tuple(2)?;
        seq.serialize_element(&value[0].to_string())?;
        seq.serialize_element(&value[1].to_string())?;
        seq.end()
    }

    /// Deserializer for a fixed array of length(2)
    pub fn deserialize<'de, D>(deserializer: D) -> Result<[Decimal; 2], D::Error>
    where
        D: Deserializer<'de>,
    {
        let strings: [String; 2] = Deserialize::deserialize(deserializer)?;
        Ok([
            strings[0].parse().map_err(serde::de::Error::custom)?,
            strings[1].parse().map_err(serde::de::Error::custom)?,
        ])
    }
}
