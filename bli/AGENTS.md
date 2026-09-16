# Repository Guidelines

## Project Structure & Module Organization

- `src/main.rs` is the binary entry point and owns argument parsing, dispatch, diagnostics, and exit status. It embeds `README.md` as the crate documentation.
- `src/commands/` contains the `generate` (`gen`), `inspect` (`info`), and `derive` commands, including their arguments and output schemas.
- `src/args/` contains shared `Input`, `Language`, and `Output` types for input selection, language mapping, and text/JSON rendering.
- Cryptographic operations belong in the sibling `bip0032` and `bip0039` libraries; the CLI handles input validation and presentation.
- Integration tests live in `tests/cli.rs`; CLI CI is defined in `../.github/workflows/bli.yml`.
- `bli/` is a standalone binary package with its own `Cargo.lock`; there is no root Cargo workspace.

## Build, Test, and Development Commands

- Run commands from `bli/`, or add `--manifest-path bli/Cargo.toml` when running Cargo from the repository root.
- `cargo build --locked` builds the CLI.
- `cargo run --locked -- --help` displays command help without installing the binary.
- `cargo doc --bin bli --no-deps --locked` builds the binary crate documentation from `README.md`.
- `cargo +nightly fmt` applies the repository's `rustfmt.toml`, including nightly-only options.
- `cargo fmt -- --check` checks formatting as CI does.
- `cargo clippy --all-targets --locked -- -D warnings` runs Clippy with warnings denied.
- `cargo test --locked` runs unit and CLI integration tests.

## Coding Style & Naming Conventions

- Rust 2024 edition, MSRV 1.85.0 (see `Cargo.toml`).
- Use Rust naming: `snake_case` for functions/modules, `CamelCase` for types, `SCREAMING_SNAKE_CASE` for constants.
- Preserve the crate-level `missing_docs`, `unused_imports`, and `unsafe_code` lints; document public API and keep the crate documentation current.
- Each command declares `-o, --output <FORMAT>` using `Output`, with `text` as the default and `json` as the alternative.
- Render results through `Output::write`; use `Display` for text and `Serialize` for JSON. The shared writer adds the final newline and handles broken pipes.
- Keep argument help, `README.md`, and integration tests aligned when changing CLI behavior. Prefer existing dependencies and library APIs.

## Testing Guidelines

- Tests use published BIP32 and BIP39 vectors; prefer deterministic cases for new behavior.
- Validate generated mnemonics and seeds through library APIs rather than expecting a particular random result.
- Cover affected text/JSON output, short/long options, file/stdin input, and generation-to-derivation behavior when changing commands.
- Preserve exit codes: `0` for success (including broken output pipes), `1` for content/I/O/runtime errors, and `2` for usage errors.
- Run `cargo test --locked` after Rust changes; when modifying a sibling library, also follow its `AGENTS.md` and run its relevant checks.

## Commit & Pull Request Guidelines

- Use conventional messages such as `feat(bli): ...` or `fix(bli): ...` when practical.
- Keep messages short and imperative; mention the affected command or CLI behavior if relevant.
- PRs should include a brief summary, the validation commands run, and a linked issue if applicable.

## Security Notes

- Do not log mnemonic phrases, seeds, passphrases, or private keys in diagnostics or test failure output. Use the published vectors in `tests/cli.rs` for fixed test data.
- Generation JSON intentionally includes the actual passphrase, and derivation from seed/private-key input intentionally outputs private keys; keep these results out of shared logs.
- Reuse test helpers that capture raw process output; avoid assertions that dump secret-bearing output on failure.
- File/stdin input is UTF-8, limited to 64 KiB, and read through EOF. Remove exactly one trailing LF or CRLF and preserve other passphrase whitespace.
- `--passphrase-prompt` reads hidden input twice; `--passphrase-file -` reads stdin through EOF. Generation defaults to an empty passphrase.
- Prefer `Zeroizing` for owned secret buffers and avoid unnecessary copies. Replace errors containing secret input with safe messages without retaining their original cause.

## Release Notes

Generate GitHub Release notes that cover these sections:

- Breaking Changes.
- Bug Fixes.
- New Features.
- Performance & Behavior Improvements.
