use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_generate(c: &mut Criterion) {
    c.bench_function("tiny-bip39::generate", |b| {
        use bip39::{Language, Mnemonic, MnemonicType};
        b.iter(|| {
            let _phrase = black_box({
                let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
                mnemonic.phrase().to_string()
            });
        })
    });
    c.bench_function("coins-bip39::from_rng_with_count", |b| {
        use coins_bip39::{English, Mnemonic};
        b.iter(|| {
            let _phrase = black_box({
                let mut rng = rand::rng();
                let mnemonic = <Mnemonic<English>>::from_rng_with_count(&mut rng, 12).unwrap();
                mnemonic.to_phrase()
            });
        })
    });
    c.bench_function("bip0039::generate", |b| {
        use bip0039::{Count, Mnemonic};
        b.iter(|| {
            let _phrase = black_box({
                let mnemonic = <Mnemonic>::generate(Count::Words12);
                mnemonic.phrase().to_string()
            });
        })
    });
}

fn bench_from_entropy(c: &mut Criterion) {
    let entropy = [
        0x1a, 0x48, 0x6a, 0x5f, 0xbe, 0x53, 0x63, 0x99, 0x84, 0xcb, 0x64, 0xb0, 0x70, 0x75, 0x5f,
        0x7b,
    ];
    c.bench_function("tiny-bip39::from_entropy", |b| {
        use bip39::{Language, Mnemonic};
        b.iter(|| {
            let _mnemonic = black_box(Mnemonic::from_entropy(&entropy, Language::English));
        })
    });
    c.bench_function("bip0039::from_entropy", |b| {
        use bip0039::Mnemonic;
        b.iter(|| {
            let _mnemonic = black_box(<Mnemonic>::from_entropy(entropy));
        })
    });
}

fn bench_from_phrase(c: &mut Criterion) {
    let phrase = "bottom drive obey lake curtain smoke basket hold race lonely fit walk";
    c.bench_function("tiny-bip39::from_phrase", |b| {
        use bip39::{Language, Mnemonic};
        b.iter(|| {
            let _mnemonic = black_box(Mnemonic::from_phrase(phrase, Language::English));
        })
    });
    c.bench_function("coins-bip39::new_from_phrase", |b| {
        use coins_bip39::{English, Mnemonic};
        b.iter(|| {
            let _mnemonic = black_box(<Mnemonic<English>>::new_from_phrase(phrase).unwrap());
        })
    });
    c.bench_function("bip0039::from_phrase", |b| {
        use bip0039::Mnemonic;
        b.iter(|| {
            let _mnemonic = black_box(<Mnemonic>::from_phrase(phrase));
        })
    });
}

fn bench_to_seed(c: &mut Criterion) {
    c.bench_function("tiny-bip39::to_seed", |b| {
        use bip39::{Language, Mnemonic, MnemonicType, Seed};
        let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
        b.iter(|| {
            let _seed = black_box(Seed::new(&mnemonic, ""));
        })
    });
    c.bench_function("coins-bip39::to_seed", |b| {
        use coins_bip39::{English, Mnemonic};
        let mut rng = rand::rng();
        let mnemonic = <Mnemonic<English>>::from_rng_with_count(&mut rng, 12).unwrap();
        b.iter(|| {
            let _key = black_box(mnemonic.master_key(None).unwrap()); // calling `to_seed`
        })
    });
    c.bench_function("bip0039::to_seed", |b| {
        use bip0039::{Count, Mnemonic};
        let mnemonic = <Mnemonic>::generate(Count::Words12);
        b.iter(|| {
            let _seed = black_box(mnemonic.to_seed(""));
        })
    });
}

criterion_group!(
    benches,
    bench_generate,
    bench_from_entropy,
    bench_from_phrase,
    bench_to_seed
);
criterion_main!(benches);
