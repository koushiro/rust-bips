//! Extended key types for BIP32.

mod core;
mod payload;
#[cfg(feature = "slip10")]
pub mod slip10;

pub use self::{
    core::{ExtendedPrivateKey, ExtendedPublicKey},
    payload::{ExtendedKeyPayload, KnownVersion, Version},
};
