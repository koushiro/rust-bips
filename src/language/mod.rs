#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use hashbrown::HashMap;
use once_cell::sync::Lazy;

pub struct WordList(Vec<&'static str>);
impl WordList {
    fn gen(words: &'static str) -> Self {
        let list = words.split_whitespace().collect::<Vec<_>>();
        debug_assert!(list.len() == 2048, "Invalid wordlist length");
        Self(list)
    }
}
pub struct WordMap(HashMap<&'static str, usize>);
impl WordMap {
    fn gen(words: &'static str) -> Self {
        let map = words
            .split_whitespace()
            .enumerate()
            .map(|(index, word)| (word, index))
            .collect::<HashMap<_, _>>();
        debug_assert!(map.len() == 2048, "Invalid wordmap length");
        Self(map)
    }
}

#[cfg(feature = "chinese-simplified")]
mod chinese_simplified {
    use super::*;
    pub static WORDMAP: Lazy<WordMap> =
        Lazy::new(|| WordMap::gen(include_str!("chinese_simplified.txt")));
    pub static WORDLIST: Lazy<WordList> =
        Lazy::new(|| WordList::gen(include_str!("chinese_simplified.txt")));
}
#[cfg(feature = "chinese-traditional")]
mod chinese_traditional {
    use super::*;
    pub static WORDMAP: Lazy<WordMap> =
        Lazy::new(|| WordMap::gen(include_str!("chinese_traditional.txt")));
    pub static WORDLIST: Lazy<WordList> =
        Lazy::new(|| WordList::gen(include_str!("chinese_traditional.txt")));
}
#[cfg(feature = "czech")]
mod czech {
    use super::*;
    pub static WORDMAP: Lazy<WordMap> = Lazy::new(|| WordMap::gen(include_str!("czech.txt")));
    pub static WORDLIST: Lazy<WordList> = Lazy::new(|| WordList::gen(include_str!("czech.txt")));
}
mod english {
    use super::*;
    pub static WORDMAP: Lazy<WordMap> = Lazy::new(|| WordMap::gen(include_str!("english.txt")));
    pub static WORDLIST: Lazy<WordList> = Lazy::new(|| WordList::gen(include_str!("english.txt")));
}
#[cfg(feature = "french")]
mod french {
    use super::*;
    pub static WORDMAP: Lazy<WordMap> = Lazy::new(|| WordMap::gen(include_str!("french.txt")));
    pub static WORDLIST: Lazy<WordList> = Lazy::new(|| WordList::gen(include_str!("french.txt")));
}
#[cfg(feature = "italian")]
mod italian {
    use super::*;
    pub static WORDMAP: Lazy<WordMap> = Lazy::new(|| WordMap::gen(include_str!("italian.txt")));
    pub static WORDLIST: Lazy<WordList> = Lazy::new(|| WordList::gen(include_str!("italian.txt")));
}
#[cfg(feature = "japanese")]
mod japanese {
    use super::*;
    pub static WORDMAP: Lazy<WordMap> = Lazy::new(|| WordMap::gen(include_str!("japanese.txt")));
    pub static WORDLIST: Lazy<WordList> = Lazy::new(|| WordList::gen(include_str!("japanese.txt")));
}
#[cfg(feature = "korean")]
mod korean {
    use super::*;
    pub static WORDMAP: Lazy<WordMap> = Lazy::new(|| WordMap::gen(include_str!("korean.txt")));
    pub static WORDLIST: Lazy<WordList> = Lazy::new(|| WordList::gen(include_str!("korean.txt")));
}
#[cfg(feature = "spanish")]
mod spanish {
    use super::*;
    pub static WORDMAP: Lazy<WordMap> = Lazy::new(|| WordMap::gen(include_str!("spanish.txt")));
    pub static WORDLIST: Lazy<WordList> = Lazy::new(|| WordList::gen(include_str!("spanish.txt")));
}

/// Language to be used for the mnemonic phrase.
///
/// The English language is always available, other languages are enabled using
/// the compilation features.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Language {
    /// The English language.
    English,
    /// The Simplified Chinese language.
    #[cfg(feature = "chinese-simplified")]
    SimplifiedChinese,
    /// The Traditional Chinese language.
    #[cfg(feature = "chinese-traditional")]
    TraditionalChinese,
    /// The Czech language.
    #[cfg(feature = "czech")]
    Czech,
    /// The French language.
    #[cfg(feature = "french")]
    French,
    /// The Italian language.
    #[cfg(feature = "italian")]
    Italian,
    /// The Japanese language.
    #[cfg(feature = "japanese")]
    Japanese,
    /// The Korean language.
    #[cfg(feature = "korean")]
    Korean,
    /// The Spanish language.
    #[cfg(feature = "spanish")]
    Spanish,
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

impl core::fmt::Display for Language {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, f)
    }
}

