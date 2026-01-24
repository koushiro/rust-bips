//! Curve abstractions and specific curve implementations.

use crate::error::IntoErrorSource;

/// Curve parameters for HD derivation.
pub trait Curve {
    /// HMAC key for master key derivation.
    const HMAC_KEY: &'static [u8];

    /// Curve public key type.
    type PublicKey: CurvePublicKey;
    /// Curve private key type.
    type PrivateKey: CurvePrivateKey<PublicKey = Self::PublicKey>;
}

/// Public key capabilities for a curve.
pub trait CurvePublicKey: Clone {
    /// Backend-specific error type.
    type Error: IntoErrorSource + Send + Sync + 'static;
    /// Serialized key representation.
    type Bytes: AsRef<[u8]> + Clone;

    /// Parses a public key from bytes.
    fn from_bytes(bytes: &Self::Bytes) -> Result<Self, Self::Error>;

    /// Serializes a public key to bytes.
    fn to_bytes(&self) -> Self::Bytes;
}

/// Private key capabilities for a curve.
pub trait CurvePrivateKey: Clone {
    /// Backend-specific error type.
    type Error: IntoErrorSource + Send + Sync + 'static;
    /// Corresponding public key type.
    type PublicKey: CurvePublicKey<Error = Self::Error>;
    /// Serialized key representation.
    type Bytes: AsRef<[u8]> + Clone;

    /// Parses a private key from bytes.
    fn from_bytes(bytes: &Self::Bytes) -> Result<Self, Self::Error>;

    /// Serializes a private key to bytes.
    fn to_bytes(&self) -> Self::Bytes;

    /// Returns the corresponding public key.
    fn to_public(&self) -> Self::PublicKey;

    /// Zeroizes this private key if possible.
    fn zeroize(&mut self) {}
}

/// Keys that support tweak addition.
pub trait TweakableKey: Sized {
    /// Backend-specific error type.
    type Error: IntoErrorSource + Send + Sync + 'static;

    /// Returns a tweaked key.
    fn add_tweak(&self, tweak: &[u8; 32]) -> Result<Self, Self::Error>;
}

pub mod secp256k1;

/// Marker trait for BIP32-encodable curves.
pub trait Bip32Curve: Curve {}
