#[cfg(feature = "rand")]
#[test]
fn test_generate() {
    use bip0039::{Count, Language, Mnemonic, language};

    fn generate<L: Language>(expected_word_count: Count) {
        let mnemonic = <Mnemonic<L>>::generate(expected_word_count);
        let actual_word_count = mnemonic.phrase().split_whitespace().count();
        assert_eq!(actual_word_count, expected_word_count.word_count());
        assert_eq!(mnemonic.to_seed("").len(), 64);
    }

    macro_rules! generate_tests {
        ($lang:path) => {{
            generate::<$lang>(Count::Words12);
            generate::<$lang>(Count::Words15);
            generate::<$lang>(Count::Words18);
            generate::<$lang>(Count::Words21);
            generate::<$lang>(Count::Words24);
        }};
    }

    #[cfg(feature = "chinese-simplified")]
    generate_tests!(language::ChineseSimplified);
    #[cfg(feature = "chinese-traditional")]
    generate_tests!(language::ChineseTraditional);
    #[cfg(feature = "czech")]
    generate_tests!(language::Czech);
    generate_tests!(language::English);
    #[cfg(feature = "french")]
    generate_tests!(language::French);
    #[cfg(feature = "italian")]
    generate_tests!(language::Italian);
    #[cfg(feature = "japanese")]
    generate_tests!(language::Japanese);
    #[cfg(feature = "korean")]
    generate_tests!(language::Korean);
    #[cfg(feature = "portuguese")]
    generate_tests!(language::Portuguese);
    #[cfg(feature = "spanish")]
    generate_tests!(language::Spanish);
}
