//! Supported languages for BIP-0039.
//!
//! This module is the public surface for language support.
//!
//! - [`Language`] is the only public capability trait exposed for consumers.
//! - [`AnyLanguage`] provides a lightweight runtime handle to a `Language` implementation.
//! - Built-in languages are enabled via Cargo features and backed by crate-private generated
//!   wordlists.

use core::{fmt, ptr};

mod wordlist;
use self::wordlist::*;

/// Language to be used for the mnemonic phrase.
///
/// Consumers may implement this trait for their own language types by providing:
/// - [`Language::word_of`]
/// - [`Language::index_of`]
///
/// Built-in languages implement a crate-private `WordlistProvider` and automatically
/// get this trait via the blanket impl below.
///
/// # Requirements
///
/// - `word_of(index)` must return a valid word for all indices `0..2048`.
/// - `index_of(word)` must return the correct index (BIP-0039 order) for all words in the language
///   wordlist; return `None` for unknown words.
pub trait Language: Sized {
    // NOTE (planned breaking change): we intend to add the following method in the next
    // minor release (e.g. `0.14.0`), and treat it as a breaking change for external
    // `Language` implementations:
    //
    // /// Returns the full BIP-0039 word list for this language (2048 words) in BIP-0039 order.
    // ///
    // /// Notes:
    // /// - This returns the full underlying word list, not just a view of a specific mnemonic.
    // /// - The returned words must be NFKD-normalized and unique.
    // fn words() -> &'static [&'static str; 2048];

    /// Returns the word at `index` (BIP-0039 order).
    fn word_of(index: usize) -> &'static str;

    /// Returns the index of `word` in the word list (BIP-0039 order).
    fn index_of(word: &str) -> Option<usize>;
}

impl<T: WordlistProvider> Language for T {
    // fn words() -> &'static [&'static str; 2048] {
    //     <T as WordlistProvider>::wordlist().words
    // }

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

/// A runtime handle to a [`Language`] implementation.
///
/// `Mnemonic<L>` selects the language at compile-time via a type parameter, which is great for
/// monomorphization and `no_std` environments, but can be awkward when you want to select a
/// language at runtime (e.g. from user configuration).
///
/// `AnyLanguage` is a lightweight, `Copy` handle backed by function pointers that allows selecting
/// a language at runtime without changing the existing `Language` trait or the generic `Mnemonic`
/// API.
#[derive(Copy, Clone)]
pub struct AnyLanguage {
    word_of: fn(usize) -> &'static str,
    index_of: fn(&str) -> Option<usize>,
}

impl fmt::Debug for AnyLanguage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AnyLanguage").finish_non_exhaustive()
    }
}

impl PartialEq for AnyLanguage {
    fn eq(&self, other: &Self) -> bool {
        ptr::fn_addr_eq(self.word_of, other.word_of)
            && ptr::fn_addr_eq(self.index_of, other.index_of)
    }
}

impl Eq for AnyLanguage {}

impl AnyLanguage {
    /// Creates an [`AnyLanguage`] from the given wordlist lookup functions.
    pub(crate) const fn new(
        word_of: fn(usize) -> &'static str,
        index_of: fn(&str) -> Option<usize>,
    ) -> Self {
        Self { word_of, index_of }
    }

    /// Creates an [`AnyLanguage`] from a [`Language`] implementation.
    #[inline]
    pub const fn of<L: Language>() -> Self {
        Self::new(L::word_of, L::index_of)
    }

    /// Returns the word at `index` (BIP-0039 order).
    #[inline]
    pub(crate) fn word_of(self, index: usize) -> &'static str {
        (self.word_of)(index)
    }

    /// Returns the index of `word` in the word list (BIP-0039 order).
    #[inline]
    pub(crate) fn index_of(self, word: &str) -> Option<usize> {
        (self.index_of)(word)
    }
}

