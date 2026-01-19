use libsecp256k1::{PublicKey, PublicKeyFormat, SecretKey};

use super::*;

/// Secp256k1 backend powered by the [`libsecp256k1`](https://github.com/paritytech/libsecp256k1) crate.
///
/// NOTE: the crate is no longer maintained.
pub struct Libsecp256k1Backend;

// RAII guard to erase secret keys derived from tweak bytes.
struct SecretKeyGuard(SecretKey);

impl SecretKeyGuard {
    fn parse(bytes: &[u8; 32]) -> Result<Self, BackendError> {
        SecretKey::parse(bytes).map(Self).map_err(BackendError::new)
    }
}

impl AsRef<SecretKey> for SecretKeyGuard {
    fn as_ref(&self) -> &SecretKey {
        &self.0
    }
}

impl Drop for SecretKeyGuard {
    fn drop(&mut self) {
        self.0.clear();
    }
}

impl Secp256k1PublicKey for PublicKey {
    type Error = BackendError;

    fn from_bytes(bytes: &[u8; 33]) -> Result<Self, Self::Error> {
        PublicKey::parse_slice(bytes, Some(PublicKeyFormat::Compressed)).map_err(BackendError::new)
    }

    fn to_bytes(&self) -> [u8; 33] {
        self.serialize_compressed()
    }

    fn add_tweak(&self, tweak: &[u8; 32]) -> Result<Self, Self::Error> {
        let tweak_key = SecretKeyGuard::parse(tweak)?;
        let mut out = *self;

        out.tweak_add_assign(tweak_key.as_ref()).map_err(BackendError::new)?;

        Ok(out)
    }
}

impl Secp256k1PrivateKey for SecretKey {
    type Error = BackendError;
    type PublicKey = PublicKey;

    fn from_bytes(bytes: &[u8; 32]) -> Result<Self, Self::Error> {
        SecretKey::parse(bytes).map_err(BackendError::new)
    }

    fn to_bytes(&self) -> [u8; 32] {
        self.serialize()
    }

    fn add_tweak(&self, tweak: &[u8; 32]) -> Result<Self, Self::Error> {
        let tweak_key = SecretKeyGuard::parse(tweak)?;
        let mut out = *self;

        out.tweak_add_assign(tweak_key.as_ref()).map_err(BackendError::new)?;

        Ok(out)
    }

    fn to_public(&self) -> Self::PublicKey {
        PublicKey::from_secret_key(self)
    }

    fn zeroize(&mut self) {
        self.clear();
    }
}

impl Secp256k1Backend for Libsecp256k1Backend {
    type PublicKey = PublicKey;
    type PrivateKey = SecretKey;
}
