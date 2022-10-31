use bip0039::{English, Language, Mnemonic};
use serde::Deserialize;
use unicode_normalization::UnicodeNormalization;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
struct Case {
    entropy: String,
    mnemonic: String,
    passphrase: String,
    seed: String,
    #[allow(dead_code)]
    bip32_xprv: String,
}

#[test]
fn test_all_vectors() {
    // https://github.com/bip32JP/bip32JP.github.io/blob/master/test_EN_BIP39.json
    // The passphrase "TREZOR" is used for all vectors.
    let en_cases = serde_json::from_str::<Vec<Case>>(include_str!("./test_EN_BIP39.json")).unwrap();
    for Case {
        entropy,
        mnemonic,
        passphrase,
        seed,
        ..
    } in en_cases
    {
        test_mnemonic::<English>(&passphrase, &entropy, &mnemonic, &seed);
    }

    #[cfg(feature = "japanese")]
    {
        use bip0039::Japanese;
        // https://github.com/bip32JP/bip32JP.github.io/blob/master/test_JP_BIP39.json
        // Japanese wordlist test with heavily normalized symbols as passphrase
        let jp_cases =
            serde_json::from_str::<Vec<Case>>(include_str!("./test_JP_BIP39.json")).unwrap();
        for Case {
            entropy,
            mnemonic,
            passphrase,
            seed,
            ..
        } in jp_cases
        {
            test_mnemonic::<Japanese>(&passphrase, &entropy, &mnemonic, &seed);
        }
    }
}

fn test_mnemonic<L: Language>(
    passphrase: &str,
    entropy_hex: &str,
    expected_phrase: &str,
    expected_seed_hex: &str,
) {
    let entropy = hex::decode(entropy_hex).unwrap();
    let mnemonic = <Mnemonic<L>>::from_entropy(entropy).unwrap();
    assert_eq!(mnemonic.phrase(), expected_phrase.nfkd().to_string());
    assert!(<Mnemonic<L>>::from_phrase(expected_phrase).is_ok());

    let seed = mnemonic.to_seed(passphrase);
    assert_eq!(hex::encode(&seed[..]), expected_seed_hex);
}
