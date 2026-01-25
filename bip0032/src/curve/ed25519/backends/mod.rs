//! Backend implementations for Ed25519.

use crate::curve::{CurvePrivateKey, CurvePublicKey};

/// Ed25519 backend interface.
pub trait Ed25519Backend {
    /// Backend-specific public key type.
    type PublicKey: CurvePublicKey<Bytes = [u8; 33]>;
    /// Backend-specific private key type.
    type PrivateKey: CurvePrivateKey<Bytes = [u8; 32], PublicKey = Self::PublicKey>;
}

#[cfg(feature = "ed25519-dalek")]
mod ed25519_dalek;

#[cfg(feature = "ed25519-dalek")]
pub use self::ed25519_dalek::Ed25519DalekBackend;
