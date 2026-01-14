use std::hint::black_box;

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};

const WORDS: [usize; 4] = [12, 15, 18, 24];

fn bench_from_phrase(c: &mut Criterion) {
    for words in WORDS {
        let mut group = c.benchmark_group("from_phrase");

        group.bench_function(format!("tiny-bip39 ({} words)", words), |b| {
            use tiny_bip39::{Language, Mnemonic, MnemonicType};

            b.iter_batched(
                || {
                    let mnemonic_type = match words {
                        12 => MnemonicType::Words12,
                        15 => MnemonicType::Words15,
                        18 => MnemonicType::Words18,
                        24 => MnemonicType::Words24,
                        _ => unreachable!("unsupported word count"),
                    };
                    let m = Mnemonic::new(mnemonic_type, Language::English);
                    m.phrase().to_owned()
                },
                |phrase| {
                    let _mnemonic =
                        black_box(Mnemonic::from_phrase(&phrase, Language::English).unwrap());
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_function(format!("bip39 ({} words)", words), |b| {
            use bip39::{Language, Mnemonic};

            b.iter_batched(
                || {
                    let m = Mnemonic::generate(words).unwrap();
                    m.to_string()
                },
                |phrase| {
                    let mnemonic =
                        black_box(Mnemonic::parse_in(Language::English, &phrase).unwrap());
                    let _entropy = mnemonic.to_entropy_array();
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_function(format!("coins-bip39 ({} words)", words), |b| {
            use coins_bip39::{English, Mnemonic};

            b.iter_batched(
                || {
                    let mut rng = rand::rng();
                    let m = <Mnemonic<English>>::from_rng_with_count(&mut rng, words).unwrap();
                    m.to_phrase()
                },
                |phrase| {
                    let _mnemonic =
                        black_box(<Mnemonic<English>>::new_from_phrase(&phrase).unwrap());
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_function(format!("bip0039 ({} words)", words), |b| {
            use bip0039::{Count, Mnemonic};

            b.iter_batched(
                || {
                    let count = match words {
                        12 => Count::Words12,
                        15 => Count::Words15,
                        18 => Count::Words18,
                        24 => Count::Words24,
                        _ => unreachable!("unsupported word count"),
                    };
                    let m = <Mnemonic>::generate(count);
                    m.phrase().to_owned()
                },
                |phrase| {
                    let _mnemonic = black_box(<Mnemonic>::from_phrase(phrase).unwrap());
                },
                BatchSize::SmallInput,
            );
        });

        group.finish();
    }
}

fn bench_from_normalized_phrase(c: &mut Criterion) {
    for words in WORDS {
        let mut group = c.benchmark_group("from_normalized_phrase");

        group.bench_function(format!("bip39 ({} words)", words), |b| {
            use bip39::{Language, Mnemonic};

            b.iter_batched(
                || {
                    let m = Mnemonic::generate(words).unwrap();
                    m.to_string()
                },
                |phrase| {
                    let _mnemonic = black_box(
                        Mnemonic::parse_in_normalized(Language::English, &phrase).unwrap(),
                    );
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_function(format!("bip0039 ({} words)", words), |b| {
            use bip0039::{Count, Mnemonic};

            b.iter_batched(
                || {
                    let count = match words {
                        12 => Count::Words12,
                        15 => Count::Words15,
                        18 => Count::Words18,
                        24 => Count::Words24,
                        _ => unreachable!("unsupported word count"),
                    };
                    let m = <Mnemonic>::generate(count);
                    m.phrase().to_owned()
                },
                |phrase| {
                    let _mnemonic = black_box(<Mnemonic>::from_normalized_phrase(&phrase).unwrap());
                },
                BatchSize::SmallInput,
            );
        });

        group.finish();
    }
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(50);
    targets = bench_from_phrase, bench_from_normalized_phrase,
);
criterion_main!(benches);
