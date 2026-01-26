#![no_main]

use arbitrary::Arbitrary;
use bip0032::{
    ChildNumber, DerivationPath, ExtendedKeyPayload, ExtendedPrivateKey, ExtendedPublicKey,
    KnownVersion, Version, curve::secp256k1::*,
};
use libfuzzer_sys::fuzz_target;

#[derive(Debug, Arbitrary)]
struct Input<'a> {
    seed: &'a [u8],
    path_bytes: &'a [u8],
    mutate_bytes: &'a [u8],
    max_seed_len: u8,
    max_children: u8,
    version_selector: u8,
    mutate_payload: bool,
    mutate_encoded: bool,
    mutate_ops: u8,
}

fn bounded_slice(bytes: &[u8], max_len: usize) -> &[u8] {
    let len = bytes.len().min(max_len);
    &bytes[..len]
}

fn build_path(bytes: &[u8], max_children: usize) -> DerivationPath {
    let mut children = Vec::new();

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
            children.push(child);
        }
    }

    DerivationPath::from(children)
}

fn pick_version(selector: u8, want_private: bool) -> Version {
    const PUB: [KnownVersion; 10] = [
        KnownVersion::Xpub,
        KnownVersion::Tpub,
        KnownVersion::Ypub,
        KnownVersion::Zpub,
        KnownVersion::Upub,
        KnownVersion::Vpub,
        KnownVersion::YpubShWsh,
        KnownVersion::ZpubWsh,
        KnownVersion::UpubShWsh,
        KnownVersion::VpubWsh,
    ];
    const PRV: [KnownVersion; 10] = [
        KnownVersion::Xprv,
        KnownVersion::Tprv,
        KnownVersion::Yprv,
        KnownVersion::Zprv,
        KnownVersion::Uprv,
        KnownVersion::Vprv,
        KnownVersion::YprvShWsh,
        KnownVersion::ZprvWsh,
        KnownVersion::UprvShWsh,
        KnownVersion::VprvWsh,
    ];

    let idx = (selector as usize) % PUB.len();
    if want_private { PRV[idx].into_version() } else { PUB[idx].into_version() }
}

fn mutate_payload_bytes(payload: &mut [u8], bytes: &[u8], ops: usize) {
    if payload.is_empty() || bytes.is_empty() || ops == 0 {
        return;
    }
    let len = bytes.len();
    for i in 0..ops {
        let pos = (bytes[i % len] as usize) % payload.len();
        let delta = bytes[(i + 1) % len];
        payload[pos] ^= delta;
    }
}

fn mutate_base58(s: String, bytes: &[u8], ops: usize) -> String {
    if bytes.is_empty() || ops == 0 {
        return s;
    }

    let alphabet = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    let mut buf = s.into_bytes();
    let len = bytes.len();

    for i in 0..ops {
        let b = bytes[i % len];
        let action = b % 3;
        let pos = if buf.is_empty() { 0 } else { (bytes[(i + 1) % len] as usize) % buf.len() };
        let ch = alphabet[(bytes[(i + 2) % len] as usize) % alphabet.len()];

        match action {
            0 => {
                let insert_pos = if buf.is_empty() { 0 } else { pos.min(buf.len()) };
                buf.insert(insert_pos, ch);
            },
            1 => {
                if !buf.is_empty() {
                    buf[pos] = ch;
                }
            },
            _ => {
                if !buf.is_empty() {
                    buf.remove(pos);
                }
            },
        }
    }

    String::from_utf8_lossy(&buf).into_owned()
}

fn decode_payload(encoded: &str) -> Option<Vec<u8>> {
    let mut data = vec![0u8; 82];
    let len = bs58::decode(encoded).with_check(None).onto(&mut data).ok()?;
    if len != 78 {
        return None;
    }
    data.truncate(len);
    Some(data)
}

fn encode_payload(data: &[u8]) -> Option<String> {
    let mut out = String::new();
    bs58::encode(data).with_check().onto(&mut out).ok()?;
    Some(out)
}

type Secp256k1 = Secp256k1Curve<K256Backend>;

fuzz_target!(|input: Input<'_>| {
    let seed_len = (input.max_seed_len as usize).min(64);
    let seed = bounded_slice(input.seed, seed_len);

    let max_children = (input.max_children as usize).min(32);
    let path = build_path(input.path_bytes, max_children);

    let master = match ExtendedPrivateKey::<Secp256k1>::new(seed) {
        Ok(master) => master,
        Err(_) => return,
    };

    let derived = match master.derive_path(&path) {
        Ok(derived) => derived,
        Err(_) => return,
    };

    let want_private = (input.version_selector & 1) == 0;
    let version = pick_version(input.version_selector, want_private);

    let encoded = if want_private {
        derived.encode_with(version).unwrap().to_string()
    } else {
        derived.public_key().encode_with(version).unwrap().to_string()
    };

    let payload = encoded.parse::<ExtendedKeyPayload>().unwrap();
    let payload_version = payload.version();
    let encoded2 = payload.to_string();
    let payload2 = encoded2.parse::<ExtendedKeyPayload>().unwrap();
    assert_eq!(payload2.version(), payload_version);

    if payload2.version().is_private() {
        if let Ok(key) = ExtendedPrivateKey::<Secp256k1>::try_from(payload2) {
            let encoded_key = key.encode_with(version).unwrap().to_string();
            let payload3 = encoded_key.parse::<ExtendedKeyPayload>().unwrap();
            assert!(payload3.version().is_private());
        }
    } else if payload2.version().is_public() {
        if let Ok(key) = ExtendedPublicKey::<Secp256k1>::try_from(payload2) {
            let encoded_key = key.encode_with(version).unwrap().to_string();
            let payload3 = encoded_key.parse::<ExtendedKeyPayload>().unwrap();
            assert!(payload3.version().is_public());
        }
    }

    let ops = (input.mutate_ops as usize).min(8);

    if input.mutate_payload {
        if let Some(mut payload_bytes) = decode_payload(&encoded) {
            mutate_payload_bytes(&mut payload_bytes, input.mutate_bytes, ops);
            if let Some(mutated) = encode_payload(&payload_bytes) {
                let _ = mutated.parse::<ExtendedKeyPayload>();
            }
        }
    }

    if input.mutate_encoded {
        let mutated = mutate_base58(encoded, input.mutate_bytes, ops);
        let _ = mutated.parse::<ExtendedKeyPayload>();
    }
});
