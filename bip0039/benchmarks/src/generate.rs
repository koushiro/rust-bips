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
                    let mut rng = rand09::rng();
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

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(50);
    targets = bench_generate,
);
criterion_main!(benches);
