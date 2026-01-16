use anyhow::{Result, anyhow};
use libsecp256k1::{PublicKey, SecretKey};

use super::{Secp256k1Backend, Secp256k1PrivateKey, Secp256k1PublicKey};

/// Secp256k1 backend powered by the `libsecp256k1` crate.
pub struct Libsecp256k1Backend;

impl Secp256k1PublicKey for PublicKey {
    fn from_bytes(bytes: [u8; 33]) -> Result<Self> {
        PublicKey::parse_slice(&bytes, None).map_err(|_| anyhow!("invalid public key"))
    }

    fn to_bytes(&self) -> [u8; 33] {
        self.serialize_compressed()
    }

    fn add_tweak(&self, tweak: [u8; 32]) -> Result<Self> {
        let tweak_key = SecretKey::parse(&tweak).map_err(|_| anyhow!("invalid scalar"))?;
        let mut out = *self;
        out.tweak_add_assign(&tweak_key)
            .map_err(|_| anyhow!("derived public key is invalid"))?;
        Ok(out)
    }
}

impl Secp256k1PrivateKey for SecretKey {
    type PublicKey = PublicKey;

    fn from_bytes(bytes: [u8; 32]) -> Result<Self> {
        SecretKey::parse(&bytes).map_err(|_| anyhow!("invalid secret key"))
    }

    fn to_bytes(&self) -> [u8; 32] {
        self.serialize()
    }

    fn add_tweak(&self, tweak: [u8; 32]) -> Result<Self> {
        let tweak_key = SecretKey::parse(&tweak).map_err(|_| anyhow!("invalid scalar"))?;
        let mut out = *self;
        out.tweak_add_assign(&tweak_key)
            .map_err(|_| anyhow!("derived secret key is invalid"))?;
        Ok(out)
    }

    fn to_public(&self) -> Self::PublicKey {
        PublicKey::from_secret_key(self)
    }
}

impl Secp256k1Backend for Libsecp256k1Backend {
    type PublicKey = PublicKey;
    type PrivateKey = SecretKey;
}
