#![no_main]

use arbitrary::Arbitrary;
use bip0039::Mnemonic;
use libfuzzer_sys::fuzz_target;

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

fuzz_target!(|input: EntropyInput| {
    let entropy = input.as_slice();

    // Encode from raw entropy.
    let m1 = <Mnemonic>::from_entropy(entropy).unwrap();

    // Basic invariants: entropy roundtrips.
    assert_eq!(m1.entropy(), entropy);

    // Phrase produced by encoding should parse back to the same entropy and normalize to itself.
    let phrase = m1.phrase();
    let m2 = <Mnemonic>::from_phrase(phrase).unwrap();
    assert_eq!(m2.entropy(), entropy);
    assert_eq!(m2.phrase(), phrase);

    // Determinism: same entropy -> same phrase for a given language/wordlist.
    let m3 = <Mnemonic>::from_entropy(entropy).unwrap();
    assert_eq!(m3.phrase(), phrase);
    assert_eq!(m3.entropy(), entropy);
});
