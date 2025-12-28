use alloy::{
    primitives::{keccak256, Address, B256},
    signers::{local::PrivateKeySigner, Signature, SignerSync},
    sol_types::{eip712_domain, SolStruct},
};
use serde::Serialize;

use crate::{
    error::{HyperliquidError, Result},
    signature::{
        agent::l1,
        eip712::{get_hyperliquid_domain, Eip712},
    },
    types::signature::Eip712Signature,
};

/// Signs a user-signed action
#[allow(dead_code)] // Pending complete exchange API examples and tests
pub fn sign_user_signed_action<T>(
    wallet: &PrivateKeySigner,
    action: &T,
    _is_mainnet: bool,
) -> Result<Eip712Signature>
where
    T: SolStruct,
{
    let signature_chain_id = 0x66eee_u64; // Arbitrum

    let domain = get_hyperliquid_domain(signature_chain_id);

    let signing_hash = action.eip712_signing_hash(&domain);
    let signature = wallet.sign_hash_sync(&signing_hash)?;

    // Convert y_parity bool to v byte (27 or 28 for legacy, or 0/1 for modern)
    let v = sig_v_from_bool(signature.v());

    Ok(Eip712Signature {
        r: format!("0x{:x}", signature.r()),
        s: format!("0x{:x}", signature.s()),
        v,
    })
}

#[allow(dead_code)] // Pending complete exchange API examples and tests
pub fn sign_l1_action(
    wallet: &PrivateKeySigner,
    action: &impl Serialize,
    vault_address: Option<String>,
    nonce: u64,
    expires_after: Option<u64>,
    is_mainnet: bool,
) -> Result<Signature> {
    use alloy::sol_types::SolStruct;

    let connection_id = action_hash(action, vault_address, nonce, expires_after)?;

    let source = if is_mainnet { "a" } else { "b" };
    let phantom_agent = l1::Agent {
        source: source.to_string(),
        connectionId: connection_id,
    };

    let struct_hash = phantom_agent.eip712_hash_struct();

    let domain = eip712_domain! {
        name: "Exchange",
        version: "1",
        chain_id: 1337,
        verifying_contract: Address::ZERO,
    };
    let domain_hash = domain.hash_struct();

    let mut digest_input = [0u8; 66];
    digest_input[0] = 0x19;
    digest_input[1] = 0x01;
    digest_input[2..34].copy_from_slice(&domain_hash[..]);
    digest_input[34..66].copy_from_slice(&struct_hash[..]);

    let signing_hash = keccak256(digest_input);
    let sig = wallet.sign_hash_sync(&signing_hash)?;

    let recovered = sig.recover_address_from_prehash(&signing_hash)?;
    if recovered != wallet.address() {
        return Err(HyperliquidError::SignatureFailure(format!(
            "Signature verification failed: recovered {:?} but expected {:?}",
            recovered,
            wallet.address()
        )));
    }

    tracing::trace!(
        "Signature r: {:?}, s: {:?}, v: {}",
        sig.r(),
        sig.s(),
        sig.v()
    );

    Ok(sig)
}

/// Computes the Hyperliquid L1 action hash.
///
/// The hash is constructed as:
/// 1. The msgpack-serialized action (named fields).
/// 2. The 8-byte BIG-ENDIAN nonce.
/// 3. A vault flag byte:
/// - `0x01` followed by the 20-byte vault address if provided.
/// - `0x00` if no vault address.
/// 4. Optional expiry:
///    - `0x00` followed by the 8-byte big-endian expiry timestamp if provided.
///
/// The concatenated bytes are then hashed with Keccak-256.
///
/// Returns the resulting `B256` hash.
fn action_hash(
    action: &impl Serialize,
    vault_address: Option<String>,
    nonce: u64,
    expires_after: Option<u64>,
) -> Result<B256> {
    let mut data = rmp_serde::to_vec_named(action)?;

    data.extend_from_slice(&nonce.to_be_bytes());

    if let Some(addr) = vault_address {
        data.push(0x01);
        data.extend_from_slice(addr.as_bytes());
    } else {
        data.push(0x00);
    }

    if let Some(expires) = expires_after {
        data.push(0x00);
        data.extend_from_slice(&expires.to_be_bytes());
    }

    let hash = keccak256(&data);

    Ok(hash)
}

/// Sign types data using the wallets private key.
#[allow(dead_code)] // Pending complete exchange API examples and tests
pub fn sign_typed_data<T: Eip712>(payload: &T, wallet: &PrivateKeySigner) -> Result<Signature> {
    wallet
        .sign_hash_sync(&payload.eip712_signing_hash())
        .map_err(|e| HyperliquidError::SignatureFailure(e.to_string()))
}

/// Returns 27/28 for ECDSA Signature `v` field based on a boolean value.
pub fn sig_v_from_bool(v: bool) -> u8 {
    if v {
        28
    } else {
        27
    }
}
