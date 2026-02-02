use std::hint::black_box;

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};

mod common;
use common::{BenchmarkGroup, random_seed};

fn bench_derive_bitcoin(group: &mut BenchmarkGroup<'_>) {
    use bitcoin::{
        Network,
        bip32::{DerivationPath, Xpriv},
        secp256k1::Secp256k1,
    };

    group.bench_function("bitcoin (secp256k1)", |b| {
        let secp = Secp256k1::new();
        let path = "m/0'/1/2'/2/1000000000".parse::<DerivationPath>().unwrap();

        b.iter_batched(
            || {
                let seed = random_seed();
                Xpriv::new_master(Network::Bitcoin, &seed).unwrap()
            },
            |xprv| {
                let key = xprv.derive_priv(black_box(&secp), black_box(&path)).unwrap();
                black_box(key);
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_derive_coins_bip32(group: &mut BenchmarkGroup<'_>) {
    use coins_bip32::prelude::{Hint, XPriv};

    group.bench_function("coins-bip32 (k256::ecdsa)", |b| {
        let path = "m/0'/1/2'/2/1000000000";

        b.iter_batched(
            || {
                let seed = random_seed();
                XPriv::root_from_seed(&seed, Some(Hint::Legacy)).unwrap()
            },
            |xprv| {
                let key = xprv.derive_path(black_box(path)).unwrap();
                black_box(key);
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_derive_bip32(group: &mut BenchmarkGroup<'_>) {
    use bip32::{DerivationPath, ExtendedPrivateKey, PrivateKey, secp256k1};

    fn bench_impl<P: PrivateKey>(group: &mut BenchmarkGroup<'_>, name: &str) {
        group.bench_function(name, |b| {
            let path = "m/0'/1/2'/2/1000000000".parse::<DerivationPath>().unwrap();

            b.iter_batched(
                || {
                    let seed = random_seed();
                    ExtendedPrivateKey::<P>::new(&seed).unwrap()
                },
                |xprv| {
                    let mut key = xprv;
                    for child in path.iter() {
                        key = key.derive_child(child).unwrap();
                    }
                    black_box(key);
                },
                BatchSize::SmallInput,
            )
        });
    }

    bench_impl::<secp256k1::SecretKey>(group, "bip32 (k256)");
    bench_impl::<secp256k1::ecdsa::SigningKey>(group, "bip32 (k256::ecdsa)");
}

fn bench_derive_bip0032(group: &mut BenchmarkGroup<'_>) {
    use bip0032::{DerivationPath, ExtendedPrivateKey, curve::secp256k1::*};

    fn bench_impl<B: Secp256k1Backend>(group: &mut BenchmarkGroup<'_>, name: &str) {
        group.bench_function(name, |b| {
            let path = "m/0'/1/2'/2/1000000000".parse::<DerivationPath>().unwrap();

            b.iter_batched(
                || {
                    let seed = random_seed();
                    ExtendedPrivateKey::<Secp256k1Curve<B>>::new(&seed).unwrap()
                },
                |master| {
                    let key = master.derive_path(black_box(&path)).unwrap();
                    black_box(key);
                },
                BatchSize::SmallInput,
            )
        });
    }

    bench_impl::<K256Backend>(group, "bip0032 (k256)");
    bench_impl::<Secp256k1FfiBackend>(group, "bip0032 (secp256k1)");
}

fn bench_derive(c: &mut Criterion) {
    let mut group = c.benchmark_group("derive");

    bench_derive_bitcoin(&mut group);
    bench_derive_coins_bip32(&mut group);
    bench_derive_bip32(&mut group);
    bench_derive_bip0032(&mut group);

    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = bench_derive
);
criterion_main!(benches);
