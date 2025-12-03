use alloy::{
    dyn_abi::Eip712Domain,
    primitives::{keccak256, Address, B256},
    sol_types::eip712_domain,
};

#[allow(dead_code)] // Pending complete exchange API examples and tests
pub trait Eip712 {
    fn domain(&self) -> Eip712Domain;
    fn struct_hash(&self) -> B256;

    fn eip712_signing_hash(&self) -> B256 {
        let mut digest_input = [0u8; 2 + 32 + 32];
        digest_input[0] = 0x19;
        digest_input[1] = 0x01;
        digest_input[2..34].copy_from_slice(&self.domain().hash_struct()[..]);
        digest_input[34..66].copy_from_slice(&self.struct_hash()[..]);
        keccak256(digest_input)
    }
}

#[allow(dead_code)]
pub fn get_l1_domain() -> Eip712Domain {
    eip712_domain! {
        name: "Exchange",
        version: "1",
        chain_id: 1337,
        verifying_contract: Address::ZERO,
    }
}

/// Creates the EIP-712 domain for Hyperliquid user-signed transactions
pub fn get_hyperliquid_domain(chain_id: u64) -> Eip712Domain {
    eip712_domain! {
        name: "HyperliquidSignTransaction",
        version: "1",
        chain_id: chain_id,
        verifying_contract: Address::ZERO,
    }
}
