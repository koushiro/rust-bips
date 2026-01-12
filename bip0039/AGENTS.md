# Repository Guidelines

## Project Structure & Module Organization

- `src/lib.rs` is the public entry point; core logic lives in `src/mnemonic/` and `src/language/`.
- Errors and shared types are in `src/error.rs`.
- Generated language tables are driven by `build.rs` and the word lists in `words/`.
- Tests and fixtures live in `tests/` (e.g., `tests/vectors.rs`, `tests/test_EN_BIP39.json`).
- Auxiliary tooling: `benchmarks/` for benches and `fuzz/` for fuzz targets.

## Build, Test, and Development Commands

- `cargo build` builds the library with default features.
- `cargo test` runs unit and integration tests in `tests/`.
- `cargo bench` runs benchmarks (see `benchmarks/`).
- `just bench <name> [args]` runs a single benchmark (uses `benchmarks/` working dir).
- `just benches [args]` runs all benchmarks.
- `just fuzz <target> [runs]` runs fuzzing with nightly (`cargo +nightly fuzz`).
- `just fuzz-clean` removes fuzz artifacts (`fuzz/artifacts`, `fuzz/corpus`).

## Coding Style & Naming Conventions

- Rust 2024 edition; default `rustfmt` style (4-space indent).
- Naming: `snake_case` for modules/functions, `CamelCase` for types, `SCREAMING_SNAKE_CASE` for constants.
- Feature flags are defined in `Cargo.toml`; keep additions grouped with existing language features.

## Testing Guidelines

- Tests are in `tests/` and use JSON vectors under `tests/`.
- Prefer deterministic test vectors over randomness when possible.
- Run `cargo test` before submitting changes that touch mnemonic generation or word lists.

## Security Notes

- Do not log or print mnemonic phrases, seeds, or passphrases in tests or examples.
- Keep test data non-sensitive; use the published BIP-39 vectors in `tests/`.
- Prefer `zeroize`-aware types and avoid cloning secret material unnecessarily.

## Release Notes

Generate GitHub Release notes that cover these sections:

- Breaking Changes
- New Features
- Performance & Behavior Improvements.
