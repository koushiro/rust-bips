use std::hint::black_box;

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};

#[path = "../common.rs"]
mod common;
use common::{BenchmarkGroup, random_seed};

fn bench_bitcoin(group: &mut BenchmarkGroup<'_>) {
    use bitcoin::{
        Network,
        bip32::{Xpriv, Xpub},
        secp256k1::Secp256k1,
    };

    group.bench_function("bitcoin (secp256k1)", |b| {
        b.iter_batched(
            || {
                let seed = random_seed();
                let xprv = Xpriv::new_master(Network::Bitcoin, &seed).unwrap();
                let secp = Secp256k1::new();
                let xpub = Xpub::from_priv(&secp, &xprv);
                xpub.to_string()
            },
            |encoded| {
                let xpub = encoded.parse::<Xpub>().unwrap();
                black_box(xpub);
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
                let xpub = xprv.verify_key();
                MainnetEncoder::xpub_to_base58(&xpub).unwrap()
            },
            |encoded| {
                let xpub = MainnetEncoder::xpub_from_base58(black_box(&encoded)).unwrap();
                black_box(xpub);
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_bip32(group: &mut BenchmarkGroup<'_>) {
    use bip32::{ExtendedPrivateKey, ExtendedPublicKey, Prefix, PrivateKey, PublicKey, secp256k1};

    fn bench_impl<Prv: PrivateKey, Pub: PublicKey>(group: &mut BenchmarkGroup<'_>, name: &str) {
        group.bench_function(format!("bip32 ({name})"), |b| {
            b.iter_batched(
                || {
                    let seed = random_seed();
                    let xprv = <ExtendedPrivateKey<Prv>>::new(&seed).unwrap();
                    let xpub = xprv.public_key();
                    xpub.to_string(Prefix::XPUB)
                },
                |encoded| {
                    let xpub = encoded.parse::<ExtendedPublicKey<Pub>>().unwrap();
                    black_box(xpub);
                },
                BatchSize::SmallInput,
            )
        });
    }

    bench_impl::<secp256k1::SecretKey, secp256k1::PublicKey>(group, "k256");
    bench_impl::<secp256k1::ecdsa::SigningKey, secp256k1::ecdsa::VerifyingKey>(
        group,
        "k256::ecdsa",
    );
}

fn bench_bip0032(group: &mut BenchmarkGroup<'_>) {
    use bip0032::{
        ExtendedKeyPayload, ExtendedPrivateKey, ExtendedPublicKey, Version, curve::secp256k1::*,
    };

    fn bench_impl<B: Secp256k1Backend>(group: &mut BenchmarkGroup<'_>, name: &str) {
        group.bench_function(format!("bip0032 ({name})"), |b| {
            b.iter_batched(
                || {
                    let seed = random_seed();
                    let master = <ExtendedPrivateKey<Secp256k1Curve<B>>>::new(&seed).unwrap();
                    let version = Version::XPUB;
                    master.public_key().encode_with_unchecked(version).to_string()
                },
                |encoded| {
                    let payload = encoded.parse::<ExtendedKeyPayload>().unwrap();
                    let xpub = ExtendedPublicKey::<Secp256k1Curve<B>>::try_from(payload).unwrap();
                    black_box(xpub);
                },
                BatchSize::SmallInput,
            )
        });
    }

    bench_impl::<K256Backend>(group, "k256");
    bench_impl::<Secp256k1FfiBackend>(group, "secp256k1");
    bench_impl::<Libsecp256k1Backend>(group, "libsecp256k1");
}

fn bench_xpub_decode(c: &mut Criterion) {
    let mut group = c.benchmark_group("xpub_decode");

    bench_bitcoin(&mut group);
    bench_coins_bip32(&mut group);
    bench_bip32(&mut group);
    bench_bip0032(&mut group);

    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = bench_xpub_decode
);
criterion_main!(benches);
