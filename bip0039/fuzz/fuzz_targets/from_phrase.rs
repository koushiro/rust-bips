#![no_main]

use arbitrary::Arbitrary;
use bip0039::{English, Error, Language, Mnemonic};
use libfuzzer_sys::fuzz_target;

/// Fuzz input aimed at exercising:
/// - UTF-8 NFKD normalization (`Mnemonic::from_phrase`, `Mnemonic::validate`)
/// - whitespace normalization (`split_whitespace`)
/// - unknown-word handling / checksum validation
#[derive(Debug, Arbitrary)]
struct Input<'a> {
    /// Raw bytes that will be interpreted as a phrase (lossy UTF-8).
    phrase_bytes: &'a [u8],

    /// If true, we will try to "perturb" a successfully-parsed phrase into an invalid one
    /// (checksum and/or unknown word) to exercise error paths.
    do_negative: bool,

    /// If true, we will intentionally insert lots of weird whitespace between words to
    /// stress `split_whitespace` normalization.
    messy_whitespace: bool,

    /// Caps to keep runtime bounded regardless of fuzzer input.
    max_phrase_len: u16,
}

fn lossy_string(bytes: &[u8], max_len: usize) -> String {
    let n = bytes.len().min(max_len);
    String::from_utf8_lossy(&bytes[..n]).into_owned()
}

fn make_whitespace_mess(s: &str) -> String {
    // Expand all ASCII spaces into a mix of whitespace forms.
    // Keep it deterministic and cheap.
    let mut out = String::with_capacity(s.len() * 2);
    for (i, ch) in s.chars().enumerate() {
        if ch == ' ' {
            match i % 6 {
                0 => out.push(' '),
                1 => out.push('\t'),
                2 => out.push('\n'),
                3 => out.push('\r'),
                4 => {
                    out.push(' ');
                    out.push(' ');
                },
                _ => out.push('\u{00A0}'), // NBSP to hit unicode whitespace behavior
            }
        } else {
            out.push(ch);
        }
    }
    out
}

fn flip_some_words_to_unknown(phrase: &str) -> String {
    // Replace one word with something definitely not in the English BIP39 list.
    // We choose tokens that should survive NFKD normalization.
    let mut words: Vec<&str> = phrase.split_whitespace().collect();
    if words.is_empty() {
        return "not_in_wordlist".to_string();
    }
    let idx = (words.len() / 2).min(words.len().saturating_sub(1));
    words[idx] = "not_in_wordlist";
    words.join(" ")
}

fn flip_checksum_like(phrase: &str) -> String {
    // Try to keep word count the same but move one word to a different valid word.
    // This usually invalidates checksum while still remaining all-known-words.
    let mut words: Vec<&str> = phrase.split_whitespace().collect();
    if words.is_empty() {
        return phrase.to_string();
    }

    // mutate the last word deterministically to another valid English word.
    let last = words.len() - 1;
    let orig = words[last];

    // If it's known, move index by +1 mod 2048; if unknown (shouldn't happen here), just replace.
    if let Some(i) = <English as Language>::index_of(orig) {
        let new = <English as Language>::word_of((i + 1) % 2048);
        words[last] = new;
    } else {
        words[last] = "abandon"; // common first word; should be valid English
    }

    words.join(" ")
}

fuzz_target!(|input: Input<'_>| {
    // Bound runtime regardless of input size.
    let max_phrase_len = (input.max_phrase_len as usize).clamp(1, 16 * 1024);

    let mut phrase = lossy_string(input.phrase_bytes, max_phrase_len);

    if input.messy_whitespace {
        phrase = make_whitespace_mess(&phrase);
    }

    // Always exercise validate (should never panic).
    // We don't assert on success/failure because that's input-dependent.
    let _ = <Mnemonic<English>>::validate(phrase.as_str());

    // Now exercise parsing. If it succeeds, we verify some invariants.
    match <Mnemonic<English>>::from_phrase(phrase.as_str()) {
        Ok(m) => {
            // `phrase()` is normalized to single ASCII spaces (per docs).
            let normalized = m.phrase();
            debug_assert!(!normalized.is_empty());

            // Normalized phrase should validate successfully.
            assert!(<Mnemonic<English>>::validate(normalized).is_ok());

            // Parsing normalized phrase should succeed and be stable.
            let m2 = <Mnemonic<English>>::from_phrase(normalized).unwrap();
            assert_eq!(m2.phrase(), normalized);
            assert_eq!(m2.entropy(), m.entropy());

            if input.do_negative {
                // 1) Unknown word path should return UnknownWord (not panic).
                let bad_unknown = flip_some_words_to_unknown(normalized);
                let err = <Mnemonic<English>>::from_phrase(bad_unknown).unwrap_err();
                match err {
                    Error::UnknownWord(word) => assert_eq!(word, "not_in_wordlist"),
                    _ => panic!("unexpected error for unknown word: {err:?}"),
                }

                // 2) Checksum invalidation path: keep all words valid but expect checksum to fail
                // most of the time.
                let bad_checksum = flip_checksum_like(normalized);
                let res = <Mnemonic<English>>::validate(bad_checksum.as_str());
                // We don't assert it must fail (collision is possible), but we do ensure it
                // doesn't crash and if it succeeds, parsing produces a self-consistent mnemonic.
                match res {
                    Ok(()) => {
                        let mm = <Mnemonic<English>>::from_phrase(bad_checksum.as_str()).unwrap();
                        <Mnemonic<English>>::validate(mm.phrase()).unwrap();
                    },
                    Err(err) => assert_eq!(err, Error::InvalidChecksum),
                }
            }
        },
        Err(_e) => {
            // For robustness fuzzing: any error is acceptable, but should never panic.
            // We still want to lightly exercise roundtripping of "already-normalized" logic by
            // re-validating the original phrase.
            let _ = <Mnemonic<English>>::validate(phrase.as_str());
        },
    }
});
