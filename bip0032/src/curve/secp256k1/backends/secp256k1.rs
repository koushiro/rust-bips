use secp256k1::{PublicKey, Scalar, SecretKey};
use zeroize::Zeroizing;

use crate::curve::{
    CurveError, CurvePrivateKey, CurvePublicKey, TweakableKey, secp256k1::Secp256k1Backend,
};

/// Secp256k1 FFI backend powered by the [`secp256k1`](https://github.com/rust-bitcoin/rust-secp256k1) crate.
pub struct Secp256k1FfiBackend;

// RAII guard to erase scalar material on drop.
struct ScalarGuard(Scalar);

impl ScalarGuard {
    fn from_bytes(bytes: &[u8; 32]) -> Result<Self, CurveError> {
        let bytes = Zeroizing::new(*bytes);
        Scalar::from_be_bytes(*bytes).map(Self).map_err(CurveError::new)
    }
}

impl AsRef<Scalar> for ScalarGuard {
    fn as_ref(&self) -> &Scalar {
        &self.0
    }
}

impl Drop for ScalarGuard {
    fn drop(&mut self) {
        self.0.non_secure_erase();
    }
}

impl CurvePublicKey for PublicKey {
    type Error = CurveError;
    type Bytes = [u8; 33];

    fn from_bytes(bytes: &Self::Bytes) -> Result<Self, Self::Error> {
        PublicKey::from_byte_array_compressed(*bytes).map_err(CurveError::new)
    }

    fn to_bytes(&self) -> Self::Bytes {
        self.serialize()
    }
}

impl TweakableKey for PublicKey {
    type Error = CurveError;

    fn add_tweak(&self, tweak: &[u8; 32]) -> Result<Self, Self::Error> {
        let scalar = ScalarGuard::from_bytes(tweak)?;

        (*self).add_exp_tweak(scalar.as_ref()).map_err(CurveError::new)
    }
}

impl CurvePrivateKey for SecretKey {
    type Error = CurveError;
    type PublicKey = PublicKey;
    type Bytes = [u8; 32];

    fn from_bytes(bytes: &Self::Bytes) -> Result<Self, Self::Error> {
        SecretKey::from_secret_bytes(*bytes).map_err(CurveError::new)
    }

    fn to_bytes(&self) -> Self::Bytes {
        self.to_secret_bytes()
    }

    fn to_public(&self) -> Self::PublicKey {
        PublicKey::from_secret_key(self)
    }

    fn zeroize(&mut self) {
        self.non_secure_erase();
    }
}

impl TweakableKey for SecretKey {
    type Error = CurveError;

    fn add_tweak(&self, tweak: &[u8; 32]) -> Result<Self, Self::Error> {
        let scalar = ScalarGuard::from_bytes(tweak)?;

        (*self).add_tweak(scalar.as_ref()).map_err(CurveError::new)
    }
}

impl Secp256k1Backend for Secp256k1FfiBackend {
    type PublicKey = PublicKey;
    type PrivateKey = SecretKey;
}
