# Repository Guidelines

## Project Structure & Module Organization

- `src/lib.rs` is the crate entry and re-exports the public API.
- `src/path/` defines derivation paths and child numbers.
- `src/xkey/` contains extended key types and payload/version handling.
- `src/curve/secp256k1/` holds backend-specific secp256k1 implementations (k256, secp256k1, libsecp256k1).
- Tests live in `tests/` (BIP-32 vectors and invalid key cases).
- Benchmarks are a workspace member under `benchmarks/`, with bench targets in `benchmarks/*.rs` and `benchmarks/serialize/`.
- Auxiliary tooling: `benchmarks/` for benches and `fuzz/` for fuzz targets.

## Build, Test, and Development Commands

- `cargo build` builds with default features (`std`, `k256`).
- `cargo build --no-default-features` checks `no_std` compatibility.
- `cargo test` runs unit tests for the enabled backend.
- `cargo test --features secp256k1` runs tests against specific backends.
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
- Prefer unit tests in `src/tests.rs` for API behavior and encoding/decoding edge cases.

## Feature Flags & Backends

- `std` enables standard library support; `k256` is the default backend.
- Optional backends: `secp256k1` and `libsecp256k1` (note: libsecp256k1 is unmaintained).
- When modifying backend code, validate the affected feature set with explicit `--features` flags.

## Commit & Pull Request Guidelines

- Recent commits often follow a conventional style like `feat(bip0032): ...` or `chore(bip0032): ...`; use this format when practical.
- Keep messages short and imperative; mention backend/features if relevant.
- PRs should include a brief summary, the test/bench commands run, and a linked issue if applicable.

## Security Notes

- Do not log or print seeds, private keys, or derived secret material in tests or examples.
- Keep test data non-sensitive; use the published BIP-32 vectors in `src/tests.rs`.
- Prefer `zeroize`-aware types and avoid cloning secret material unnecessarily.

## Release Notes

Generate GitHub Release notes that cover these sections:

- Breaking Changes
- New Features
- Performance & Behavior Improvements.
