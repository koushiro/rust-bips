//! secp256k1 curve support and backend selection.

use core::marker::PhantomData;

use super::*;

mod backends;
pub use self::backends::{BackendError, K256Backend, Libsecp256k1Backend, Secp256k1FfiBackend};

/// Secp256k1 backend interface.
pub trait Secp256k1Backend {
    /// Backend-specific public key type.
    type PublicKey: CurvePublicKey<Bytes = [u8; 33]> + TweakableKey;
    /// Backend-specific private key type.
    type PrivateKey: CurvePrivateKey<Bytes = [u8; 32], PublicKey = Self::PublicKey> + TweakableKey;
}

/// A secp256k1 curve parameterization for a specific backend.
pub struct Secp256k1Curve<B>(PhantomData<B>);

impl<B: Secp256k1Backend> Curve for Secp256k1Curve<B> {
    const HMAC_KEY: &'static [u8] = b"Bitcoin seed";

    type PublicKey = <B as Secp256k1Backend>::PublicKey;
    type PrivateKey = <B as Secp256k1Backend>::PrivateKey;
}

impl<B: Secp256k1Backend> Bip32Curve for Secp256k1Curve<B> {}
