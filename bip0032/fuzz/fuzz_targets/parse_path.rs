#![no_main]

use arbitrary::Arbitrary;
use bip0032::{ChildNumber, DerivationPath};
use libfuzzer_sys::fuzz_target;

#[derive(Debug, Arbitrary)]
struct Input<'a> {
    path_bytes: &'a [u8],
    mutate_bytes: &'a [u8],
    max_children: u8,
    mutate: bool,
    mutate_ops: u8,
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

fn mutate_path(s: String, bytes: &[u8], ops: usize) -> String {
    if bytes.is_empty() || ops == 0 {
        return s;
    }

    let alphabet = b"m/0123456789'hH";
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

fn assert_child_bounds(path: &DerivationPath) {
    for child in path.children() {
        assert!(child.index() < (1u32 << 31));
    }
}

fuzz_target!(|input: Input<'_>| {
    let max_children = (input.max_children as usize).min(32);
    let path = build_path(input.path_bytes, max_children);

    let canonical = path.to_string();
    let parsed = canonical.parse::<DerivationPath>().unwrap();
    assert_eq!(parsed.to_string(), canonical);
    assert_child_bounds(&parsed);

    for child in parsed.children() {
        let text = child.to_string();
        let parsed_child = text.parse::<ChildNumber>().unwrap();
        assert_eq!(parsed_child.index(), child.index());
        assert_eq!(parsed_child.is_hardened(), child.is_hardened());
    }

    if input.mutate {
        let ops = (input.mutate_ops as usize).min(8);
        let mutated = mutate_path(canonical, input.mutate_bytes, ops);
        if let Ok(parsed_mut) = mutated.parse::<DerivationPath>() {
            assert_child_bounds(&parsed_mut);
            let normalized = parsed_mut.to_string();
            let parsed_again = normalized.parse::<DerivationPath>().unwrap();
            assert_eq!(parsed_again.to_string(), normalized);
        }
    }
});
