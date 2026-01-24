use libsecp256k1::{PublicKey, PublicKeyFormat, SecretKey};

use super::BackendError;
use crate::curve::{CurvePrivateKey, CurvePublicKey, TweakableKey, secp256k1::Secp256k1Backend};

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

impl CurvePublicKey for PublicKey {
    type Error = BackendError;
    type Bytes = [u8; 33];

    fn from_bytes(bytes: &Self::Bytes) -> Result<Self, Self::Error> {
        PublicKey::parse_slice(bytes, Some(PublicKeyFormat::Compressed)).map_err(BackendError::new)
    }

    fn to_bytes(&self) -> Self::Bytes {
        self.serialize_compressed()
    }
}

impl TweakableKey for PublicKey {
    type Error = BackendError;

    fn add_tweak(&self, tweak: &[u8; 32]) -> Result<Self, Self::Error> {
        let tweak_key = SecretKeyGuard::parse(tweak)?;
        let mut out = *self;

        out.tweak_add_assign(tweak_key.as_ref()).map_err(BackendError::new)?;

        Ok(out)
    }
}

impl CurvePrivateKey for SecretKey {
    type Error = BackendError;
    type PublicKey = PublicKey;
    type Bytes = [u8; 32];

    fn from_bytes(bytes: &Self::Bytes) -> Result<Self, Self::Error> {
        SecretKey::parse(bytes).map_err(BackendError::new)
    }

    fn to_bytes(&self) -> Self::Bytes {
        self.serialize()
    }

    fn to_public(&self) -> Self::PublicKey {
        PublicKey::from_secret_key(self)
    }

    fn zeroize(&mut self) {
        self.clear();
    }
}

impl TweakableKey for SecretKey {
    type Error = BackendError;

    fn add_tweak(&self, tweak: &[u8; 32]) -> Result<Self, Self::Error> {
        let tweak_key = SecretKeyGuard::parse(tweak)?;
        let mut out = *self;

        out.tweak_add_assign(tweak_key.as_ref()).map_err(BackendError::new)?;

        Ok(out)
    }
}

impl Secp256k1Backend for Libsecp256k1Backend {
    type PublicKey = PublicKey;
    type PrivateKey = SecretKey;
}