/// Built-in BIP-0039 languages enabled in this build.
///
/// Variants are gated by Cargo features (English is always available).
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum BuiltInLanguage {
    /// English (always available).
    English,
    /// Simplified Chinese.
    #[cfg(feature = "chinese-simplified")]
    ChineseSimplified,
    /// Traditional Chinese.
    #[cfg(feature = "chinese-traditional")]
    ChineseTraditional,
    /// Czech.
    #[cfg(feature = "czech")]
    Czech,
    /// French.
    #[cfg(feature = "french")]
    French,
    /// Italian.
    #[cfg(feature = "italian")]
    Italian,
    /// Japanese.
    #[cfg(feature = "japanese")]
    Japanese,
    /// Korean.
    #[cfg(feature = "korean")]
    Korean,
    /// Portuguese.
    #[cfg(feature = "portuguese")]
    Portuguese,
    /// Spanish.
    #[cfg(feature = "spanish")]
    Spanish,
}

impl BuiltInLanguage {
    /// Returns an [`AnyLanguage`] for this built-in language.
    pub const fn as_any_language(self) -> AnyLanguage {
        match self {
            Self::English => AnyLanguage::of::<English>(),
            #[cfg(feature = "chinese-simplified")]
            Self::ChineseSimplified => AnyLanguage::of::<ChineseSimplified>(),
            #[cfg(feature = "chinese-traditional")]
            Self::ChineseTraditional => AnyLanguage::of::<ChineseTraditional>(),
            #[cfg(feature = "czech")]
            Self::Czech => AnyLanguage::of::<Czech>(),
            #[cfg(feature = "french")]
            Self::French => AnyLanguage::of::<French>(),
            #[cfg(feature = "italian")]
            Self::Italian => AnyLanguage::of::<Italian>(),
            #[cfg(feature = "japanese")]
            Self::Japanese => AnyLanguage::of::<Japanese>(),
            #[cfg(feature = "korean")]
            Self::Korean => AnyLanguage::of::<Korean>(),
            #[cfg(feature = "portuguese")]
            Self::Portuguese => AnyLanguage::of::<Portuguese>(),
            #[cfg(feature = "spanish")]
            Self::Spanish => AnyLanguage::of::<Spanish>(),
        }
    }
}

impl From<BuiltInLanguage> for AnyLanguage {
    #[inline]
    fn from(value: BuiltInLanguage) -> Self {
        value.as_any_language()
    }
}

macro_rules! define_builtin_language {
    (
        $(doc = $doc:literal,)+
        name = $name:ident,
        wordlist = $wordlist:ident $(,)?
    ) => {
        $(#[doc = $doc])*
        #[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
        pub struct $name;

        impl WordlistProvider for $name {
            #[inline]
            fn wordlist() -> &'static Wordlist {
                &wordlists::$wordlist::WORDLIST
            }
        }

        impl $name {
            #[doc = concat!(
                "Returns the full BIP-0039 `",
                stringify!($name),
                "` word list (2048 words) in BIP-0039 order."
            )]
            #[inline]
            pub fn words() -> &'static [&'static str; 2048] {
                &wordlists::$wordlist::WORDS
            }
        }
    };
    (
        $(doc = $doc:literal,)+
        name = $name:ident,
        wordlist = $wordlist:ident,
        feature = $feature:literal,
        $(,)?
    ) => {
        $(#[doc = $doc])*
        #[cfg(feature = $feature)]
        #[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
        pub struct $name;

        #[cfg(feature = $feature)]
        impl WordlistProvider for $name {
            #[inline]
            fn wordlist() -> &'static Wordlist {
                &wordlists::$wordlist::WORDLIST
            }
        }

        #[cfg(feature = $feature)]
        impl $name {
            #[doc = concat!(
                "Returns the full BIP-0039 `",
                stringify!($name),
                "` word list (2048 words) in BIP-0039 order."
            )]
            #[inline]
            pub fn words() -> &'static [&'static str; 2048] {
                &wordlists::$wordlist::WORDS
            }
        }
    };
}

define_builtin_language!(
    doc = r#"The `English` language.

The `English` language is always available; other languages are enabled via
compilation features."#,
    name = English,
    wordlist = english,
);

