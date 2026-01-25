//! Curve-related error helpers.

#[cfg(not(feature = "std"))]
use alloc::string::String;
use core::{error, fmt};

use crate::error::{ErrorSource, IntoErrorSource};

/// Common curve error type.
pub struct CurveError(ErrorSource);

impl CurveError {
    /// Creates a curve error from a source error.
    #[cfg(feature = "std")]
    pub fn new<E>(error: E) -> Self
    where
        E: IntoErrorSource,
    {
        Self(error.into_error_source())
    }

    /// Creates a curve error from a source error.
    #[cfg(not(feature = "std"))]
    pub fn new<E>(error: E) -> Self
    where
        E: fmt::Display + fmt::Debug + Send + Sync + 'static,
    {
        let error = anyhow::Error::msg(error);
        Self(error.into_error_source())
    }
}

impl fmt::Debug for CurveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for CurveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl From<ErrorSource> for CurveError {
    fn from(error: ErrorSource) -> Self {
        Self(error)
    }
}

impl From<String> for CurveError {
    fn from(message: String) -> Self {
        Self(ErrorSource::from(message))
    }
}

impl From<&'static str> for CurveError {
    fn from(message: &'static str) -> Self {
        Self(ErrorSource::from(message))
    }
}

impl error::Error for CurveError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(self.0.as_error())
    }
}
