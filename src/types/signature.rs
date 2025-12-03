use serde::{Deserialize, Serialize};

/// An Ethereum ECDSA signature.
#[derive(Debug, Serialize, Deserialize)]
pub struct Eip712Signature {
    pub r: String,
    pub s: String,
    pub v: u8,
}
