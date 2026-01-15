use std::hint::black_box;

use criterion::{
    BatchSize, BenchmarkGroup, Criterion, criterion_group, criterion_main, measurement::WallTime,
};

#[path = "../common.rs"]
mod common;
use common::random_seed;

fn bench_bitcoin(group: &mut BenchmarkGroup<'_, WallTime>) {
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
                Xpub::from_priv(&secp, &xprv)
            },
            |xpub| {
                let encoded = xpub.to_string();
                black_box(encoded);
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_coins_bip32(group: &mut BenchmarkGroup<'_, WallTime>) {
    use coins_bip32::prelude::{Hint, MainnetEncoder, XKeyEncoder, XPriv};

    group.bench_function("coins-bip32 (k256::ecdsa)", |b| {
        b.iter_batched(
            || {
                let seed = random_seed();
                let xprv = XPriv::root_from_seed(&seed, Some(Hint::Legacy)).unwrap();
                xprv.verify_key()
            },
            |xpub| {
                let encoded = MainnetEncoder::xpub_to_base58(&xpub).unwrap();
                black_box(encoded);
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_bip32(group: &mut BenchmarkGroup<'_, WallTime>) {
    use bip32::{
        ExtendedPrivateKey, Prefix, PrivateKey,
        secp256k1::{self, ecdsa},
    };

    fn bench_impl<P: PrivateKey>(group: &mut BenchmarkGroup<'_, WallTime>, name: &str) {
        group.bench_function(format!("bip32 ({name})"), |b| {
            b.iter_batched(
                || {
                    let seed = random_seed();
                    let xprv = <ExtendedPrivateKey<P>>::new(&seed).unwrap();
                    xprv.public_key()
                },
                |xpub| {
                    let encoded = xpub.to_string(Prefix::XPUB);
                    black_box(encoded);
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
    use bip0032::{ExtendedPrivateKey, Version, backend::*};

    fn bench_impl<B: Secp256k1Backend>(group: &mut BenchmarkGroup<'_, WallTime>, name: &str) {
        group.bench_function(format!("bip0032 ({name})"), |b| {
            b.iter_batched(
                || {
                    let seed = random_seed();
                    <ExtendedPrivateKey<B>>::new(&seed).unwrap()
                },
                |xprv| {
                    let encoded = xprv.public_key().encode_with(Version::XPUB).to_string();
                    black_box(encoded);
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

fn bench_xpub_encode(c: &mut Criterion) {
    let mut group = c.benchmark_group("xpub_encode");

    bench_bitcoin(&mut group);
    bench_coins_bip32(&mut group);
    bench_bip32(&mut group);
    // bench_bip0032(&mut group);

    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = bench_xpub_encode
);
criterion_main!(benches);
