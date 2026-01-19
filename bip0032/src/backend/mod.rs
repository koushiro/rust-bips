//! Backend selection for secp256k1 operations.

#[cfg(not(feature = "std"))]
use alloc::string::String;
use core::{error, fmt};

use crate::error::{ErrorSource, IntoErrorSource};

/// Common backend error.
pub struct BackendError(ErrorSource);

impl BackendError {
    /// Creates a backend error from a source error.
    #[cfg(feature = "std")]
    pub fn new<E>(error: E) -> Self
    where
        E: IntoErrorSource,
    {
        Self(error.into_error_source())
    }

    /// Creates a backend error from a source error.
    #[cfg(not(feature = "std"))]
    pub fn new<E>(error: E) -> BackendError
    where
        E: fmt::Display + fmt::Debug + Send + Sync + 'static,
    {
        let error = anyhow::Error::msg(error);
        Self(error.into_error_source())
    }
}

impl fmt::Debug for BackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for BackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl From<ErrorSource> for BackendError {
    fn from(error: ErrorSource) -> Self {
        Self(error)
    }
}

impl From<String> for BackendError {
    fn from(message: String) -> Self {
        Self(ErrorSource::from(message))
    }
}

impl From<&'static str> for BackendError {
    fn from(message: &'static str) -> Self {
        Self(ErrorSource::from(message))
    }
}

impl error::Error for BackendError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(self.0.as_error())
    }
}

/// Byte serialization for secp256k1 public key.
pub trait Secp256k1PublicKey: Clone {
    /// Backend-specific error type.
    type Error: IntoErrorSource + Send + Sync + 'static;

    /// Parses a 33-byte compressed public key representation.
    fn from_bytes(bytes: &[u8; 33]) -> core::result::Result<Self, Self::Error>;

    /// Returns the 33-byte compressed public key representation.
    fn to_bytes(&self) -> [u8; 33];

    /// Returns a tweaked public key.
    fn add_tweak(&self, tweak: &[u8; 32]) -> core::result::Result<Self, Self::Error>;
}

/// Byte serialization for secp256k1 private key.
pub trait Secp256k1PrivateKey: Clone {
    /// Backend-specific error type.
    type Error: IntoErrorSource + Send + Sync + 'static;

    /// Corresponding public key type.
    type PublicKey: Secp256k1PublicKey<Error = Self::Error>;

    /// Parses a 32-byte private key representation.
    fn from_bytes(bytes: &[u8; 32]) -> core::result::Result<Self, Self::Error>;

    /// Returns the 32-byte private key representation.
    fn to_bytes(&self) -> [u8; 32];

    /// Returns a tweaked private key.
    fn add_tweak(&self, tweak: &[u8; 32]) -> core::result::Result<Self, Self::Error>;

    /// Returns the corresponding public key.
    fn to_public(&self) -> Self::PublicKey;

    /// Zeroizes this private key if possible.
    fn zeroize(&mut self) {}
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
pub use self::k256_impls::K256Backend;
#[cfg(feature = "k256ecdsa")]
pub use self::k256ecdsa_impls::K256EcdsaBackend;
#[cfg(feature = "libsecp256k1")]
pub use self::libsecp256k1_impls::Libsecp256k1Backend;
#[cfg(feature = "secp256k1")]
pub use self::secp256k1_impls::Secp256k1FfiBackend;
