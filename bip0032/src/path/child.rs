use core::{fmt, str::FromStr};

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

impl From<ChildNumber> for u32 {
    fn from(value: ChildNumber) -> Self {
        value.0
    }
}

impl From<HardenedChildNumber> for ChildNumber {
    fn from(value: HardenedChildNumber) -> Self {
        value.0
    }
}

/// A hardened-only child number.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HardenedChildNumber(ChildNumber);

impl HardenedChildNumber {
    /// Creates a hardened child number from an index.
    pub fn new(index: u32) -> Result<Self> {
        ChildNumber::new(index, true).map(Self)
    }

    /// Returns the child index without hardening bit.
    pub const fn index(self) -> u32 {
        self.0.index()
    }

    /// Returns this child number as the big-endian bytes.
    pub const fn to_bytes(self) -> [u8; 4] {
        self.0.to_bytes()
    }

    pub(crate) fn from_child_unchecked(child: ChildNumber) -> Self {
        debug_assert!(child.is_hardened());
        Self(child)
    }
}

impl fmt::Display for HardenedChildNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl TryFrom<ChildNumber> for HardenedChildNumber {
    type Error = Error;

    fn try_from(value: ChildNumber) -> Result<Self> {
        if value.is_hardened() {
            Ok(Self(value))
        } else {
            Err(Error::new(ErrorKind::InvalidPath, "expected hardened child number")
                .with_context("child_index", value.index()))
        }
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
    fn parse_child_number_hardened() {
        let child = "7'".parse::<ChildNumber>().unwrap();
        assert!(child.is_hardened());
        assert_eq!(child.index(), 7);
        assert_eq!(child.to_string(), "7'");
    }

    #[test]
    fn parse_child_number_non_hardened() {
        let child = "42".parse::<ChildNumber>().unwrap();
        assert!(!child.is_hardened());
        assert_eq!(child.index(), 42);
        assert_eq!(child.to_string(), "42");
    }

    #[test]
    fn child_number_bytes_roundtrip() {
        let child = ChildNumber::new(1, true).unwrap();
        let bytes = child.to_bytes();
        let restored = ChildNumber::from_bytes(bytes);
        assert_eq!(restored, child);
        assert!(restored.is_hardened());
    }

    #[test]
    fn hardened_child_number_new_and_try_from() {
        let child = HardenedChildNumber::new(5).unwrap();
        assert_eq!(child.index(), 5);
        assert_eq!(child.to_string(), "5'");

        let non_hardened = ChildNumber::new(5, false).unwrap();
        let err = HardenedChildNumber::try_from(non_hardened).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidPath);
        assert_eq!(err.message(), "expected hardened child number");
    }

    #[test]
    fn error_missing_child_index() {
        let err = "h".parse::<ChildNumber>().unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidPath);
        assert_eq!(err.message(), "missing child index");
    }

    #[test]
    fn error_child_index_too_large() {
        let err = ChildNumber::new(u32::MAX, false).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidPath);
        assert_eq!(err.message(), "child index must be less than 2^31");
    }
}
