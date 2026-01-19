//! Derivation path parsing and child number handling.

#[cfg(not(feature = "std"))]
use alloc::{vec, vec::Vec};
use core::{fmt, slice, str::FromStr};
#[cfg(feature = "std")]
use std::vec;

use crate::{Error, ErrorKind, Result};

/// A BIP32 child number with optional hardened flag.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ChildNumber(u32);

impl ChildNumber {
    // normal child keys: 0x0000_0000 ~ 0x7FFF_FFFF
    // hardened child keys: 0x8000_0000 ~ 0xFFFF_FFFF
    const HARDENED_OFFSET: u32 = 0x8000_0000;

    /// Creates a child number from an index and hardened flag.
    pub fn new(index: u32, hardened: bool) -> Result<Self> {
        if index >= Self::HARDENED_OFFSET {
            return Err(Error::new(ErrorKind::InvalidPath, "child index must be less than 2^31")
                .with_context("child_index", index));
        }
        let value = if hardened { index + Self::HARDENED_OFFSET } else { index };
        Ok(Self(value))
    }

    /// Returns the child index without hardening bit.
    pub const fn index(self) -> u32 {
        self.0 & !Self::HARDENED_OFFSET
    }

    /// Returns true if this child is hardened.
    pub const fn is_hardened(self) -> bool {
        self.0 >= Self::HARDENED_OFFSET
    }

    /// Creates a child number from the big-endian bytes.
    pub const fn from_bytes(bytes: [u8; 4]) -> Self {
        Self(u32::from_be_bytes(bytes))
    }

    /// Returns this child number as the big-endian bytes.
    pub const fn to_bytes(self) -> [u8; 4] {
        self.0.to_be_bytes()
    }
}

impl From<ChildNumber> for u32 {
    fn from(value: ChildNumber) -> Self {
        value.0
    }
}

impl fmt::Display for ChildNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.index())?;

        if self.is_hardened() {
            f.write_str("\'")?;
        }

        Ok(())
    }
}

impl FromStr for ChildNumber {
    type Err = Error;

    fn from_str(component: &str) -> Result<Self> {
        parse_child_component(component)
    }
}

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

fn parse_child_component(component: &str) -> Result<ChildNumber> {
    let (number, hardened) = if let Some(stripped) = component.strip_suffix('\'') {
        (stripped, true)
    } else if let Some(stripped) = component.strip_suffix('h') {
        (stripped, true)
    } else if let Some(stripped) = component.strip_suffix('H') {
        (stripped, true)
    } else {
        (component, false)
    };

    if number.is_empty() {
        return Err(Error::new(ErrorKind::InvalidPath, "missing child index")
            .with_context("component", component));
    }

    let index = number.parse::<u32>().map_err(|_| {
        Error::new(ErrorKind::InvalidPath, "invalid child index")
            .with_context("component", component)
    })?;
    ChildNumber::new(index, hardened)
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
    fn parse_child_number() {
        let child = "7'".parse::<ChildNumber>().unwrap();
        assert!(child.is_hardened());
        assert_eq!(child.index(), 7);
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
}
