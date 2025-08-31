use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_generate(c: &mut Criterion) {
    let mut group = c.benchmark_group("generate");
    group.bench_function("tiny-bip39", |b| {
        use bip39::{Language, Mnemonic, MnemonicType};
        b.iter(|| {
            let _phrase = black_box({
                let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
                mnemonic.phrase().to_string()
            });
        });
    });
    group.bench_function("coins-bip39", |b| {
        use coins_bip39::{English, Mnemonic};
        b.iter(|| {
            let _phrase = black_box({
                let mut rng = rand::rng();
                let mnemonic = <Mnemonic<English>>::from_rng(&mut rng);
                mnemonic.to_phrase()
            });
        });
    });
    group.bench_function("bip0039", |b| {
        use bip0039::{Count, Mnemonic};
        b.iter(|| {
            let _phrase = black_box({
                let mnemonic = <Mnemonic>::generate(Count::Words12);
                mnemonic.phrase().to_string()
            });
        });
    });
    group.finish();
}

fn bench_from_entropy(c: &mut Criterion) {
    let entropy = [
        0x1a, 0x48, 0x6a, 0x5f, 0xbe, 0x53, 0x63, 0x99, 0x84, 0xcb, 0x64, 0xb0, 0x70, 0x75, 0x5f,
        0x7b,
    ];

    let mut group = c.benchmark_group("from_entropy");
    group.bench_function("tiny-bip39", |b| {
        use bip39::{Language, Mnemonic};
        b.iter(|| {
            let _mnemonic = black_box(Mnemonic::from_entropy(&entropy, Language::English).unwrap());
        });
    });
    group.bench_function("coins-bip39", |b| {
        use coins_bip39::{English, Entropy, Mnemonic};
        let entropy = Entropy::Sixteen(entropy);
        b.iter(|| {
            let _mnemonic = black_box(<Mnemonic<English>>::new_from_entropy(entropy));
        });
    });
    group.bench_function("bip0039", |b| {
        use bip0039::Mnemonic;
        b.iter(|| {
            let _mnemonic = black_box(<Mnemonic>::from_entropy(entropy).unwrap());
        });
    });
    group.finish();
}

fn bench_from_phrase(c: &mut Criterion) {
    let phrase = "bottom drive obey lake curtain smoke basket hold race lonely fit walk";

    let mut group = c.benchmark_group("from_phrase");
    group.bench_function("tiny-bip39", |b| {
        use bip39::{Language, Mnemonic};
        b.iter(|| {
            let _mnemonic = black_box(Mnemonic::from_phrase(phrase, Language::English).unwrap());
        });
    });
    group.bench_function("coins-bip39", |b| {
        use coins_bip39::{English, Mnemonic};
        b.iter(|| {
            let _mnemonic = black_box(<Mnemonic<English>>::new_from_phrase(phrase).unwrap());
        });
    });
    group.bench_function("bip0039", |b| {
        use bip0039::Mnemonic;
        b.iter(|| {
            let _mnemonic = black_box(<Mnemonic>::from_phrase(phrase).unwrap());
        });
    });
    group.finish();
}

fn bench_to_seed(c: &mut Criterion) {
    let mut group = c.benchmark_group("to_seed");
    group.bench_function("tiny-bip39", |b| {
        use bip39::{Language, Mnemonic, MnemonicType, Seed};
        let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
        b.iter(|| {
            let _seed = black_box(Seed::new(&mnemonic, ""));
        })
    });
    group.bench_function("coins-bip39", |b| {
        use coins_bip39::{English, Mnemonic};
        let mut rng = rand::rng();
        let mnemonic = <Mnemonic<English>>::from_rng_with_count(&mut rng, 12).unwrap();
        b.iter(|| {
            let _key = black_box(mnemonic.to_seed(None).unwrap());
        })
    });
    group.bench_function("bip0039", |b| {
        use bip0039::{Count, Mnemonic};
        let mnemonic = <Mnemonic>::generate(Count::Words12);
        b.iter(|| {
            let _seed = black_box(mnemonic.to_seed(""));
        })
    });
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(70);
    targets = bench_generate, bench_from_entropy, bench_from_phrase, bench_to_seed
);
criterion_main!(benches);
