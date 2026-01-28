//! Seed derivation for BIP-0039.
//!
//! This module contains the PBKDF2-HMAC-SHA512 logic for turning a normalized mnemonic phrase
//! into a 64-byte seed.

#[cfg(not(feature = "std"))]
use alloc::{borrow::Cow, format};
#[cfg(feature = "std")]
use std::borrow::Cow;

use hmac::Hmac;
use sha2::Sha512;

use super::normalize_utf8;

const PBKDF2_ROUNDS: u32 = 2048;
const PBKDF2_BYTES: usize = 64;

/// Derives a 64-byte seed from a normalized mnemonic phrase and a passphrase.
///
/// Notes:
/// - `normalized_phrase` must already be normalized to UTF-8 NFKD.
/// - The salt is `"mnemonic{passphrase}"`, normalized to UTF-8 NFKD.
pub fn to_seed(normalized_phrase: &str, passphrase: &str) -> [u8; 64] {
    // use the PBKDF2 function with a mnemonic sentence (in UTF-8 NFKD) used as the password
    // and the string "mnemonic" + passphrase (again in UTF-8 NFKD) used as the salt.
    // The iteration count is set to 2048 and HMAC-SHA512 is used as the pseudo-random function.
    // The length of the derived key is 512 bits (= 64 bytes).

    let normalized_password = normalized_phrase;
    let normalized_salt = {
        let mut salt = Cow::Owned(format!("mnemonic{}", passphrase));
        normalize_utf8(&mut salt);
        salt
    };
    let mut seed = [0u8; PBKDF2_BYTES];
    pbkdf2::pbkdf2::<Hmac<Sha512>>(
        normalized_password.as_bytes(),
        normalized_salt.as_bytes(),
        PBKDF2_ROUNDS,
        &mut seed,
    )
    .expect("HMAC can be initialized with any key length");
    seed
}
