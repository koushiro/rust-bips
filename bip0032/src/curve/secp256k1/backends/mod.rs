//! Backend implementations for secp256k1 curve.

use crate::curve::{CurvePrivateKey, CurvePublicKey, TweakableKey};

/// Secp256k1 backend interface.
pub trait Secp256k1Backend {
    /// Backend-specific public key type.
    type PublicKey: CurvePublicKey<Bytes = [u8; 33]> + TweakableKey;
    /// Backend-specific private key type.
    type PrivateKey: CurvePrivateKey<Bytes = [u8; 32], PublicKey = Self::PublicKey> + TweakableKey;
}

#[cfg(feature = "k256")]
mod k256;
#[cfg(feature = "libsecp256k1")]
mod libsecp256k1;
#[cfg(feature = "secp256k1")]
mod secp256k1;

#[cfg(feature = "k256")]
pub use self::k256::K256Backend;
#[cfg(feature = "libsecp256k1")]
pub use self::libsecp256k1::Libsecp256k1Backend;
#[cfg(feature = "secp256k1")]
pub use self::secp256k1::Secp256k1FfiBackend;
