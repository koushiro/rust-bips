# bip0032

[![](https://github.com/koushiro/rust-bips/actions/workflows/bip0032.yml/badge.svg)][actions]
[![](https://img.shields.io/docsrs/bip0032)][docs.rs]
[![](https://img.shields.io/crates/v/bip0032)][crates.io]
[![](https://img.shields.io/crates/l/bip0032)][crates.io]
[![](https://img.shields.io/crates/d/bip0032.svg)][crates.io]
[![](https://img.shields.io/badge/MSRV-1.85.0-green?logo=rust)][whatrustisit]

[actions]: https://github.com/koushiro/rust-bips/actions
[docs.rs]: https://docs.rs/bip0032
[crates.io]: https://crates.io/crates/bip0032
[whatrustisit]: https://www.whatrustisit.com

Another Rust implementation of [BIP-0032](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki) standard.

## Usage

Seed material is typically derived from a BIP-0039 mnemonic (for example, via
[bip0039](https://crates.io/crates/bip0039)).

```rust
use bip0039::{Count, English, Mnemonic};

let mnemonic = <Mnemonic<English>>::generate(Count::Words12);
let seed = mnemonic.to_seed("");
```

The examples below assume the `seed` from above.

1. Private parent key -> private child key (supports hardened).

```rust
use bip0032::{DerivationPath, ExtendedPrivateKey, Version, backend::K256Backend};

let master = ExtendedPrivateKey::<K256Backend>::new(&seed).unwrap();
let path: DerivationPath = "m/0H/1".parse().unwrap();
let child = master.derive_path(&path).unwrap();
let xprv = child
    .encode_with(Version::XPRV)
    .unwrap()
    .to_string();
```

2. Private parent key -> public child key.

```rust
use bip0032::{DerivationPath, ExtendedPrivateKey, Version, backend::K256Backend};

let master = ExtendedPrivateKey::<K256Backend>::new(&seed).unwrap();
let path: DerivationPath = "m/0H/1".parse().unwrap();
let child = master.derive_path(&path).unwrap();
let xpub = child
    .public_key()
    .encode_with(Version::XPUB)
    .unwrap()
    .to_string();
```

3. Public parent key -> public child key (non-hardened only).

```rust
use bip0032::{
    DerivationPath, ExtendedKeyPayload, ExtendedPublicKey, Version, backend::K256Backend,
};

let parent_xpub = "xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8";
let payload = parent_xpub.parse::<ExtendedKeyPayload>().unwrap();
let parent = ExtendedPublicKey::<K256Backend>::try_from(payload).unwrap();
let path: DerivationPath = "m/0/1".parse().unwrap();
let child = parent.derive_path(&path).unwrap();
let xpub = child
    .encode_with(Version::XPUB)
    .unwrap()
    .to_string();
```

4. Public parent key -> private child key: impossible (BIP-0032 does not allow it).

## Documentation

See documentation and examples at https://docs.rs/bip0032.

## Features

- [x] Derivation path parsing with hardened suffixes (`'`, `h`, `H`)
- [x] Extended key Base58Check encoding/decoding (xpub/xprv)
- [x] Multiple secp256k1 backends (`k256` default, `k256ecdsa`, `secp256k1`, `libsecp256k1`)
- [x] Support `no_std` environment

## Performance

See [benchmarks](https://github.com/koushiro/rust-bips/blob/main/bip0032/benchmarks/README.md) for more details

## Alternatives

- [bip32](https://github.com/iqlusioninc/crates/tree/main/bip32)
- [coins-bip32](https://github.com/summa-tx/bitcoins-rs/tree/main/bip32)
- [bitcoin](https://github.com/rust-bitcoin/rust-bitcoin/blob/master/bitcoin/src/bip32.rs)

## License

This project is licensed under the Apache License, Version 2.0 - see the [LICENSE](../LICENSE) file for details.
