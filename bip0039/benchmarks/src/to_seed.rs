use std::hint::black_box;

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};

const WORDS: [usize; 4] = [12, 15, 18, 24];

fn bench_to_seed(c: &mut Criterion) {
    for words in WORDS {
        let mut group = c.benchmark_group("to_seed");

        group.bench_function(format!("tiny-bip39 ({} words)", words), |b| {
            use tiny_bip39::{Language, Mnemonic, MnemonicType, Seed};

            b.iter_batched(
                || {
                    let mnemonic_type = match words {
                        12 => MnemonicType::Words12,
                        15 => MnemonicType::Words15,
                        18 => MnemonicType::Words18,
                        24 => MnemonicType::Words24,
                        _ => unreachable!("unsupported word count"),
                    };
                    Mnemonic::new(mnemonic_type, Language::English)
                },
                |mnemonic| {
                    let _seed = black_box(Seed::new(&mnemonic, ""));
                },
                BatchSize::SmallInput,
            )
        });

        group.bench_function(format!("bip39 ({} words)", words), |b| {
            use bip39::Mnemonic;

            b.iter_batched(
                || Mnemonic::generate(words).unwrap(),
                |mnemonic| {
                    let _seed = black_box(mnemonic.to_seed(""));
                },
                BatchSize::SmallInput,
            )
        });

        group.bench_function(format!("coins-bip39 ({} words)", words), |b| {
            use coins_bip39::{English, Mnemonic};

            b.iter_batched(
                || {
                    let mut rng = rand::rng();
                    <Mnemonic<English>>::from_rng_with_count(&mut rng, words).unwrap()
                },
                |mnemonic| {
                    let _key = black_box(mnemonic.to_seed(None).unwrap());
                },
                BatchSize::SmallInput,
            )
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
                    <Mnemonic>::generate(count)
                },
                |mnemonic| {
                    let _seed = black_box(mnemonic.to_seed(""));
                },
                BatchSize::SmallInput,
            )
        });

        group.finish();
    }
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(50);
    targets = bench_to_seed,
);
criterion_main!(benches);
