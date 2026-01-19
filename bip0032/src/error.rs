//! Error types for bip0032 operations.

#[cfg(not(feature = "std"))]
use alloc::{
    borrow::Cow,
    string::{String, ToString},
    vec::Vec,
};
use core::{error, fmt};
#[cfg(feature = "std")]
use std::borrow::Cow;

/// Result type for bip0032 operations.
pub type Result<T> = core::result::Result<T, Error>;

/// Error categories for bip0032 operations.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    /// Invalid derivation path input.
    InvalidPath,
    /// Invalid extended key payload input.
    InvalidPayload,
    /// Invalid extended key version usage.
    InvalidVersion,
    /// Invalid key material or encoding.
    InvalidKeyData,
    /// Invalid key derivation request or result.
    InvalidDerivation,
}

/// The lower-level source of [`Error`].
///
/// NOTE: we don't implement `core::error::Error` for `ErrorSource`.
pub struct ErrorSource(anyhow::Error);

impl ErrorSource {
    pub(crate) fn as_error(&self) -> &(dyn error::Error + 'static) {
        self.0.as_ref()
    }
}

impl fmt::Debug for ErrorSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for ErrorSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl From<String> for ErrorSource {
    fn from(message: String) -> Self {
        ErrorSource(anyhow::Error::msg(message))
    }
}

impl From<&'static str> for ErrorSource {
    fn from(message: &'static str) -> Self {
        ErrorSource(anyhow::Error::msg(message))
    }
}

/// Conversion trait for attaching source errors.
pub trait IntoErrorSource {
    /// Converts the error into an error source.
    fn into_error_source(self) -> ErrorSource;
}

impl IntoErrorSource for ErrorSource {
    fn into_error_source(self) -> ErrorSource {
        self
    }
}

impl<E> IntoErrorSource for E
where
    E: Into<anyhow::Error>,
{
    fn into_error_source(self) -> ErrorSource {
        ErrorSource(self.into())
    }
}

/// Error type for bip0032 operations.
pub struct Error {
    kind: ErrorKind,
    message: Cow<'static, str>,
    context: Vec<(&'static str, String)>,
    source: Option<ErrorSource>,
}

impl Error {
    /// Creates a new error with the given kind and message.
    pub fn new(kind: ErrorKind, message: impl Into<Cow<'static, str>>) -> Self {
        Self { kind, message: message.into(), context: Vec::new(), source: None }
    }

    /// Returns the error kind.
    pub const fn kind(&self) -> ErrorKind {
        self.kind
    }

    /// Returns the error message.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the error context entries.
    pub fn context(&self) -> &[(&'static str, String)] {
        &self.context
    }

    /// Adds a context entry to the error.
    pub fn with_context(mut self, key: &'static str, value: impl ToString) -> Self {
        self.context.push((key, value.to_string()));
        self
    }

    /// Attaches a error source.
    pub fn set_source(mut self, source: impl IntoErrorSource) -> Self {
        debug_assert!(self.source.is_none(), "source already set");
        self.source = Some(source.into_error_source());
        self
    }

    /// Returns the source error, if any.
    pub fn source(&self) -> Option<&ErrorSource> {
        self.source.as_ref()
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::InvalidPath => f.write_str("InvalidPath"),
            ErrorKind::InvalidPayload => f.write_str("InvalidPayload"),
            ErrorKind::InvalidVersion => f.write_str("InvalidVersion"),
            ErrorKind::InvalidKeyData => f.write_str("InvalidKeyData"),
            ErrorKind::InvalidDerivation => f.write_str("InvalidDerivation"),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.message)?;

        if !self.context.is_empty() {
            f.write_str(" { ")?;
            for (index, (key, value)) in self.context.iter().enumerate() {
                if index > 0 {
                    f.write_str(", ")?;
                }
                write!(f, "{key}={value}")?;
            }
            f.write_str(" }")?;
        }

        Ok(())
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}: {}", self.kind, self.message)?;
        if !self.context.is_empty() {
            writeln!(f, "Context:")?;
            for (key, value) in &self.context {
                writeln!(f, "    {key}: {value}")?;
            }
        }
        if let Some(source) = &self.source {
            writeln!(f, "Source: {source}")?;
        }
        Ok(())
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.source.as_ref().map(|err| err.0.as_ref())
    }
}