define_builtin_language!(
    doc = "The `Simplified Chinese` language.",
    name = ChineseSimplified,
    wordlist = chinese_simplified,
    feature = "chinese-simplified",
);

define_builtin_language!(
    doc = "The `Traditional Chinese` language.",
    name = ChineseTraditional,
    wordlist = chinese_traditional,
    feature = "chinese-traditional",
);

define_builtin_language!(
    doc = "The `Czech` language.",
    name = Czech,
    wordlist = czech,
    feature = "czech",
);

define_builtin_language!(
    doc = "The `French` language.",
    name = French,
    wordlist = french,
    feature = "french",
);

define_builtin_language!(
    doc = "The `Italian` language.",
    name = Italian,
    wordlist = italian,
    feature = "italian",
);

define_builtin_language!(
    doc = "The `Japanese` language.",
    name = Japanese,
    wordlist = japanese,
    feature = "japanese",
);

define_builtin_language!(
    doc = "The `Korean` language.",
    name = Korean,
    wordlist = korean,
    feature = "korean",
);

define_builtin_language!(
    doc = "The `Portuguese` language.",
    name = Portuguese,
    wordlist = portuguese,
    feature = "portuguese",
);

define_builtin_language!(
    doc = "The `Spanish` language.",
    name = Spanish,
    wordlist = spanish,
    feature = "spanish",
);

#[cfg(test)]
mod tests {
    use super::*;

    fn calculate_checksum(name: &str, words: &[&'static str; 2048]) -> String {
        use sha2::{Digest, Sha256};

        let mut digest = Sha256::new();

        for (i, &word) in words.iter().enumerate() {
            assert!(
                unicode_normalization::is_nfkd(word),
                "word list '{name}' is not NFKD normalized at index {i}",
            );
            digest.update(word.as_bytes());
            digest.update(b"\n");
        }

        const_hex::encode(digest.finalize())
    }

    #[test]
    fn validate_word_list_checksums() {
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
            expected_hex_checksum: &'static str,
            words: fn() -> &'static [&'static str; 2048],
            word_of: fn(usize) -> &'static str,
            index_of: fn(&str) -> Option<usize>,
        }

        let mut cases: Vec<Case> = Vec::new();

        cases.push(Case {
            name: "english",
            expected_hex_checksum: "2f5eed53a4727b4bf8880d8f3f199efc90e58503646d9ff8eff3a2ed3b24dbda",
            words: English::words,
            word_of: <English as Language>::word_of,
            index_of: <English as Language>::index_of,
        });

        #[cfg(feature = "chinese-simplified")]
        cases.push(Case {
            name: "chinese-simplified",
            expected_hex_checksum: "5c5942792bd8340cb8b27cd592f1015edf56a8c5b26276ee18a482428e7c5726",
            words: ChineseSimplified::words,
            word_of: <ChineseSimplified as Language>::word_of,
            index_of: <ChineseSimplified as Language>::index_of,
        });

        #[cfg(feature = "chinese-traditional")]
        cases.push(Case {
            name: "chinese-traditional",
            expected_hex_checksum: "417b26b3d8500a4ae3d59717d7011952db6fc2fb84b807f3f94ac734e89c1b5f",
            words: ChineseTraditional::words,
            word_of: <ChineseTraditional as Language>::word_of,
            index_of: <ChineseTraditional as Language>::index_of,
        });

        #[cfg(feature = "czech")]
        cases.push(Case {
            name: "czech",
            expected_hex_checksum: "7e80e161c3e93d9554c2efb78d4e3cebf8fc727e9c52e03b83b94406bdcc95fc",
            words: Czech::words,
            word_of: <Czech as Language>::word_of,
            index_of: <Czech as Language>::index_of,
        });

        #[cfg(feature = "french")]
        cases.push(Case {
            name: "french",
            expected_hex_checksum: "ebc3959ab7801a1df6bac4fa7d970652f1df76b683cd2f4003c941c63d517e59",
            words: French::words,
            word_of: <French as Language>::word_of,
            index_of: <French as Language>::index_of,
        });

