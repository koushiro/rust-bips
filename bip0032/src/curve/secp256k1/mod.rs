//! secp256k1 curve implementation.

use core::marker::PhantomData;

use super::*;

mod backends;
pub use self::backends::*;

/// A secp256k1 curve parameterization for a specific backend.
pub struct Secp256k1Curve<B>(PhantomData<B>);

impl<B: Secp256k1Backend> Curve for Secp256k1Curve<B> {
    const HMAC_KEY: &'static [u8] = b"Bitcoin seed";

    type PublicKey = <B as Secp256k1Backend>::PublicKey;
    type PrivateKey = <B as Secp256k1Backend>::PrivateKey;
}

impl<B: Secp256k1Backend> Bip32Curve for Secp256k1Curve<B> {}

#[cfg(feature = "slip10")]
impl<B: Secp256k1Backend> Slip10Curve for Secp256k1Curve<B> {}

#[cfg(feature = "slip10")]
impl<B: Secp256k1Backend> Slip10NonHardenedCurve for Secp256k1Curve<B> {}
