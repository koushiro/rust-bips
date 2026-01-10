use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

const WORDS: [usize; 4] = [12, 15, 18, 24];

fn bench_generate(c: &mut Criterion) {
    for words in WORDS {
        let mut group = c.benchmark_group("generate");

        group.bench_function(format!("tiny-bip39 ({} words)", words), |b| {
            use tiny_bip39::{Language, Mnemonic, MnemonicType};

            let mnemonic_type = match words {
                12 => MnemonicType::Words12,
                15 => MnemonicType::Words15,
                18 => MnemonicType::Words18,
                24 => MnemonicType::Words24,
                _ => unreachable!("unsupported word count"),
            };

            b.iter(|| {
                let mnemonic = black_box(Mnemonic::new(mnemonic_type, Language::English));
                let _phrase = mnemonic.phrase();
            });
        });

        group.bench_function(format!("bip39 ({} words)", words), |b| {
            use bip39::Mnemonic;

            b.iter(|| {
                let _phrase = black_box({
                    let mnemonic = Mnemonic::generate(words).unwrap();
                    mnemonic.to_string()
                });
            });
        });

        group.bench_function(format!("coins-bip39 ({} words)", words), |b| {
            use coins_bip39::{English, Mnemonic};

            b.iter(|| {
                let _phrase = black_box({
                    let mut rng = rand::rng();
                    let mnemonic =
                        <Mnemonic<English>>::from_rng_with_count(&mut rng, words).unwrap();
                    mnemonic.to_phrase()
                });
            });
        });

        group.bench_function(format!("bip0039 ({} words)", words), |b| {
            use bip0039::{Count, Mnemonic};

            let count = match words {
                12 => Count::Words12,
                15 => Count::Words15,
                18 => Count::Words18,
                24 => Count::Words24,
                _ => unreachable!("unsupported word count"),
            };

            b.iter(|| {
                let mnemonic = black_box(<Mnemonic>::generate(count));
                let _phrase = mnemonic.phrase();
            });
        });

        group.finish();
    }
}

fn bench_from_entropy(c: &mut Criterion) {
    for words in WORDS {
        let mut group = c.benchmark_group("from_entropy");

        group.bench_function(format!("tiny-bip39 ({} words)", words), |b| {
            use tiny_bip39::{Language, Mnemonic, MnemonicType};

            let entropy = {
                let mnemonic_type = match words {
                    12 => MnemonicType::Words12,
                    15 => MnemonicType::Words15,
                    18 => MnemonicType::Words18,
                    24 => MnemonicType::Words24,
                    _ => unreachable!("unsupported word count"),
                };
                let m = Mnemonic::new(mnemonic_type, Language::English);
                m.entropy().to_vec()
            };

            b.iter(|| {
                let mnemonic =
                    black_box(Mnemonic::from_entropy(&entropy, Language::English).unwrap());
                let _phrase = mnemonic.phrase();
            });
        });

        group.bench_function(format!("bip39 ({} words)", words), |b| {
            use bip39::Mnemonic;

            let entropy = {
                let m = Mnemonic::generate(words).unwrap();
                m.to_entropy()
            };

            b.iter(|| {
                let _phrase = black_box({
                    let mnemonic = Mnemonic::from_entropy(&entropy).unwrap();
                    mnemonic.to_string()
                });
            });
        });

        group.bench_function(format!("coins-bip39 ({} words)", words), |b| {
            use coins_bip39::{English, Entropy, Mnemonic};

            let entropy: Entropy = {
                let bytes: usize = match words {
                    12 => 16,
                    15 => 20,
                    18 => 24,
                    21 => 28,
                    24 => 32,
                    _ => unreachable!("unsupported word count"),
                };
                let mut rng = rand::rng();
                Entropy::from_rng(bytes, &mut rng).unwrap()
            };

            b.iter(|| {
                let _phrase = black_box({
                    let mnemonic = <Mnemonic<English>>::new_from_entropy(entropy);
                    mnemonic.to_phrase()
                });
            });
        });

        group.bench_function(format!("bip0039 ({} words)", words), |b| {
            use bip0039::{Count, Mnemonic};

            let entropy = {
                let count = match words {
                    12 => Count::Words12,
                    15 => Count::Words15,
                    18 => Count::Words18,
                    24 => Count::Words24,
                    _ => unreachable!("unsupported word count"),
                };
                let m = <Mnemonic>::generate(count);
                m.entropy().to_vec()
            };

            b.iter(|| {
                let mnemonic = black_box(<Mnemonic>::from_entropy(entropy.as_slice()).unwrap());
                let _phrase = mnemonic.phrase();
            });
        });

        group.finish();
    }
}

