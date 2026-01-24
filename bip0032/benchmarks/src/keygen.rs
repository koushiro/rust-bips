use std::hint::black_box;

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};

mod common;
use common::{BenchmarkGroup, random_seed};

fn bench_keygen_bitcoin(group: &mut BenchmarkGroup<'_>) {
    use bitcoin::{Network, bip32::Xpriv};

    group.bench_function("bitcoin (secp256k1)", |b| {
        b.iter_batched(
            random_seed,
            |seed| {
                let xprv = Xpriv::new_master(Network::Bitcoin, black_box(&seed)).unwrap();
                black_box(xprv);
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_keygen_coins_bip32(group: &mut BenchmarkGroup<'_>) {
    use coins_bip32::prelude::{Hint, XPriv};

    group.bench_function("coins-bip32 (k256::ecdsa)", |b| {
        b.iter_batched(
            random_seed,
            |seed| {
                let xprv = XPriv::root_from_seed(black_box(&seed), Some(Hint::Legacy)).unwrap();
                black_box(xprv);
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_keygen_bip32(group: &mut BenchmarkGroup<'_>) {
    use bip32::{ExtendedPrivateKey, PrivateKey, secp256k1};

    fn bench_impl<P: PrivateKey>(group: &mut BenchmarkGroup<'_>, name: &str) {
        group.bench_function(name, |b| {
            b.iter_batched(
                random_seed,
                |seed| {
                    let xprv = <ExtendedPrivateKey<P>>::new(seed).unwrap();
                    black_box(xprv);
                },
                BatchSize::SmallInput,
            )
        });
    }

    bench_impl::<secp256k1::SecretKey>(group, "bip32 (k256)");
    bench_impl::<secp256k1::ecdsa::SigningKey>(group, "bip32 (k256::ecdsa)");
}

fn bench_keygen_bip0032(group: &mut BenchmarkGroup<'_>) {
    use bip0032::{ExtendedPrivateKey, curve::secp256k1::*};

    fn bench_impl<B: Secp256k1Backend>(group: &mut BenchmarkGroup<'_>, name: &str) {
        group.bench_function(name, |b| {
            b.iter_batched(
                random_seed,
                |seed| {
                    let key = <ExtendedPrivateKey<Secp256k1Curve<B>>>::new(&seed).unwrap();
                    black_box(key);
                },
                BatchSize::SmallInput,
            )
        });
    }

    bench_impl::<K256Backend>(group, "bip0032 (k256)");
    bench_impl::<Secp256k1FfiBackend>(group, "bip0032 (secp256k1)");
    bench_impl::<Libsecp256k1Backend>(group, "bip0032 (libsecp256k1)");
}

fn bench_keygen(c: &mut Criterion) {
    let mut group = c.benchmark_group("keygen");

    bench_keygen_bitcoin(&mut group);
    bench_keygen_coins_bip32(&mut group);
    bench_keygen_bip32(&mut group);
    bench_keygen_bip0032(&mut group);

    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = bench_keygen
);
criterion_main!(benches);
