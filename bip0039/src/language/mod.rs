#[cfg(feature = "chinese-simplified")]
mod chinese_simplified;
#[cfg(feature = "chinese-traditional")]
mod chinese_traditional;
#[cfg(feature = "czech")]
mod czech;
mod english;
#[cfg(feature = "french")]
mod french;
#[cfg(feature = "italian")]
mod italian;
#[cfg(feature = "japanese")]
mod japanese;
#[cfg(feature = "korean")]
mod korean;
#[cfg(feature = "portuguese")]
mod portuguese;
#[cfg(feature = "spanish")]
mod spanish;

/// Language to be used for the mnemonic phrase.
///
/// The English language is always available, other languages are enabled using
/// the compilation features.
pub trait Language: Sized {
    /// The word list for this language.
    const WORD_LIST: &'static [&'static str];

    /// Returns the word of `index` in the word list.
    #[inline]
    fn word_of(index: usize) -> &'static str {
        debug_assert!(index < 2048, "Invalid wordlist index");
        Self::WORD_LIST[index]
    }

    /// Returns the index of the word in the word list.
    #[inline]
    fn index_of(word: &str) -> Option<usize> {
        // For ordered word lists, we can use binary search to improve the search speed.
        if Self::is_sorted() {
            Self::WORD_LIST.binary_search(&word).ok()
        } else {
            Self::WORD_LIST.iter().position(|&w| w == word)
        }
    }

    /// Checks if the word list of this language are sorted by the byte values.
    ///
    /// The words in the word list are ordered lexicographically, which means that we cannot use
    /// `binary_search` to find words more efficiently if the ordering of the words is not be
    /// sorted by the byte values, because the Rust ordering is based on the byte values.
    #[inline]
    fn is_sorted() -> bool {
        false
    }

    /// Returns words from the word list that start with the given prefix.
    ///
    /// The words in the word list are ordered lexicographically, which means that we cannot use
    /// `binary_search` to find words more efficiently if the ordering of the words is not be
    /// sorted by the byte values, because the Rust ordering is based on the byte values.
    /// However, it does mean that words that share a prefix will follow each other.
    fn words_by_prefix(prefix: &str) -> &[&'static str] {
        let first = match Self::WORD_LIST.iter().position(|w| w.starts_with(prefix)) {
            Some(i) => i,
            None => return &[],
        };
        let count = Self::WORD_LIST[first..]
            .iter()
            .take_while(|w| w.starts_with(prefix))
            .count();
        &Self::WORD_LIST[first..first + count]
    }
}

/// The `English` language.
///
/// The `English` language is always available,
/// other languages are enabled using the compilation features.
#[derive(Copy, Clone, Debug)]
pub struct English;
impl Language for English {
    const WORD_LIST: &'static [&'static str] = &english::WORDS;

    fn is_sorted() -> bool {
        true
    }
}

/// The `Simplified Chinese` language.
#[cfg(feature = "chinese-simplified")]
#[derive(Copy, Clone, Debug)]
pub struct ChineseSimplified;
#[cfg(feature = "chinese-simplified")]
impl Language for ChineseSimplified {
    const WORD_LIST: &'static [&'static str] = &chinese_simplified::WORDS;
}

/// The `Traditional Chinese` language.
#[cfg(feature = "chinese-traditional")]
#[derive(Copy, Clone, Debug)]
pub struct ChineseTraditional;
#[cfg(feature = "chinese-traditional")]
impl Language for ChineseTraditional {
    const WORD_LIST: &'static [&'static str] = &chinese_traditional::WORDS;
}

/// The `Czech` language.
#[cfg(feature = "czech")]
#[derive(Copy, Clone, Debug)]
pub struct Czech;
#[cfg(feature = "czech")]
impl Language for Czech {
    const WORD_LIST: &'static [&'static str] = &czech::WORDS;
}

