use anyhow::{Result, anyhow};
use k256::{
    ProjectivePoint, PublicKey, Scalar, SecretKey,
    elliptic_curve::{
        Group, PrimeField,
        sec1::{FromEncodedPoint, ToEncodedPoint},
    },
};

use super::*;

/// Secp256k1 backend powered by the k256 crate.
pub struct K256Backend;

impl Secp256k1PublicKey for PublicKey {
    fn from_bytes(bytes: [u8; 33]) -> Result<Self> {
        PublicKey::from_sec1_bytes(&bytes).map_err(|_| anyhow!("invalid public key"))
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
        PublicKey::from_sec1_bytes(child_encoded.as_bytes())
            .map_err(|_| anyhow!("derived public key is invalid"))
    }
}

impl Secp256k1PrivateKey for SecretKey {
    type PublicKey = PublicKey;

    fn from_bytes(bytes: [u8; 32]) -> Result<Self> {
        SecretKey::from_slice(&bytes).map_err(|_| anyhow!("invalid secret key"))
    }

    fn to_bytes(&self) -> [u8; 32] {
        self.to_bytes().into()
    }

    fn add_tweak(&self, tweak: [u8; 32]) -> Result<Self> {
        let tweak_scalar = scalar_from_bytes(tweak)?;
        let key_bytes = <SecretKey as Secp256k1PrivateKey>::to_bytes(self);
        let key_scalar = scalar_from_bytes(key_bytes)?;
        let child = tweak_scalar + key_scalar;
        if child.is_zero().into() {
            return Err(anyhow!("derived secret key is invalid"));
        }
        SecretKey::from_slice(child.to_bytes().as_ref())
            .map_err(|_| anyhow!("derived secret key is invalid"))
    }

    fn to_public(&self) -> Self::PublicKey {
        self.public_key()
    }
}

impl Secp256k1Backend for K256Backend {
    type PublicKey = PublicKey;
    type PrivateKey = SecretKey;
}

fn scalar_from_bytes(bytes: [u8; 32]) -> Result<Scalar> {
    let scalar = Scalar::from_repr(bytes.into());
    Option::<Scalar>::from(scalar).ok_or_else(|| anyhow!("invalid scalar"))
}
