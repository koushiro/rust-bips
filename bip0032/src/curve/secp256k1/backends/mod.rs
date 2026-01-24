//! Backend implementations for secp256k1 curve.

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
    pub fn new<E>(error: E) -> Self
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
