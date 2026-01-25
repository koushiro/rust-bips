//! NIST P-256 curve implementation.

use core::marker::PhantomData;

use super::*;

mod backends;
pub use self::backends::*;

/// A NIST P-256 curve parameterization for a specific backend.
pub struct Nist256p1Curve<B>(PhantomData<B>);

impl<B: Nist256p1Backend> Curve for Nist256p1Curve<B> {
    const HMAC_KEY: &'static [u8] = b"Nist256p1 seed";

    type PublicKey = <B as Nist256p1Backend>::PublicKey;
    type PrivateKey = <B as Nist256p1Backend>::PrivateKey;
}

impl<B: Nist256p1Backend> Slip10Curve for Nist256p1Curve<B> {}

impl<B: Nist256p1Backend> Slip10NonHardenedCurve for Nist256p1Curve<B> {}
