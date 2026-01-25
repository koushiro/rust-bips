//! Backend implementations for NIST P-256.

use crate::curve::{CurvePrivateKey, CurvePublicKey, TweakableKey};

/// NIST P-256 backend interface.
pub trait Nist256p1Backend {
    /// Backend-specific public key type.
    type PublicKey: CurvePublicKey<Bytes = [u8; 33]> + TweakableKey;
    /// Backend-specific private key type.
    type PrivateKey: CurvePrivateKey<Bytes = [u8; 32], PublicKey = Self::PublicKey> + TweakableKey;
}

#[cfg(feature = "p256")]
mod p256;

#[cfg(feature = "p256")]
pub use self::p256::P256Backend;
