# bip0039

[![](https://github.com/koushiro/rust-bips/actions/workflows/bip0039.yml/badge.svg)][actions]
[![](https://img.shields.io/docsrs/bip0039)][docs.rs]
[![](https://img.shields.io/crates/v/bip0039)][crates.io]
[![](https://img.shields.io/crates/l/bip0039)][crates.io]
[![](https://img.shields.io/crates/d/bip0039.svg)][crates.io]
[![](https://img.shields.io/badge/MSRV-1.85.0-green?logo=rust)][whatrustisit]

[actions]: https://github.com/koushiro/rust-bips/actions
[docs.rs]: https://docs.rs/bip0039
[crates.io]: https://crates.io/crates/bip0039
[whatrustisit]: https://www.whatrustisit.com

Another Rust implementation of [BIP-0039](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki) standard.

## Usage

Generate a random BIP-0039 mnemonic in English.

```rust
use bip0039::{Count, English, Mnemonic};

// Generates an English mnemonic with 12 words randomly
let mnemonic = <Mnemonic<English>>::generate(Count::Words12);
// Or use the default generic type (English) of struct Mnemonic.
let mnemonic = <Mnemonic>::generate(Count::Words12);
// Gets the phrase
let phrase = mnemonic.phrase();
// Generates the HD wallet seed from the mnemonic and the passphrase.
let seed = mnemonic.to_seed("");
```

## Documentation

See documentation and examples at https://docs.rs/bip0039.

## Features

- [x] Support all languages in the [BIP-0039 Word Lists](https://github.com/bitcoin/bips/blob/master/bip-0039/bip-0039-wordlists.md)
  - [x] English
  - [x] Japanese
  - [x] Korean
  - [x] Spanish
  - [x] Chinese (Simplified)
  - [x] Chinese (Traditional)
  - [x] French
  - [x] Italian
  - [x] Czech
  - [x] Portuguese
- [x] Support `no_std` environment

## Alternatives

- [bip39](https://github.com/rust-bitcoin/rust-bip39)
- [tiny-bip39](https://github.com/maciejhirsz/tiny-bip39)
- [coins-bip39](https://github.com/summa-tx/bitcoins-rs/tree/main/bip39)

## License

This project is licensed under the Apache License, Version 2.0 - see the [LICENSE](../LICENSE) file for details.
