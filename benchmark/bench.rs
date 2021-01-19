use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_generate(c: &mut Criterion) {
    c.bench_function("tiny-bip39::generate", |b| {
        use tiny_bip39::{Language, Mnemonic, MnemonicType};
        b.iter(|| {
            let _mnemonic = black_box(Mnemonic::new(MnemonicType::Words12, Language::English));
        })
    });
    c.bench_function("bip0039::generate", |b| {
        use bip0039::{Count, Language, Mnemonic};
        b.iter(|| {
            let _mnemonic = black_box(Mnemonic::generate_in(Language::English, Count::Words12));
        })
    });
}

fn bench_from_entropy(c: &mut Criterion) {
    let entropy = [
        0x1a, 0x48, 0x6a, 0x5f, 0xbe, 0x53, 0x63, 0x99, 0x84, 0xcb, 0x64, 0xb0, 0x70, 0x75, 0x5f,
        0x7b,
    ];
    c.bench_function("tiny-bip39::from_entropy", |b| {
        use tiny_bip39::{Language, Mnemonic};
        b.iter(|| {
            let _mnemonic = black_box(Mnemonic::from_entropy(&entropy, Language::English));
        })
    });
    c.bench_function("bip0039::from_entropy", |b| {
        use bip0039::{Language, Mnemonic};
        b.iter(|| {
            let _mnemonic = black_box(Mnemonic::from_entropy_in(Language::English, entropy));
        })
    });
}

fn bench_from_phrase(c: &mut Criterion) {
    let phrase = "bottom drive obey lake curtain smoke basket hold race lonely fit walk";
    c.bench_function("tiny-bip39::from_phrase", |b| {
        use tiny_bip39::{Language, Mnemonic};
        b.iter(|| {
            let _mnemonic = black_box(Mnemonic::from_phrase(phrase, Language::English));
        })
    });
    c.bench_function("bip0039::from_phrase", |b| {
        use bip0039::{Language, Mnemonic};
        b.iter(|| {
            let _mnemonic = black_box(Mnemonic::from_phrase_in(Language::English, phrase));
        })
    });
}

fn bench_to_seed(c: &mut Criterion) {
    c.bench_function("tiny-bip39::to_seed", |b| {
        use tiny_bip39::{Language, Mnemonic, MnemonicType, Seed};
        let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
        b.iter(|| {
            let _seed = black_box(Seed::new(&mnemonic, ""));
        })
    });
    c.bench_function("bip0039::to_seed", |b| {
        use bip0039::{Count, Language, Mnemonic};
        let mnemonic = Mnemonic::generate_in(Language::English, Count::Words12);
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
