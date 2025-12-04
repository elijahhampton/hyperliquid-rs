#[derive(Clone)]
pub enum NetworkType {
    Mainnet,
    Testnet,
}

impl NetworkType {
    pub fn signature_chain_id(&self) -> u64 {
        match self {
            NetworkType::Mainnet => 42161,
            NetworkType::Testnet => 421614,
        }
    }

    pub fn hyperliquid_chain(&self) -> String {
        match self {
            NetworkType::Mainnet => "Mainnet".to_string(),
            NetworkType::Testnet => "Testnet".to_string(),
        }
    }
}
