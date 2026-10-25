#![doc = include_str!("../README.md")]
#![deny(unused_imports)]
#![deny(missing_docs)]
#![deny(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod curve;
mod error;
mod path;
mod xkey;

#[cfg(feature = "slip10")]
pub use self::xkey::slip10;
pub use self::{
    error::*,
    path::{ChildNumber, DerivationPath, HardenedChildNumber, HardenedDerivationPath},
    xkey::{ExtendedKeyPayload, ExtendedPrivateKey, ExtendedPublicKey, KnownVersion, Version},
};
