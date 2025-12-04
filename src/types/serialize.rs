use crate::api::request_util::normalize_decimal;

pub fn serialize_decimal<S>(v: &str, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let normalized = normalize_decimal(v);
    s.serialize_str(&normalized)
}
