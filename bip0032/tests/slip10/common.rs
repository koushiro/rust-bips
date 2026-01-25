#![cfg(feature = "slip10")]
#![allow(dead_code)]

use bip0032::{DerivationPath, ExtendedPrivateKey, HardenedDerivationPath, curve::*, slip10::*};

pub struct Case {
    pub path: &'static str,
    pub fingerprint: &'static str,
    pub chain_code: &'static str,
    pub private: &'static str,
    pub public: &'static str,
}

pub fn decode_hex<const N: usize>(value: &str) -> [u8; N] {
    let bytes = const_hex::decode(value).expect("hex decode failed");
    assert_eq!(bytes.len(), N, "unexpected hex length for {value}");
    let mut out = [0u8; N];
    out.copy_from_slice(&bytes);
    out
}

pub fn assert_hardened_private_case<C>(seed: &str, case: &Case)
where
    C: Slip10HardenedOnlyCurve,
    C::PrivateKey: CurvePrivateKey<Bytes = [u8; 32]>,
    C::PublicKey: CurvePublicKey<Bytes = [u8; 33]>,
{
    let seed = const_hex::decode(seed).expect("seed hex decode failed");
    let master = <ExtendedPrivateKey<C> as Slip10MasterKey>::new_slip10(&seed).unwrap();
    let path = case.path.parse::<HardenedDerivationPath>().unwrap();
    let derived = Slip10HardenedOnlyDerivation::derive_slip10_path(&master, &path).unwrap();

    assert_eq!(derived.parent_fingerprint(), decode_hex(case.fingerprint));
    assert_eq!(derived.chain_code(), decode_hex(case.chain_code));

    let private_bytes = derived.to_bytes();
    assert_eq!(private_bytes.as_ref(), &decode_hex::<32>(case.private));

    let public_bytes = derived.public_key().to_bytes();
    assert_eq!(public_bytes, decode_hex(case.public));
}

pub fn assert_nonhardened_private_case<C>(seed: &str, case: &Case)
where
    C: Slip10NonHardenedCurve,
    C::PrivateKey: CurvePrivateKey<Bytes = [u8; 32]> + TweakableKey,
    C::PublicKey: CurvePublicKey<Bytes = [u8; 33]> + TweakableKey,
{
    let seed = const_hex::decode(seed).expect("seed hex decode failed");
    let master = <ExtendedPrivateKey<C> as Slip10MasterKey>::new_slip10(&seed).unwrap();
    let path = case.path.parse::<DerivationPath>().unwrap();
    let derived = Slip10NonHardenedDerivation::derive_slip10_path(&master, &path).unwrap();

    assert_eq!(derived.parent_fingerprint(), decode_hex(case.fingerprint));
    assert_eq!(derived.chain_code(), decode_hex(case.chain_code));

    let private_bytes = derived.to_bytes();
    assert_eq!(private_bytes.as_ref(), &decode_hex::<32>(case.private));

    let public_bytes = derived.public_key().to_bytes();
    assert_eq!(public_bytes, decode_hex(case.public));
}

pub fn assert_nonhardened_public_case<C>(seed: &str, case: &Case)
where
    C: Slip10NonHardenedCurve,
    C::PrivateKey: CurvePrivateKey<Bytes = [u8; 32]> + TweakableKey,
    C::PublicKey: CurvePublicKey<Bytes = [u8; 33]> + TweakableKey,
{
    let seed = const_hex::decode(seed).expect("seed hex decode failed");
    let master = <ExtendedPrivateKey<C> as Slip10MasterKey>::new_slip10(&seed).unwrap();
    let master_public = master.public_key();
    let path = case.path.parse::<DerivationPath>().unwrap();
    let derived = Slip10NonHardenedDerivation::derive_slip10_path(&master_public, &path).unwrap();

    assert_eq!(derived.parent_fingerprint(), decode_hex(case.fingerprint));
    assert_eq!(derived.chain_code(), decode_hex(case.chain_code));

    let public_bytes = derived.to_bytes();
    assert_eq!(public_bytes, decode_hex(case.public));
}
