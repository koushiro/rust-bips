#![no_main]

use bip0039::{Count, Mnemonic};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|_data: &[u8]| {
    let _ = <Mnemonic>::generate(Count::Words12);
    let _ = <Mnemonic>::generate(Count::Words15);
    let _ = <Mnemonic>::generate(Count::Words18);
    let _ = <Mnemonic>::generate(Count::Words21);
    let _ = <Mnemonic>::generate(Count::Words24);
});
