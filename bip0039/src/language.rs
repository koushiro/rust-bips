//! Supported languages for BIP-0039.
//!
//! This module is the public surface for language support.
//!
//! - [`Language`] is the only public capability trait exposed for consumers.
//! - Built-in languages are enabled via Cargo features and backed by crate-private generated
//!   wordlists.

use crate::wordlist::*;

/// Language to be used for the mnemonic phrase.
///
/// Consumers may implement this trait for their own language types by providing:
/// - [`Language::word_of`]
/// - [`Language::index_of`]
///
/// Built-in languages implement a crate-private [`WordlistProvider`] and automatically
/// get this trait via the blanket impl below.
///
/// # Requirements
///
/// - `word_of(index)` must return a valid word for all indices `0..2048`.
/// - `index_of(word)` must return the correct index (BIP-0039 order) for all words in the language
///   wordlist; return `None` for unknown words.
pub trait Language: Sized {
    /// Returns the word at `index` (BIP-0039 order).
    fn word_of(index: usize) -> &'static str;

    /// Returns the index of `word` in the word list (BIP-0039 order).
    fn index_of(word: &str) -> Option<usize>;
}

impl<T: WordlistProvider> Language for T {
    #[inline]
    fn word_of(index: usize) -> &'static str {
        debug_assert!(index < 2048, "Invalid wordlist index");
        <T as WordlistProvider>::wordlist().words[index]
    }

    #[inline]
    fn index_of(word: &str) -> Option<usize> {
        <T as WordlistProvider>::wordlist().index.get(word).copied().map(|i| i as usize)
    }
}

/// The `English` language.
///
/// The `English` language is always available; other languages are enabled via
/// compilation features.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct English;

impl WordlistProvider for English {
    #[inline]
    fn wordlist() -> &'static Wordlist {
        &wordlists::english::WORDLIST
    }
}

/// The `Simplified Chinese` language.
#[cfg(feature = "chinese-simplified")]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ChineseSimplified;

#[cfg(feature = "chinese-simplified")]
impl WordlistProvider for ChineseSimplified {
    #[inline]
    fn wordlist() -> &'static Wordlist {
        &wordlists::chinese_simplified::WORDLIST
    }
}

/// The `Traditional Chinese` language.
#[cfg(feature = "chinese-traditional")]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ChineseTraditional;

#[cfg(feature = "chinese-traditional")]
impl WordlistProvider for ChineseTraditional {
    #[inline]
    fn wordlist() -> &'static Wordlist {
        &wordlists::chinese_traditional::WORDLIST
    }
}

/// The `Czech` language.
#[cfg(feature = "czech")]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Czech;

#[cfg(feature = "czech")]
impl WordlistProvider for Czech {
    #[inline]
    fn wordlist() -> &'static Wordlist {
        &wordlists::czech::WORDLIST
    }
}

/// The `French` language.
#[cfg(feature = "french")]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct French;

#[cfg(feature = "french")]
impl WordlistProvider for French {
    #[inline]
    fn wordlist() -> &'static Wordlist {
        &wordlists::french::WORDLIST
    }
}

/// The `Italian` language.
#[cfg(feature = "italian")]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Italian;

#[cfg(feature = "italian")]
impl WordlistProvider for Italian {
    #[inline]
    fn wordlist() -> &'static Wordlist {
        &wordlists::italian::WORDLIST
    }
}

/// The `Japanese` language.
#[cfg(feature = "japanese")]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Japanese;

#[cfg(feature = "japanese")]
impl WordlistProvider for Japanese {
    #[inline]
    fn wordlist() -> &'static Wordlist {
        &wordlists::japanese::WORDLIST
    }
}

/// The `Korean` language.
#[cfg(feature = "korean")]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Korean;

#[cfg(feature = "korean")]
impl WordlistProvider for Korean {
    #[inline]
    fn wordlist() -> &'static Wordlist {
        &wordlists::korean::WORDLIST
    }
}

/// The `Portuguese` language.
#[cfg(feature = "portuguese")]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Portuguese;

#[cfg(feature = "portuguese")]
impl WordlistProvider for Portuguese {
    #[inline]
    fn wordlist() -> &'static Wordlist {
        &wordlists::portuguese::WORDLIST
    }
}

/// The `Spanish` language.
#[cfg(feature = "spanish")]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Spanish;

#[cfg(feature = "spanish")]
impl WordlistProvider for Spanish {
    #[inline]
    fn wordlist() -> &'static Wordlist {
        &wordlists::spanish::WORDLIST
    }
}

#[cfg(test)]
mod tests {
    use sha2::{Digest, Sha256};

    use super::*;

