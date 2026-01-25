//! Ed25519 curve implementation.

use core::marker::PhantomData;

use super::*;

mod backends;
pub use self::backends::*;

/// Converts SLIP-0010 ed25519 public key bytes (0x00 || 32 bytes) into raw bytes.
pub fn ed25519_pubkey_from_slip10_bytes(bytes: &[u8; 33]) -> Result<[u8; 32], CurveError> {
    if bytes[0] != 0x00 {
        return Err(CurveError::from("SLIP-0010 ed25519 public key needs a 0x00 prefix"));
    }

    let mut raw = [0u8; 32];
    raw.copy_from_slice(&bytes[1..]);
    Ok(raw)
}

/// Builds SLIP-0010 ed25519 public key bytes (0x00 || 32 bytes) from raw bytes.
pub fn ed25519_pubkey_to_slip10_bytes(raw: &[u8; 32]) -> [u8; 33] {
    let mut out = [0u8; 33];
    out[1..].copy_from_slice(raw);
    out
}

/// An Ed25519 curve parameterization for a specific backend.
pub struct Ed25519Curve<B>(PhantomData<B>);

impl<B: Ed25519Backend> Curve for Ed25519Curve<B> {
    const HMAC_KEY: &'static [u8] = b"ed25519 seed";

    type PublicKey = <B as Ed25519Backend>::PublicKey;
    type PrivateKey = <B as Ed25519Backend>::PrivateKey;
}

impl<B: Ed25519Backend> Slip10Curve for Ed25519Curve<B> {}

impl<B: Ed25519Backend> Slip10HardenedOnlyCurve for Ed25519Curve<B> {}
