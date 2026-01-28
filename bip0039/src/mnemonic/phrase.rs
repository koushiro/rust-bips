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
    string::{String, ToString},
    vec,
    vec::Vec,
};

use sha2::{Digest, Sha256};

use super::{BITS_PER_BYTE, BITS_PER_WORD, BitAccumulator, Count};
use crate::{
    error::Error,
    language::{AnyLanguage, Language},
};

/// Controls whether decoding also constructs a normalized phrase (single ASCII spaces).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DecodeMode {
    /// Only validate / decode entropy. Do not allocate a normalized phrase.
    ValidateOnly,
    /// Also build a normalized phrase (single ASCII spaces).
    BuildNormalizedPhrase,
}

/// Internal decoded result used by `Mnemonic`.
pub struct DecodedPhrase {
    pub entropy: Vec<u8>,
    pub normalized_phrase: Option<String>,
}

/// Decode a phrase into entropy and (optionally) a normalized phrase using `Language` generic type.
///
/// Note:
/// - This function assumes the input has already been normalized to UTF-8 NFKD by the caller.
pub fn decode_phrase<L: Language>(phrase: &str, mode: DecodeMode) -> Result<DecodedPhrase, Error> {
    decode_phrase_with(AnyLanguage::of::<L>(), phrase, mode)
}

/// Decode a phrase into entropy and (optionally) a normalized phrase using `AnyLanguage` type.
///
/// Note:
/// - This function assumes the input has already been normalized to UTF-8 NFKD by the caller.
pub fn decode_phrase_with(
    language: AnyLanguage,
    phrase: &str,
    mode: DecodeMode,
) -> Result<DecodedPhrase, Error> {
    let params = parse_params_from_phrase(phrase)?;

    let mut normalized_phrase = match mode {
        DecodeMode::ValidateOnly => None,
        DecodeMode::BuildNormalizedPhrase => {
            let words = params.count.word_count();
            // Rough capacity: avg word len ~8 bytes + spaces.
            let rough_phrase_cap = words * 8 + (words.saturating_sub(1));
            Some(String::with_capacity(rough_phrase_cap))
        },
    };

    let mut state = DecodeState::new(params);

    for word in phrase.split_whitespace() {
        if let Some(out) = normalized_phrase.as_mut() {
            if !out.is_empty() {
                out.push(' ');
            }
            out.push_str(word);
        }

        let index = match language.index_of(word) {
            Some(i) => i as u64,
            None => return Err(Error::UnknownWord(word.to_string())),
        };

        state.push_index(index);
    }

    let entropy = state.finish()?;

    Ok(DecodedPhrase { entropy, normalized_phrase })
}

struct DecodeParams {
    entropy_byte_length: usize,
    checksum_bit_length: usize,
    count: Count,
}

fn parse_params_from_phrase(phrase: &str) -> Result<DecodeParams, Error> {
    let count = Count::from_phrase(phrase)?;
    let entropy_byte_length = count.entropy_bit_length() / BITS_PER_BYTE;
    let checksum_bit_length = count.checksum_bit_length();

    Ok(DecodeParams { entropy_byte_length, checksum_bit_length, count })
}

struct DecodeState {
    params: DecodeParams,
    entropy: Vec<u8>,
    entropy_out: usize,
    accumulator: BitAccumulator,
    actual_checksum: u8,
    actual_checksum_filled: usize,
}

impl DecodeState {
    fn new(params: DecodeParams) -> Self {
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
        // If they fail, it's a bug in this library, not user input.
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
