#[derive(Clone)]
pub enum NetworkType {
    Mainnet,
    Testnet,
}

impl NetworkType {
    pub fn signature_chain_id(&self) -> u64 {
        match self {
            Self::Mainnet => 42161,
            Self::Testnet => 421_614,
        }
    }

    pub fn hyperliquid_chain(&self) -> String {
        match self {
            Self::Mainnet => "Mainnet".to_string(),
            Self::Testnet => "Testnet".to_string(),
        }
    }
}
