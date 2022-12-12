#![no_main]

use bip0039::Mnemonic;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(phrase) = std::str::from_utf8(data) {
        let _ = <Mnemonic>::from_phrase(phrase);
    }
});
