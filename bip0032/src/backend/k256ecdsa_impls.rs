use anyhow::{Result, anyhow};
use k256::{
    ProjectivePoint, Scalar,
    ecdsa::{SigningKey, VerifyingKey},
    elliptic_curve::{
        Group, PrimeField,
        sec1::{FromEncodedPoint, ToEncodedPoint},
    },
};

use super::{Secp256k1Backend, Secp256k1PrivateKey, Secp256k1PublicKey};

/// Secp256k1 backend backed by `k256::ecdsa` key types.
#[allow(dead_code)]
pub struct K256EcdsaBackend;

impl Secp256k1PublicKey for VerifyingKey {
    fn from_bytes(bytes: [u8; 33]) -> Result<Self> {
        VerifyingKey::from_sec1_bytes(&bytes).map_err(|_| anyhow!("invalid public key"))
    }

    fn to_bytes(&self) -> [u8; 33] {
        let encoded = self.to_encoded_point(true);
        let mut out = [0u8; 33];
        out.copy_from_slice(encoded.as_bytes());
        out
    }

    fn add_tweak(&self, tweak: [u8; 32]) -> Result<Self> {
        let tweak_scalar = scalar_from_bytes(tweak)?;
        let encoded = self.to_encoded_point(true);
        let parent_point =
            Option::<ProjectivePoint>::from(ProjectivePoint::from_encoded_point(&encoded))
                .ok_or_else(|| anyhow!("invalid parent public key"))?;
        let child_point = ProjectivePoint::GENERATOR * tweak_scalar + parent_point;
        if child_point.is_identity().into() {
            return Err(anyhow!("derived public key is invalid"));
        }
        let child_encoded = child_point.to_affine().to_encoded_point(true);
        VerifyingKey::from_sec1_bytes(child_encoded.as_bytes())
            .map_err(|_| anyhow!("derived public key is invalid"))
    }
}

impl Secp256k1PrivateKey for SigningKey {
    type PublicKey = VerifyingKey;

    fn from_bytes(bytes: [u8; 32]) -> Result<Self> {
        let field_bytes = bytes.into();
        SigningKey::from_bytes(&field_bytes).map_err(|_| anyhow!("invalid private key"))
    }

    fn to_bytes(&self) -> [u8; 32] {
        self.to_bytes().into()
    }

    fn add_tweak(&self, tweak: [u8; 32]) -> Result<Self> {
        let tweak_scalar = scalar_from_bytes(tweak)?;
        let key_bytes = <SigningKey as Secp256k1PrivateKey>::to_bytes(self);
        let key_scalar = scalar_from_bytes(key_bytes)?;
        let child = tweak_scalar + key_scalar;
        if child.is_zero().into() {
            return Err(anyhow!("derived private key is invalid"));
        }
        let child_bytes = child.to_bytes();
        SigningKey::from_bytes(&child_bytes).map_err(|_| anyhow!("derived private key is invalid"))
    }

    fn to_public(&self) -> Self::PublicKey {
        *self.verifying_key()
    }
}

impl Secp256k1Backend for K256EcdsaBackend {
    type PublicKey = VerifyingKey;
    type PrivateKey = SigningKey;
}

fn scalar_from_bytes(bytes: [u8; 32]) -> Result<Scalar> {
    let scalar = Scalar::from_repr(bytes.into());
    Option::<Scalar>::from(scalar).ok_or_else(|| anyhow!("invalid scalar"))
}
