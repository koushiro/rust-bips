//! Another Rust implementation of [BIP-0032](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki) standard.

#![deny(unused_imports)]
#![deny(missing_docs)]
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
