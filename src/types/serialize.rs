use crate::api::request_util::normalize_decimal;
use serde::Serializer;

pub fn serialize_decimal<S>(v: &str, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let normalized = normalize_decimal(v);
    s.serialize_str(&normalized)
}

#[allow(clippy::trivially_copy_pass_by_ref)] // Prevent a clone()
pub fn serialize_hex<S>(val: &u64, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&format!("0x{val:x}"))
}
