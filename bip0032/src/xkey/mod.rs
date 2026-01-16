//! Extended key types and shared helpers for BIP32.

use hmac::{Hmac, Mac};
use sha2::Sha512;

mod payload;
mod private;
mod public;

pub use self::{
    payload::{ExtendedKeyPayload, KnownVersion, Version},
    private::ExtendedPrivateKey,
    public::ExtendedPublicKey,
};

/// Common metadata for extended keys (depth, parent link, and chain code).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ExtendedKeyMetadata {
    pub(crate) depth: u8,
    pub(crate) parent_fingerprint: [u8; 4],
    pub(crate) child_number: u32,
    pub(crate) chain_code: [u8; 32],
}

pub(crate) fn key_fingerprint(public_key_bytes: &[u8; 33]) -> [u8; 4] {
    use sha2::Digest;

    // Extended keys can be identified by the Hash160 (RIPEMD160 after SHA256) of
    // the serialized ECDSA public key K, ignoring the chain code.
    let hash = sha2::Sha256::digest(public_key_bytes);
    let identifier = ripemd::Ripemd160::digest(hash);

    // the first 32 bits of the identifier are called the key fingerprint
    let mut out = [0u8; 4];
    out.copy_from_slice(&identifier[..4]);
    out
}

pub(crate) fn hmac_sha512_split(
    key: &[u8],
    update: impl FnOnce(&mut Hmac<Sha512>),
) -> ([u8; 32], [u8; 32]) {
    let mut mac = Hmac::<Sha512>::new_from_slice(key)
        .expect("HMAC-SHA512 must accept the provided key length");

    update(&mut mac);

    let output = mac.finalize().into_bytes();
    debug_assert_eq!(output.len(), 64, "HMAC-SHA512 should produce a 64-byte output");

    let mut left = [0u8; 32];
    let mut right = [0u8; 32];
    left.copy_from_slice(&output[..32]);
    right.copy_from_slice(&output[32..]);
    (left, right)
}
