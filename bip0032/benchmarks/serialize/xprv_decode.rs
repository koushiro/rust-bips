use std::hint::black_box;

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};

#[path = "../common.rs"]
mod common;
use common::{BenchmarkGroup, random_seed};

fn bench_bitcoin(group: &mut BenchmarkGroup<'_>) {
    use bitcoin::{Network, bip32::Xpriv};

    group.bench_function("bitcoin (secp256k1)", |b| {
        b.iter_batched(
            || {
                let seed = random_seed();
                let xprv = Xpriv::new_master(Network::Bitcoin, &seed).unwrap();
                xprv.to_string()
            },
            |encoded| {
                let xprv = encoded.parse::<Xpriv>().unwrap();
                black_box(xprv);
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_coins_bip32(group: &mut BenchmarkGroup<'_>) {
    use coins_bip32::prelude::{Hint, MainnetEncoder, XKeyEncoder, XPriv};

    group.bench_function("coins-bip32 (k256::ecdsa)", |b| {
        b.iter_batched(
            || {
                let seed = random_seed();
                let xprv = XPriv::root_from_seed(&seed, Some(Hint::Legacy)).unwrap();
                MainnetEncoder::xpriv_to_base58(&xprv).unwrap()
            },
            |encoded| {
                let xprv = MainnetEncoder::xpriv_from_base58(black_box(&encoded)).unwrap();
                black_box(xprv);
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_bip32(group: &mut BenchmarkGroup<'_>) {
    use bip32::{
        ExtendedPrivateKey, Prefix, PrivateKey,
        secp256k1::{self, ecdsa},
    };

    fn bench_impl<P: PrivateKey>(group: &mut BenchmarkGroup<'_>, name: &str) {
        group.bench_function(format!("bip32 ({name})"), |b| {
            b.iter_batched(
                || {
                    let seed = random_seed();
                    let xprv = <ExtendedPrivateKey<P>>::new(&seed).unwrap();
                    xprv.to_string(Prefix::XPRV).to_string()
                },
                |encoded| {
                    let xprv = encoded.parse::<ExtendedPrivateKey<P>>().unwrap();
                    black_box(xprv);
                },
                BatchSize::SmallInput,
            )
        });
    }

    bench_impl::<secp256k1::SecretKey>(group, "k256");
    bench_impl::<ecdsa::SigningKey>(group, "k256::ecdsa");
}

/*
fn bench_bip0032(group: &mut BenchmarkGroup<'_, WallTime>) {
    use bip0032::{ExtendedKey, ExtendedPrivateKey, Version, backend::*};

    fn bench_impl<B: Secp256k1Backend>(group: &mut BenchmarkGroup<'_, WallTime>, name: &str) {
        group.bench_function(format!("bip0032 ({name})"), |b| {
            b.iter_batched(
                || {
                    let seed = random_seed();
                    let xprv = <ExtendedPrivateKey<B>>::new(&seed).unwrap();
                    xprv.encode_with(Version::XPRV).to_string()
                },
                |encoded| {
                    let xprv = encoded
                        .parse::<ExtendedKey>()
                        .unwrap()
                        .into_private::<B>()
                        .unwrap();
                    black_box(xprv);
                },
                BatchSize::SmallInput,
            )
        });
    }

    bench_impl::<K256Backend>(group, "k256");
    bench_impl::<K256EcdsaBackend>(group, "k256::ecdsa");
    bench_impl::<Secp256k1CoreBackend>(group, "secp256k1");
    bench_impl::<Libsecp256k1Backend>(group, "libsecp256k1");
}
*/

fn bench_xprv_decode(c: &mut Criterion) {
    let mut group = c.benchmark_group("xprv_decode");

    bench_bitcoin(&mut group);
    bench_coins_bip32(&mut group);
    bench_bip32(&mut group);
    // bench_bip0032(&mut group);

    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = bench_xprv_decode
);
criterion_main!(benches);
