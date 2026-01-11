#![no_main]

use arbitrary::Arbitrary;
use bip0039::Mnemonic;
use libfuzzer_sys::fuzz_target;

/// A structured input focused on the PBKDF2/seed derivation path.
///
/// Goals:
/// - deterministically derive a valid mnemonic from entropy
/// - exercise NFKD normalization of passphrase input
/// - ensure seed derivation is consistent across equivalent mnemonic instances
#[derive(Clone, Debug, Arbitrary)]
struct Input<'a> {
    entropy: EntropyInput,
    /// Optional passphrase bytes to exercise NFKD normalization and PBKDF2 path.
    passphrase_bytes: &'a [u8],
}

#[derive(Clone, Debug, Arbitrary)]
enum EntropyInput {
    // 12 words: 128 bits / 8 = 16
    Words12([u8; 16]),
    // 15 words: 160 bits / 8 = 20
    Words15([u8; 20]),
    // 18 words: 192 bits / 8 = 24
    Words18([u8; 24]),
    // 21 words: 224 bits / 8 = 28
    Words21([u8; 28]),
    // 24 words: 256 bits / 8 = 32
    Words24([u8; 32]),
}

impl EntropyInput {
    fn as_slice(&self) -> &[u8] {
        match self {
            Self::Words12(e) => e,
            Self::Words15(e) => e,
            Self::Words18(e) => e,
            Self::Words21(e) => e,
            Self::Words24(e) => e,
        }
    }
}

fn bytes_to_string_lossy(bytes: &[u8]) -> String {
    // We fuzz the passphrase; it can be arbitrary bytes. For the API we need `&str`,
    // so we do a lossy conversion. This still exercises normalization logic.
    String::from_utf8_lossy(bytes).into_owned()
}

fuzz_target!(|input: Input<'_>| {
    let entropy = input.entropy.as_slice();

    let mnemonic = <Mnemonic>::from_entropy(entropy).expect("valid entropy lengths must succeed");
    let phrase = mnemonic.phrase();
    let passphrase = bytes_to_string_lossy(input.passphrase_bytes);

    let seed1 = mnemonic.to_seed(&passphrase);

    // Re-parse the phrase and ensure seed derivation is stable.
    let mnemonic2 = <Mnemonic>::from_phrase(phrase).expect("encoder output must be parseable");
    let seed2 = mnemonic2.to_seed(&passphrase);
    assert_eq!(seed1, seed2);
});
