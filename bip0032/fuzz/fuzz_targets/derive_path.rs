#![no_main]

use arbitrary::Arbitrary;
use bip0032::{
    ChildNumber, DerivationPath, ExtendedKeyPayload, ExtendedPrivateKey, ExtendedPublicKey,
    KnownVersion, curve::secp256k1::*,
};
use libfuzzer_sys::fuzz_target;

#[derive(Debug, Arbitrary)]
struct Input<'a> {
    seed: &'a [u8],
    path_bytes: &'a [u8],
    max_seed_len: u8,
    max_path_len: u8,
}

fn bounded_slice(bytes: &[u8], max_len: usize) -> &[u8] {
    let len = bytes.len().min(max_len);
    &bytes[..len]
}

fn build_path(bytes: &[u8], max_children: usize) -> (DerivationPath, bool) {
    let mut children = Vec::new();
    let mut has_hardened = false;

    for chunk in bytes.chunks(5).take(max_children) {
        if chunk.len() < 5 {
            break;
        }
        let mut index_bytes = [0u8; 4];
        index_bytes.copy_from_slice(&chunk[..4]);
        let mut index = u32::from_le_bytes(index_bytes);
        index &= 0x7FFF_FFFF;
        let hardened = (chunk[4] & 1) == 1;

        if let Ok(child) = ChildNumber::new(index, hardened) {
            has_hardened |= hardened;
            children.push(child);
        }
    }

    (DerivationPath::from(children), has_hardened)
}

type Secp256k1 = Secp256k1Curve<K256Backend>;

fn roundtrip_xprv(key: &ExtendedPrivateKey<Secp256k1>) -> String {
    let encoded = key.encode_with(KnownVersion::Xprv.version()).unwrap().to_string();
    let payload = encoded.parse::<ExtendedKeyPayload>().unwrap();
    let decoded = ExtendedPrivateKey::<Secp256k1>::try_from(payload).unwrap();
    let encoded2 = decoded.encode_with(KnownVersion::Xprv.version()).unwrap().to_string();
    assert_eq!(encoded2, encoded);
    encoded
}

fn roundtrip_xpub(key: &ExtendedPublicKey<Secp256k1>) -> String {
    let encoded = key.encode_with(KnownVersion::Xpub.version()).unwrap().to_string();
    let payload = encoded.parse::<ExtendedKeyPayload>().unwrap();
    let decoded = ExtendedPublicKey::<Secp256k1>::try_from(payload).unwrap();
    let encoded2 = decoded.encode_with(KnownVersion::Xpub.version()).unwrap().to_string();
    assert_eq!(encoded2, encoded);
    encoded
}

fuzz_target!(|input: Input<'_>| {
    let seed_len = (input.max_seed_len as usize).min(64);
    let seed = bounded_slice(input.seed, seed_len);

    let max_children = (input.max_path_len as usize).min(32);
    let (path, has_hardened) = build_path(input.path_bytes, max_children);

    let master = match ExtendedPrivateKey::<Secp256k1>::new(seed) {
        Ok(master) => master,
        Err(_) => return,
    };

    let derived = match master.derive_path(&path) {
        Ok(derived) => derived,
        Err(_) => {
            if !has_hardened {
                let _ = master.public_key().derive_path(&path);
            }
            return;
        },
    };

    let xprv = roundtrip_xprv(&derived);
    let xpub = roundtrip_xpub(&derived.public_key());

    let path_str = path.to_string();
    let parsed_path = path_str.parse::<DerivationPath>().unwrap();
    assert_eq!(parsed_path.to_string(), path_str);

    if !has_hardened {
        let derived_pub = master.public_key().derive_path(&path).unwrap();
        let xpub_from_pub = roundtrip_xpub(&derived_pub);
        assert_eq!(xpub_from_pub, xpub);
    } else {
        assert!(master.public_key().derive_path(&path).is_err());
    }

    let xprv_payload = xprv.parse::<ExtendedKeyPayload>().unwrap();
    assert!(xprv_payload.version().is_private());
    let xpub_payload = xpub.parse::<ExtendedKeyPayload>().unwrap();
    assert!(xpub_payload.version().is_public());
});