fn bench_from_phrase(c: &mut Criterion) {
    for words in WORDS {
        let mut group = c.benchmark_group("from_phrase");

        group.bench_function(format!("tiny-bip39 ({} words)", words), |b| {
            use tiny_bip39::{Language, Mnemonic, MnemonicType};

            let phrase = {
                let mnemonic_type = match words {
                    12 => MnemonicType::Words12,
                    15 => MnemonicType::Words15,
                    18 => MnemonicType::Words18,
                    24 => MnemonicType::Words24,
                    _ => unreachable!("unsupported word count"),
                };
                let m = Mnemonic::new(mnemonic_type, Language::English);
                m.phrase().to_owned()
            };

            b.iter(|| {
                let mnemonic =
                    black_box(Mnemonic::from_phrase(&phrase, Language::English).unwrap());
                let _entropy = mnemonic.entropy();
            });
        });

        group.bench_function(format!("bip39 ({} words)", words), |b| {
            use bip39::{Language, Mnemonic};

            let phrase = {
                let m = Mnemonic::generate(words).unwrap();
                m.to_string()
            };

            b.iter(|| {
                let _entropy = black_box({
                    let mnemonic = Mnemonic::parse_in(Language::English, &phrase).unwrap();
                    mnemonic.to_entropy_array()
                });
            });
        });

        group.bench_function(format!("coins-bip39 ({} words)", words), |b| {
            use coins_bip39::{English, Mnemonic};

            let phrase: String = {
                let mut rng = rand::rng();
                let m = <Mnemonic<English>>::from_rng_with_count(&mut rng, words).unwrap();
                m.to_phrase()
            };

            b.iter(|| {
                let _mnemonic = black_box(<Mnemonic<English>>::new_from_phrase(&phrase).unwrap());
            });
        });

        group.bench_function(format!("bip0039 ({} words)", words), |b| {
            use bip0039::{Count, Mnemonic};

            let phrase = {
                let count = match words {
                    12 => Count::Words12,
                    15 => Count::Words15,
                    18 => Count::Words18,
                    24 => Count::Words24,
                    _ => unreachable!("unsupported word count"),
                };
                let m = <Mnemonic>::generate(count);
                m.phrase().to_owned()
            };

            b.iter(|| {
                let mnemonic = black_box(<Mnemonic>::from_phrase(&phrase).unwrap());
                let _entropy = mnemonic.entropy();
            });
        });

        group.finish();
    }
}

fn bench_from_normalized_phrase(c: &mut Criterion) {
    for words in WORDS {
        let mut group = c.benchmark_group("from_normalized_phrase");

        group.bench_function(format!("bip39 ({} words)", words), |b| {
            use bip39::{Language, Mnemonic};

            let phrase = {
                let m = Mnemonic::generate(words).unwrap();
                m.to_string()
            };

            b.iter(|| {
                let _entropy = black_box({
                    let mnemonic =
                        Mnemonic::parse_in_normalized(Language::English, &phrase).unwrap();
                    mnemonic.to_entropy_array()
                });
            });
        });

        group.bench_function(format!("bip0039 ({} words)", words), |b| {
            use bip0039::{Count, Mnemonic};

            let phrase = {
                let count = match words {
                    12 => Count::Words12,
                    15 => Count::Words15,
                    18 => Count::Words18,
                    24 => Count::Words24,
                    _ => unreachable!("unsupported word count"),
                };
                let m = <Mnemonic>::generate(count);
                m.phrase().to_owned()
            };

            b.iter(|| {
                let mnemonic = black_box(<Mnemonic>::from_normalized_phrase(&phrase).unwrap());
                let _entropy = mnemonic.entropy();
            });
        });

        group.finish();
    }
}

fn bench_to_seed(c: &mut Criterion) {
    for words in WORDS {
        let mut group = c.benchmark_group("to_seed");

        group.bench_function(format!("tiny-bip39 ({} words)", words), |b| {
            use tiny_bip39::{Language, Mnemonic, MnemonicType, Seed};

            let mnemonic = {
                let mnemonic_type = match words {
                    12 => MnemonicType::Words12,
                    15 => MnemonicType::Words15,
                    18 => MnemonicType::Words18,
                    24 => MnemonicType::Words24,
                    _ => unreachable!("unsupported word count"),
                };
                Mnemonic::new(mnemonic_type, Language::English)
            };

            b.iter(|| {
                let _seed = black_box(Seed::new(&mnemonic, ""));
            })
        });

        group.bench_function(format!("bip39 ({} words)", words), |b| {
            use bip39::Mnemonic;

            let mnemonic = Mnemonic::generate(words).unwrap();

            b.iter(|| {
                let _seed = black_box(&mnemonic.to_seed(""));
            })
        });

        group.bench_function(format!("coins-bip39 ({} words)", words), |b| {
            use coins_bip39::{English, Mnemonic};

            let mnemonic = {
                let mut rng = rand::rng();
                <Mnemonic<English>>::from_rng_with_count(&mut rng, words).unwrap()
            };

            b.iter(|| {
                let _key = black_box(mnemonic.to_seed(None).unwrap());
            })
        });

        group.bench_function(format!("bip0039 ({} words)", words), |b| {
            use bip0039::{Count, Mnemonic};

            let mnemonic = {
                let count = match words {
                    12 => Count::Words12,
                    15 => Count::Words15,
                    18 => Count::Words18,
                    24 => Count::Words24,
                    _ => unreachable!("unsupported word count"),
                };
                <Mnemonic>::generate(count)
            };

            b.iter(|| {
                let _seed = black_box(mnemonic.to_seed(""));
            })
        });

        group.finish();
    }
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(50);
    targets =
        bench_generate,
        bench_from_entropy,
        bench_from_phrase,
        bench_from_normalized_phrase,
        bench_to_seed,
);
criterion_main!(benches);
