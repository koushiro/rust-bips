# bip0039-rs

[![ga-svg]][ga-url]
[![crates-svg]][crates-url]
[![docs-svg]][docs-url]
[![msrv-svg]][msrv-url]
[![codecov-svg]][codecov-url]
[![deps-svg]][deps-url]

[ga-svg]: https://github.com/koushiro/bip0039-rs/workflows/build/badge.svg
[ga-url]: https://github.com/koushiro/bip0039-rs/actions
[crates-svg]: https://img.shields.io/crates/v/bip0039
[crates-url]: https://crates.io/crates/bip0039
[docs-svg]: https://docs.rs/bip0039/badge.svg
[docs-url]: https://docs.rs/bip0039
[msrv-svg]: https://img.shields.io/badge/rustc-1.44+-blue.svg
[msrv-url]: https://blog.rust-lang.org/2020/06/04/Rust-1.44.0.html
[codecov-svg]: https://img.shields.io/codecov/c/github/koushiro/bip0039-rs
[codecov-url]: https://codecov.io/gh/koushiro/bip0039-rs
[deps-svg]: https://deps.rs/repo/github/koushiro/bip0039-rs/status.svg
[deps-url]: https://deps.rs/repo/github/koushiro/bip0039-rs

Another Rust implementation of [BIP-0039](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki) standard.

## Usage

- Add the `bip0039` into the `Cargo.toml`.

  ```toml
  [dependencies]
  bip0039 = "0.7"
  ```

- Generate a random BIP-0039 mnemonic in English.
  ```rust
  use bip0039::{Count, Mnemonic};

  let mnemonic = Mnemonic::generate(Count::Words12);
  let phrase = mnemonic.phrase();
  ```

## Documentation

See documentation and examples at https://docs.rs/bip0039.

## Features

- [x] Support all languages in the [BIP-0039 Word Lists](https://github.com/bitcoin/bips/blob/master/bip-0039/bip-0039-wordlists.md)
  - [x] English
  - [x] Chinese (Simplified)
  - [x] Chinese (Traditional)
  - [x] Czech
  - [x] French
  - [x] Italian
  - [x] Japanese
  - [x] Korean
  - [x] Spanish  
- [x] Support `no_std` environment

## Alternatives

- [bip39](https://github.com/rust-bitcoin/rust-bip39)
- [tiny-bip39](https://github.com/maciejhirsz/tiny-bip39)

## LICENSE

Licensed under either of

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT license](LICENSE-MIT)

at your option.