impl Language {
    /// The list of supported languages.
    /// Language support is managed by compile features.
    pub fn all() -> &'static [Language] {
        &[
            Language::English,
            #[cfg(feature = "chinese-simplified")]
            Language::SimplifiedChinese,
            #[cfg(feature = "chinese-traditional")]
            Language::TraditionalChinese,
            #[cfg(feature = "czech")]
            Language::Czech,
            #[cfg(feature = "french")]
            Language::French,
            #[cfg(feature = "italian")]
            Language::Italian,
            #[cfg(feature = "japanese")]
            Language::Japanese,
            #[cfg(feature = "korean")]
            Language::Korean,
            #[cfg(feature = "spanish")]
            Language::Spanish,
        ]
    }

    /// The word list for this language.
    #[inline]
    pub(crate) fn word_list(self) -> &'static WordList {
        match self {
            Language::English => &english::WORDLIST,
            #[cfg(feature = "chinese-simplified")]
            Language::SimplifiedChinese => &chinese_simplified::WORDLIST,
            #[cfg(feature = "chinese-traditional")]
            Language::TraditionalChinese => &chinese_traditional::WORDLIST,
            #[cfg(feature = "czech")]
            Language::Czech => &czech::WORDLIST,
            #[cfg(feature = "french")]
            Language::French => &french::WORDLIST,
            #[cfg(feature = "italian")]
            Language::Italian => &italian::WORDLIST,
            #[cfg(feature = "japanese")]
            Language::Japanese => &japanese::WORDLIST,
            #[cfg(feature = "korean")]
            Language::Korean => &korean::WORDLIST,
            #[cfg(feature = "spanish")]
            Language::Spanish => &spanish::WORDLIST,
        }
    }

    /// Returns the word of `index` in the word list.
    #[inline]
    pub(crate) fn word_of(self, index: usize) -> &'static str {
        debug_assert!(index < 2048, "Invalid wordlist index");
        self.word_list().0[index]
    }

    /// Returns a word map that allows word -> index lookups in the word list.
    #[inline]
    pub(crate) fn word_map(self) -> &'static WordMap {
        match self {
            Language::English => &english::WORDMAP,
            #[cfg(feature = "chinese-simplified")]
            Language::SimplifiedChinese => &chinese_simplified::WORDMAP,
            #[cfg(feature = "chinese-traditional")]
            Language::TraditionalChinese => &chinese_traditional::WORDMAP,
            #[cfg(feature = "czech")]
            Language::Czech => &czech::WORDMAP,
            #[cfg(feature = "french")]
            Language::French => &french::WORDMAP,
            #[cfg(feature = "italian")]
            Language::Italian => &italian::WORDMAP,
            #[cfg(feature = "japanese")]
            Language::Japanese => &japanese::WORDMAP,
            #[cfg(feature = "korean")]
            Language::Korean => &korean::WORDMAP,
            #[cfg(feature = "spanish")]
            Language::Spanish => &spanish::WORDMAP,
        }
    }

    /// Returns the index of the word in the word list.
    #[inline]
    pub(crate) fn index_of(self, word: &str) -> Option<usize> {
        self.word_map().0.get(word).cloned()
    }

    /// Returns words from the word list that start with the given prefix.
    pub fn words_by_prefix(self, prefix: &str) -> &[&'static str] {
        // The words in the word list are ordered lexicographically.
        // This means that we cannot use `binary_search` to find words more efficiently,
        // because the Rust ordering is based on the byte values.
        // However, it does mean that words that share a prefix will follow each other.

        let first = match self
            .word_list()
            .0
            .iter()
            .position(|w| w.starts_with(prefix))
        {
            Some(i) => i,
            None => return &[],
        };
        let count = self.word_list().0[first..]
            .iter()
            .take_while(|w| w.starts_with(prefix))
            .count();
        &self.word_list().0[first..first + count]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn words_by_prefix() {
        let lang = Language::English;

        let res = lang.words_by_prefix("woo");
        assert_eq!(res, ["wood", "wool"]);

        let res = lang.words_by_prefix("");
        assert_eq!(res.len(), 2048);

        let res = lang.words_by_prefix("woof");
        assert!(res.is_empty());
    }

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
        //   spanish.txt             : 46846a5a0139d1e3cb77293e521c2865f7bcdb82c44e8d0a06a2cd0ecba48c0b

        use sha2::{Digest, Sha256};

        let checksums = [
            (
                Language::SimplifiedChinese,
                "5c5942792bd8340cb8b27cd592f1015edf56a8c5b26276ee18a482428e7c5726",
            ),
            (
                Language::TraditionalChinese,
                "417b26b3d8500a4ae3d59717d7011952db6fc2fb84b807f3f94ac734e89c1b5f",
            ),
            (
                Language::Czech,
                "7e80e161c3e93d9554c2efb78d4e3cebf8fc727e9c52e03b83b94406bdcc95fc",
            ),
            (
                Language::English,
                "2f5eed53a4727b4bf8880d8f3f199efc90e58503646d9ff8eff3a2ed3b24dbda",
            ),
            (
                Language::French,
                "ebc3959ab7801a1df6bac4fa7d970652f1df76b683cd2f4003c941c63d517e59",
            ),
            (
                Language::Italian,
                "d392c49fdb700a24cd1fceb237c1f65dcc128f6b34a8aacb58b59384b5c648c2",
            ),
            (
                Language::Japanese,
                "2eed0aef492291e061633d7ad8117f1a2b03eb80a29d0e4e3117ac2528d05ffd",
            ),
            (
                Language::Korean,
                "9e95f86c167de88f450f0aaf89e87f6624a57f973c67b516e338e8e8b8897f60",
            ),
            (
                Language::Spanish,
                "46846a5a0139d1e3cb77293e521c2865f7bcdb82c44e8d0a06a2cd0ecba48c0b",
            ),
        ];

        for &(lang, sha256sum) in &checksums {
            let mut digest = Sha256::new();
            for &word in lang.word_list().0.as_slice() {
                assert!(unicode_normalization::is_nfkd(word));
                digest.update(format!("{}\n", word));
            }
            assert_eq!(hex::encode(digest.finalize()), sha256sum);
        }
    }
}
