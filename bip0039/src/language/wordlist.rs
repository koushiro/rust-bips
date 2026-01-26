//! Crate-private wordlist internals.
//!
//! This module is an implementation detail of this crate.
//! Consumers should only depend on `crate::language::Language`.
//!
//! The build script generates per-language wordlist files into `OUT_DIR`.
//! Each generated file defines:
//! - `WORDS`: `&'static str` array in BIP-0039 order (index -> word)
//! - `INDEX`: `phf::Map<&'static str, u16>` (word -> index)
//! - `WORDLIST`: `crate::language::wordlist::Wordlist`

/// A generated word list + fast lookup index for a language.
///
/// This type is crate-private on purpose: external users should not depend on
/// internal representation choices (e.g. `phf`, integer widths, etc).
pub(crate) struct Wordlist {
    /// Index => word (BIP-0039 order).
    pub(crate) words: &'static [&'static str; 2048],
    /// Word => index (BIP-0039 order).
    ///
    /// Stored as a reference so generated code can safely refer to a `static INDEX`
    /// without moving it.
    pub(crate) index: &'static phf::Map<&'static str, u16>,
}

/// Minimal internal capability trait: a type that can provide a static [`Wordlist`].
///
/// Built-in language marker types (e.g. `English`) implement this trait by
/// returning a reference to their generated `WORDLIST`.
pub(crate) trait WordlistProvider: Sized {
    fn wordlist() -> &'static Wordlist;
}

/// Per-language BIP-0039 wordlists generated at build time into `OUT_DIR`.
///
/// This module centralizes the `include!`s so we don't need one file per language.
pub(crate) mod wordlists {
    pub(crate) mod english {
        include!(concat!(env!("OUT_DIR"), "/bip0039_wordlist_english.rs"));
    }

    #[cfg(feature = "chinese-simplified")]
    pub(crate) mod chinese_simplified {
        include!(concat!(env!("OUT_DIR"), "/bip0039_wordlist_chinese_simplified.rs"));
    }

    #[cfg(feature = "chinese-traditional")]
    pub(crate) mod chinese_traditional {
        include!(concat!(env!("OUT_DIR"), "/bip0039_wordlist_chinese_traditional.rs"));
    }

    #[cfg(feature = "czech")]
    pub(crate) mod czech {
        include!(concat!(env!("OUT_DIR"), "/bip0039_wordlist_czech.rs"));
    }

    #[cfg(feature = "french")]
    pub(crate) mod french {
        include!(concat!(env!("OUT_DIR"), "/bip0039_wordlist_french.rs"));
    }

    #[cfg(feature = "italian")]
    pub(crate) mod italian {
        include!(concat!(env!("OUT_DIR"), "/bip0039_wordlist_italian.rs"));
    }

    #[cfg(feature = "japanese")]
    pub(crate) mod japanese {
        include!(concat!(env!("OUT_DIR"), "/bip0039_wordlist_japanese.rs"));
    }

    #[cfg(feature = "korean")]
    pub(crate) mod korean {
        include!(concat!(env!("OUT_DIR"), "/bip0039_wordlist_korean.rs"));
    }

    #[cfg(feature = "portuguese")]
    pub(crate) mod portuguese {
        include!(concat!(env!("OUT_DIR"), "/bip0039_wordlist_portuguese.rs"));
    }

    #[cfg(feature = "spanish")]
    pub(crate) mod spanish {
        include!(concat!(env!("OUT_DIR"), "/bip0039_wordlist_spanish.rs"));
    }
}
