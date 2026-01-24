//! Extended key types for BIP32.

mod core;
mod payload;

pub use self::{
    core::{ExtendedPrivateKey, ExtendedPublicKey},
    payload::{ExtendedKeyPayload, KnownVersion, Version},
};