/// The `French` language.
#[cfg(feature = "french")]
#[derive(Copy, Clone, Debug)]
pub struct French;
#[cfg(feature = "french")]
impl Language for French {
    const WORD_LIST: &'static [&'static str] = &french::WORDS;
}

/// The `Italian` language.
#[cfg(feature = "italian")]
#[derive(Copy, Clone, Debug)]
pub struct Italian;
#[cfg(feature = "italian")]
impl Language for Italian {
    const WORD_LIST: &'static [&'static str] = &italian::WORDS;

    fn is_sorted() -> bool {
        true
    }
}

/// The `Japanese` language.
#[cfg(feature = "japanese")]
#[derive(Copy, Clone, Debug)]
pub struct Japanese;
#[cfg(feature = "japanese")]
impl Language for Japanese {
    const WORD_LIST: &'static [&'static str] = &japanese::WORDS;
}

/// The `Korean` language.
#[cfg(feature = "korean")]
#[derive(Copy, Clone, Debug)]
pub struct Korean;
#[cfg(feature = "korean")]
impl Language for Korean {
    const WORD_LIST: &'static [&'static str] = &korean::WORDS;

    fn is_sorted() -> bool {
        true
    }
}

/// The `Portuguese` language.
#[cfg(feature = "portuguese")]
#[derive(Copy, Clone, Debug)]
pub struct Portuguese;
#[cfg(feature = "portuguese")]
impl Language for Portuguese {
    const WORD_LIST: &'static [&'static str] = &portuguese::WORDS;

    fn is_sorted() -> bool {
        true
    }
}

