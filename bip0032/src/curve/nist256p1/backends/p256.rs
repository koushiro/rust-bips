use p256::{
    AffinePoint, NonZeroScalar, ProjectivePoint, PublicKey, SecretKey,
    elliptic_curve::sec1::ToEncodedPoint,
};
use zeroize::Zeroizing;

use crate::curve::{
    CurveError, CurvePrivateKey, CurvePublicKey, TweakableKey, nist256p1::Nist256p1Backend,
};

/// NIST P-256 backend powered by the [`p256`](https://github.com/RustCrypto/elliptic-curves/tree/master/p256) crate.
pub struct P256Backend;

impl CurvePublicKey for PublicKey {
    type Error = CurveError;
    type Bytes = [u8; 33];

    fn from_bytes(bytes: &Self::Bytes) -> Result<Self, Self::Error> {
        PublicKey::from_sec1_bytes(bytes).map_err(CurveError::new)
    }

    fn to_bytes(&self) -> Self::Bytes {
        let encoded = self.to_encoded_point(true);
        let mut out = [0u8; 33];
        out.copy_from_slice(encoded.as_bytes());
        out
    }
}

impl TweakableKey for PublicKey {
    type Error = CurveError;

    fn add_tweak(&self, tweak: &[u8; 32]) -> Result<Self, Self::Error> {
        let tweak_scalar = Zeroizing::new(nonzero_scalar_from_bytes(tweak)?);
        let parent_point = self.to_projective();

        let child_point = ProjectivePoint::GENERATOR * tweak_scalar.as_ref() + parent_point;
        let child_affine = AffinePoint::from(child_point);

        PublicKey::from_affine(child_affine).map_err(CurveError::new)
    }
}

impl CurvePrivateKey for SecretKey {
    type Error = CurveError;
    type PublicKey = PublicKey;
    type Bytes = [u8; 32];

    fn from_bytes(bytes: &Self::Bytes) -> Result<Self, Self::Error> {
        SecretKey::from_slice(bytes).map_err(CurveError::new)
    }

    fn to_bytes(&self) -> Self::Bytes {
        self.to_bytes().into()
    }

    fn to_public(&self) -> Self::PublicKey {
        self.public_key()
    }

    fn zeroize(&mut self) {
        // `p256::SecretKey` implements `ZeroizeOnDrop`, so `Drop` handles cleanup.
    }
}

impl TweakableKey for SecretKey {
    type Error = CurveError;

    fn add_tweak(&self, tweak: &[u8; 32]) -> Result<Self, Self::Error> {
        let tweak_scalar = Zeroizing::new(nonzero_scalar_from_bytes(tweak)?);
        let key_scalar = Zeroizing::new(self.to_nonzero_scalar());

        let child = tweak_scalar.as_ref() + key_scalar.as_ref();

        SecretKey::from_bytes(&child.to_bytes()).map_err(CurveError::new)
    }
}

impl Nist256p1Backend for P256Backend {
    type PublicKey = PublicKey;
    type PrivateKey = SecretKey;
}

fn nonzero_scalar_from_bytes(bytes: &[u8; 32]) -> Result<NonZeroScalar, &'static str> {
    let bytes = Zeroizing::new(*bytes);
    let scalar = NonZeroScalar::from_repr((*bytes).into());

    Option::<NonZeroScalar>::from(scalar).ok_or("invalid tweak scalar")
}
