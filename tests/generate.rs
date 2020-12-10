use bip0039::{Language, Mnemonic, MnemonicWordCount};

#[cfg(feature = "rand")]
fn generate(language: Language, expected_word_count: MnemonicWordCount) {
    let mnemonic = Mnemonic::generate_in(language, expected_word_count);
    let actual_word_count = mnemonic.phrase().split_whitespace().count();
    assert_eq!(actual_word_count, expected_word_count.word_count());
    assert_eq!(mnemonic.to_seed("").len(), 64);
}

#[cfg(feature = "rand")]
#[test]
fn test_generate() {
    for language in Language::all().iter().cloned() {
        generate(language, MnemonicWordCount::Words12);
        generate(language, MnemonicWordCount::Words15);
        generate(language, MnemonicWordCount::Words18);
        generate(language, MnemonicWordCount::Words21);
        generate(language, MnemonicWordCount::Words24);
    }
}
