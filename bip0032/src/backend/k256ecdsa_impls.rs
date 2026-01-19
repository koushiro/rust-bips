use k256::{
    NonZeroScalar, ProjectivePoint,
    ecdsa::{SigningKey, VerifyingKey},
};
use zeroize::Zeroizing;

use super::*;

/// Secp256k1 backend backed by [`k256::ecdsa`](https://github.com/RustCrypto/elliptic-curves/tree/master/k256) key types.
#[allow(dead_code)]
pub struct K256EcdsaBackend;

impl Secp256k1PublicKey for VerifyingKey {
    type Error = BackendError;

    fn from_bytes(bytes: &[u8; 33]) -> Result<Self, Self::Error> {
        VerifyingKey::from_sec1_bytes(bytes).map_err(BackendError::new)
    }

    fn to_bytes(&self) -> [u8; 33] {
        let encoded = self.to_encoded_point(true);
        let mut out = [0u8; 33];
        out.copy_from_slice(encoded.as_bytes());
        out
    }

    fn add_tweak(&self, tweak: &[u8; 32]) -> Result<Self, Self::Error> {
        let tweak_scalar = Zeroizing::new(nonzero_scalar_from_bytes(tweak)?);
        let parent_point: ProjectivePoint = self.as_affine().into();

        let child_point = ProjectivePoint::GENERATOR * tweak_scalar.as_ref() + parent_point;
        let child_affine = child_point.to_affine();

        VerifyingKey::from_affine(child_affine).map_err(BackendError::new)
    }
}

impl Secp256k1PrivateKey for SigningKey {
    type Error = BackendError;
    type PublicKey = VerifyingKey;

    fn from_bytes(bytes: &[u8; 32]) -> Result<Self, Self::Error> {
        let field_bytes = (*bytes).into();
        SigningKey::from_bytes(&field_bytes).map_err(BackendError::new)
    }

    fn to_bytes(&self) -> [u8; 32] {
        self.to_bytes().into()
    }

    fn add_tweak(&self, tweak: &[u8; 32]) -> Result<Self, Self::Error> {
        let tweak_scalar = Zeroizing::new(nonzero_scalar_from_bytes(tweak)?);
        let key_scalar = Zeroizing::new(*self.as_nonzero_scalar());

        let child = tweak_scalar.as_ref() + key_scalar.as_ref();
        let child_bytes = child.to_bytes();

        SigningKey::from_bytes(&child_bytes).map_err(BackendError::new)
    }

    fn to_public(&self) -> Self::PublicKey {
        *self.verifying_key()
    }

    fn zeroize(&mut self) {
        // `k256::ecdsa::SigningKey` manually implements `Drop`,
        // and clears its own secret state there.
    }
}

impl Secp256k1Backend for K256EcdsaBackend {
    type PublicKey = VerifyingKey;
    type PrivateKey = SigningKey;
}

fn nonzero_scalar_from_bytes(bytes: &[u8; 32]) -> Result<NonZeroScalar, &'static str> {
    let bytes = Zeroizing::new(*bytes);
    let scalar = NonZeroScalar::from_repr((*bytes).into());

    Option::<NonZeroScalar>::from(scalar).ok_or("invalid tweak scalar")
}
