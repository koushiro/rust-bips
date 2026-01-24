//! Derivation path parsing and child number handling.

#[cfg(not(feature = "std"))]
use alloc::{vec, vec::Vec};
use core::{fmt, slice, str::FromStr};
#[cfg(feature = "std")]
use std::vec;

use crate::{Error, ErrorKind, Result};

mod child;

pub use child::{ChildNumber, HardenedChildNumber};

/// A parsed BIP32 derivation path.
///
/// This type supports parsing paths with or without a leading `m/` prefix.
/// The root path can be written as `m` or `M`. Hardened components accept
/// the `'`, `h`, or `H` suffix and display as `'`.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DerivationPath {
    children: Vec<ChildNumber>,
}

impl DerivationPath {
    /// Returns the child numbers in this path.
    pub fn children(&self) -> &[ChildNumber] {
        &self.children
    }
}

impl AsRef<[ChildNumber]> for DerivationPath {
    fn as_ref(&self) -> &[ChildNumber] {
        &self.children
    }
}

impl From<Vec<ChildNumber>> for DerivationPath {
    fn from(children: Vec<ChildNumber>) -> Self {
        Self { children }
    }
}

impl FromIterator<ChildNumber> for DerivationPath {
    fn from_iter<T: IntoIterator<Item = ChildNumber>>(iter: T) -> Self {
        Self { children: iter.into_iter().collect() }
    }
}

impl Extend<ChildNumber> for DerivationPath {
    fn extend<T: IntoIterator<Item = ChildNumber>>(&mut self, iter: T) {
        self.children.extend(iter);
    }
}

impl IntoIterator for DerivationPath {
    type Item = ChildNumber;
    type IntoIter = vec::IntoIter<ChildNumber>;

    fn into_iter(self) -> Self::IntoIter {
        self.children.into_iter()
    }
}

impl<'a> IntoIterator for &'a DerivationPath {
    type Item = &'a ChildNumber;
    type IntoIter = slice::Iter<'a, ChildNumber>;

    fn into_iter(self) -> Self::IntoIter {
        self.children.iter()
    }
}

impl fmt::Display for DerivationPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("m")?;

        for child in &self.children {
            write!(f, "/{child}")?;
        }

        Ok(())
    }
}

impl FromStr for DerivationPath {
    type Err = Error;

    fn from_str(path: &str) -> Result<Self> {
        if path.is_empty() {
            return Err(Error::new(ErrorKind::InvalidPath, "derivation path is empty")
                .with_context("path", path));
        }

        if path == "m" || path == "M" {
            return Ok(Self::default());
        }

        let rest = if let Some(stripped) = path.strip_prefix("m/") {
            stripped
        } else if let Some(stripped) = path.strip_prefix("M/") {
            stripped
        } else {
            path
        };

        if rest.is_empty() {
            return Err(Error::new(ErrorKind::InvalidPath, "empty path component")
                .with_context("path", path));
        }

        let mut children =
            Vec::with_capacity(rest.as_bytes().iter().filter(|&&b| b == b'/').count() + 1);

        for part in rest.split('/') {
            if part.is_empty() {
                return Err(Error::new(ErrorKind::InvalidPath, "empty path component")
                    .with_context("path", path));
            }
            children.push(part.parse()?);
        }

        Ok(Self { children })
    }
}

impl From<HardenedDerivationPath> for DerivationPath {
    fn from(value: HardenedDerivationPath) -> Self {
        value.inner
    }
}

/// A derivation path containing only hardened components.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct HardenedDerivationPath {
    inner: DerivationPath,
}

impl HardenedDerivationPath {
    /// Returns the child numbers in this path.
    pub fn children(&self) -> impl Iterator<Item = HardenedChildNumber> + '_ {
        self.inner
            .children()
            .iter()
            .copied()
            .map(HardenedChildNumber::from_child_unchecked)
    }
}

impl fmt::Display for HardenedDerivationPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl FromStr for HardenedDerivationPath {
    type Err = Error;

    fn from_str(path: &str) -> Result<Self> {
        let path = path.parse::<DerivationPath>()?;
        Self::try_from(path)
    }
}

impl TryFrom<DerivationPath> for HardenedDerivationPath {
    type Error = Error;

    fn try_from(value: DerivationPath) -> Result<Self> {
        if let Some(child) = value.children().iter().find(|child| !child.is_hardened()) {
            return Err(Error::new(ErrorKind::InvalidPath, "expected hardened derivation path")
                .with_context("child_index", child.index()));
        }

        Ok(Self { inner: value })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_root() {
        let path = "m".parse::<DerivationPath>().unwrap();
        assert!(path.children().is_empty());
        assert_eq!(path.to_string(), "m");
    }

    #[test]
    fn parse_uppercase_prefix() {
        let path = "M/0".parse::<DerivationPath>().unwrap();
        assert_eq!(path.to_string(), "m/0");
    }

    #[test]
    fn parse_without_prefix() {
        let path = "0/1".parse::<DerivationPath>().unwrap();
        assert_eq!(path.to_string(), "m/0/1");
    }

    #[test]
    fn parse_hardened_suffixes() {
        let path = "m/0'/1h/2H".parse::<DerivationPath>().unwrap();
        let children = path.children();
        assert_eq!(children.len(), 3);
        assert!(children[0].is_hardened());
        assert!(children[1].is_hardened());
        assert!(children[2].is_hardened());
        assert_eq!(children[0].index(), 0);
        assert_eq!(children[1].index(), 1);
        assert_eq!(children[2].index(), 2);
        assert_eq!(path.to_string(), "m/0'/1'/2'");
    }

    #[test]
    fn parse_hardened_path() {
        let path = "m/0'/1H/2h".parse::<HardenedDerivationPath>().unwrap();
        let children: Vec<_> = path.children().collect();
        assert_eq!(children.len(), 3);
        assert_eq!(children[0].index(), 0);
        assert_eq!(children[1].index(), 1);
        assert_eq!(children[2].index(), 2);
    }

    #[test]
    fn reject_non_hardened_path() {
        let err = "m/0/1'".parse::<HardenedDerivationPath>().unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidPath);
    }

    #[test]
    fn error_empty_path() {
        let err = "".parse::<DerivationPath>().unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidPath);
        assert_eq!(err.message(), "derivation path is empty");
    }

    #[test]
    fn error_empty_component() {
        let err = "m//1".parse::<DerivationPath>().unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidPath);
        assert_eq!(err.message(), "empty path component");
    }

    #[test]
    fn error_trailing_slash() {
        let err = "m/".parse::<DerivationPath>().unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidPath);
        assert_eq!(err.message(), "empty path component");
    }

    #[test]
    fn error_missing_child_index() {
        let err = "m/'".parse::<DerivationPath>().unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidPath);
        assert_eq!(err.message(), "missing child index");
    }

    #[test]
    fn error_child_index_too_large() {
        let err = "m/2147483648".parse::<DerivationPath>().unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidPath);
        assert_eq!(err.message(), "child index must be less than 2^31");
    }

    #[test]
    fn display_hardened_path() {
        let path = "m/0'/1'".parse::<HardenedDerivationPath>().unwrap();
        assert_eq!(path.to_string(), "m/0'/1'");
    }

    #[test]
    fn derive_hardened_path_from_derivation_path() {
        let path = "m/1'/2'".parse::<DerivationPath>().unwrap();
        let hardened = HardenedDerivationPath::try_from(path).unwrap();
        assert_eq!(hardened.to_string(), "m/1'/2'");
    }
}