    // Check the sha256sum of the word lists.
    //
    // They are as follows in the [bips](https://github.com/bitcoin/bips/blob/master/bip-0039/bip-0039-wordlists.md):
    //
    // Chinese(simplified):     5c5942792bd8340cb8b27cd592f1015edf56a8c5b26276ee18a482428e7c5726
    // Chinese(traditional):    417b26b3d8500a4ae3d59717d7011952db6fc2fb84b807f3f94ac734e89c1b5f
    // Czech:                   7e80e161c3e93d9554c2efb78d4e3cebf8fc727e9c52e03b83b94406bdcc95fc
    // English:                 2f5eed53a4727b4bf8880d8f3f199efc90e58503646d9ff8eff3a2ed3b24dbda
    // French:                  ebc3959ab7801a1df6bac4fa7d970652f1df76b683cd2f4003c941c63d517e59
    // Italian:                 d392c49fdb700a24cd1fceb237c1f65dcc128f6b34a8aacb58b59384b5c648c2
    // Japanese:                2eed0aef492291e061633d7ad8117f1a2b03eb80a29d0e4e3117ac2528d05ffd
    // Korean:                  9e95f86c167de88f450f0aaf89e87f6624a57f973c67b516e338e8e8b8897f60
    // Portuguese:              2685e9c194c82ae67e10ba59d9ea5345a23dc093e92276fc5361f6667d79cd3f
    // Spanish:                 46846a5a0139d1e3cb77293e521c2865f7bcdb82c44e8d0a06a2cd0ecba48c0b
    struct Case {
        name: &'static str,
        expected_hex_sha256: &'static str,
        actual_hex_sha256: fn() -> String,
    }

    fn actual_checksum<L: WordlistProvider>() -> String {
        let mut digest = Sha256::new();
        for &word in <L as WordlistProvider>::wordlist().words {
            assert!(unicode_normalization::is_nfkd(word));
            digest.update(format!("{}\n", word));
        }
        const_hex::encode(digest.finalize())
    }

    #[test]
    fn validate_word_list_checksums() {
        let mut cases: Vec<Case> = Vec::new();

        cases.push(Case {
            name: "english",
            expected_hex_sha256: "2f5eed53a4727b4bf8880d8f3f199efc90e58503646d9ff8eff3a2ed3b24dbda",
            actual_hex_sha256: actual_checksum::<English>,
        });

        #[cfg(feature = "chinese-simplified")]
        cases.push(Case {
            name: "chinese-simplified",
            expected_hex_sha256: "5c5942792bd8340cb8b27cd592f1015edf56a8c5b26276ee18a482428e7c5726",
            actual_hex_sha256: actual_checksum::<ChineseSimplified>,
        });

        #[cfg(feature = "chinese-traditional")]
        cases.push(Case {
            name: "chinese-traditional",
            expected_hex_sha256: "417b26b3d8500a4ae3d59717d7011952db6fc2fb84b807f3f94ac734e89c1b5f",
            actual_hex_sha256: actual_checksum::<ChineseTraditional>,
        });

        #[cfg(feature = "czech")]
        cases.push(Case {
            name: "czech",
            expected_hex_sha256: "7e80e161c3e93d9554c2efb78d4e3cebf8fc727e9c52e03b83b94406bdcc95fc",
            actual_hex_sha256: actual_checksum::<Czech>,
        });

        #[cfg(feature = "french")]
        cases.push(Case {
            name: "french",
            expected_hex_sha256: "ebc3959ab7801a1df6bac4fa7d970652f1df76b683cd2f4003c941c63d517e59",
            actual_hex_sha256: actual_checksum::<French>,
        });

        #[cfg(feature = "italian")]
        cases.push(Case {
            name: "italian",
            expected_hex_sha256: "d392c49fdb700a24cd1fceb237c1f65dcc128f6b34a8aacb58b59384b5c648c2",
            actual_hex_sha256: actual_checksum::<Italian>,
        });

        #[cfg(feature = "japanese")]
        cases.push(Case {
            name: "japanese",
            expected_hex_sha256: "2eed0aef492291e061633d7ad8117f1a2b03eb80a29d0e4e3117ac2528d05ffd",
            actual_hex_sha256: actual_checksum::<Japanese>,
        });

        #[cfg(feature = "korean")]
        cases.push(Case {
            name: "korean",
            expected_hex_sha256: "9e95f86c167de88f450f0aaf89e87f6624a57f973c67b516e338e8e8b8897f60",
            actual_hex_sha256: actual_checksum::<Korean>,
        });

        #[cfg(feature = "portuguese")]
        cases.push(Case {
            name: "portuguese",
            expected_hex_sha256: "2685e9c194c82ae67e10ba59d9ea5345a23dc093e92276fc5361f6667d79cd3f",
            actual_hex_sha256: actual_checksum::<Portuguese>,
        });

        #[cfg(feature = "spanish")]
        cases.push(Case {
            name: "spanish",
            expected_hex_sha256: "46846a5a0139d1e3cb77293e521c2865f7bcdb82c44e8d0a06a2cd0ecba48c0b",
            actual_hex_sha256: actual_checksum::<Spanish>,
        });

        for case in cases {
            let actual = (case.actual_hex_sha256)();
            assert_eq!(
                actual, case.expected_hex_sha256,
                "checksum mismatch for language '{}'",
                case.name
            );
        }
    }
}