/// The `Spanish` language.
#[cfg(feature = "spanish")]
#[derive(Copy, Clone, Debug)]
pub struct Spanish;
#[cfg(feature = "spanish")]
impl Language for Spanish {
    const WORD_LIST: &'static [&'static str] = &spanish::WORDS;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "all-languages")]
    #[test]
    fn validate_word_list_checksum() {
        // Check the sha256sum of the word lists.
        //
        // They are as follows in the [bips](https://github.com/bitcoin/bips/blob/master/bip-0039/bip-0039-wordlists.md):
        //
        //   chinese_simplified.txt  : 5c5942792bd8340cb8b27cd592f1015edf56a8c5b26276ee18a482428e7c5726
        //   chinese_traditional.txt : 417b26b3d8500a4ae3d59717d7011952db6fc2fb84b807f3f94ac734e89c1b5f
        //   czech.txt               : 7e80e161c3e93d9554c2efb78d4e3cebf8fc727e9c52e03b83b94406bdcc95fc
        //   english.txt             : 2f5eed53a4727b4bf8880d8f3f199efc90e58503646d9ff8eff3a2ed3b24dbda
        //   french.txt              : ebc3959ab7801a1df6bac4fa7d970652f1df76b683cd2f4003c941c63d517e59
        //   italian.txt             : d392c49fdb700a24cd1fceb237c1f65dcc128f6b34a8aacb58b59384b5c648c2
        //   japanese.txt            : 2eed0aef492291e061633d7ad8117f1a2b03eb80a29d0e4e3117ac2528d05ffd
        //   korean.txt              : 9e95f86c167de88f450f0aaf89e87f6624a57f973c67b516e338e8e8b8897f60
        //   portuguese.txt          : 2685e9c194c82ae67e10ba59d9ea5345a23dc093e92276fc5361f6667d79cd3f
        //   spanish.txt             : 46846a5a0139d1e3cb77293e521c2865f7bcdb82c44e8d0a06a2cd0ecba48c0b

        use sha2::{Digest, Sha256};
        macro_rules! generate_checksum_test {
            ($lang:ident => $checksum:expr) => {{
                let mut digest = Sha256::new();
                for &word in $lang::WORD_LIST {
                    assert!(unicode_normalization::is_nfkd(word));
                    digest.update(format!("{}\n", word));
                }
                assert_eq!(hex::encode(digest.finalize()), $checksum);
            }};
        }

        generate_checksum_test!(ChineseSimplified => "5c5942792bd8340cb8b27cd592f1015edf56a8c5b26276ee18a482428e7c5726");
        generate_checksum_test!(ChineseTraditional => "417b26b3d8500a4ae3d59717d7011952db6fc2fb84b807f3f94ac734e89c1b5f");
        generate_checksum_test!(Czech => "7e80e161c3e93d9554c2efb78d4e3cebf8fc727e9c52e03b83b94406bdcc95fc");
        generate_checksum_test!(English => "2f5eed53a4727b4bf8880d8f3f199efc90e58503646d9ff8eff3a2ed3b24dbda");
        generate_checksum_test!(French => "ebc3959ab7801a1df6bac4fa7d970652f1df76b683cd2f4003c941c63d517e59");
        generate_checksum_test!(Italian => "d392c49fdb700a24cd1fceb237c1f65dcc128f6b34a8aacb58b59384b5c648c2");
        generate_checksum_test!(Japanese => "2eed0aef492291e061633d7ad8117f1a2b03eb80a29d0e4e3117ac2528d05ffd");
        generate_checksum_test!(Korean => "9e95f86c167de88f450f0aaf89e87f6624a57f973c67b516e338e8e8b8897f60");
        generate_checksum_test!(Portuguese => "2685e9c194c82ae67e10ba59d9ea5345a23dc093e92276fc5361f6667d79cd3f");
        generate_checksum_test!(Spanish => "46846a5a0139d1e3cb77293e521c2865f7bcdb82c44e8d0a06a2cd0ecba48c0b");
    }

    #[cfg(feature = "all-languages")]
    #[test]
    fn word_list_is_sorted() {
        use std::cmp::Ordering;
        fn is_sorted<L: Language>() -> bool {
            L::WORD_LIST.windows(2).all(|w| {
                w[0].partial_cmp(w[1])
                    .map(|o| o != Ordering::Greater)
                    .unwrap_or(false)
            })
        }

        macro_rules! generate_is_sorted_test {
            ($lang:ident) => {
                assert_eq!(is_sorted::<$lang>(), $lang::is_sorted());
            };
        }

        generate_is_sorted_test!(ChineseSimplified);
        generate_is_sorted_test!(ChineseTraditional);
        generate_is_sorted_test!(Czech);
        generate_is_sorted_test!(English);
        generate_is_sorted_test!(French);
        generate_is_sorted_test!(Italian);
        generate_is_sorted_test!(Japanese);
        generate_is_sorted_test!(Korean);
        generate_is_sorted_test!(Portuguese);
        generate_is_sorted_test!(Spanish);
    }

    #[cfg(feature = "all-languages")]
    #[test]
    fn word_list_is_normalized() {
        fn check_normalized<L: Language>() {
            for &word in L::WORD_LIST {
                assert!(
                    unicode_normalization::is_nfkd(word),
                    "word '{}' is not normalized",
                    word
                )
            }
        }

        macro_rules! generate_check_normalized_test {
            ($lang:ident) => {
                check_normalized::<$lang>();
            };
        }

        generate_check_normalized_test!(ChineseSimplified);
        generate_check_normalized_test!(ChineseTraditional);
        generate_check_normalized_test!(Czech);
        generate_check_normalized_test!(English);
        generate_check_normalized_test!(French);
        generate_check_normalized_test!(Italian);
        generate_check_normalized_test!(Japanese);
        generate_check_normalized_test!(Korean);
        generate_check_normalized_test!(Portuguese);
        generate_check_normalized_test!(Spanish);
    }

    #[test]
    fn words_by_prefix() {
        let res = English::words_by_prefix("woo");
        assert_eq!(res, ["wood", "wool"]);

        let res = English::words_by_prefix("");
        assert_eq!(res.len(), 2048);

        let res = English::words_by_prefix("woof");
        assert!(res.is_empty());
    }
}
