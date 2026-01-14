use std::hint::black_box;

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use rand::RngCore;

const WORDS: [usize; 4] = [12, 15, 18, 24];

fn entropy_bytes(words: usize) -> usize {
    match words {
        12 => 16,
        15 => 20,
        18 => 24,
        24 => 32,
        _ => unreachable!("unsupported word count"),
    }
}

fn bench_from_entropy(c: &mut Criterion) {
    for words in WORDS {
        let mut group = c.benchmark_group("from_entropy");

        group.bench_function(format!("tiny-bip39 ({} words)", words), |b| {
            use tiny_bip39::{Language, Mnemonic};

            b.iter_batched(
                || {
                    let mut entropy = vec![0u8; entropy_bytes(words)];
                    rand::rng().fill_bytes(&mut entropy);
                    entropy
                },
                |entropy| {
                    let mnemonic =
                        black_box(Mnemonic::from_entropy(&entropy, Language::English).unwrap());
                    let _phrase = mnemonic.phrase();
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_function(format!("bip39 ({} words)", words), |b| {
            use bip39::Mnemonic;

            b.iter_batched(
                || {
                    let mut entropy = vec![0u8; entropy_bytes(words)];
                    rand::rng().fill_bytes(&mut entropy);
                    entropy
                },
                |entropy| {
                    let _phrase = black_box({
                        let mnemonic = Mnemonic::from_entropy(&entropy).unwrap();
                        mnemonic.to_string()
                    });
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_function(format!("coins-bip39 ({} words)", words), |b| {
            use coins_bip39::{English, Entropy, Mnemonic};

            b.iter_batched(
                || {
                    let mut rng = rand::rng();
                    Entropy::from_rng(entropy_bytes(words), &mut rng).unwrap()
                },
                |entropy| {
                    let _phrase = black_box({
                        let mnemonic = <Mnemonic<English>>::new_from_entropy(entropy);
                        mnemonic.to_phrase()
                    });
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_function(format!("bip0039 ({} words)", words), |b| {
            use bip0039::Mnemonic;

            b.iter_batched(
                || {
                    let mut entropy = vec![0u8; entropy_bytes(words)];
                    rand::rng().fill_bytes(&mut entropy);
                    entropy
                },
                |entropy| {
                    let mnemonic = black_box(<Mnemonic>::from_entropy(entropy.as_slice()).unwrap());
                    let _phrase = mnemonic.phrase();
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
    targets = bench_from_entropy,
);
criterion_main!(benches);
