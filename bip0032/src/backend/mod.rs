//! Backend selection for secp256k1 operations.

use anyhow::Result;

/// Byte serialization for secp256k1 public key.
pub trait Secp256k1PublicKey: Clone {
    /// Parses a 33-byte compressed public key representation.
    fn from_bytes(bytes: [u8; 33]) -> Result<Self>;

    /// Returns the 33-byte compressed public key representation.
    fn to_bytes(&self) -> [u8; 33];

    /// Returns a tweaked public key.
    fn add_tweak(&self, tweak: [u8; 32]) -> Result<Self>;
}

/// Byte serialization for secp256k1 private key.
pub trait Secp256k1PrivateKey: Clone {
    /// Corresponding public key type.
    type PublicKey: Secp256k1PublicKey;

    /// Parses a 32-byte private key representation.
    fn from_bytes(bytes: [u8; 32]) -> Result<Self>;

    /// Returns the 32-byte private key representation.
    fn to_bytes(&self) -> [u8; 32];

    /// Returns a tweaked private key.
    fn add_tweak(&self, tweak: [u8; 32]) -> Result<Self>;

    /// Returns the corresponding public key.
    fn to_public(&self) -> Self::PublicKey;
}

/// Secp256k1 backend interface.
pub trait Secp256k1Backend {
    /// Backend-specific public key type.
    type PublicKey: Secp256k1PublicKey;
    /// Backend-specific private key type.
    type PrivateKey: Secp256k1PrivateKey<PublicKey = Self::PublicKey>;
}

/// Backend-specific public key type.
pub(crate) type PublicKey<B> = <B as Secp256k1Backend>::PublicKey;
/// Backend-specific private key type.
pub(crate) type PrivateKey<B> = <B as Secp256k1Backend>::PrivateKey;

#[cfg(feature = "k256")]
mod k256_impls;
#[cfg(feature = "k256ecdsa")]
mod k256ecdsa_impls;
#[cfg(feature = "libsecp256k1")]
mod libsecp256k1_impls;
#[cfg(feature = "secp256k1")]
mod secp256k1_impls;

#[cfg(feature = "k256")]
pub use k256_impls::K256Backend;
#[cfg(feature = "k256ecdsa")]
pub use k256ecdsa_impls::K256EcdsaBackend;
#[cfg(feature = "libsecp256k1")]
pub use libsecp256k1_impls::Libsecp256k1Backend;
#[cfg(feature = "secp256k1")]
pub use secp256k1_impls::Secp256k1CoreBackend;
