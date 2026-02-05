//! # bip0039
//!
//! [![](https://github.com/koushiro/rust-bips/actions/workflows/bip0039.yml/badge.svg)][actions]
//! [![](https://img.shields.io/docsrs/bip0039)][docs.rs]
//! [![](https://img.shields.io/crates/v/bip0039)][crates.io]
//! [![](https://img.shields.io/crates/l/bip0039)][crates.io]
//! [![](https://img.shields.io/crates/d/bip0039.svg)][crates.io]
//! [![](https://img.shields.io/badge/MSRV-1.85.0-green?logo=rust)][whatrustisit]
//!
//! [actions]: https://github.com/koushiro/rust-bips/actions
//! [docs.rs]: https://docs.rs/bip0039
//! [crates.io]: https://crates.io/crates/bip0039
//! [whatrustisit]: https://www.whatrustisit.com
//!
//! Another Rust implementation of [BIP-0039](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki) standard.
//!
//! ## Usage
//!
//! ### Compile-time language selection
//!
//! ```rust
//! use bip0039::{Count, English, Mnemonic};
//!
//! // Generate an English mnemonic with 12 words randomly
//! let mnemonic = <Mnemonic<English>>::generate(Count::Words12);
//! // Or use the default generic type (English).
//! let mnemonic = <Mnemonic>::generate(Count::Words12);
//! println!("phrase: {}", mnemonic.phrase());
//!
//! // Generate the HD wallet seed from the mnemonic and the passphrase.
//! let seed = mnemonic.to_seed("");
//! assert_eq!(seed.len(), 64);
//! println!("seed: {}", const_hex::encode(seed));
//! ```
//!
//! ### Runtime language selection
//!
//! ```rust
//! use bip0039::{AnyLanguage, AnyMnemonic, BuiltInLanguage, Count, English};
//!
//! // Generate an English mnemonic with 12 words randomly
//! let mnemonic = AnyMnemonic::generate(BuiltInLanguage::English, Count::Words12);
//! assert_eq!(mnemonic.language(), AnyLanguage::of::<English>());
//! println!("phrase: {}", mnemonic.phrase());
//!
//! // Generate the HD wallet seed from the mnemonic and the passphrase.
//! let seed = mnemonic.to_seed("");
//! assert_eq!(seed.len(), 64);
//! println!("seed: {}", const_hex::encode(seed));
//! ```

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
