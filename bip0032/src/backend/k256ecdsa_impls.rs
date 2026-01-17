use anyhow::{Result, anyhow};
use k256::{
    NonZeroScalar, ProjectivePoint,
    ecdsa::{SigningKey, VerifyingKey},
    elliptic_curve::Group,
};
use zeroize::Zeroizing;

use super::{Secp256k1Backend, Secp256k1PrivateKey, Secp256k1PublicKey};

/// Secp256k1 backend backed by [`k256::ecdsa`](https://github.com/RustCrypto/elliptic-curves/tree/master/k256) key types.
#[allow(dead_code)]
pub struct K256EcdsaBackend;

impl Secp256k1PublicKey for VerifyingKey {
    fn from_bytes(bytes: &[u8; 33]) -> Result<Self> {
        VerifyingKey::from_sec1_bytes(bytes).map_err(|_| anyhow!("invalid public key"))
    }

    fn to_bytes(&self) -> [u8; 33] {
        let encoded = self.to_encoded_point(true);
        let mut out = [0u8; 33];
        out.copy_from_slice(encoded.as_bytes());
        out
    }

    fn add_tweak(&self, tweak: &[u8; 32]) -> Result<Self> {
        let tweak_scalar = Zeroizing::new(nonzero_scalar_from_bytes(tweak)?);
        let parent_point: ProjectivePoint = self.as_affine().into();

        let child_point = ProjectivePoint::GENERATOR * tweak_scalar.as_ref() + parent_point;
        if child_point.is_identity().into() {
            return Err(anyhow!("derived public key is invalid"));
        }

        let child_affine = child_point.to_affine();
        VerifyingKey::from_affine(child_affine)
            .map_err(|_| anyhow!("derived public key is invalid"))
    }
}

impl Secp256k1PrivateKey for SigningKey {
    type PublicKey = VerifyingKey;

    fn from_bytes(bytes: &[u8; 32]) -> Result<Self> {
        let field_bytes = (*bytes).into();
        SigningKey::from_bytes(&field_bytes).map_err(|_| anyhow!("invalid private key"))
    }

    fn to_bytes(&self) -> [u8; 32] {
        self.to_bytes().into()
    }

    fn add_tweak(&self, tweak: &[u8; 32]) -> Result<Self> {
        let tweak_scalar = Zeroizing::new(nonzero_scalar_from_bytes(tweak)?);
        let key_scalar = Zeroizing::new(*self.as_nonzero_scalar());

        let child = tweak_scalar.as_ref() + key_scalar.as_ref();
        if child.is_zero().into() {
            return Err(anyhow!("derived private key is invalid"));
        }

        let child_bytes = child.to_bytes();
        SigningKey::from_bytes(&child_bytes).map_err(|_| anyhow!("derived private key is invalid"))
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

fn nonzero_scalar_from_bytes(bytes: &[u8; 32]) -> Result<NonZeroScalar> {
    let bytes = Zeroizing::new(*bytes);
    let scalar = NonZeroScalar::from_repr((*bytes).into());

    Option::<NonZeroScalar>::from(scalar).ok_or_else(|| anyhow!("invalid scalar"))
}
