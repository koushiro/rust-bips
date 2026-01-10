//! Phrase decoding internals.
//!
//! This module contains the logic to decode a BIP-0039 mnemonic phrase into:
//! - the original entropy bytes
//! - (optionally) a normalized phrase representation (single ASCII spaces)
//!
//! It is intentionally crate-private and designed to be the single source of truth
//! for decoding/validation logic used by `Mnemonic::from_phrase` and `Mnemonic::validate`.

#[cfg(not(feature = "std"))]
use alloc::{
    borrow::Cow,
    string::{String, ToString},
    vec,
    vec::Vec,
};
#[cfg(feature = "std")]
use std::borrow::Cow;

use sha2::{Digest, Sha256};

use super::{BITS_PER_BYTE, BITS_PER_WORD, BitAccumulator, Count};
use crate::{error::Error, language::Language};

/// Ensure the content of the `s` is normalized UTF8.
/// Avoid allocation for normalization when there are no special UTF8 characters in the string.
#[inline]
pub fn normalize_utf8(s: &mut Cow<'_, str>) {
    use unicode_normalization::{IsNormalized, UnicodeNormalization, is_nfkd_quick};
    if is_nfkd_quick(s.as_ref().chars()) != IsNormalized::Yes {
        *s = Cow::Owned(s.as_ref().nfkd().to_string())
    }
}

/// Controls whether decoding also constructs a normalized phrase (single ASCII spaces).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ParseMode {
    /// Only validate / decode entropy. Do not allocate a normalized phrase.
    ValidateOnly,
    /// Also build a normalized phrase (single ASCII spaces).
    BuildNormalizedPhrase,
}

/// Internal parse result used by `Mnemonic`.
pub struct ParsedPhrase {
    pub entropy: Vec<u8>,
    pub normalized_phrase: Option<String>,
}

/// Decode a phrase into entropy and (optionally) a normalized phrase.
///
/// The input is normalized to UTF-8 NFKD prior to processing (same behavior as the
/// rest of this crate).
pub fn decode_phrase<'a, L: Language, P: Into<Cow<'a, str>>>(
    phrase: P,
    mode: ParseMode,
) -> Result<ParsedPhrase, Error> {
    let mut phrase = phrase.into();
    normalize_utf8(&mut phrase);

    let params = parse_params_from_phrase(&phrase)?;

    let mut normalized_phrase = match mode {
        ParseMode::ValidateOnly => None,
        ParseMode::BuildNormalizedPhrase => {
            let words = params.count.word_count();
            // Rough capacity: avg word len ~8 bytes + spaces.
            let rough_phrase_cap = words * 8 + (words.saturating_sub(1));
            Some(String::with_capacity(rough_phrase_cap))
        },
    };

    let mut state = ParseState::new(params);

    for word in phrase.as_ref().split_whitespace() {
        if let Some(out) = normalized_phrase.as_mut() {
            if !out.is_empty() {
                out.push(' ');
            }
            out.push_str(word);
        }

        let index = match L::index_of(word) {
            Some(i) => i as u64,
            None => return Err(Error::UnknownWord(word.to_string())),
        };

        state.push_index(index);
    }

    let entropy = state.finish()?;

    Ok(ParsedPhrase { entropy, normalized_phrase })
}

struct ParseParams {
    entropy_byte_length: usize,
    checksum_bit_length: usize,
    count: Count,
}

fn parse_params_from_phrase(phrase: &str) -> Result<ParseParams, Error> {
    let count = Count::from_phrase(phrase)?;
    let entropy_byte_length = count.entropy_bit_length() / BITS_PER_BYTE;
    let checksum_bit_length = count.checksum_bit_length();

    Ok(ParseParams { entropy_byte_length, checksum_bit_length, count })
}

struct ParseState {
    params: ParseParams,
    entropy: Vec<u8>,
    entropy_out: usize,
    accumulator: BitAccumulator,
    actual_checksum: u8,
    actual_checksum_filled: usize,
}

impl ParseState {
    fn new(params: ParseParams) -> Self {
        let entropy_byte_length = params.entropy_byte_length;
        Self {
            params,
            entropy: vec![0u8; entropy_byte_length],
            entropy_out: 0,
            accumulator: BitAccumulator::new(),
            actual_checksum: 0,
            actual_checksum_filled: 0,
        }
    }

    fn push_index(&mut self, index: u64) {
        let entropy_byte_length = self.params.entropy_byte_length;
        let checksum_bit_length = self.params.checksum_bit_length;

        self.accumulator.push_bits(index, BITS_PER_WORD);

        // Drain full bytes into entropy first.
        while self.entropy_out < entropy_byte_length && self.accumulator.can_take(BITS_PER_BYTE) {
            self.entropy[self.entropy_out] = self.accumulator.take_bits(BITS_PER_BYTE) as u8;
            self.entropy_out += 1;
        }

        // After entropy is complete, drain checksum bits (4..=8).
        while self.entropy_out == entropy_byte_length
            && self.actual_checksum_filled < checksum_bit_length
            && self.accumulator.can_take(1)
        {
            let bit = self.accumulator.take_bits(1) as u8;
            self.actual_checksum = (self.actual_checksum << 1) | bit;
            self.actual_checksum_filled += 1;
        }
    }

    fn finish(self) -> Result<Vec<u8>, Error> {
        // These conditions should be guaranteed by `Count::from_phrase` and correct decode logic.
        // If they fail, it's a bug in this module (or in the wordlist/index mapping), not user input.
        debug_assert_eq!(
            self.entropy_out, self.params.entropy_byte_length,
            "decoded entropy length mismatch (bytes)"
        );
        debug_assert_eq!(
            self.actual_checksum_filled, self.params.checksum_bit_length,
            "decoded checksum length mismatch (bits)"
        );
        debug_assert_eq!(self.accumulator.bits(), 0, "trailing bits remained after decoding");

        // Verify checksum (user-visible validation).
        const fn checksum(source: u8, bit_length: usize) -> u8 {
            source >> (BITS_PER_BYTE - bit_length)
        }

        let checksum_byte = Sha256::digest(&self.entropy)[0];
        let expected_checksum = checksum(checksum_byte, self.params.checksum_bit_length);
        if self.actual_checksum != expected_checksum {
            return Err(Error::InvalidChecksum);
        }

        Ok(self.entropy)
    }
}
