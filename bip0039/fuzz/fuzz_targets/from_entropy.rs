#![no_main]

use arbitrary::Arbitrary;
use bip0039::Mnemonic;
use libfuzzer_sys::fuzz_target;

#[derive(Debug, Arbitrary)]
enum EntropySize {
    // 12 words: 128 bits / 8 bits = 16
    Words12([u8; 16]),
    // 15 words: 160 bits / 8 bits = 20
    Words15([u8; 20]),
    // 18 words: 192 bits / 8 bits = 24
    Words18([u8; 24]),
    // 21 words: 224 bits / 8 bits = 28
    Words21([u8; 28]),
    // 24 words: 256 bits / 8 bits = 32
    Words24([u8; 32]),
}

fuzz_target!(|entropys: Vec<EntropySize>| {
    for entropy in entropys {
        match entropy {
            EntropySize::Words12(entropy) => {
                let _ = <Mnemonic>::from_entropy(entropy).unwrap();
            },
            EntropySize::Words15(entropy) => {
                let _ = <Mnemonic>::from_entropy(entropy).unwrap();
            },
            EntropySize::Words18(entropy) => {
                let _ = <Mnemonic>::from_entropy(entropy).unwrap();
            },
            EntropySize::Words21(entropy) => {
                let _ = <Mnemonic>::from_entropy(entropy).unwrap();
            },
            EntropySize::Words24(entropy) => {
                let _ = <Mnemonic>::from_entropy(entropy).unwrap();
            },
        }
    }
});
