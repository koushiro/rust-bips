#![doc = include_str!("../README.md")]
#![deny(unused_imports)]
#![deny(missing_docs)]
#![deny(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(not(feature = "std"))]
extern crate alloc;

mod error;
pub mod language;
mod mnemonic;

pub use self::{
    error::Error,
    language::*,
    mnemonic::{AnyMnemonic, Count, Mnemonic},
};
