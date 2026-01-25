//! Ed25519 curve implementation.

use core::marker::PhantomData;

use super::*;

mod backends;
pub use self::backends::*;

/// An Ed25519 curve parameterization for a specific backend.
pub struct Ed25519Curve<B>(PhantomData<B>);

impl<B: Ed25519Backend> Curve for Ed25519Curve<B> {
    const HMAC_KEY: &'static [u8] = b"ed25519 seed";

    type PublicKey = <B as Ed25519Backend>::PublicKey;
    type PrivateKey = <B as Ed25519Backend>::PrivateKey;
}

impl<B: Ed25519Backend> Slip10Curve for Ed25519Curve<B> {}

impl<B: Ed25519Backend> Slip10HardenedOnlyCurve for Ed25519Curve<B> {}
