use k256::{
    NonZeroScalar, ProjectivePoint, PublicKey, SecretKey,
    elliptic_curve::{Group, sec1::ToEncodedPoint},
};
use zeroize::Zeroizing;

use super::*;

/// Secp256k1 backend powered by the [`k256`](https://github.com/RustCrypto/elliptic-curves/tree/master/k256) crate.
pub struct K256Backend;

impl Secp256k1PublicKey for PublicKey {
    type Error = BackendError;

    fn from_bytes(bytes: &[u8; 33]) -> Result<Self, Self::Error> {
        PublicKey::from_sec1_bytes(bytes).map_err(BackendError::new)
    }

    fn to_bytes(&self) -> [u8; 33] {
        let encoded = self.to_encoded_point(true);
        let mut out = [0u8; 33];
        out.copy_from_slice(encoded.as_bytes());
        out
    }

    fn add_tweak(&self, tweak: &[u8; 32]) -> Result<Self, Self::Error> {
        let tweak_scalar = Zeroizing::new(nonzero_scalar_from_bytes(tweak)?);
        let parent_point = self.to_projective();

        let child_point = ProjectivePoint::GENERATOR * tweak_scalar.as_ref() + parent_point;
        if child_point.is_identity().into() {
            return Err(BackendError::from("invalid derived public key"));
        }

        let child_affine = child_point.to_affine();
        PublicKey::from_affine(child_affine).map_err(BackendError::new)
    }
}

impl Secp256k1PrivateKey for SecretKey {
    type Error = BackendError;
    type PublicKey = PublicKey;

    fn from_bytes(bytes: &[u8; 32]) -> Result<Self, Self::Error> {
        SecretKey::from_slice(bytes).map_err(BackendError::new)
    }

    fn to_bytes(&self) -> [u8; 32] {
        self.to_bytes().into()
    }

    fn add_tweak(&self, tweak: &[u8; 32]) -> Result<Self, Self::Error> {
        let tweak_scalar = Zeroizing::new(nonzero_scalar_from_bytes(tweak)?);
        let key_scalar = Zeroizing::new(self.to_nonzero_scalar());

        let child = tweak_scalar.as_ref() + key_scalar.as_ref();

        if child.is_zero().into() {
            return Err(BackendError::from("invalid derived private key"));
        }

        SecretKey::from_bytes(&child.to_bytes()).map_err(BackendError::new)
    }

    fn to_public(&self) -> Self::PublicKey {
        self.public_key()
    }

    fn zeroize(&mut self) {
        // `k256::SecretKey` implements `ZeroizeOnDrop`, so `Drop` handles cleanup.
    }
}

impl Secp256k1Backend for K256Backend {
    type PublicKey = PublicKey;
    type PrivateKey = SecretKey;
}

fn nonzero_scalar_from_bytes(bytes: &[u8; 32]) -> Result<NonZeroScalar, &'static str> {
    let bytes = Zeroizing::new(*bytes);
    let scalar = NonZeroScalar::from_repr((*bytes).into());

    Option::<NonZeroScalar>::from(scalar).ok_or("invalid tweak scalar")
}
