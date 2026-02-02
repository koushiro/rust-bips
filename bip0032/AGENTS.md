# Repository Guidelines

## Project Structure & Module Organization

- `src/lib.rs` is the crate entry and re-exports the public API.
- `src/path/` defines derivation paths and child numbers.
- `src/xkey/` contains extended key types and payload/version handling.
- `src/curve/secp256k1/` holds backend-specific secp256k1 implementations (k256, secp256k1).
- `src/curve/nist256p1/` holds the NIST P-256 (p256) backend implementation for SLIP-0010.
- `src/curve/ed25519/` holds the ed25519 backend implementation for SLIP-0010.
- `src/curve/slip10.rs` contains SLIP-0010 marker traits.
- `src/xkey/slip10.rs` contains SLIP-0010 derivation implementations and includes docs from `SLIP-0010.md`.
- Tests live in `tests/` (`bip32.rs` for BIP-32 vectors and `slip10/*` for SLIP-0010 vectors).
- `SLIP-0010.md` documents SLIP-0010 usage and feature matrix.
- Benchmarks are a workspace member under `benchmarks/`, with bench targets in `benchmarks/*.rs` and `benchmarks/serialize/`.
- Auxiliary tooling: `benchmarks/` for benches and `fuzz/` for fuzz targets.

## Build, Test, and Development Commands

- `cargo build` builds with default features (`std`, `k256`).
- `cargo build --no-default-features` checks `no_std` compatibility.
- `cargo test` runs unit tests for the enabled backend.
- `cargo test --features k256` runs BIP-0032 tests against specific secp256k1 backends.
- `cargo test --features slip10,p256` runs SLIP-0010 tests for NIST P-256 curve (p256 backend).
- `just bench keygen` or `just benches` runs benchmarks (uses `benchmarks/` as the working dir); equivalent: `cargo bench --bench keygen -- --quiet` from `benchmarks/`.
- `just fuzz <target> [runs]` runs fuzzing with nightly (`cargo +nightly fuzz`).
- `just fuzz-clean` removes fuzz artifacts (`fuzz/artifacts`, `fuzz/corpus`).

## Coding Style & Naming Conventions

- Rust 2024 edition, MSRV 1.85.0 (see `Cargo.toml`).
- Format with `rustfmt` defaults (4-space indentation).
- Use Rust naming: `snake_case` for functions/modules, `CamelCase` for types.
- Public items must be documented (`#![deny(missing_docs)]`), and unused imports are denied.
- Preserve `no_std` friendliness by avoiding unnecessary `std` usage outside the `std` feature.

## Testing Guidelines

- Tests use fixed BIP-32 vectors; add similar deterministic cases for new behavior.
- Run `cargo test` for default features; use `cargo test --all-features` when changing backend-related code.
- Use `tests/bip32.rs` for BIP-0032 vectors and `tests/slip10/*` for SLIP-0010 vectors.

## Feature Flags & Backends

- `std` enables standard library support; `k256` is the default secp256k1 backend.
- Optional backends: `k256` and `secp256k1`.
- SLIP-0010 features: `slip10` (core), `k256`|`secp256k1` (secp256k1), `p256` (nist256p1), `ed25519-dalek` (ed25519).
- When modifying backend code, validate the affected feature set with explicit `--features` flags.

## Commit & Pull Request Guidelines

- Recent commits often follow a conventional style like `feat(bip0032): ...` or `chore(bip0032): ...`; use this format when practical.
- Keep messages short and imperative; mention backend/features if relevant.
- PRs should include a brief summary, the test/bench commands run, and a linked issue if applicable.

## Security Notes

- Do not log or print seeds, private keys, or derived secret material in tests or examples.
- Keep test data non-sensitive; use the published BIP-0032 vectors in `tests/bip32.rs`; use the published SLIP-0010 vectors in `tests/slip10/*`.
- Prefer `zeroize`-aware types and avoid cloning secret material unnecessarily.

## Release Notes

Generate GitHub Release notes that cover these sections:

- Breaking Changes.
- Bug Fixes.
- New Features.
- Performance & Behavior Improvements.
