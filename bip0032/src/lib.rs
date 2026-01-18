//! Another Rust implementation of [BIP-0032](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki) standard.
//!
//! # Usage
//!
//! Seed material is typically derived from a BIP-0039 mnemonic (for example, via
//! [bip0039](https://crates.io/crates/bip0039)).
//!
//! ```rust,ignore
//! use bip0039::{Count, English, Mnemonic};
//!
//! let mnemonic = <Mnemonic<English>>::generate(Count::Words12);
//! let seed = mnemonic.to_seed("");
//! ```
//!
//! The examples below assume the `seed` from above.
//!
//! 1. Private parent key -> private child key (supports hardened).
//!
//! ```rust
//! use bip0032::{DerivationPath, ExtendedPrivateKey, Version, backend::K256Backend};
//!
//! # let seed = [0u8; 64];
//! let master = ExtendedPrivateKey::<K256Backend>::new(&seed).unwrap();
//! let path: DerivationPath = "m/0H/1".parse().unwrap();
//! let child = master.derive_path(&path).unwrap();
//! let xprv = child
//!     .encode_with(Version::XPRV)
//!     .unwrap()
//!     .to_string();
//! ```
//!
//! 2. Private parent key -> public child key.
//!
//! ```rust
//! use bip0032::{DerivationPath, ExtendedPrivateKey, Version, backend::K256Backend};
//!
//! # let seed = [0u8; 64];
//! let master = ExtendedPrivateKey::<K256Backend>::new(&seed).unwrap();
//! let path: DerivationPath = "m/0H/1".parse().unwrap();
//! let child = master.derive_path(&path).unwrap();
//! let xpub = child
//!     .public_key()
//!     .encode_with(Version::XPUB)
//!     .unwrap()
//!     .to_string();
//! ```
//!
//! 3. Public parent key -> public child key (non-hardened only).
//!
//! ```rust
//! use bip0032::{DerivationPath, ExtendedPublicKey, Version, backend::K256Backend};
//!
//! let parent_xpub = "xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8";
//! let parent: ExtendedPublicKey<K256Backend> = parent_xpub.parse().unwrap();
//! let path: DerivationPath = "m/0/1".parse().unwrap();
//! let child = parent.derive_path(&path).unwrap();
//! let xpub = child
//!     .encode_with(Version::XPUB)
//!     .unwrap()
//!     .to_string();
//! ```
//!
//! 4. Public parent key -> private child key: impossible (BIP-0032 does not allow it).

#![deny(unused_imports)]
#![deny(missing_docs)]
#![deny(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod backend;
mod path;
#[cfg(test)]
mod tests;
mod xkey;

pub use crate::{
    path::{ChildNumber, DerivationPath},
    xkey::{ExtendedKeyPayload, ExtendedPrivateKey, ExtendedPublicKey, KnownVersion, Version},
};
