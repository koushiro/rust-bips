//! Another Rust implementation of [BIP-0039](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki) standard.
//!
//! # Usage
//!
//! ```rust
//! use bip0039::{Count, Mnemonic};
//!
//! // Generates an English mnemonic with 12 words randomly
//! let mnemonic = <Mnemonic>::generate(Count::Words12);
//! let phrase = mnemonic.phrase();
//! println!("phrase: {}", phrase);
//!
//! // Generates the HD wallet seed from the mnemonic and the passphrase.
//! let seed = mnemonic.to_seed("");
//! println!("seed: {}", const_hex::encode(&seed[..]));
//! ```
#![cfg_attr(
    feature = "chinese-simplified",
    doc = r##"
```rust
use bip0039::{ChineseSimplified, Count, Mnemonic};

// Generates a Simplified Chinese mnemonic with 12 words randomly
let mnemonic = <Mnemonic<ChineseSimplified>>::generate(Count::Words12);
println!("phrase: {}", mnemonic.phrase());
```
"##
)]
//!

#![deny(unused_imports)]
#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(not(feature = "std"))]
extern crate alloc;

mod error;
pub mod language;
mod mnemonic;

pub use self::language::*;
pub use self::{
    error::Error,
    mnemonic::{Count, Mnemonic},
};