        #[cfg(feature = "italian")]
        cases.push(Case {
            name: "italian",
            expected_hex_checksum: "d392c49fdb700a24cd1fceb237c1f65dcc128f6b34a8aacb58b59384b5c648c2",
            words: Italian::words,
            word_of: <Italian as Language>::word_of,
            index_of: <Italian as Language>::index_of,
        });

        #[cfg(feature = "japanese")]
        cases.push(Case {
            name: "japanese",
            expected_hex_checksum: "2eed0aef492291e061633d7ad8117f1a2b03eb80a29d0e4e3117ac2528d05ffd",
            words: Japanese::words,
            word_of: <Japanese as Language>::word_of,
            index_of: <Japanese as Language>::index_of,
        });

        #[cfg(feature = "korean")]
        cases.push(Case {
            name: "korean",
            expected_hex_checksum: "9e95f86c167de88f450f0aaf89e87f6624a57f973c67b516e338e8e8b8897f60",
            words: Korean::words,
            word_of: <Korean as Language>::word_of,
            index_of: <Korean as Language>::index_of,
        });

        #[cfg(feature = "portuguese")]
        cases.push(Case {
            name: "portuguese",
            expected_hex_checksum: "2685e9c194c82ae67e10ba59d9ea5345a23dc093e92276fc5361f6667d79cd3f",
            words: Portuguese::words,
            word_of: <Portuguese as Language>::word_of,
            index_of: <Portuguese as Language>::index_of,
        });

        #[cfg(feature = "spanish")]
        cases.push(Case {
            name: "spanish",
            expected_hex_checksum: "46846a5a0139d1e3cb77293e521c2865f7bcdb82c44e8d0a06a2cd0ecba48c0b",
            words: Spanish::words,
            word_of: <Spanish as Language>::word_of,
            index_of: <Spanish as Language>::index_of,
        });

        for case in cases {
            let words = (case.words)();
            assert_eq!(words.len(), 2048);

            let actual_hex_checksum = calculate_checksum(case.name, words);

            for (i, &word) in words.iter().enumerate() {
                assert_eq!((case.word_of)(i), word);
                assert_eq!((case.index_of)(word), Some(i));
            }

            assert_eq!(
                actual_hex_checksum, case.expected_hex_checksum,
                "checksum mismatch for language '{}'",
                case.name
            );
        }
    }

    #[test]
    fn test_builtin_language_match() {
        macro_rules! assert_builtin {
            ($enum:expr, $ty:ty) => {{
                let from_enum = $enum.as_any_language();
                let from_type = AnyLanguage::of::<$ty>();
                assert_eq!(from_enum, from_type);
                assert_eq!(AnyLanguage::from($enum), from_type);
            }};
        }

        assert_builtin!(BuiltInLanguage::English, English);

        #[cfg(feature = "chinese-simplified")]
        assert_builtin!(BuiltInLanguage::ChineseSimplified, ChineseSimplified);

        #[cfg(feature = "chinese-traditional")]
        assert_builtin!(BuiltInLanguage::ChineseTraditional, ChineseTraditional);

        #[cfg(feature = "czech")]
        assert_builtin!(BuiltInLanguage::Czech, Czech);

        #[cfg(feature = "french")]
        assert_builtin!(BuiltInLanguage::French, French);

        #[cfg(feature = "italian")]
        assert_builtin!(BuiltInLanguage::Italian, Italian);

        #[cfg(feature = "japanese")]
        assert_builtin!(BuiltInLanguage::Japanese, Japanese);

        #[cfg(feature = "korean")]
        assert_builtin!(BuiltInLanguage::Korean, Korean);

        #[cfg(feature = "portuguese")]
        assert_builtin!(BuiltInLanguage::Portuguese, Portuguese);

        #[cfg(feature = "spanish")]
        assert_builtin!(BuiltInLanguage::Spanish, Spanish);
    }
}
